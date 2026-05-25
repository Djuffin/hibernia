//! Tests for `StreamFormat` content and `on_format_changed`
//! semantics observed through the public API.

use std::sync::Arc;

use crate::api::config::{Codec, DecoderConfig};
use crate::api::decoder::{FlushMode, VideoDecoder};
use crate::api::{ColorRange, PixelFormat};

use super::support::{
    annexb_packet, baseline_bframe_packets, default_decoder, drive_through, fixture,
    read_fixture_nals, CountingCallbacks,
};

#[test]
fn single_sps_stream_fires_format_changed_exactly_once() {
    let callbacks = CountingCallbacks::shared();
    let mut decoder = default_decoder(DecoderConfig::new(Codec::H264), callbacks.clone())
        .expect("construct");
    let pics = drive_through(&mut decoder, baseline_bframe_packets()).expect("drive");
    assert!(!pics.is_empty());
    assert_eq!(
        callbacks.format_change_count(),
        1,
        "format_changed should fire exactly once for a single-SPS stream",
    );
}

#[test]
fn on_format_changed_payload_matches_decoded_picture_format() {
    let callbacks = CountingCallbacks::shared();
    let mut decoder =
        default_decoder(DecoderConfig::new(Codec::H264), callbacks.clone()).expect("construct");
    let pics = drive_through(&mut decoder, baseline_bframe_packets()).expect("drive");
    let from_callback = callbacks
        .last_format
        .lock()
        .unwrap()
        .clone()
        .expect("on_format_changed must have fired");
    for pic in &pics {
        assert_eq!(pic.format, from_callback);
    }
}

#[test]
fn pixel_format_is_i420() {
    let callbacks = CountingCallbacks::shared();
    let mut decoder =
        default_decoder(DecoderConfig::new(Codec::H264), callbacks).expect("construct");
    let pics = drive_through(&mut decoder, baseline_bframe_packets()).expect("drive");
    for pic in &pics {
        assert_eq!(pic.format.pixel_format, PixelFormat::I420);
    }
}

#[test]
fn bit_depth_is_eight() {
    let callbacks = CountingCallbacks::shared();
    let mut decoder =
        default_decoder(DecoderConfig::new(Codec::H264), callbacks).expect("construct");
    let pics = drive_through(&mut decoder, baseline_bframe_packets()).expect("drive");
    for pic in &pics {
        assert_eq!(pic.format.bit_depth, 8);
    }
}

#[test]
fn no_color_space_when_vui_absent() {
    // The baseline fixture has no VUI signaling. color_space must be None.
    let callbacks = CountingCallbacks::shared();
    let mut decoder =
        default_decoder(DecoderConfig::new(Codec::H264), callbacks).expect("construct");
    let pics = drive_through(&mut decoder, baseline_bframe_packets()).expect("drive");
    for pic in &pics {
        assert!(pic.format.color_space.is_none(), "no VUI -> no color_space");
    }
}

#[test]
fn codec_field_is_h264() {
    let callbacks = CountingCallbacks::shared();
    let mut decoder =
        default_decoder(DecoderConfig::new(Codec::H264), callbacks).expect("construct");
    let pics = drive_through(&mut decoder, baseline_bframe_packets()).expect("drive");
    assert!(pics.iter().all(|p| p.format.codec == Codec::H264));
}

#[test]
fn discard_resets_last_format_and_refires_on_next_decode() {
    let callbacks = CountingCallbacks::shared();
    let mut decoder =
        default_decoder(DecoderConfig::new(Codec::H264), callbacks.clone()).expect("construct");

    let first = drive_through(&mut decoder, baseline_bframe_packets()).expect("first");
    assert!(!first.is_empty());
    assert_eq!(callbacks.format_change_count(), 1);

    decoder.flush(FlushMode::Discard).expect("discard");

    // Feed the same stream; Discard cleared last_format, so the
    // callback fires again even though the format is identical.
    let second = drive_through(&mut decoder, baseline_bframe_packets()).expect("second");
    assert!(!second.is_empty());
    assert_eq!(
        callbacks.format_change_count(),
        2,
        "format_changed must refire after Discard",
    );
}

#[test]
fn coded_geometry_is_macroblock_aligned() {
    let callbacks = CountingCallbacks::shared();
    let mut decoder =
        default_decoder(DecoderConfig::new(Codec::H264), callbacks).expect("construct");
    let pics = drive_through(&mut decoder, baseline_bframe_packets()).expect("drive");
    for pic in &pics {
        assert!(pic.format.coded_width >= pic.format.display_width);
        assert!(pic.format.coded_height >= pic.format.display_height);
        assert_eq!(pic.format.coded_width % 16, 0);
        assert_eq!(pic.format.coded_height % 16, 0);
    }
}

#[test]
fn crop_offsets_consistent_with_display_geometry() {
    let callbacks = CountingCallbacks::shared();
    let mut decoder =
        default_decoder(DecoderConfig::new(Codec::H264), callbacks).expect("construct");
    let pics = drive_through(&mut decoder, baseline_bframe_packets()).expect("drive");
    for pic in &pics {
        assert!(pic.format.crop_left + pic.format.display_width <= pic.format.coded_width);
        assert!(pic.format.crop_top + pic.format.display_height <= pic.format.coded_height);
    }
}

#[test]
fn streams_at_different_resolutions_report_different_formats() {
    // Feed two streams in succession (Discard between) and verify
    // their reported coded dimensions differ. Confirms the format
    // pipeline reflects per-SPS state rather than caching the first.
    let callbacks = CountingCallbacks::shared();
    let mut decoder =
        default_decoder(DecoderConfig::new(Codec::H264), callbacks.clone()).expect("construct");

    let small = drive_through(&mut decoder, baseline_bframe_packets()).expect("small");
    let small_format = small[0].format.clone();

    decoder.flush(FlushMode::Discard).expect("discard");

    // CAWP1_TOSHIBA_E is 352x288 CIF, distinct from SVA_BA2_D's
    // 176x144 QCIF. Both fixtures are committed in-tree so CI can
    // see them.
    let large_nals = read_fixture_nals(&fixture("data/CAWP1_TOSHIBA_E/CAWP1_TOSHIBA_E.264"));
    let large_packets: Vec<_> = large_nals.iter().map(|n| annexb_packet(n)).collect();
    let large = drive_through(&mut decoder, large_packets).expect("large");

    assert!(!large.is_empty(), "second-resolution fixture must decode");
    let large_format = large[0].format.clone();
    assert_ne!(small_format.coded_width, large_format.coded_width);
    assert_ne!(small_format.coded_height, large_format.coded_height);
    // Each format announcement should have fired.
    assert_eq!(callbacks.format_change_count(), 2);
}

#[test]
fn color_range_defaults_to_limited_when_absent() {
    // Without a VUI, the API doesn't construct a ColorSpace at all.
    // This is the "absent => None" path verified in no_color_space_when_vui_absent.
    // What we additionally check here is the unit-test helper in
    // h264::format that maps a hypothetical "VUI present but no
    // color description" case to Limited. The behavior is exercised
    // by the format-mapping unit tests in h264::format::tests; this
    // test just asserts the type-level default for ColorRange.
    assert_eq!(ColorRange::default(), ColorRange::Limited);
}
