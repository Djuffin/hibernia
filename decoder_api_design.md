# Rust Video Decoder API Design

This document defines the object-safe Rust API design for a codec-independent software video decoder.

---

## Key Design Goals
1.  **Codec Independence**: Unified interface for all common codec properties, with support for
    arbitrary, strongly-typed codec-specific parameters.
2.  **Runtime Codec Swapping**: Decoupled dynamic interfaces allowing different codec implementations
    (H.264, VP8, AV1) to be loaded and swapped interchangeably.
3.  **Zero-Copy Allocation**: Memory allocation delegated to the caller, using a unique write-only
    phase for decoding and a shared read-only phase for reference and display.
4.  **Unified Execution Flow**: Single, identical data-flow interface for both synchronous
    (blocking) and asynchronous (threaded) implementations.
5.  **Context & Timestamp Tracking**: Type-safe, automatic propagation of packet-specific
    timestamps and metadata through the reordered decoder pipeline.
6.  **Consolidated Pipeline Control**: Unification of queue discarding (seeking) and pipeline
    draining (EOF) into a single control method.

---

## API Definition

```rust
use std::any::Any;
use std::sync::Arc;

// ===========================================================================
// 1. Core Enums & Configuration
// ===========================================================================

/// Supported video codecs.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Codec {
    /// H.264 / Advanced Video Coding (AVC)
    H264,
    /// H.265 / High Efficiency Video Coding (HEVC)
    H265,
    /// Google VP8
    VP8,
    /// Google VP9
    VP9,
    /// AOMedia Video 1 (AV1)
    AV1,
    /// AOMedia Video 2 (AV2)
    AV2,
}

/// Supported color spaces for video stream mapping.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorSpace {
    /// Color space is unspecified or unknown.
    Unknown,
    /// ITU-R BT.601 (standard definition video).
    Bt601,
    /// ITU-R BT.709 (high definition video).
    Bt709,
    /// ITU-R BT.2020 (ultra high definition / HDR video).
    Bt2020,
}

/// Pixel memory layout and chroma subsampling format.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PixelFormat {
    /// Planar YUV 4:2:0 (e.g., I420: Y plane, then U plane, then V plane).
    I420,
    /// Semi-planar YUV 4:2:0 (e.g., NV12: Y plane, then interleaved UV plane).
    NV12,
    /// Planar YUV 4:2:2 (e.g., I422).
    I422,
    /// Planar YUV 4:4:4 (e.g., I444).
    I444,
    /// Single Y plane (grayscale).
    Monochrome,
    /// Packed Red-Green-Blue-Alpha.
    Rgba,
}

/// General configuration for instantiating a video decoder.
pub struct DecoderConfig {
    /// The target video codec to decode.
    pub codec: Codec,
    /// Maximum number of worker threads for parallel decoding (0 or 1 indicates single-threaded).
    pub max_threads: usize,
    /// Optimize for low latency rather than throughput (e.g., disable frame reordering if possible).
    pub low_latency: bool,
    /// Strongly-typed, codec-specific configuration struct (e.g., `H264Config`, `Vp9Config`)
    pub custom_params: Option<Box<dyn Any + Send>>,
}



// ===========================================================================
// 2. Stream Format & Decoded/Encoded Chunks
// ===========================================================================

/// Active stream geometric and color format parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StreamFormat {
    /// The active video codec.
    pub codec: Codec,
    /// Coded width of the picture in pixels (includes stride/macroblock alignment).
    pub coded_width: usize,
    /// Coded height of the picture in pixels (includes stride/macroblock alignment).
    pub coded_height: usize,
    /// Crop offset from the left edge for display.
    pub crop_left: usize,
    /// Crop offset from the top edge for display.
    pub crop_top: usize,
    /// Visible width of the picture in pixels.
    pub display_width: usize,
    /// Visible height of the picture in pixels.
    pub display_height: usize,
    /// Active color space of the stream.
    pub color_space: ColorSpace,
    /// Memory layout and pixel format of the decoded pictures.
    pub pixel_format: PixelFormat,
    /// Bits per pixel component (typically 8 for standard, 10 or 12 for HDR).
    pub bit_depth: u8,
}

/// A read-only chunk of compressed bitstream data.
/// Implement this to wrap your own memory storage (e.g., mapped files,
/// network rings) without copying bytes.
pub trait EncodedData: Send + Sync {
    /// Get a read-only slice of the underlying bitstream bytes.
    fn as_slice(&self) -> &[u8];
}

/// An encoded packet containing compressed bitstream data and optional metadata.
pub struct EncodedPacket {
    /// The compressed bitstream data, wrapped in a zero-copy thread-safe pointer.
    pub data: Arc<dyn EncodedData>,
    /// Opaque user metadata (e.g., PTS, DTS, custom IDs) propagated through the decoder
    pub opaque: Option<Box<dyn Any + Send>>,
}


/// A fully decoded picture containing pixel data and propagated metadata.
pub struct DecodedPicture {
    /// The decoded pixel data buffer (shared and read-only).
    pub frame: Arc<dyn VideoFrame>,
    /// The format of this decoded picture (resolution, cropping, color space).
    pub format: StreamFormat,
    /// Propagated metadata matching the original EncodedPacket (reordered if necessary).
    pub opaque: Option<Box<dyn Any + Send>>,
}


// ===========================================================================
// 3. Memory Allocation Traits (Zero-Copy)
// ===========================================================================

/// Plane channels inside a video frame.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VideoPlane {
    /// Luma (Y) plane, or the single packed plane for packed formats (like RGBA).
    Y,
    /// Chroma Cb (U) plane, or the interleaved UV plane for semi-planar formats (like NV12).
    U,
    /// Chroma Cr (V) plane.
    V,
    /// Alpha (transparency) plane. Optional.
    Alpha,
}

/// A safe, immutable window into a single video plane's geometric layout.
#[derive(Debug, Clone, Copy)]
pub struct PlaneView<'a> {
    /// The channel type this plane represents.
    pub plane: VideoPlane,
    /// Read-only borrow of the underlying pixel memory slice.
    pub data: &'a [u8],
    /// Row stride (width + horizontal padding) in bytes.
    pub stride: usize,
    /// Active visible width in pixels.
    pub width: usize,
    /// Active visible height in pixels.
    pub height: usize,
}

/// A safe, mutable window into a single video plane's geometric layout.
#[derive(Debug)]
pub struct PlaneViewMut<'a> {
    /// The channel type this plane represents.
    pub plane: VideoPlane,
    /// Mutable borrow of the underlying pixel memory slice.
    pub data: &'a mut [u8],
    /// Row stride (width + horizontal padding) in bytes.
    pub stride: usize,
    /// Active visible width in pixels.
    pub width: usize,
    /// Active visible height in pixels.
    pub height: usize,
}


/// Represents a unique, writeable video frame owned exclusively by the decoder loop.
/// Does not require `Sync` because it is owned by a single thread at a time.
pub trait WritableVideoFrame: Send {
    /// Get a mutable geometric view of the specified plane.
    /// Returns `None` if the plane is not present, or if the memory is not CPU-mappable.
    fn plane_mut(&mut self, plane: VideoPlane) -> Option<PlaneViewMut<'_>>;

    /// Retrieve mutable views of all active planes in the frame concurrently.
    fn planes_mut(&mut self) -> Vec<PlaneViewMut<'_>>;

    /// "Freezes" this writeable frame, consuming it and returning a shared,
    /// read-only `VideoFrame` thread-safe pointer.
    fn freeze(self: Box<Self>) -> Arc<dyn VideoFrame>;
}

/// Represents a shared, read-only video frame. Safe to be read concurrently
/// by the rendering engine and DPB reference engines.
pub trait VideoFrame: Send + Sync {
    /// Get a read-only geometric view of the specified plane.
    /// Returns `None` if the plane is not present.
    fn plane(&self, plane: VideoPlane) -> Option<PlaneView<'_>>;

    /// Retrieve read-only views of all active planes in the frame.
    fn planes(&self) -> Vec<PlaneView<'_>>;
}


/// Implemented by the API user to allocate and manage video frame memory.
/// Enables zero-copy decoding by allowing the user to supply custom buffers.
pub trait VideoFrameAllocator: Send + Sync {
    /// Request a unique, writeable video frame for decoding.
    fn alloc_frame(&self, format: &StreamFormat) -> Result<Box<dyn WritableVideoFrame>, AllocError>;
}

/// Error types returned during video frame memory allocation.
#[derive(Debug, Clone)]
pub enum AllocError {
    /// The allocator does not support the requested stream format (resolution, chroma subsampling, etc.).
    UnsupportedFormat(String),
    /// The allocator has run out of memory buffers (Out of Memory).
    OutOfMemory,
}


// ===========================================================================
// 4. Asynchronous Notification & Errors
// ===========================================================================

/// Callback interface implemented by the API user to receive asynchronous decoder events.
/// In async execution, background threads will call these methods to signal picture
/// availability or format changes.
pub trait VideoDecoderCallbacks: Send + Sync {
    /// Signaled when one or more pictures are decoded and ready in the output queue.
    /// The user should call `VideoDecoder::get_picture` to retrieve them.
    fn on_picture_available(&self);

    /// Signaled when resolution, color space, or cropping parameters change.
    /// The user should update their rendering pipeline and buffer pool.
    fn on_format_changed(&self, format: StreamFormat);

    /// Signaled on non-fatal or fatal decoding errors.
    fn on_error(&self, error: DecoderError);
}

/// Error types returned by the video decoder.
#[derive(Debug, Clone)]
pub enum DecoderError {
    /// Failed to initialize the decoder (e.g., invalid parameters, unsupported codec config).
    InitializationFailed(String),
    /// The compressed bitstream was corrupted or malformed. Non-fatal if future keyframes allow recovery.
    BitstreamCorrupted(String),
    /// The bitstream requires a codec feature not implemented by this decoder.
    FeatureNotSupported(String),
    /// The internal packet queue is full. The caller must drain pictures to continue.
    QueueFull,
    /// The decoder encountered a resource allocation failure (Out of Memory).
    OutOfMemory,
    /// An unrecoverable internal system error occurred (the decoder is now dead).
    Fatal(String),
}



// ===========================================================================
// 5. Main Decoder Trait
// ===========================================================================

/// Modes for flushing the decoder pipeline.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FlushMode {
    /// Fast discard: instantly clears input/output queues and DPB.
    /// In-flight thread work is allowed to finish but its results are discarded.
    /// Used immediately when seeking in a video player.
    Discard,

    /// Drain pipeline: forces DPB to release all remaining frames to the output queue.
    /// Does NOT stop the decoder from accepting new inputs afterwards.
    /// Used at End of Stream (EOS) or sequence boundaries.
    Drain,
}

/// Codec-agnostic software video decoder interface.
pub trait VideoDecoder: Send {
    /// Submit an encoded packet to the decoder's input queue. Non-blocking in async mode.
    fn decode(&mut self, packet: EncodedPacket) -> Result<(), DecoderError>;

    /// Pull the next available decoded picture from the output queue.
    /// Returns `Ok(None)` if the queue is empty (non-blocking).
    fn get_picture(&mut self) -> Result<Option<DecodedPicture>, DecoderError>;

    /// Flushes the decoder pipeline according to the specified `FlushMode`.
    fn flush(&mut self, mode: FlushMode) -> Result<(), DecoderError>;
}


// ===========================================================================
// 6. Decoder Creation & Configuration
// ===========================================================================

/// The primary entry point to instantiate any software video decoder.
/// Concrete implementations are enabled/disabled compile-time via Cargo features.
pub fn create_decoder(
    config: DecoderConfig,
    allocator: Arc<dyn VideoFrameAllocator>,
    callback: Arc<dyn VideoDecoderCallbacks>,
) -> Result<Box<dyn VideoDecoder>, DecoderError> {
    match config.codec {
        Codec::H264 => {
            #[cfg(feature = "h264")]
            {
                // Concrete decoders extract their strongly-typed configuration struct
                // from `config.custom_params` using `downcast_ref`.
                h264::H264Decoder::new(config, allocator, callback)
            }
            #[cfg(not(feature = "h264"))]
            {
                Err(DecoderError::FeatureNotSupported("H.264 decoder feature not enabled".into()))
            }
        }
        Codec::VP8 => {
            #[cfg(feature = "vp8")]
            {
                vp8::Vp8Decoder::new(config, allocator, callback)
            }
            #[cfg(not(feature = "vp8"))]
            {
                Err(DecoderError::FeatureNotSupported("VP8 decoder feature not enabled".into()))
            }
        }
        _ => Err(DecoderError::FeatureNotSupported("Requested codec not supported".into())),
    }
}

// ===========================================================================
// 7. Codec-Specific Configurations
// ===========================================================================

/// H.264/AVC bitstream packaging formats.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AvcBitstreamFormat {
    /// Bitstream with start codes (0x000001 or 0x00000001) separating NAL units.
    /// Common in raw bitstream files (.264) and MPEG-TS.
    AnnexB,

    /// Bitstream where each NAL unit is prefixed by its length (typically 4 bytes).
    /// Common in MP4, MKV, and WebM containers.
    Avc,
}


/// H.264/AVC-specific configuration parameters.
/// Passed inside `DecoderConfig::custom_params` using `Box<dyn Any>`.
pub struct H264Config {
    /// The format of the input bitstream.
    pub bitstream_format: AvcBitstreamFormat,
}

```

