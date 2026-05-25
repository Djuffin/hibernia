//! Tests for `H264Config::extradata` and `H264SetExtradata`.

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

/// Concatenate NALs with 4-byte start codes.
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

    let packets = avc_packets_from_nals(&samples, 4);
    let pics = drive_through(&mut decoder, packets).expect("drive");
    assert!(!pics.is_empty());
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
    let mut decoder = default_decoder(config, CountingCallbacks::shared()).expect("construct");

    let packets = annexb_packets_from_nals(&samples);
    let pics = drive_through(&mut decoder, packets).expect("drive");
    assert!(!pics.is_empty());
}

#[test]
fn extradata_path_yields_same_frame_count_as_in_band_path() {
    let (sps, pps, samples) = bucket_fixture_nals(&fixture(BASELINE_BFRAME_FIXTURE));

    let in_band_count = {
        let mut decoder =
            default_decoder(DecoderConfig::new(Codec::H264), CountingCallbacks::shared())
                .expect("in-band decoder");
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
    // SVA_BA2_D is QCIF, so all sample NALs fit in a 2-byte length.
    let (sps, pps, samples) = bucket_fixture_nals(&fixture(BASELINE_BFRAME_FIXTURE));
    for nal in &samples {
        assert!(nal.len() <= 0xFFFF);
    }
    let avcc = build_avcc(&sps, &pps, 2);

    let config = DecoderConfig::new(Codec::H264).with_custom_params(H264Config {
        bitstream_format: AvcBitstreamFormat::Avc,
        extradata: Some(avcc),
    });
    let mut decoder =
        default_decoder(config, CountingCallbacks::shared()).expect("construct");

    let packets = avc_packets_from_nals(&samples, 2);
    let pics = drive_through(&mut decoder, packets).expect("drive");
    assert!(!pics.is_empty());
}

#[test]
fn empty_extradata_is_accepted_and_noop() {
    let config = DecoderConfig::new(Codec::H264).with_custom_params(H264Config {
        bitstream_format: AvcBitstreamFormat::AnnexB,
        extradata: Some(Vec::new()),
    });
    let _ = default_decoder(config, CountingCallbacks::shared()).expect("construct");
}

#[test]
fn malformed_avcc_extradata_fails_construction() {
    let config = DecoderConfig::new(Codec::H264).with_custom_params(H264Config {
        bitstream_format: AvcBitstreamFormat::Avc,
        extradata: Some(vec![0x01, 0x02]), // avcC-tag + truncated header
    });
    let err = default_decoder(config, CountingCallbacks::shared()).unwrap_err();
    assert!(matches!(err, DecoderError::MisformedData(_)), "got {err:?}");
}

#[test]
fn malformed_annexb_extradata_skips_silently() {
    // Annex-B-form input with no start codes yields zero NALs; not an error.
    let config = DecoderConfig::new(Codec::H264).with_custom_params(H264Config {
        bitstream_format: AvcBitstreamFormat::AnnexB,
        extradata: Some(vec![0x42, 0x99, 0xff]),
    });
    let _ = default_decoder(config, CountingCallbacks::shared()).expect("construct");
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
    let (sps, pps, samples) = bucket_fixture_nals(&fixture(BASELINE_BFRAME_FIXTURE));
    let avcc = build_avcc(&sps, &pps, 4);

    let config = DecoderConfig::new(Codec::H264).with_custom_params(H264Config {
        bitstream_format: AvcBitstreamFormat::Avc,
        extradata: None,
    });
    let mut decoder: Box<dyn VideoDecoder> = Box::new(
        Decoder::new(config, Arc::new(DefaultAllocator), CountingCallbacks::shared())
            .expect("construct"),
    );

    let mut cmd = H264SetExtradata { data: avcc };
    decoder.control(&mut cmd).expect("control");

    let packets = avc_packets_from_nals(&samples, 4);
    let pics = drive_through(decoder.as_mut(), packets).expect("drive");
    assert!(!pics.is_empty());
}

#[test]
fn set_extradata_via_control_with_annexb_blob_works() {
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
    decoder.control(&mut cmd).expect("control");

    let packets = annexb_packets_from_nals(&samples);
    let pics = drive_through(decoder.as_mut(), packets).expect("drive");
    assert!(!pics.is_empty());
}

#[test]
fn set_extradata_via_control_after_partial_decode() {
    // Feed in-band SPS+PPS, then re-apply via control(). Decoding
    // must continue uninterrupted.
    let (sps, pps, samples) = bucket_fixture_nals(&fixture(BASELINE_BFRAME_FIXTURE));

    let config = DecoderConfig::new(Codec::H264).with_custom_params(H264Config {
        bitstream_format: AvcBitstreamFormat::AnnexB,
        extradata: None,
    });
    let mut decoder: Box<dyn VideoDecoder> = Box::new(
        Decoder::new(config, Arc::new(DefaultAllocator), CountingCallbacks::shared())
            .expect("construct"),
    );

    for nal in sps.iter().chain(pps.iter()) {
        decoder.decode(annexb_packet(nal)).expect("decode parameter set");
    }

    let avcc = build_avcc(&sps, &pps, 4);
    let mut cmd = H264SetExtradata { data: avcc };
    decoder.control(&mut cmd).expect("control");

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

    let mut cmd = H264SetExtradata { data: vec![0x01, 0x02] };
    let err = decoder.control(&mut cmd).unwrap_err();
    assert!(matches!(err, DecoderError::MisformedData(_)), "got {err:?}");
}

// Unknown control payloads: see api_tests/control.rs.

// Real-world avcC + sample layout from data/bear.mp4, extracted once
// with ffprobe so the tests don't need to walk MP4 boxes.

/// avcC blob from data/bear.mp4. High profile, 320x180, 30 fps.
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
    let parsed = crate::h264::extradata::parse_extradata(BEAR_AVCC).expect("parse BEAR_AVCC");
    assert_eq!(parsed.length_size, Some(4));
    assert_eq!(parsed.nals.len(), 2);
    assert_eq!(parsed.nals[0][0] & 0x1F, 7); // SPS
    assert_eq!(parsed.nals[1][0] & 0x1F, 8); // PPS
}

