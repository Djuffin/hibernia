//! Shared test fixtures and helpers for the API-level test suite.
//!
//! Tests under `h264::api_tests` exercise the public surface
//! ([`create_decoder`], [`VideoDecoder`], allocators, callbacks). The
//! helpers here keep individual tests focused on what they assert.

#![allow(dead_code)] // some helpers are used only by later phases.

use std::collections::VecDeque;
use std::fs::{self, File};
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::ptr::NonNull;
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
    workspace_root().join(rel)
}

/// Path to the cargo workspace root.
pub fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

/// Scratch directory created relative to the workspace root. Removed
/// on drop. Mirrors the helper in h264::e2e_tests.
pub struct TestDir {
    path: PathBuf,
}

impl TestDir {
    pub fn new(rel: &str) -> io::Result<Self> {
        let path = workspace_root().join(rel);
        fs::create_dir_all(&path)?;
        Ok(Self { path })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn join(&self, name: &str) -> PathBuf {
        self.path.join(name)
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

/// Invoke `ffmpeg` with the given arguments. Returns `Ok(true)` on
/// success, `Ok(false)` when ffmpeg is absent (caller is expected to
/// skip the test), and `Err(_)` on any other failure to spawn it.
pub fn run_ffmpeg(args: &[&str]) -> Result<bool, String> {
    let output = match Command::new("ffmpeg").args(args).output() {
        Ok(o) => o,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            println!("ffmpeg not found, skipping test");
            return Ok(false);
        }
        Err(e) => return Err(format!("failed to execute ffmpeg: {e}")),
    };
    if !output.status.success() {
        println!(
            "ffmpeg execution failed (exit={:?}), skipping test\nstderr: {}",
            output.status.code(),
            String::from_utf8_lossy(&output.stderr),
        );
        return Ok(false);
    }
    Ok(true)
}

/// Walk an AVC-framed buffer (concatenated length-prefixed NALs) and
/// return each NAL as a separate byte vector. `length_size` matches
/// `lengthSizeMinusOne + 1` from the avcC record (typically 4).
pub fn split_avc_buffer(bytes: &[u8], length_size: usize) -> Result<Vec<Vec<u8>>, String> {
    let mut nals = Vec::new();
    let mut off = 0;
    while off < bytes.len() {
        if off + length_size > bytes.len() {
            return Err(format!(
                "truncated NAL length prefix at offset {off} ({} bytes remaining)",
                bytes.len() - off,
            ));
        }
        let mut len = 0usize;
        for i in 0..length_size {
            len = (len << 8) | bytes[off + i] as usize;
        }
        off += length_size;
        if off + len > bytes.len() {
            return Err(format!(
                "NAL payload at offset {off} declares {len} bytes but only {} remain",
                bytes.len() - off,
            ));
        }
        nals.push(bytes[off..off + len].to_vec());
        off += len;
    }
    Ok(nals)
}

/// Encode pictures as a y4m byte stream comparable via
/// `crate::y4m_cmp::compare_y4m_buffers`.
pub fn pictures_to_y4m_bytes(
    pictures: &[DecodedPicture],
    framerate: y4m::Ratio,
) -> Vec<u8> {
    let Some(first) = pictures.first() else { return Vec::new() };
    let display_width = first.format.display_width;
    let display_height = first.format.display_height;

    let mut out = Vec::<u8>::new();
    {
        let writer = io::BufWriter::new(&mut out);
        let mut encoder = y4m::encode(display_width, display_height, framerate)
            .with_colorspace(y4m::Colorspace::C420)
            .write_header(writer)
            .expect("write y4m header");

        for pic in pictures {
            let dw = pic.format.display_width;
            let dh = pic.format.display_height;
            let cx = pic.format.crop_left;
            let cy = pic.format.crop_top;

            let mut planes: [Vec<u8>; 3] = [Vec::new(), Vec::new(), Vec::new()];
            for (i, channel) in [VideoPlane::Y, VideoPlane::U, VideoPlane::V].iter().enumerate() {
                let view = pic.frame.plane(*channel).expect("plane present");
                let (cw, ch, cx_p, cy_p) = if i == 0 {
                    (dw, dh, cx, cy)
                } else {
                    (dw / 2, dh / 2, cx / 2, cy / 2)
                };
                let buf = &mut planes[i];
                buf.resize(cw * ch, 0);
                for row in 0..ch {
                    let src = (cy_p + row) * view.stride + cx_p;
                    let dst = row * cw;
                    buf[dst..dst + cw].copy_from_slice(&view.data[src..src + cw]);
                }
            }
            let yuv = y4m::Frame::new(
                [planes[0].as_slice(), planes[1].as_slice(), planes[2].as_slice()],
                None,
            );
            encoder.write_frame(&yuv).expect("write y4m frame");
        }
    }
    out
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

struct PooledFrameBuffer {
    inner: Option<Box<dyn FrameBuffer>>,
    pool: Arc<Mutex<VecDeque<Box<dyn FrameBuffer>>>>,
}

impl FrameBuffer for PooledFrameBuffer {
    fn plane_ptr(&self, plane: VideoPlane) -> Option<NonNull<[u8]>> {
        self.inner.as_ref().and_then(|b| b.plane_ptr(plane))
    }
}

impl Drop for PooledFrameBuffer {
    fn drop(&mut self) {
        if let Some(buf) = self.inner.take() {
            let mut list = self.pool.lock().unwrap();
            list.push_back(buf);
        }
    }
}

/// Allocator that recycles buffers through a free list. Verifies
/// that the decoder doesn't retain references past `DecodedPicture`
/// lifetimes.
pub struct PoolAllocator {
    inner: DefaultAllocator,
    pub alloc_count: AtomicUsize,
    pool: Arc<Mutex<VecDeque<Box<dyn FrameBuffer>>>>,
}

impl PoolAllocator {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            inner: DefaultAllocator,
            alloc_count: AtomicUsize::new(0),
            pool: Arc::new(Mutex::new(VecDeque::new())),
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
        let mut list = self.pool.lock().unwrap();

        // Scan the pool for any recycled buffer that is large enough:
        let found_idx = list.iter().position(|buf| {
            for plane_alloc in alloc.planes.iter().flatten() {
                if let Some(slice) = buf.plane_ptr(plane_alloc.plane) {
                    if slice.len() < plane_alloc.size_bytes {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            true
        });

        let inner_buf = if let Some(idx) = found_idx {
            list.remove(idx).unwrap()
        } else {
            // Cache miss: allocate fresh memory
            self.alloc_count.fetch_add(1, Ordering::Relaxed);
            self.inner.alloc_frame(alloc)?
        };

        Ok(Box::new(PooledFrameBuffer {
            inner: Some(inner_buf),
            pool: self.pool.clone(),
        }))
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
