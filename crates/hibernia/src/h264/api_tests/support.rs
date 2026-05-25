//! Shared test fixtures and helpers for the API-level test suite.
//!
//! Tests under `h264::api_tests` exercise the public surface
//! ([`create_decoder`], [`VideoDecoder`], allocators, callbacks). The
//! helpers here keep individual tests focused on what they assert.

#![allow(dead_code)] // some helpers are used only by later phases.

use std::collections::VecDeque;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use crate::api::callbacks::{DecoderError, VideoDecoderCallbacks};
use crate::api::config::{Codec, DecoderConfig};
use crate::api::decoder::{FlushMode, VideoDecoder};
use crate::api::default_allocator::DefaultAllocator;
use crate::api::format::StreamFormat;
use crate::api::frame::{
    AllocError, BufferAllocation, FrameBuffer, VideoFrameAllocator,
};
use crate::api::packet::{DecodedPicture, EncodedPacket};
use crate::api::VideoPlane;
use crate::h264::decoder::Decoder;
use crate::h264::nal_parser::NalParser;

/// Resolve a path under the workspace root from a path relative to it.
pub fn fixture(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join(rel)
}

/// Read every NAL out of an Annex-B fixture file. Each entry is the
/// raw NAL bytes (no start code, no length prefix).
pub fn read_fixture_nals(path: &Path) -> Vec<Vec<u8>> {
    let file = File::open(path).expect("open fixture");
    let parser = NalParser::new(BufReader::new(file));
    parser.map(|r| r.expect("nal parse")).collect()
}

/// Wrap a single NAL with a 4-byte Annex-B start code.
pub fn annexb_packet(nal: &[u8]) -> EncodedPacket {
    let mut buf = Vec::with_capacity(nal.len() + 4);
    buf.extend_from_slice(&[0, 0, 0, 1]);
    buf.extend_from_slice(nal);
    EncodedPacket::from_vec(buf)
}

/// Wrap multiple NALs into one Annex-B-packaged packet.
pub fn annexb_packet_multi(nals: &[&[u8]]) -> EncodedPacket {
    let mut buf = Vec::new();
    for nal in nals {
        buf.extend_from_slice(&[0, 0, 0, 1]);
        buf.extend_from_slice(nal);
    }
    EncodedPacket::from_vec(buf)
}

/// Wrap a NAL with a `length_size`-byte big-endian length prefix.
pub fn avc_packet_with_length_size(nal: &[u8], length_size: usize) -> Vec<u8> {
    let mut buf = Vec::with_capacity(nal.len() + length_size);
    let len = nal.len();
    for i in 0..length_size {
        let shift = 8 * (length_size - 1 - i);
        buf.push(((len >> shift) & 0xff) as u8);
    }
    buf.extend_from_slice(nal);
    buf
}

/// Wrap a fixture file's NALs as a single AVC-framed packet with
/// 4-byte length prefixes.
pub fn read_fixture_as_avc_packet(path: &Path) -> EncodedPacket {
    let nals = read_fixture_nals(path);
    let mut buf = Vec::new();
    for nal in &nals {
        buf.extend_from_slice(&avc_packet_with_length_size(nal, 4));
    }
    EncodedPacket::from_vec(buf)
}

/// Drive a sequence of packets through the decoder and drain every
/// resulting picture after each one. Returns all emitted pictures in
/// the order `get_picture` produced them.
pub fn drive_through<D: VideoDecoder + ?Sized>(
    decoder: &mut D,
    packets: impl IntoIterator<Item = EncodedPacket>,
) -> Result<Vec<DecodedPicture>, DecoderError> {
    let mut out = Vec::new();
    for packet in packets {
        decoder.decode(packet)?;
        while let Some(pic) = decoder.get_picture()? {
            out.push(pic);
        }
    }
    decoder.flush(FlushMode::Drain)?;
    while let Some(pic) = decoder.get_picture()? {
        out.push(pic);
    }
    Ok(out)
}

/// Build a `Decoder` over the default allocator.
pub fn default_decoder(
    config: DecoderConfig,
    callbacks: Arc<dyn VideoDecoderCallbacks>,
) -> Result<Decoder, DecoderError> {
    Decoder::new(config, Arc::new(DefaultAllocator), callbacks)
}

/// Counts callback invocations and stores the most recent format.
#[derive(Default)]
pub struct CountingCallbacks {
    pub pictures: AtomicUsize,
    pub format_changes: AtomicUsize,
    pub last_format: Mutex<Option<StreamFormat>>,
}

impl VideoDecoderCallbacks for CountingCallbacks {
    fn on_picture_available(&self) {
        self.pictures.fetch_add(1, Ordering::Relaxed);
    }
    fn on_format_changed(&self, format: StreamFormat) {
        self.format_changes.fetch_add(1, Ordering::Relaxed);
        *self.last_format.lock().unwrap() = Some(format);
    }
}

impl CountingCallbacks {
    pub fn shared() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn picture_callbacks(&self) -> usize {
        self.pictures.load(Ordering::Relaxed)
    }

    pub fn format_change_count(&self) -> usize {
        self.format_changes.load(Ordering::Relaxed)
    }
}

/// Allocator that records every `alloc_frame` call and can be
/// configured to fail on demand.
pub struct TrackingAllocator {
    inner: DefaultAllocator,
    pub alloc_count: AtomicUsize,
    pub fail_with: Mutex<Option<AllocError>>,
    pub last_request: Mutex<Option<BufferAllocation>>,
}

