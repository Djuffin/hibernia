//! Tests for `H264Config::extradata` -- the construction-time
//! out-of-band parameter-set delivery path.

use std::sync::Arc;

use crate::api::bitstream::{AvcBitstreamFormat, H264Config};
use crate::api::callbacks::DecoderError;
use crate::api::config::{Codec, DecoderConfig};
use crate::api::decoder::{FlushMode, VideoDecoder};
use crate::api::h264_commands::H264SetExtradata;
use crate::api::{build_avcc, DefaultAllocator};
use crate::h264::decoder::Decoder;

use super::support::{
    annexb_packet, annexb_packets_from_nals, avc_packets_from_nals, bucket_fixture_nals,
    default_decoder, drive_through, fixture, BASELINE_BFRAME_FIXTURE,
};
use super::support::CountingCallbacks;

/// Build an Annex-B-style extradata blob by concatenating the given
/// NALs with 4-byte start codes.
fn annexb_extradata_blob(nals: &[Vec<u8>]) -> Vec<u8> {
    let mut out = Vec::new();
    for nal in nals {
        out.extend_from_slice(&[0, 0, 0, 1]);
        out.extend_from_slice(nal);
    }
    out
}

#[test]
fn construct_with_avcc_decodes_slice_only_packets() {
    let (sps, pps, samples) = bucket_fixture_nals(&fixture(BASELINE_BFRAME_FIXTURE));
    assert!(!sps.is_empty() && !pps.is_empty(), "fixture must have SPS+PPS");
    let avcc = build_avcc(&sps, &pps, 4);

    let config = DecoderConfig::new(Codec::H264).with_custom_params(H264Config {
        bitstream_format: AvcBitstreamFormat::Avc,
        extradata: Some(avcc),
    });
    let mut decoder =
        default_decoder(config, CountingCallbacks::shared()).expect("construct with avcC");

    // Feed only the samples (no SPS/PPS). The decoder must already
    // have the parameter sets loaded from the avcC blob.
    let packets = avc_packets_from_nals(&samples, 4);
    let pics = drive_through(&mut decoder, packets).expect("drive");
    assert!(!pics.is_empty(), "avcC-preloaded decoder must produce frames");
}

#[test]
fn construct_with_annexb_extradata_decodes_slice_only_packets() {
    let (sps, pps, samples) = bucket_fixture_nals(&fixture(BASELINE_BFRAME_FIXTURE));
    let mut concat = sps;
    concat.extend(pps);
    let extradata = annexb_extradata_blob(&concat);

    let config = DecoderConfig::new(Codec::H264).with_custom_params(H264Config {
        bitstream_format: AvcBitstreamFormat::AnnexB,
        extradata: Some(extradata),
    });
    let mut decoder = default_decoder(config, CountingCallbacks::shared())
        .expect("construct with Annex-B extradata");

    // Feed only the slice NALs as Annex-B packets.
    let packets = annexb_packets_from_nals(&samples);
    let pics = drive_through(&mut decoder, packets).expect("drive");
    assert!(!pics.is_empty(), "Annex-B-extradata decoder must produce frames");
}

#[test]
fn extradata_path_yields_same_frame_count_as_in_band_path() {
    // Cross-check: decoding the same fixture via the (extradata +
    // samples) path produces the same picture count as the all-in-band
    // path that the rest of the test suite uses.
    let (sps, pps, samples) = bucket_fixture_nals(&fixture(BASELINE_BFRAME_FIXTURE));

    let in_band_count = {
        let mut decoder = default_decoder(
            DecoderConfig::new(Codec::H264),
            CountingCallbacks::shared(),
        )
        .expect("in-band decoder");
        // Reconstitute by feeding NALs back in order: SPS, then PPS,
        // then samples in stream order.
        let mut packets = annexb_packets_from_nals(&sps);
        packets.extend(annexb_packets_from_nals(&pps));
        packets.extend(annexb_packets_from_nals(&samples));
        drive_through(&mut decoder, packets).expect("drive in-band").len()
    };

    let extradata_count = {
        let avcc = build_avcc(&sps, &pps, 4);
        let config = DecoderConfig::new(Codec::H264).with_custom_params(H264Config {
            bitstream_format: AvcBitstreamFormat::Avc,
            extradata: Some(avcc),
        });
        let mut decoder =
            default_decoder(config, CountingCallbacks::shared()).expect("extradata decoder");
        let packets = avc_packets_from_nals(&samples, 4);
        drive_through(&mut decoder, packets).expect("drive extradata").len()
    };

    assert_eq!(in_band_count, extradata_count);
}

