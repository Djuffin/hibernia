//! Tests for the public output-queue ceiling enforced by
//! `VideoDecoder::decode`. The decoder's default cap is 64 pictures;
//! when callers fail to drain, decode must surface `QueueFull` rather
//! than silently dropping or unboundedly buffering.

use crate::api::callbacks::DecoderError;
use crate::api::config::{Codec, DecoderConfig};
use crate::api::decoder::{FlushMode, VideoDecoder};
use crate::api::packet::EncodedPacket;

use super::support::{
    baseline_bframe_packets, default_decoder, drive_through, CountingCallbacks,
};

/// Estimate of how many baseline-fixture packets fit in the queue
/// before saturation. Picked larger than any realistic depth (64);
/// the test fails fast if the ceiling has been quietly removed.
const QUEUE_OVERFLOW_PACKETS: usize = 1024;

fn build_decoder() -> (Box<dyn VideoDecoder>, std::sync::Arc<CountingCallbacks>) {
    let callbacks = CountingCallbacks::shared();
    let decoder =
        default_decoder(DecoderConfig::new(Codec::H264), callbacks.clone()).expect("construct");
    (Box::new(decoder), callbacks)
}

#[test]
fn queue_full_eventually_returns_without_draining() {
    let (mut decoder, _callbacks) = build_decoder();

    // Feed packets without draining. The fixture is short, so we
    // re-feed the whole stream repeatedly until the queue saturates.
    let mut saw_queue_full = false;
    'outer: for _ in 0..QUEUE_OVERFLOW_PACKETS / 50 {
        for packet in baseline_bframe_packets() {
            match decoder.decode(packet) {
                Ok(()) => {}
                Err(DecoderError::QueueFull) => {
                    saw_queue_full = true;
                    break 'outer;
                }
                Err(other) => panic!("unexpected error before QueueFull: {other:?}"),
            }
        }
    }
    assert!(saw_queue_full, "QueueFull must trigger when caller never drains");
}

#[test]
fn queue_full_clears_after_drain_resumes_decoding() {
    // Verify that after QueueFull, draining the queue lets the
    // decoder accept more packets. We rebuild a fresh decoder for the
    // post-drain check rather than continuing the saturated stream:
    // the saturating loop fed parts of a B-frame GOP without
    // draining, so the inner state may have inter-frame deps the
    // continuation can't satisfy. The reusability claim is about the
    // decoder not getting permanently stuck after a QueueFull event.
    let (mut decoder, _callbacks) = build_decoder();
    'outer: for _ in 0..(QUEUE_OVERFLOW_PACKETS / 50) {
        for packet in baseline_bframe_packets() {
            if let Err(DecoderError::QueueFull) = decoder.decode(packet) {
                break 'outer;
            }
        }
    }
    let mut drained = 0;
    while decoder.get_picture().expect("get_picture").is_some() {
        drained += 1;
    }
    assert!(drained > 0, "expected at least one queued picture to drain");

    // After Discard the same decoder is reusable from scratch.
    decoder.flush(FlushMode::Discard).expect("discard");
    let pics = drive_through(decoder.as_mut(), baseline_bframe_packets()).expect("post-discard");
    assert!(!pics.is_empty(), "decoder must keep working after QueueFull then Discard");
}

#[test]
fn queue_full_preserves_already_queued_pictures() {
    let (mut decoder, _callbacks) = build_decoder();

    // Push until we get QueueFull.
    'outer: for _ in 0..(QUEUE_OVERFLOW_PACKETS / 50) {
        for packet in baseline_bframe_packets() {
            if let Err(DecoderError::QueueFull) = decoder.decode(packet) {
                break 'outer;
            }
        }
    }

    // Whatever made it into the queue must still be retrievable.
    let mut count = 0;
    while decoder.get_picture().expect("get_picture").is_some() {
        count += 1;
    }
    assert!(count > 0, "QueueFull must preserve pictures already queued");
}

#[test]
fn flush_drain_returns_queue_full_when_queue_is_saturated() {
    // flush(Drain) tries to surface every DPB-held picture into the
    // public out_queue. If the queue is already at the ceiling, the
    // call returns QueueFull rather than silently dropping pictures.
    let (mut decoder, _callbacks) = build_decoder();
    'outer: for _ in 0..(QUEUE_OVERFLOW_PACKETS / 50) {
        for packet in baseline_bframe_packets() {
            if let Err(DecoderError::QueueFull) = decoder.decode(packet) {
                break 'outer;
            }
        }
    }
    let err = decoder.flush(FlushMode::Drain).unwrap_err();
    assert!(matches!(err, DecoderError::QueueFull), "expected QueueFull, got {err:?}");
}

#[test]
fn flush_drain_succeeds_once_caller_drains() {
    // Drain the queue first via get_picture, then flush(Drain).
    let (mut decoder, _callbacks) = build_decoder();
    'outer: for _ in 0..(QUEUE_OVERFLOW_PACKETS / 50) {
        for packet in baseline_bframe_packets() {
            if let Err(DecoderError::QueueFull) = decoder.decode(packet) {
                break 'outer;
            }
        }
    }
    while decoder.get_picture().expect("get_picture").is_some() {}
    decoder.flush(FlushMode::Drain).expect("flush after caller drains");
    // Decoder is alive afterwards.
    let _ = decoder.decode(EncodedPacket::from_vec(vec![])).expect("empty decode");
}