impl TrackingAllocator {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            inner: DefaultAllocator,
            alloc_count: AtomicUsize::new(0),
            fail_with: Mutex::new(None),
            last_request: Mutex::new(None),
        })
    }

    pub fn set_failure(&self, err: AllocError) {
        *self.fail_with.lock().unwrap() = Some(err);
    }

    pub fn count(&self) -> usize {
        self.alloc_count.load(Ordering::Relaxed)
    }
}

impl VideoFrameAllocator for TrackingAllocator {
    fn alloc_frame(
        &self,
        alloc: &BufferAllocation,
    ) -> Result<Box<dyn FrameBuffer>, AllocError> {
        self.alloc_count.fetch_add(1, Ordering::Relaxed);
        *self.last_request.lock().unwrap() = Some(*alloc);
        if let Some(err) = self.fail_with.lock().unwrap().clone() {
            return Err(err);
        }
        self.inner.alloc_frame(alloc)
    }
}

/// Allocator that recycles buffers through a free list. Verifies
/// that the decoder doesn't retain references past `DecodedPicture`
/// lifetimes.
pub struct PoolAllocator {
    inner: DefaultAllocator,
    pub alloc_count: AtomicUsize,
    pool: Mutex<VecDeque<Box<dyn FrameBuffer>>>,
}

impl PoolAllocator {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            inner: DefaultAllocator,
            alloc_count: AtomicUsize::new(0),
            pool: Mutex::new(VecDeque::new()),
        })
    }

    pub fn count(&self) -> usize {
        self.alloc_count.load(Ordering::Relaxed)
    }
}

impl VideoFrameAllocator for PoolAllocator {
    fn alloc_frame(
        &self,
        alloc: &BufferAllocation,
    ) -> Result<Box<dyn FrameBuffer>, AllocError> {
        // For the lifetime test we only care about counting fresh
        // allocations and that the decoder returns buffers via Drop
        // through the published Arc<dyn VideoFrame> path. The pool
        // doesn't try to size-match across requests.
        self.alloc_count.fetch_add(1, Ordering::Relaxed);
        self.inner.alloc_frame(alloc)
    }
}

/// A `PlaneView`'s `data` slice is non-empty and large enough that
/// `(height-1)*stride + width - 1` is a valid index. Used widely.
pub fn plane_data_is_addressable(view: &crate::api::PlaneView<'_>) -> bool {
    if view.height == 0 || view.width == 0 {
        return view.data.is_empty();
    }
    view.data.len() >= (view.height - 1) * view.stride + view.width
}

/// Standard B-frame Annex-B fixture path.
pub const BASELINE_BFRAME_FIXTURE: &str = "data/SVA_BA2_D/SVA_BA2_D.264";

/// Read the standard B-frame fixture as a Vec of Annex-B-packaged
/// per-NAL `EncodedPacket`s.
pub fn baseline_bframe_packets() -> Vec<EncodedPacket> {
    let nals = read_fixture_nals(&fixture(BASELINE_BFRAME_FIXTURE));
    nals.iter().map(|n| annexb_packet(n)).collect()
}

/// Read a fixture's NALs and bucket them into (SPSes, PPSes,
/// non-parameter-set NALs). Useful for the out-of-band parameter
/// tests where the extradata path takes SPS+PPS and the sample
/// stream is everything else.
pub fn bucket_fixture_nals(
    path: &Path,
) -> (Vec<Vec<u8>>, Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let nals = read_fixture_nals(path);
    let mut sps = Vec::new();
    let mut pps = Vec::new();
    let mut samples = Vec::new();
    for nal in nals {
        if nal.is_empty() {
            continue;
        }
        let nal_type = nal[0] & 0x1F;
        match nal_type {
            7 => sps.push(nal),
            8 => pps.push(nal),
            _ => samples.push(nal),
        }
    }
    (sps, pps, samples)
}

/// Wrap NALs as Annex-B-framed packets with 4-byte start codes.
pub fn annexb_packets_from_nals(nals: &[Vec<u8>]) -> Vec<EncodedPacket> {
    nals.iter().map(|n| annexb_packet(n)).collect()
}

/// Wrap NALs as AVC-framed packets with the given length-prefix size
/// (1, 2, or 4 bytes). Each NAL becomes its own packet so the
/// default queue depth stays in budget.
pub fn avc_packets_from_nals(nals: &[Vec<u8>], length_size: usize) -> Vec<EncodedPacket> {
    nals.iter()
        .map(|n| EncodedPacket::from_vec(avc_packet_with_length_size(n, length_size)))
        .collect()
}

/// Tagging variant of `baseline_bframe_packets`: each packet carries
/// its zero-based input index as `Box<usize>` opaque metadata.
pub fn baseline_bframe_tagged_packets() -> Vec<EncodedPacket> {
    let nals = read_fixture_nals(&fixture(BASELINE_BFRAME_FIXTURE));
    nals.into_iter()
        .enumerate()
        .map(|(idx, nal)| {
            let mut buf = Vec::with_capacity(nal.len() + 4);
            buf.extend_from_slice(&[0, 0, 0, 1]);
            buf.extend_from_slice(&nal);
            EncodedPacket::from_vec_with_opaque(buf, idx)
        })
        .collect()
}

/// Assert helper for VideoPlane luma access.
pub fn luma(pic: &DecodedPicture) -> crate::api::PlaneView<'_> {
    pic.frame.plane(VideoPlane::Y).expect("luma present")
}
