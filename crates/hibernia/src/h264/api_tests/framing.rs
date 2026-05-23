//! Tests for the bitstream framing dispatch inside
//! `VideoDecoder::decode`: Annex-B vs AVC packaging, multi-NAL
//! packets, malformed inputs, and `EncodedPacket` construction.

use std::any::Any;
use std::sync::Arc;

use crate::api::bitstream::{AvcBitstreamFormat, H264Config};
use crate::api::callbacks::DecoderError;
use crate::api::config::{Codec, DecoderConfig};
use crate::api::decoder::{FlushMode, VideoDecoder};
use crate::api::packet::EncodedPacket;

use super::support::{
    annexb_packet, annexb_packet_multi, avc_packet_with_length_size, baseline_bframe_packets,
    default_decoder, drive_through, fixture, read_fixture_as_avc_packet, read_fixture_nals,
    CountingCallbacks, BASELINE_BFRAME_FIXTURE,
};

fn annexb_decoder() -> Box<dyn VideoDecoder> {
    let callbacks = CountingCallbacks::shared();
    Box::new(
        default_decoder(DecoderConfig::new(Codec::H264), callbacks).expect("construct"),
    )
}

fn avc_decoder() -> Box<dyn VideoDecoder> {
    let callbacks = CountingCallbacks::shared();
    Box::new(
        default_decoder(
            DecoderConfig::new(Codec::H264)
                .with_custom_params(H264Config { bitstream_format: AvcBitstreamFormat::Avc }),
            callbacks,
        )
        .expect("construct"),
    )
}

#[test]
fn annexb_single_nal_packet_decodes() {
    let nals = read_fixture_nals(&fixture(BASELINE_BFRAME_FIXTURE));
    let mut decoder = annexb_decoder();
    // Feed every NAL as its own packet -- standard Annex-B usage.
    for nal in &nals {
        decoder.decode(annexb_packet(nal)).expect("decode");
    }
    decoder.flush(FlushMode::Drain).expect("flush");
    let mut frames = 0;
    while decoder.get_picture().expect("get_picture").is_some() {
        frames += 1;
    }
    assert!(frames > 0, "single-NAL Annex-B packets must decode");
}

#[test]
fn annexb_multi_nal_packet_decodes() {
    let nals = read_fixture_nals(&fixture(BASELINE_BFRAME_FIXTURE));
    let nal_refs: Vec<&[u8]> = nals.iter().map(|v| v.as_slice()).collect();
    let mut decoder = annexb_decoder();
    decoder
        .decode(annexb_packet_multi(&nal_refs))
        .expect("decode multi-NAL packet");
    // Queue may be saturated; drain explicitly.
    while decoder.get_picture().expect("get_picture").is_some() {}
    decoder.flush(FlushMode::Drain).expect("flush");
    let mut frames = 0;
    while decoder.get_picture().expect("get_picture").is_some() {
        frames += 1;
    }
    // The fixture has been observed to emit at least one frame this way.
    assert!(frames > 0, "multi-NAL Annex-B packet must decode some frames");
}

#[test]
fn annexb_three_byte_start_code_works() {
    let nals = read_fixture_nals(&fixture(BASELINE_BFRAME_FIXTURE));
    let mut decoder = annexb_decoder();
    for nal in &nals {
        let mut buf = Vec::with_capacity(nal.len() + 3);
        buf.extend_from_slice(&[0, 0, 1]); // 3-byte start code
        buf.extend_from_slice(nal);
        decoder.decode(EncodedPacket::from_vec(buf)).expect("decode");
    }
    decoder.flush(FlushMode::Drain).expect("flush");
    let mut frames = 0;
    while decoder.get_picture().expect("get_picture").is_some() {
        frames += 1;
    }
    assert!(frames > 0, "3-byte start code must be honored");
}