#[test]
fn avcc_lengthsize_minus_one_overrides_default_four_bytes() {
    // Build avcC with a 2-byte length prefix and feed samples with the
    // matching length prefix. The decoder must pick up length_size = 2
    // from avcC rather than its default of 4.
    let (sps, pps, samples) = bucket_fixture_nals(&fixture(BASELINE_BFRAME_FIXTURE));
    // All sample NALs in this fixture fit in a 2-byte length field
    // (the longest is < 65535 bytes); SVA_BA2_D is QCIF 176x144.
    for nal in &samples {
        assert!(nal.len() <= 0xFFFF, "sample exceeds 2-byte length budget");
    }
    let avcc = build_avcc(&sps, &pps, 2);

    let config = DecoderConfig::new(Codec::H264).with_custom_params(H264Config {
        bitstream_format: AvcBitstreamFormat::Avc,
        extradata: Some(avcc),
    });
    let mut decoder = default_decoder(config, CountingCallbacks::shared())
        .expect("avcC lengthSize=2 must initialize");

    let packets = avc_packets_from_nals(&samples, 2);
    let pics = drive_through(&mut decoder, packets).expect("drive lengthSize=2");
    assert!(!pics.is_empty(), "samples with 2-byte length prefix must decode");
}

#[test]
fn empty_extradata_is_accepted_and_noop() {
    let config = DecoderConfig::new(Codec::H264).with_custom_params(H264Config {
        bitstream_format: AvcBitstreamFormat::AnnexB,
        extradata: Some(Vec::new()),
    });
    let _decoder = default_decoder(config, CountingCallbacks::shared())
        .expect("empty extradata must be accepted");
    // No SPS/PPS were loaded; the test only checks construction.
}

#[test]
fn malformed_avcc_extradata_fails_construction() {
    let config = DecoderConfig::new(Codec::H264).with_custom_params(H264Config {
        bitstream_format: AvcBitstreamFormat::Avc,
        // First byte 0x01 dispatches to avcC parser; rest is garbage.
        extradata: Some(vec![0x01, 0x02]),
    });
    let err = default_decoder(config, CountingCallbacks::shared()).unwrap_err();
    assert!(
        matches!(err, DecoderError::MisformedData(_)),
        "expected MisformedData from malformed avcC, got {err:?}",
    );
}

#[test]
fn malformed_annexb_extradata_skips_silently() {
    // Annex-B-form extradata that contains no start codes parses to
    // an empty NAL list. Not an error -- matches FFmpeg behavior.
    let config = DecoderConfig::new(Codec::H264).with_custom_params(H264Config {
        bitstream_format: AvcBitstreamFormat::AnnexB,
        extradata: Some(vec![0x42, 0x99, 0xff]), // no start code
    });
    let _ = default_decoder(config, CountingCallbacks::shared())
        .expect("Annex-B extradata without start codes must not error");
}

#[test]
fn h264_config_default_has_no_extradata() {
    let config = H264Config::default();
    assert_eq!(config.bitstream_format, AvcBitstreamFormat::AnnexB);
    assert!(config.extradata.is_none());
}

// ---------------------------------------------------------------
// H264SetExtradata via control()
// ---------------------------------------------------------------

#[test]
fn set_extradata_via_control_preloads_parameter_sets() {
    // Fresh decoder with no extradata at construction. Send the
    // parameter sets via control() instead, then feed only slice
    // NALs as AVC-framed packets.
    let (sps, pps, samples) = bucket_fixture_nals(&fixture(BASELINE_BFRAME_FIXTURE));
    let avcc = build_avcc(&sps, &pps, 4);

    let config = DecoderConfig::new(Codec::H264).with_custom_params(H264Config {
        bitstream_format: AvcBitstreamFormat::Avc,
        extradata: None,
    });
    let mut decoder: Box<dyn VideoDecoder> = Box::new(
        Decoder::new(config, Arc::new(DefaultAllocator), CountingCallbacks::shared())
            .expect("construct without extradata"),
    );

    let mut cmd = H264SetExtradata { data: avcc };
    decoder.control(&mut cmd).expect("set_extradata via control");

    let packets = avc_packets_from_nals(&samples, 4);
    let pics = drive_through(decoder.as_mut(), packets).expect("drive after control");
    assert!(!pics.is_empty(), "samples must decode after control() preload");
}