---

## Memory Safety & Two-Phase Frame Lifecycle

Decoded pictures must often be retained inside an internal Decoded Picture
Buffer (DPB) to serve as reference frames for future temporal predictions. This requires shared ownership (`Arc`)
between the decoder's DPB, the output queue, and the API user's rendering engine.

If the allocator returns a shared `Arc` directly to the decoder, any write operation (like decoding slices or performing
post-processing deblocking filters) requires exclusive mutable access (`&mut self`). In Rust, getting a mutable reference
from an `Arc` requires calling `Arc::get_mut`, which succeeds **only if the reference count is exactly 1**.

If any other thread or internal structure clones the `Arc` early (e.g., during parallel frame decoding or early DPB
registration), the reference count exceeds 1, causing `Arc::get_mut` to fail (`None`) at runtime. This leads to runtime
panics, stalls, or forces the developer to use dangerous `unsafe` raw pointers to bypass the borrow checker.

### The Solution: Two-Phase Freeze Lifecycle
To solve this, the API enforces a strict **two-phase lifecycle** directly in the type system, shifting safety checks
from **runtime panics to compile-time guarantees**:

1.  **Phase 1: Uniquely Owned Write-Handle (`Box<dyn WritableVideoFrame>`)**
    *   The allocator returns a `Box<dyn WritableVideoFrame>`. A `Box` guarantees 100% exclusive, single-thread ownership.
    *   The decoder can mutate the frame freely (decoding slices, applying loop filters) using standard, safe Rust
        borrowing (`&mut self`) with absolutely no runtime overhead or safety checks.
2.  **Phase 2: Shared Read-Only Pointer (`Arc<dyn VideoFrame>`)**
    *   Once the picture is fully reconstructed and post-processed, the decoder calls `freeze(self: Box<Self>)`.
    *   This consumes the unique write-handle forever, returning a shared, read-only `Arc<dyn VideoFrame>`.
    *   This `Arc` can be cloned safely and shared concurrently among the DPB and the output queue. Because the
        `VideoFrame` trait lacks any mutable methods, the compiler statically guarantees it can never be mutated again.