#[test]
fn annexb_empty_packet_is_noop() {
    let mut decoder = annexb_decoder();
    decoder.decode(EncodedPacket::from_vec(Vec::new())).expect("empty decode");
    assert!(decoder.get_picture().expect("get_picture").is_none());
}

#[test]
fn annexb_packet_without_start_code_is_noop() {
    // Bytes that don't form a 0x000001 / 0x00000001 prefix must not
    // crash. The splitter yields nothing; no NAL reaches the inner
    // pipeline.
    let mut decoder = annexb_decoder();
    decoder
        .decode(EncodedPacket::from_vec(vec![0xff, 0xee, 0xdd, 0xcc]))
        .expect("noop decode");
    assert!(decoder.get_picture().expect("get_picture").is_none());
}

#[test]
fn avc_decoder_decodes_length_prefixed_stream() {
    let packet = read_fixture_as_avc_packet(&fixture(BASELINE_BFRAME_FIXTURE));
    let mut decoder = avc_decoder();
    decoder.decode(packet).expect("decode avc");
    while decoder.get_picture().expect("get_picture").is_some() {}
    decoder.flush(FlushMode::Drain).expect("flush");
    let mut frames = 0;
    while decoder.get_picture().expect("get_picture").is_some() {
        frames += 1;
    }
    assert!(frames > 0, "AVC-framed fixture must decode");
}

#[test]
fn avc_multi_nal_packet_decodes() {
    let nals = read_fixture_nals(&fixture(BASELINE_BFRAME_FIXTURE));
    // Feed a handful at a time to keep the queue depth in budget.
    let mut decoder = avc_decoder();
    for chunk in nals.chunks(4) {
        let mut buf = Vec::new();
        for nal in chunk {
            buf.extend_from_slice(&avc_packet_with_length_size(nal, 4));
        }
        decoder.decode(EncodedPacket::from_vec(buf)).expect("decode");
        while decoder.get_picture().expect("get_picture").is_some() {}
    }
    decoder.flush(FlushMode::Drain).expect("flush");
    let mut frames = 0;
    while decoder.get_picture().expect("get_picture").is_some() {
        frames += 1;
    }
    assert!(frames > 0);
}

#[test]
fn avc_truncated_length_prefix_returns_misformed() {
    let mut decoder = avc_decoder();
    // Only 2 bytes when 4 are required.
    let err = decoder.decode(EncodedPacket::from_vec(vec![0, 0])).unwrap_err();
    assert!(
        matches!(err, DecoderError::MisformedData(_)),
        "expected MisformedData, got {err:?}"
    );
}

#[test]
fn avc_truncated_payload_returns_misformed() {
    let mut decoder = avc_decoder();
    // Length prefix says 100 bytes but only 1 follows.
    let buf = vec![0x00, 0x00, 0x00, 0x64, 0x42];
    let err = decoder.decode(EncodedPacket::from_vec(buf)).unwrap_err();
    assert!(
        matches!(err, DecoderError::MisformedData(_)),
        "expected MisformedData, got {err:?}"
    );
}

#[test]
fn avc_zero_length_nal_is_skipped() {
    let mut decoder = avc_decoder();
    // Single zero-length NAL.
    let buf = vec![0x00, 0x00, 0x00, 0x00];
    decoder.decode(EncodedPacket::from_vec(buf)).expect("decode");
    assert!(decoder.get_picture().expect("get_picture").is_none());
}

#[test]
fn encoded_packet_from_vec_round_trips_bytes() {
    let packet = EncodedPacket::from_vec(vec![0x11, 0x22, 0x33]);
    assert_eq!(packet.bytes(), &[0x11, 0x22, 0x33]);
    assert!(packet.opaque.is_none());
}

#[test]
fn encoded_packet_from_vec_with_opaque_attaches_metadata() {
    let packet = EncodedPacket::from_vec_with_opaque(vec![0xaa], 42u32);
    assert_eq!(packet.bytes(), &[0xaa]);
    let tag = packet.opaque.expect("opaque").downcast::<u32>().expect("u32");
    assert_eq!(*tag, 42);
}