#[test]
fn set_extradata_via_control_with_annexb_blob_works() {
    // Runtime path also accepts Annex-B-form extradata.
    let (sps, pps, samples) = bucket_fixture_nals(&fixture(BASELINE_BFRAME_FIXTURE));
    let mut nals_in_extradata = sps;
    nals_in_extradata.extend(pps);
    let extradata = annexb_extradata_blob(&nals_in_extradata);

    let config = DecoderConfig::new(Codec::H264).with_custom_params(H264Config {
        bitstream_format: AvcBitstreamFormat::AnnexB,
        extradata: None,
    });
    let mut decoder: Box<dyn VideoDecoder> = Box::new(
        Decoder::new(config, Arc::new(DefaultAllocator), CountingCallbacks::shared())
            .expect("construct"),
    );

    let mut cmd = H264SetExtradata { data: extradata };
    decoder.control(&mut cmd).expect("Annex-B extradata via control");

    let packets = annexb_packets_from_nals(&samples);
    let pics = drive_through(decoder.as_mut(), packets).expect("drive");
    assert!(!pics.is_empty());
}

#[test]
fn set_extradata_via_control_after_partial_decode() {
    // The runtime path overwrites parameter-set table entries with
    // matching IDs. Feed an in-band SPS+PPS, then replace them via
    // control() with the same parameter sets re-packaged as avcC.
    // Subsequent slices must continue to decode.
    let (sps, pps, samples) = bucket_fixture_nals(&fixture(BASELINE_BFRAME_FIXTURE));

    let config = DecoderConfig::new(Codec::H264).with_custom_params(H264Config {
        bitstream_format: AvcBitstreamFormat::AnnexB,
        extradata: None,
    });
    let mut decoder: Box<dyn VideoDecoder> = Box::new(
        Decoder::new(config, Arc::new(DefaultAllocator), CountingCallbacks::shared())
            .expect("construct"),
    );

    // In-band: SPS + PPS first.
    for nal in sps.iter().chain(pps.iter()) {
        decoder
            .decode(annexb_packet(nal))
            .expect("decode parameter-set NAL");
    }

    // Now replace them via control() with the same blob wrapped as avcC.
    let avcc = build_avcc(&sps, &pps, 4);
    let mut cmd = H264SetExtradata { data: avcc };
    decoder.control(&mut cmd).expect("re-set extradata mid-stream");

    // Feeding the slice NALs must still work.
    let packets = annexb_packets_from_nals(&samples);
    let pics = drive_through(decoder.as_mut(), packets).expect("drive");
    assert!(!pics.is_empty());
}

#[test]
fn malformed_extradata_via_control_returns_misformed_data() {
    let config = DecoderConfig::new(Codec::H264).with_custom_params(H264Config {
        bitstream_format: AvcBitstreamFormat::Avc,
        extradata: None,
    });
    let mut decoder: Box<dyn VideoDecoder> = Box::new(
        Decoder::new(config, Arc::new(DefaultAllocator), CountingCallbacks::shared())
            .expect("construct"),
    );

    let mut cmd = H264SetExtradata { data: vec![0x01, 0x02] }; // truncated avcC
    let err = decoder.control(&mut cmd).unwrap_err();
    assert!(
        matches!(err, DecoderError::MisformedData(_)),
        "expected MisformedData from malformed extradata, got {err:?}",
    );
}

// `control()` with unknown payloads is covered in
// `api_tests/control.rs` -- the same FeatureNotSupported path that
// existed before extradata gained a downcast arm.

// ---------------------------------------------------------------
// Real-world avcC: bear.mp4's avcC blob, hard-coded.
//
// BEAR_AVCC is the actual avcC byte sequence inside data/bear.mp4's
// `avc1` sample entry, extracted once with:
//   ffprobe -v error -select_streams v:0 -show_streams -show_data \
//           -of json data/bear.mp4
// Hard-coded here so the test doesn't need to parse MP4 boxes. The
// goal of these tests is to exercise the extradata-parsing path
// against output from a real-world encoder (x264).
//
// We don't try to decode bear.mp4's mdat samples directly: the very
// first sample is a non-IDR slice (an x264 open-GOP intra refresh,
// not an IDR), which Hibernia's slice handler cannot start fresh
// on. A real out-of-band-parameters bitstream consumer would either
// require an IDR or honor a recovery_point SEI -- neither path is
// in scope for these tests. The cross-pixel cross-check that would
// catch decoder-side regressions belongs in a fixture engineered
// for it.
// ---------------------------------------------------------------

