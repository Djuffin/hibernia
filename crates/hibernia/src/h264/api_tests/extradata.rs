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
// Other smoke tests
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