#[test]
fn encoded_packet_accepts_custom_arc_data_source() {
    // The design promises any Arc<dyn AsRef<[u8]> + Send + Sync> works.
    // Verify by handing the decoder a packet whose data lives in a
    // user-defined wrapper rather than a Vec.
    struct Wrapper(Vec<u8>);
    impl AsRef<[u8]> for Wrapper {
        fn as_ref(&self) -> &[u8] {
            self.0.as_slice()
        }
    }

    let nals = read_fixture_nals(&fixture(BASELINE_BFRAME_FIXTURE));
    let mut buf = Vec::new();
    for nal in &nals {
        buf.extend_from_slice(&[0, 0, 0, 1]);
        buf.extend_from_slice(nal);
    }
    let data: Arc<dyn AsRef<[u8]> + Send + Sync> = Arc::new(Wrapper(buf));
    let packet = EncodedPacket { data, opaque: None };
    let mut decoder = annexb_decoder();
    let _ = decoder.decode(packet); // May QueueFull; that's fine.
    // Drain whatever did land.
    while decoder.get_picture().expect("get_picture").is_some() {}
    decoder.flush(FlushMode::Drain).expect("flush");
    let mut frames = 0;
    while decoder.get_picture().expect("get_picture").is_some() {
        frames += 1;
    }
    assert!(frames > 0, "custom-data-source packet must decode");
}

#[test]
fn avc_config_decodes_baseline_fixture_end_to_end() {
    // End-to-end sanity check that AVC packaging produces the same
    // frame count as Annex-B packaging on the same fixture.
    let annexb_count = {
        let callbacks = CountingCallbacks::shared();
        let mut decoder =
            default_decoder(DecoderConfig::new(Codec::H264), callbacks).expect("construct");
        let pics = drive_through(&mut decoder, baseline_bframe_packets()).expect("drive");
        pics.len()
    };
    let avc_count = {
        let mut decoder = avc_decoder();
        let packet = read_fixture_as_avc_packet(&fixture(BASELINE_BFRAME_FIXTURE));
        decoder.decode(packet).expect("decode");
        let mut count = 0;
        while decoder.get_picture().expect("get_picture").is_some() {
            count += 1;
        }
        decoder.flush(FlushMode::Drain).expect("flush");
        while decoder.get_picture().expect("get_picture").is_some() {
            count += 1;
        }
        count
    };
    assert_eq!(annexb_count, avc_count, "AVC and Annex-B paths must emit the same frames");
}

#[test]
fn h264config_default_is_annexb() {
    // DecoderConfig with no custom_params should default to Annex-B
    // framing -- a packet built as Annex-B must decode.
    let callbacks = CountingCallbacks::shared();
    let mut decoder =
        default_decoder(DecoderConfig::new(Codec::H264), callbacks).expect("construct");
    let nals = read_fixture_nals(&fixture(BASELINE_BFRAME_FIXTURE));
    for nal in &nals[..3] {
        decoder.decode(annexb_packet(nal)).expect("decode");
    }
    // Decoding succeeded; opportunistic frame retrieval.
    let _ = decoder.flush(FlushMode::Drain);
}

#[test]
fn unsupported_custom_params_returns_initialization_failed() {
    // Pass a value whose type isn't H264Config.
    let bogus: Box<dyn Any + Send> = Box::new(99u64);
    let config = DecoderConfig {
        codec: Codec::H264,
        latency_mode: crate::api::config::LatencyMode::Throughput,
        custom_params: Some(bogus),
    };
    let err = default_decoder(config, CountingCallbacks::shared()).unwrap_err();
    assert!(
        matches!(err, DecoderError::InitializationFailed(_)),
        "expected InitializationFailed, got {err:?}"
    );
}