/// avcC blob from data/bear.mp4. High profile, 320x180, 30fps.
const BEAR_AVCC: &[u8] = &[
    0x01,                   // configurationVersion
    0x64, 0x00, 0x0d,       // profile=High(100), compat=0, level=13 (1.3)
    0xff,                   // reserved(6) | lengthSizeMinusOne=3 (4-byte)
    0xe1,                   // reserved(3) | numOfSPS=1
    0x00, 0x18,             // SPS length = 24
    0x67, 0x64, 0x00, 0x0d, 0xac, 0x34, 0xe5, 0x05,
    0x06, 0x7e, 0x78, 0x40, 0x00, 0x00, 0x19, 0x00,
    0x00, 0x05, 0xda, 0xa3, 0xc5, 0x0a, 0x45, 0x80,
    0x01,                   // numOfPPS=1
    0x00, 0x05,             // PPS length = 5
    0x68, 0xee, 0xb2, 0xc8, 0xb0,
];

#[test]
fn bear_avcc_parses_through_extradata_pipeline() {
    // Real-world x264 avcC must parse cleanly via parse_extradata
    // and surface SPS+PPS NALs of the expected NAL types. This
    // catches regressions where our parser would reject real-world
    // encoder output (e.g., trailing High-profile fields, specific
    // reserved-bit patterns).
    let parsed = crate::h264::extradata::parse_extradata(BEAR_AVCC).expect("parse BEAR_AVCC");
    assert_eq!(parsed.length_size, Some(4));
    assert_eq!(parsed.nals.len(), 2, "one SPS + one PPS expected");
    assert_eq!(parsed.nals[0][0] & 0x1F, 7, "first NAL must be SPS");
    assert_eq!(parsed.nals[1][0] & 0x1F, 8, "second NAL must be PPS");
}

#[test]
fn bear_avcc_at_construction_initializes_decoder() {
    // Constructing a decoder with bear.mp4's avcC as extradata must
    // succeed: the parser accepts it, the inner SPS parser accepts
    // the High-profile SPS, and the parameter-set tables get
    // populated. No actual samples are fed.
    let config = DecoderConfig::new(Codec::H264).with_custom_params(H264Config {
        bitstream_format: AvcBitstreamFormat::Avc,
        extradata: Some(BEAR_AVCC.to_vec()),
    });
    let _decoder = default_decoder(config, CountingCallbacks::shared())
        .expect("construct with bear.mp4's avcC");
}

#[test]
fn bear_avcc_via_control_initializes_decoder() {
    let config = DecoderConfig::new(Codec::H264).with_custom_params(H264Config {
        bitstream_format: AvcBitstreamFormat::Avc,
        extradata: None,
    });
    let mut decoder: Box<dyn VideoDecoder> = Box::new(
        Decoder::new(config, Arc::new(DefaultAllocator), CountingCallbacks::shared())
            .expect("construct"),
    );
    let mut cmd = H264SetExtradata { data: BEAR_AVCC.to_vec() };
    decoder.control(&mut cmd).expect("apply BEAR_AVCC via control");
}

// ---------------------------------------------------------------
// Smoke tests
// ---------------------------------------------------------------

#[test]
fn extradata_then_decode_succeeds_via_avc_framing() {
    // Smoke-test: a single decode pass through the (avcC + AVC
    // samples) path drains cleanly and the queue is empty at end.
    let (sps, pps, samples) = bucket_fixture_nals(&fixture(BASELINE_BFRAME_FIXTURE));
    let avcc = build_avcc(&sps, &pps, 4);

    let config = DecoderConfig::new(Codec::H264).with_custom_params(H264Config {
        bitstream_format: AvcBitstreamFormat::Avc,
        extradata: Some(avcc),
    });
    let mut decoder: Box<dyn VideoDecoder> = Box::new(
        Decoder::new(config, Arc::new(DefaultAllocator), CountingCallbacks::shared())
            .expect("construct"),
    );

    let packets = avc_packets_from_nals(&samples, 4);
    drive_through(decoder.as_mut(), packets).expect("drive");
    // After drain the queue is empty.
    assert!(decoder.get_picture().unwrap().is_none());
    // Flush(Drain) on an idle decoder is a no-op.
    decoder.flush(FlushMode::Drain).expect("idle drain");
    assert!(decoder.get_picture().unwrap().is_none());
}
