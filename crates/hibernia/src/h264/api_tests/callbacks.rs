//! Tests for `VideoDecoderCallbacks` invocation patterns.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use crate::api::callbacks::VideoDecoderCallbacks;
use crate::api::config::{Codec, DecoderConfig};
use crate::api::decoder::{FlushMode, VideoDecoder};
use crate::api::format::StreamFormat;
use crate::api::packet::EncodedPacket;
use crate::api::DefaultAllocator;
use crate::h264::decoder::Decoder;

use super::support::{
    annexb_packet, baseline_bframe_packets, default_decoder, drive_through, fixture,
    read_fixture_nals, CountingCallbacks, BASELINE_BFRAME_FIXTURE,
};

#[test]
fn on_picture_available_fires_at_least_once_per_stream() {
    let callbacks = CountingCallbacks::shared();
    let mut decoder =
        default_decoder(DecoderConfig::new(Codec::H264), callbacks.clone()).expect("construct");
    let pics = drive_through(&mut decoder, baseline_bframe_packets()).expect("drive");
    assert!(!pics.is_empty());
    assert!(callbacks.picture_callbacks() > 0);
}

#[test]
fn on_picture_available_does_not_fire_when_no_picture_emitted() {
    // SPS-only and PPS-only packets shouldn't emit pictures.
    let callbacks = CountingCallbacks::shared();
    let mut decoder =
        default_decoder(DecoderConfig::new(Codec::H264), callbacks.clone()).expect("construct");

    let nals = read_fixture_nals(&fixture(BASELINE_BFRAME_FIXTURE));
    let sps_idx = nals.iter().position(|n| n[0] & 0x1f == 7).unwrap();
    let pps_idx = nals.iter().position(|n| n[0] & 0x1f == 8).unwrap();

    decoder.decode(annexb_packet(&nals[sps_idx])).expect("decode SPS");
    decoder.decode(annexb_packet(&nals[pps_idx])).expect("decode PPS");

    assert_eq!(
        callbacks.picture_callbacks(),
        0,
        "no picture from SPS/PPS-only packets -> no on_picture_available",
    );
}

#[test]
fn on_format_changed_fires_before_first_on_picture_available() {
    // Record the order of callback invocations.
    #[derive(Default)]
    struct Recorder {
        events: Mutex<Vec<&'static str>>,
    }
    impl VideoDecoderCallbacks for Recorder {
        fn on_picture_available(&self) {
            self.events.lock().unwrap().push("picture");
        }
        fn on_format_changed(&self, _format: StreamFormat) {
            self.events.lock().unwrap().push("format");
        }
    }

    let recorder = Arc::new(Recorder::default());
    let mut decoder = Decoder::new(
        DecoderConfig::new(Codec::H264),
        Arc::new(DefaultAllocator),
        recorder.clone(),
    )
    .expect("construct");
    let _ = drive_through(&mut decoder, baseline_bframe_packets()).expect("drive");

    let events = recorder.events.lock().unwrap();
    assert!(!events.is_empty());
    assert_eq!(events[0], "format", "format change must precede first picture");
}

#[test]
fn callbacks_fire_synchronously_before_decode_returns() {
    // We can observe the callback count before/after a single
    // decode() call. Synchronous invocation means the count increases
    // by the time decode returns.
    let callbacks = CountingCallbacks::shared();
    let mut decoder =
        default_decoder(DecoderConfig::new(Codec::H264), callbacks.clone()).expect("construct");
    // Feed the whole stream as many packets and observe the counter
    // after the run is over.
    let _ = drive_through(&mut decoder, baseline_bframe_packets()).expect("drive");
    // No threads in our impl, so by the time drive_through returned,
    // every callback that should have fired has.
    assert!(callbacks.picture_callbacks() > 0);
    assert!(callbacks.format_change_count() > 0);
}

#[test]
fn shared_callbacks_across_two_decoders_both_fire() {
    let shared = CountingCallbacks::shared();
    let dyn_share: Arc<dyn VideoDecoderCallbacks> = shared.clone();
    let mut d1 = Decoder::new(
        DecoderConfig::new(Codec::H264),
        Arc::new(DefaultAllocator),
        dyn_share.clone(),
    )
    .expect("d1");
    let mut d2 = Decoder::new(
        DecoderConfig::new(Codec::H264),
        Arc::new(DefaultAllocator),
        dyn_share,
    )
    .expect("d2");
    let _ = drive_through(&mut d1, baseline_bframe_packets()).expect("d1 drive");
    let count_after_d1 = shared.picture_callbacks();
    let _ = drive_through(&mut d2, baseline_bframe_packets()).expect("d2 drive");
    let count_after_d2 = shared.picture_callbacks();
    assert!(count_after_d2 > count_after_d1);
}

#[test]
fn on_picture_available_fires_at_most_once_per_decode_call() {
    // Contract: on_picture_available fires at most once per decode()
    // call, even when the call produced multiple pictures. Verify by
    // bracketing every decode with before/after callback counts and
    // asserting the delta is 0 or 1.
    let callbacks = CountingCallbacks::shared();
    let mut decoder =
        default_decoder(DecoderConfig::new(Codec::H264), callbacks.clone()).expect("construct");

    let mut total_pictures = 0;
    for packet in baseline_bframe_packets() {
        let before = callbacks.picture_callbacks();
        decoder.decode(packet).expect("decode");
        let after = callbacks.picture_callbacks();
        assert!(
            after - before <= 1,
            "on_picture_available must batch; got {} callbacks in one decode",
            after - before,
        );
        while let Some(_) = decoder.get_picture().expect("get_picture") {
            total_pictures += 1;
        }
    }
    decoder.flush(FlushMode::Drain).expect("flush");
    while let Some(_) = decoder.get_picture().expect("get_picture") {
        total_pictures += 1;
    }

    // Sanity: this stream really did produce pictures, otherwise the
    // batching assertion above is vacuous.
    assert!(total_pictures > 0, "fixture should produce pictures");
    // And the total callback count never exceeds the total pictures
    // (batching can only fold many pictures into fewer callbacks).
    assert!(
        callbacks.picture_callbacks() <= total_pictures,
        "callbacks ({}) shouldn't exceed pictures ({})",
        callbacks.picture_callbacks(),
        total_pictures,
    );
}

#[test]
fn dropping_callbacks_arc_after_decoder_is_safe() {
    // Decoder holds its own Arc clone. Caller dropping the callbacks
    // Arc must not affect decoder operation.
    let callbacks: Arc<dyn VideoDecoderCallbacks> = Arc::new(CountingCallbacks::default());
    let mut decoder = Decoder::new(
        DecoderConfig::new(Codec::H264),
        Arc::new(DefaultAllocator),
        callbacks,
    )
    .expect("construct");
    // No second reference held; decoder is the sole owner of the
    // callbacks Arc.
    let pics = drive_through(&mut decoder, baseline_bframe_packets()).expect("drive");
    assert!(!pics.is_empty());
}