#[test]
fn bear_avcc_at_construction_initializes_decoder() {
    let config = DecoderConfig::new(Codec::H264).with_custom_params(H264Config {
        bitstream_format: AvcBitstreamFormat::Avc,
        extradata: Some(BEAR_AVCC.to_vec()),
    });
    let _ = default_decoder(config, CountingCallbacks::shared()).expect("construct");
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
    decoder.control(&mut cmd).expect("control");
}

/// mdat sample (file_offset, byte_size) tuples in decode order,
/// from `ffprobe -show_packets ... data/bear.mp4`.
const BEAR_SAMPLES: &[(usize, usize)] = &[
    (6619, 6355),  (12974, 1028), (14002, 216),  (14218, 1223), (15441, 329),
    (15770, 1160), (16930, 260),  (17190, 1071), (18261, 212),  (18473, 1014),
    (19487, 242),  (19729, 1271), (21000, 220),  (21220, 1375), (22595, 318),
    (27600, 1310), (28910, 359),  (29269, 1275), (30544, 357),  (30901, 1462),
    (32363, 357),  (32720, 1287), (34007, 283),  (34290, 1261), (35551, 344),
    (35895, 1368), (37263, 358),  (37621, 1433), (39054, 403),  (39925, 1075),
];

/// Decode bear.mp4 with parameter sets delivered via the chosen
/// strategy; compare per-pixel against ffmpeg's golden y4m.
fn bear_avc_matches_ffmpeg_golden_with(
    extradata_delivery: ExtradataDelivery,
) -> Result<(), String> {
    use std::fs;

    use super::support::{pictures_to_y4m_bytes, run_ffmpeg, workspace_root, TestDir};
    use crate::api::EncodedPacket;
    use crate::y4m_cmp::compare_y4m_buffers;

    let bear_mp4 = workspace_root().join("data/bear.mp4");
    if !bear_mp4.exists() {
        println!("{} missing, skipping test", bear_mp4.display());
        return Ok(());
    }
    let mp4_bytes = fs::read(&bear_mp4).map_err(|e| format!("read bear.mp4: {e}"))?;

    let test_dir =
        TestDir::new("target/tmp_extradata_bear_golden").map_err(|e| e.to_string())?;
    let golden_path = test_dir.join("golden.y4m");
    let mp4_str = bear_mp4.to_string_lossy().into_owned();
    let golden_str = golden_path.to_string_lossy().into_owned();
    if !run_ffmpeg(&["-y", "-v", "error", "-i", &mp4_str, &golden_str])? {
        return Ok(());
    }
    let expected_y4m = fs::read(&golden_path).map_err(|e| format!("read golden: {e}"))?;

    let (config, send_control) = match extradata_delivery {
        ExtradataDelivery::AtConstruction => (
            DecoderConfig::new(Codec::H264).with_custom_params(H264Config {
                bitstream_format: AvcBitstreamFormat::Avc,
                extradata: Some(BEAR_AVCC.to_vec()),
            }),
            false,
        ),
        ExtradataDelivery::ViaControl => (
            DecoderConfig::new(Codec::H264).with_custom_params(H264Config {
                bitstream_format: AvcBitstreamFormat::Avc,
                extradata: None,
            }),
            true,
        ),
    };
    let mut decoder: Box<dyn VideoDecoder> = Box::new(
        Decoder::new(config, Arc::new(DefaultAllocator), CountingCallbacks::shared())
            .map_err(|e| format!("construct: {e:?}"))?,
    );
    if send_control {
        let mut cmd = H264SetExtradata { data: BEAR_AVCC.to_vec() };
        decoder.control(&mut cmd).map_err(|e| format!("control: {e:?}"))?;
    }

    let packets: Vec<_> = BEAR_SAMPLES
        .iter()
        .map(|(off, size)| EncodedPacket::from_vec(mp4_bytes[*off..*off + *size].to_vec()))
        .collect();
    let pictures =
        drive_through(decoder.as_mut(), packets).map_err(|e| format!("drive: {e:?}"))?;
    assert_eq!(pictures.len(), 30, "expected 30 frames, got {}", pictures.len());

    let actual_y4m = pictures_to_y4m_bytes(&pictures, y4m::Ratio { num: 30000, den: 1001 });
    compare_y4m_buffers(&actual_y4m, &expected_y4m)
}

