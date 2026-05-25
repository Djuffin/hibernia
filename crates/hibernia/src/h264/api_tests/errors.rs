//! Tests for `DecoderError` shape, trait impls, and construction
//! from real malformed inputs.

use crate::api::bitstream::{AvcBitstreamFormat, H264Config};
use crate::api::callbacks::DecoderError;
use crate::api::config::{Codec, DecoderConfig};
use crate::api::decoder::VideoDecoder;
use crate::api::frame::AllocError;
use crate::api::packet::EncodedPacket;

use super::support::{default_decoder, CountingCallbacks};

#[test]
fn decoder_error_implements_display() {
    let cases = [
        DecoderError::InitializationFailed("init".into()),
        DecoderError::MisformedData("md".into()),
        DecoderError::OutOfRange("oor".into()),
        DecoderError::ReferenceNotFound("rnf".into()),
        DecoderError::FeatureNotSupported("fns".into()),
        DecoderError::QueueFull,
        DecoderError::Alloc(AllocError::OutOfMemory),
        DecoderError::Fatal("dead".into()),
    ];
    for case in cases {
        let s = format!("{case}");
        assert!(!s.is_empty(), "Display impl produced empty string for {case:?}");
    }
}

#[test]
fn decoder_error_implements_std_error() {
    fn assert_error<T: std::error::Error>() {}
    assert_error::<DecoderError>();
}

#[test]
fn decoder_error_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<DecoderError>();
}

#[test]
fn decoder_error_is_clone_and_partial_eq() {
    let a = DecoderError::MisformedData("x".into());
    let b = a.clone();
    assert_eq!(a, b);
    let c = DecoderError::MisformedData("y".into());
    assert_ne!(a, c);
}

#[test]
fn unsupported_codec_returns_feature_not_supported() {
    let callbacks = CountingCallbacks::shared();
    for codec in [Codec::VP8, Codec::VP9, Codec::AV1, Codec::AV2] {
        let result = crate::api::create_decoder(
            DecoderConfig::new(codec),
            std::sync::Arc::new(crate::api::DefaultAllocator),
            callbacks.clone(),
        );
        let err = result.map(|_| ()).unwrap_err();
        assert!(
            matches!(err, DecoderError::FeatureNotSupported(_)),
            "{codec:?} should return FeatureNotSupported, got {err:?}",
        );
    }
}

#[test]
fn malformed_avc_length_yields_misformed_data() {
    let mut decoder = default_decoder(
        DecoderConfig::new(Codec::H264)
            .with_custom_params(H264Config {
                bitstream_format: AvcBitstreamFormat::Avc,
                extradata: None,
            }),
        CountingCallbacks::shared(),
    )
    .expect("construct");
    // Two bytes when 4 are required for the length prefix.
    let err = decoder.decode(EncodedPacket::from_vec(vec![0, 0])).unwrap_err();
    assert!(matches!(err, DecoderError::MisformedData(_)));
}

#[test]
fn corrupted_sps_yields_decoder_error() {
    // Use a valid SPS NAL bytes pattern but with a wrong profile
    // field that pushes the decoder off the rails. Use a tiny
    // hand-crafted SPS that the parser will reject as MisformedData.
    let mut decoder =
        default_decoder(DecoderConfig::new(Codec::H264), CountingCallbacks::shared()).expect("construct");

    // NAL header byte 0x67 = nal_unit_type 7 (SPS). Truncate the
    // body so exponential-Golomb decoding runs off the end.
    let nal_bytes = vec![0x00, 0x00, 0x00, 0x01, 0x67];
    let err = decoder
        .decode(EncodedPacket::from_vec(nal_bytes))
        .unwrap_err();
    // The exact variant can be MisformedData or OutOfRange depending
    // on how far the parser got; both are accepted.
    assert!(
        matches!(
            err,
            DecoderError::MisformedData(_)
                | DecoderError::OutOfRange(_)
                | DecoderError::ReferenceNotFound(_)
                | DecoderError::FeatureNotSupported(_)
        ),
        "expected bitstream error variant, got {err:?}"
    );
}

#[test]
fn from_alloc_error_round_trips() {
    let e = AllocError::OutOfMemory;
    let de: DecoderError = e.clone().into();
    match de {
        DecoderError::Alloc(inner) => assert_eq!(inner, e),
        other => panic!("expected Alloc, got {other:?}"),
    }
}
