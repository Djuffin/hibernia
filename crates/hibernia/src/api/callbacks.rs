use super::format::StreamFormat;
use super::frame::AllocError;

/// Decoder event sink. Callbacks may fire from any thread, including
/// synchronously inside a `VideoDecoder` method before it returns.
/// Implementations must not call back into the decoder from a callback.
pub trait VideoDecoderCallbacks: Send + Sync {
    /// Signaled when one or more pictures are decoded and ready in the output queue.
    /// The user should call `VideoDecoder::get_picture` to retrieve them.
    fn on_picture_available(&self);

    /// Signaled when resolution, color space, or cropping parameters
    /// change. The user should update their rendering pipeline.
    /// Per-plane allocation sizes / alignments arrive separately
    /// through `alloc_frame`.
    fn on_format_changed(&self, format: StreamFormat);
}

/// Error types returned by the video decoder.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecoderError {
    /// Failed to initialize the decoder (e.g., invalid parameters, unsupported codec config).
    InitializationFailed(String),
    /// Syntax-level violation in the bitstream (illegal bit pattern,
    /// parse failure, etc.). Recoverable at the next IDR.
    MisformedData(String),
    /// A signaled value is outside its permitted range (level, profile,
    /// parameter limits, etc.). Recoverable at the next IDR.
    OutOfRange(String),
    /// A slice referenced a picture that's not present in the DPB.
    /// Recoverable at the next IDR.
    ReferenceNotFound(String),
    /// The bitstream requires a codec feature not implemented by this decoder.
    FeatureNotSupported(String),
    /// The output queue is full. The caller must drain pictures to continue.
    QueueFull,
    /// The user-supplied frame allocator rejected an allocation request.
    Alloc(AllocError),
    /// An unrecoverable internal system error occurred (the decoder is now dead).
    Fatal(String),
}

impl std::fmt::Display for DecoderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecoderError::InitializationFailed(s) => write!(f, "initialization failed: {s}"),
            DecoderError::MisformedData(s) => write!(f, "misformed data: {s}"),
            DecoderError::OutOfRange(s) => write!(f, "out of range: {s}"),
            DecoderError::ReferenceNotFound(s) => write!(f, "reference not found: {s}"),
            DecoderError::FeatureNotSupported(s) => write!(f, "feature not supported: {s}"),
            DecoderError::QueueFull => write!(f, "output queue full"),
            DecoderError::Alloc(e) => write!(f, "frame allocation failed: {e:?}"),
            DecoderError::Fatal(s) => write!(f, "fatal: {s}"),
        }
    }
}

impl std::error::Error for DecoderError {}

impl From<AllocError> for DecoderError {
    fn from(e: AllocError) -> Self {
        DecoderError::Alloc(e)
    }
}
