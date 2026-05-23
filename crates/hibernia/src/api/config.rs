use std::any::Any;

/// Supported video codecs.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Codec {
    H264,
    VP8,
    VP9,
    AV1,
    AV2,
}

/// Optimize for end-to-end latency vs. throughput.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum LatencyMode {
    /// Maximize throughput; allow frame reordering and lookahead.
    #[default]
    Throughput,
    /// Minimize latency; disable reordering / lookahead where possible.
    LowLatency,
}

/// General configuration for instantiating a video decoder.
pub struct DecoderConfig {
    /// The target video codec to decode.
    pub codec: Codec,
    pub latency_mode: LatencyMode,
    /// Strongly-typed, codec-specific configuration struct (e.g., `H264Config`).
    pub custom_params: Option<Box<dyn Any + Send>>,
}

impl DecoderConfig {
    pub fn new(codec: Codec) -> Self {
        Self { codec, latency_mode: LatencyMode::Throughput, custom_params: None }
    }

    pub fn with_latency_mode(mut self, mode: LatencyMode) -> Self {
        self.latency_mode = mode;
        self
    }

    pub fn with_custom_params<T: Any + Send>(mut self, params: T) -> Self {
        self.custom_params = Some(Box::new(params));
        self
    }
}
