use std::collections::VecDeque;
use std::sync::Arc;

use crate::api::bitstream::{AnnexBSplitter, AvcBitstreamFormat, AvcSplitter, H264Config};
use crate::api::callbacks::{DecoderError, VideoDecoderCallbacks};
use crate::api::config::DecoderConfig;
use crate::api::decoder::{ControlCmd, FlushMode, VideoDecoder};
use crate::api::format::StreamFormat;
use crate::api::frame::VideoFrameAllocator;
use crate::api::packet::{DecodedPicture, EncodedPacket};
use crate::api::DefaultAllocator;
use crate::h264::decoder::{Decoder as H264InnerDecoder, Picture};
use crate::h264::frame::PublishedFrame;

/// Default output-queue ceiling. Picked generously: real streams emit
/// at most ~16 pictures from a single `decode()` call (DPB capacity).
const DEFAULT_QUEUE_DEPTH: usize = 64;

pub struct H264VideoDecoder {
    inner: H264InnerDecoder,
    callbacks: Arc<dyn VideoDecoderCallbacks>,
    bitstream_format: AvcBitstreamFormat,
    avc_length_size: usize,
    /// Output pictures waiting for `get_picture`.
    out_queue: VecDeque<DecodedPicture>,
    /// Last format announced to the caller. None until the first SPS
    /// or first picture; used to detect format changes.
    last_format: Option<StreamFormat>,
    max_queue_depth: usize,
}

impl H264VideoDecoder {
    pub fn new(
        config: DecoderConfig,
        allocator: Arc<dyn VideoFrameAllocator>,
        callbacks: Arc<dyn VideoDecoderCallbacks>,
    ) -> Result<Self, DecoderError> {
        // Pick bitstream framing from H264Config in custom_params, or
        // default to AnnexB.
        let (bitstream_format, avc_length_size) = if let Some(params) = config.custom_params {
            let h264_cfg = params.downcast::<H264Config>().map_err(|_| {
                DecoderError::InitializationFailed("custom_params is not H264Config".into())
            })?;
            (h264_cfg.bitstream_format, 4)
        } else {
            (AvcBitstreamFormat::AnnexB, 4)
        };

        Ok(Self {
            inner: H264InnerDecoder::with_allocator(allocator),
            callbacks,
            bitstream_format,
            avc_length_size,
            out_queue: VecDeque::new(),
            last_format: None,
            max_queue_depth: DEFAULT_QUEUE_DEPTH,
        })
    }

    /// Drain the inner decoder's output, wrap each picture in a
    /// `DecodedPicture`, and push onto the API output queue. Fires
    /// `on_format_changed` when the format differs from the last
    /// observed one, and `on_picture_available` if anything landed.
    fn drain_inner(&mut self) -> Result<(), DecoderError> {
        let mut emitted = false;
        while let Some(pic) = self.inner.take_picture() {
            if self.out_queue.len() >= self.max_queue_depth {
                // Put it back at the head and signal full. The inner
                // decoder no longer holds it, so we keep it in the queue
                // for next time.
                self.out_queue.push_back(picture_to_decoded(pic));
                if emitted {
                    self.callbacks.on_picture_available();
                }
                return Err(DecoderError::QueueFull);
            }
            if self.last_format.as_ref() != Some(&pic.format) {
                self.callbacks.on_format_changed(pic.format.clone());
                self.last_format = Some(pic.format.clone());
            }
            self.out_queue.push_back(picture_to_decoded(pic));
            emitted = true;
        }
        if emitted {
            self.callbacks.on_picture_available();
        }
        Ok(())
    }
}

fn picture_to_decoded(pic: Picture) -> DecodedPicture {
    let format = pic.format.clone();
    let opaque = pic.opaque;
    let frame: Arc<dyn crate::api::VideoFrame> = Arc::new(PublishedFrame::new(pic.frame));
    DecodedPicture { frame, format, opaque }
}

impl VideoDecoder for H264VideoDecoder {
    fn decode(&mut self, packet: EncodedPacket) -> Result<(), DecoderError> {
        let EncodedPacket { data, opaque } = packet;
        // Attach opaque to the next primary coded picture.
        if let Some(op) = opaque {
            self.inner.set_pending_opaque(op);
        }

        let bytes: &[u8] = (*data).as_ref();
        match self.bitstream_format {
            AvcBitstreamFormat::AnnexB => {
                for nal in AnnexBSplitter::new(bytes) {
                    if nal.is_empty() {
                        continue;
                    }
                    self.inner.decode_nal(nal)?;
                }
            }
            AvcBitstreamFormat::Avc => {
                for nal in AvcSplitter::new(bytes, self.avc_length_size) {
                    let nal = nal.map_err(|e| DecoderError::BitstreamCorrupted(e.into()))?;
                    if nal.is_empty() {
                        continue;
                    }
                    self.inner.decode_nal(nal)?;
                }
            }
        }
        self.drain_inner()
    }

    fn get_picture(&mut self) -> Result<Option<DecodedPicture>, DecoderError> {
        Ok(self.out_queue.pop_front())
    }

    fn flush(&mut self, mode: FlushMode) -> Result<(), DecoderError> {
        match mode {
            FlushMode::Drain => {
                self.inner.finalize_and_drain()?;
                self.drain_inner()?;
            }
            FlushMode::Discard => {
                // Build a fresh inner decoder over the same allocator
                // and reset queues. Faster than running DPB output
                // bumps for seek-style flushes.
                let allocator = self.inner.allocator();
                self.inner = H264InnerDecoder::with_allocator(allocator);
                self.out_queue.clear();
            }
        }
        Ok(())
    }

    fn control(&mut self, _cmd: &mut ControlCmd) -> Result<(), DecoderError> {
        Err(DecoderError::FeatureNotSupported(
            "H264VideoDecoder defines no control commands".into(),
        ))
    }
}
