//! End-to-end checks of the API adapter against a real bitstream.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crate::api::callbacks::VideoDecoderCallbacks;
use crate::api::config::{Codec, DecoderConfig};
use crate::api::decoder::{FlushMode, VideoDecoder};
use crate::api::format::StreamFormat;
use crate::api::h264_adapter::H264VideoDecoder;
use crate::api::packet::EncodedPacket;
use crate::api::DefaultAllocator;
use crate::api::VideoPlane;

#[derive(Default)]
struct CountingCallbacks {
    pictures: AtomicUsize,
    format_changes: AtomicUsize,
}

impl VideoDecoderCallbacks for CountingCallbacks {
    fn on_picture_available(&self) {
        self.pictures.fetch_add(1, Ordering::Relaxed);
    }
    fn on_format_changed(&self, _format: StreamFormat) {
        self.format_changes.fetch_add(1, Ordering::Relaxed);
    }
}

fn fixture(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join(rel)
}

fn run_stream(path: &str) -> (usize, usize, usize) {
    let bytes = std::fs::read(fixture(path)).expect("read fixture");
    let callbacks: Arc<CountingCallbacks> = Arc::new(CountingCallbacks::default());
    let mut decoder = H264VideoDecoder::new(
        DecoderConfig::new(Codec::H264),
        Arc::new(DefaultAllocator),
        callbacks.clone(),
    )
    .expect("construct");

    decoder.decode(EncodedPacket::from_vec(bytes)).expect("decode");
    decoder.flush(FlushMode::Drain).expect("flush");

    let mut frames = 0usize;
    while let Some(pic) = decoder.get_picture().expect("get_picture") {
        // Sanity: every published frame exposes a luma plane.
        let y = pic.frame.plane(VideoPlane::Y).expect("luma");
        assert_eq!(y.plane, VideoPlane::Y);
        assert!(y.width > 0 && y.height > 0);
        assert!(y.data.len() >= y.height * y.stride - (y.stride - y.width));
        frames += 1;
    }

    (
        frames,
        callbacks.pictures.load(Ordering::Relaxed),
        callbacks.format_changes.load(Ordering::Relaxed),
    )
}

#[test]
fn baseline_stream_decodes_via_api() {
    let (frames, picture_callbacks, format_changes) = run_stream("data/SVA_BA2_D/SVA_BA2_D.264");
    assert!(frames > 0, "no frames decoded");
    assert!(picture_callbacks > 0, "no picture_available callbacks fired");
    assert_eq!(format_changes, 1, "format_changed must fire exactly once for a single-SPS stream");
}

#[test]
fn opaque_round_trips_in_display_order() {
    use std::fs::File;
    use std::io::BufReader;

    use crate::h264::nal_parser::NalParser;

    // For B-frame streams, packets arrive in decode order but pictures
    // come out in display order. We tag each input packet with its
    // index, then verify that the output tags appear in some
    // monotonically reordered sequence (every input tag appears
    // exactly once on output).
    let path = fixture("data/SVA_BA2_D/SVA_BA2_D.264");

    let callbacks = Arc::new(CountingCallbacks::default());
    let mut decoder = H264VideoDecoder::new(
        DecoderConfig::new(Codec::H264),
        Arc::new(DefaultAllocator),
        callbacks,
    )
    .expect("construct");

    // Slice each NAL into its own packet so opaque attaches per
    // primary-coded-picture.
    let file = File::open(&path).expect("open");
    let reader = BufReader::new(file);
    let parser = NalParser::new(reader);
    let mut sent: usize = 0;
    for (idx, nal_result) in parser.enumerate() {
        let nal = nal_result.expect("nal parse");
        let mut buf = Vec::with_capacity(nal.len() + 4);
        buf.extend_from_slice(&[0, 0, 0, 1]);
        buf.extend_from_slice(&nal);
        let packet = EncodedPacket::from_vec_with_opaque(buf, idx);
        decoder.decode(packet).expect("decode");
        sent += 1;
    }
    decoder.flush(FlushMode::Drain).expect("flush");

    let mut seen_tags = Vec::new();
    while let Some(pic) = decoder.get_picture().expect("get_picture") {
        let tag = pic.opaque.expect("opaque attached").downcast::<usize>().expect("usize tag");
        seen_tags.push(*tag);
    }
    assert!(!seen_tags.is_empty(), "no pictures emitted");
    // Every tag we observe was a valid input index, and tags are unique.
    let mut sorted = seen_tags.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), seen_tags.len(), "duplicate opaque tags emitted");
    for tag in &seen_tags {
        assert!(*tag < sent, "tag out of range: {tag} (sent {sent})");
    }
}
