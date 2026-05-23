//! Tests for `create_decoder` and `DecoderConfig` construction paths.

use std::sync::Arc;

use crate::api::bitstream::{AvcBitstreamFormat, H264Config};
use crate::api::callbacks::DecoderError;
use crate::api::config::{Codec, DecoderConfig, LatencyMode};
use crate::api::create_decoder;
use crate::api::DefaultAllocator;

use super::support::CountingCallbacks;

#[test]
fn create_decoder_with_default_config_succeeds() {
    let result = create_decoder(
        DecoderConfig::new(Codec::H264),
        Arc::new(DefaultAllocator),
        CountingCallbacks::shared(),
    );
    assert!(result.is_ok());
}

#[test]
fn create_decoder_with_explicit_annexb_config_succeeds() {
    let config = DecoderConfig::new(Codec::H264).with_custom_params(H264Config {
        bitstream_format: AvcBitstreamFormat::AnnexB,
    });
    let result = create_decoder(config, Arc::new(DefaultAllocator), CountingCallbacks::shared());
    assert!(result.is_ok());
}

#[test]
fn create_decoder_with_avc_config_succeeds() {
    let config = DecoderConfig::new(Codec::H264).with_custom_params(H264Config {
        bitstream_format: AvcBitstreamFormat::Avc,
    });
    let result = create_decoder(config, Arc::new(DefaultAllocator), CountingCallbacks::shared());
    assert!(result.is_ok());
}

#[test]
fn create_decoder_low_latency_accepted() {
    let config = DecoderConfig::new(Codec::H264).with_latency_mode(LatencyMode::LowLatency);
    let result = create_decoder(config, Arc::new(DefaultAllocator), CountingCallbacks::shared());
    assert!(result.is_ok(), "LowLatency must be accepted even if not honored specially");
}

#[test]
fn decoder_config_builder_chain_works() {
    let config = DecoderConfig::new(Codec::H264)
        .with_latency_mode(LatencyMode::Throughput)
        .with_custom_params(H264Config { bitstream_format: AvcBitstreamFormat::AnnexB });
    assert_eq!(config.codec, Codec::H264);
    assert_eq!(config.latency_mode, LatencyMode::Throughput);
    assert!(config.custom_params.is_some());
}

#[test]
fn create_decoder_returns_box_dyn_video_decoder() {
    // The factory's return type is the abstract trait object, not the
    // concrete Decoder. Verify by calling a trait method on it.
    let decoder = create_decoder(
        DecoderConfig::new(Codec::H264),
        Arc::new(DefaultAllocator),
        CountingCallbacks::shared(),
    )
    .expect("create");
    // Trait-object method must compile and execute.
    let mut decoder = decoder;
    assert!(decoder.get_picture().unwrap().is_none());
}

#[test]
fn create_decoder_unsupported_codecs_table() {
    // Cover every non-H264 variant in one go.
    for codec in [Codec::VP8, Codec::VP9, Codec::AV1, Codec::AV2] {
        let result = create_decoder(
            DecoderConfig::new(codec),
            Arc::new(DefaultAllocator),
            CountingCallbacks::shared(),
        );
        let err = result.map(|_| ()).unwrap_err();
        assert!(
            matches!(err, DecoderError::FeatureNotSupported(_)),
            "{codec:?} should return FeatureNotSupported, got {err:?}",
        );
    }
}

#[test]
fn create_decoder_with_wrong_custom_params_type_fails() {
    // Pass something that isn't H264Config.
    let bogus: Box<dyn std::any::Any + Send> = Box::new(123u32);
    let config = DecoderConfig {
        codec: Codec::H264,
        latency_mode: LatencyMode::Throughput,
        custom_params: Some(bogus),
    };
    let err = create_decoder(config, Arc::new(DefaultAllocator), CountingCallbacks::shared())
        .map(|_| ())
        .unwrap_err();
    assert!(matches!(err, DecoderError::InitializationFailed(_)));
}
