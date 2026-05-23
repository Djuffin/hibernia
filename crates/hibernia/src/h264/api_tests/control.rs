//! Tests for `VideoDecoder::control`. The H.264 implementation
//! defines no control commands and surfaces `FeatureNotSupported`
//! for every payload.

use std::any::Any;

use crate::api::callbacks::DecoderError;
use crate::api::config::{Codec, DecoderConfig};
use crate::api::decoder::VideoDecoder;

use super::support::{default_decoder, CountingCallbacks};

#[test]
fn unknown_control_payload_returns_feature_not_supported() {
    let mut decoder =
        default_decoder(DecoderConfig::new(Codec::H264), CountingCallbacks::shared())
            .expect("construct");

    let mut payload: Box<dyn Any> = Box::new(0xCAFEu32);
    let err = decoder.control(&mut *payload).unwrap_err();
    assert!(matches!(err, DecoderError::FeatureNotSupported(_)));
}

#[test]
fn control_does_not_mutate_payload_on_failure() {
    // The trait reserves the right to write back through &mut on
    // success. On the FeatureNotSupported path, the payload must
    // remain whatever the caller passed.
    let mut decoder =
        default_decoder(DecoderConfig::new(Codec::H264), CountingCallbacks::shared())
            .expect("construct");

    #[derive(Debug, PartialEq, Eq)]
    struct Sentinel(u64);
    let mut payload: Box<dyn Any> = Box::new(Sentinel(0xDEADBEEF));
    let _ = decoder.control(&mut *payload).unwrap_err();
    let observed = payload.downcast_ref::<Sentinel>().expect("downcast");
    assert_eq!(*observed, Sentinel(0xDEADBEEF), "payload must be unchanged");
}

#[test]
fn control_payload_can_be_unit_struct() {
    // Smoke-test that arbitrary zero-sized payloads also work.
    let mut decoder =
        default_decoder(DecoderConfig::new(Codec::H264), CountingCallbacks::shared())
            .expect("construct");
    struct Cmd;
    let mut payload: Box<dyn Any> = Box::new(Cmd);
    let err = decoder.control(&mut *payload).unwrap_err();
    assert!(matches!(err, DecoderError::FeatureNotSupported(_)));
}
