use std::any::Any;
use std::sync::Arc;

use super::callbacks::{DecoderError, VideoDecoderCallbacks};
use super::config::{Codec, DecoderConfig};
use super::frame::VideoFrameAllocator;
use super::packet::{DecodedPicture, EncodedPacket};

/// Modes for flushing the decoder pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlushMode {
    /// Fast discard: instantly clears input/output queues and DPB.
    /// In-flight thread work is allowed to finish but its results are
    /// discarded. Used immediately when seeking in a video player.
    Discard,

    /// Drain pipeline: forces DPB to release all remaining frames to
    /// the output queue. Does NOT stop the decoder from accepting new
    /// inputs afterwards. Used at End of Stream (EOS) or sequence
    /// boundaries.
    Drain,
}

/// Codec-specific control payload. Concrete decoders define their own
/// command structs (e.g. `Vp8SetReference`, `H264GetLastQuantizer`)
/// and downcast to them.
pub type ControlCmd = dyn Any;

/// Codec-agnostic software video decoder interface.
pub trait VideoDecoder: Send {
    /// Submit an encoded packet to the decoder's input queue.
    /// Non-blocking in async mode.
    fn decode(&mut self, packet: EncodedPacket) -> Result<(), DecoderError>;

    /// Pull the next decoded picture from the output queue, or
    /// `Ok(None)` if the queue is empty.
    ///
    /// One `decode()` can yield zero or several pictures: B-frame
    /// reordering holds pictures back until their display order is
    /// resolved, then releases them in a batch. Pictures are emitted
    /// in display order. Callers should drain after each `decode()`.
    ///
    /// The queue holds `Arc<dyn VideoFrame>` clones, so undrained
    /// pictures keep their frame buffers alive in the user's
    /// allocator. A caller that stops draining will eventually see
    /// `QueueFull` from `decode()`.
    fn get_picture(&mut self) -> Result<Option<DecodedPicture>, DecoderError>;

    /// Flushes the decoder pipeline according to the specified `FlushMode`.
    fn flush(&mut self, mode: FlushMode) -> Result<(), DecoderError>;

    /// Dispatch a codec-specific command. The payload is downcast by
    /// the concrete decoder; unknown payload types should return
    /// `DecoderError::FeatureNotSupported`. Outputs are written back
    /// through `&mut` fields on the payload.
    fn control(&mut self, cmd: &mut ControlCmd) -> Result<(), DecoderError>;
}

/// The primary entry point: instantiate a software video decoder for
/// the codec requested in `config`. Concrete implementations live in
/// codec-specific submodules; this dispatches on `config.codec`.
pub fn create_decoder(
    config: DecoderConfig,
    allocator: Arc<dyn VideoFrameAllocator>,
    callback: Arc<dyn VideoDecoderCallbacks>,
) -> Result<Box<dyn VideoDecoder>, DecoderError> {
    match config.codec {
        Codec::H264 => Ok(Box::new(crate::h264::decoder::Decoder::new(
            config, allocator, callback,
        )?)),
        other => Err(DecoderError::FeatureNotSupported(format!(
            "codec {other:?} is not built in"
        ))),
    }
}