enum ExtradataDelivery {
    AtConstruction,
    ViaControl,
}

#[test]
fn bear_avc_extradata_at_construction_matches_ffmpeg_golden() -> Result<(), String> {
    bear_avc_matches_ffmpeg_golden_with(ExtradataDelivery::AtConstruction)
}

#[test]
fn bear_avc_extradata_via_control_matches_ffmpeg_golden() -> Result<(), String> {
    bear_avc_matches_ffmpeg_golden_with(ExtradataDelivery::ViaControl)
}

#[test]
fn bear_avc_control_overwrites_construction_extradata() -> Result<(), String> {
    // Install SVA_BA2_D's parameter sets at construction (176x144
    // baseline), overwrite via control() with bear's avcC (320x180
    // High), then decode bear's samples. Both fixtures use
    // parameter-set id 0, so a no-op control() would leave the wrong
    // SPS in the table and decoding would error.
    use std::fs;

    use super::support::{
        bucket_fixture_nals, pictures_to_y4m_bytes, run_ffmpeg, workspace_root, TestDir,
    };
    use crate::api::EncodedPacket;
    use crate::y4m_cmp::compare_y4m_buffers;

    let bear_mp4 = workspace_root().join("data/bear.mp4");
    if !bear_mp4.exists() {
        println!("{} missing, skipping test", bear_mp4.display());
        return Ok(());
    }
    let mp4_bytes = fs::read(&bear_mp4).map_err(|e| format!("read bear.mp4: {e}"))?;

    let test_dir =
        TestDir::new("target/tmp_extradata_bear_overwrite").map_err(|e| e.to_string())?;
    let golden_path = test_dir.join("golden.y4m");
    let mp4_str = bear_mp4.to_string_lossy().into_owned();
    let golden_str = golden_path.to_string_lossy().into_owned();
    if !run_ffmpeg(&["-y", "-v", "error", "-i", &mp4_str, &golden_str])? {
        return Ok(());
    }
    let expected_y4m = fs::read(&golden_path).map_err(|e| format!("read golden: {e}"))?;

    let (other_sps, other_pps, _) = bucket_fixture_nals(&fixture(BASELINE_BFRAME_FIXTURE));
    let wrong_avcc = build_avcc(&other_sps, &other_pps, 4);
    assert_ne!(wrong_avcc.as_slice(), BEAR_AVCC);

    let config = DecoderConfig::new(Codec::H264).with_custom_params(H264Config {
        bitstream_format: AvcBitstreamFormat::Avc,
        extradata: Some(wrong_avcc),
    });
    let mut decoder: Box<dyn VideoDecoder> = Box::new(
        Decoder::new(config, Arc::new(DefaultAllocator), CountingCallbacks::shared())
            .map_err(|e| format!("construct: {e:?}"))?,
    );

    let mut cmd = H264SetExtradata { data: BEAR_AVCC.to_vec() };
    decoder.control(&mut cmd).map_err(|e| format!("control: {e:?}"))?;

    let packets: Vec<_> = BEAR_SAMPLES
        .iter()
        .map(|(off, size)| EncodedPacket::from_vec(mp4_bytes[*off..*off + *size].to_vec()))
        .collect();
    let pictures =
        drive_through(decoder.as_mut(), packets).map_err(|e| format!("drive: {e:?}"))?;
    assert_eq!(pictures.len(), 30, "expected 30 frames, got {}", pictures.len());

    let actual_y4m = pictures_to_y4m_bytes(&pictures, y4m::Ratio { num: 30000, den: 1001 });
    compare_y4m_buffers(&actual_y4m, &expected_y4m)
}

#[test]
fn extradata_then_decode_drains_cleanly() {
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
    assert!(decoder.get_picture().unwrap().is_none());
    decoder.flush(FlushMode::Drain).expect("idle drain");
    assert!(decoder.get_picture().unwrap().is_none());
}
