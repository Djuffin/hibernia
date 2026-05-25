# Rust Video Decoder API Design

This document defines the object-safe Rust API design for a codec-independent software video decoder.

---

## Key Design Goals
1.  **Codec Independence**: Unified interface for all common codec properties, with support for
    arbitrary, strongly-typed codec-specific parameters.
2.  **Runtime Codec Swapping**: Decoupled dynamic interfaces allowing different codec implementations
    to be used interchangeably.
3.  **Zero-Copy Allocation**: Frame memory delegated to the caller via the `FrameBuffer` trait. The
    decoder writes through raw pointers internally; user code sees only the read-only `VideoFrame`
    view, which borrows directly into the user-supplied allocation without copies.
4.  **Unified Execution Flow**: Single, identical data-flow interface for both synchronous
    (blocking) and asynchronous (threaded) implementations.
5.  **Context & Timestamp Tracking**: Type-safe, automatic propagation of packet-specific
    timestamps and metadata through the reordered decoder pipeline.
6.  **Consolidated Pipeline Control**: Unification of queue discarding (seeking) and pipeline
    draining (EOF) into a single control method.

---

## API Definition

```rust
// ===========================================================================
// 1. Core Enums & Configuration
// ===========================================================================

/// Supported video codecs.
pub enum Codec {
    H264,
    VP8,
    VP9,
    AV1,
    AV2,
}

/// Color primaries (ISO/IEC 23091-2 §8.1).
pub enum ColorPrimaries {
    Unspecified,
    Bt709,
    /// BT.601 525-line (NTSC).
    Smpte170m,
    /// BT.601 625-line (PAL/SECAM).
    Bt470bg,
    Bt2020,
    /// Display P3 (SMPTE EG 432-1, D65 white point).
    Smpte432,
}

/// Opto-electronic transfer characteristic (ISO/IEC 23091-2 §8.2).
pub enum TransferCharacteristics {
    Unspecified,
    Bt709,
    Bt601,
    Smpte240,
    Linear,
    Srgb,
    Bt2020_10,
    Bt2020_12,
    /// SMPTE ST 2084 / HDR10.
    SmptePq,
    /// ARIB STD-B67 / HLG.
    AribStdB67,
}

/// YUV → RGB matrix (ISO/IEC 23091-2 §8.3).
pub enum MatrixCoefficients {
    Unspecified,
    /// RGB / GBR — no YUV conversion.
    Identity,
    Bt709,
    Bt601,
    Smpte240,
    /// BT.2020 non-constant luminance.
    Bt2020Ncl,
    /// BT.2020 constant luminance.
    Bt2020Cl,
}

/// Sample range. When the bitstream doesn't signal range, decoders
/// default to `Limited`.
pub enum ColorRange {
    Limited,
    Full,
}

/// Full color signaling for a stream. Primaries, transfer, and matrix
/// are orthogonal in the spec and codecs may carry them independently
/// (H.264/HEVC/AV1 VUI). Legacy codecs that carry only a combined
/// label (VP9's 3-bit field) map onto this struct via a fixed table.
pub struct ColorSpace {
    pub primaries: ColorPrimaries,
    pub transfer: TransferCharacteristics,
    pub matrix: MatrixCoefficients,
    pub range: ColorRange,
}

/// Pixel memory layout and chroma subsampling format.
pub enum PixelFormat {
    /// Planar YUV 4:2:0 (e.g., I420: Y plane, then U plane, then V plane).
    I420,
    /// Semi-planar YUV 4:2:0 (e.g., NV12: Y plane, then interleaved UV plane).
    NV12,
    /// Planar YUV 4:2:2 (e.g., I422).
    I422,
    /// Planar YUV 4:4:4 (e.g., I444).
    I444,
    /// Planar YUV 4:2:0 with alpha plane (I420 + A).
    I420A,
    /// Single Y plane (grayscale).
    Monochrome,
}

/// Plane channels inside a video frame.
pub enum VideoPlane {
    /// Luma plane.
    Y,
    /// Chroma Cb plane (planar formats only).
    U,
    /// Chroma Cr plane (planar formats only).
    V,
    /// Interleaved Cb/Cr plane (semi-planar formats like NV12).
    UV,
    /// Transparency plane.
    Alpha,
}

/// Optimize for end-to-end latency vs. throughput.
pub enum LatencyMode {
    /// Maximize throughput; allow frame reordering and lookahead.
    Throughput,
    /// Minimize latency; disable reordering / lookahead where possible.
    LowLatency,
}

/// General configuration for instantiating a video decoder.
pub struct DecoderConfig {
    /// The target video codec to decode.
    pub codec: Codec,
    pub latency_mode: LatencyMode,
    /// Strongly-typed, codec-specific configuration struct (e.g., `H264Config`, `Vp9Config`)
    pub custom_params: Option<Box<dyn Any + Send>>,
}



// ===========================================================================
// 2. Stream Format & Decoded/Encoded Chunks
// ===========================================================================

/// Active stream geometric and color format parameters.
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
    /// Active color space of the stream, if specified.
    pub color_space: Option<ColorSpace>,
    /// Pixel format of the decoded pictures.
    pub pixel_format: PixelFormat,
    /// Bits per pixel component (typically 8 for standard, 10 or 12 for HDR).
    pub bit_depth: u8,
}

/// An encoded packet containing compressed bitstream data and optional metadata.
pub struct EncodedPacket {
    /// The compressed bitstream bytes. Any type that implements
    /// [`AsRef<[u8]>`] works — `Vec<u8>`, `Box<[u8]>`, or a user-defined
    /// wrapper around mapped files, network ring buffers, etc. — so the
    /// decoder can ingest existing storage without copying.
    pub data: Arc<dyn AsRef<[u8]> + Send + Sync>,
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

/// A safe, immutable window into a single video plane's geometric layout.
pub struct PlaneView<'a> {
    /// The channel type this plane represents.
    pub plane: VideoPlane,
    /// Read-only borrow of the underlying pixel memory slice.
    pub data: &'a [u8],
    /// Row stride (width + horizontal padding) in bytes.
    pub stride: usize,
    /// Active visible width in pixels / samples.
    pub width: usize,
    /// Active visible height in pixels / samples.
    pub height: usize,
}

/// Shared, read-only video frame. Safe to read concurrently from the
/// DPB and the renderer. The concrete impl is decoder-internal; user
/// code only sees `Arc<dyn VideoFrame>`.
pub trait VideoFrame: Send + Sync {
    /// Read-only view of one plane's visible area, or `None` if absent.
    fn plane(&self, plane: VideoPlane) -> Option<PlaneView<'_>>;

    /// Read-only views of all active planes.
    fn planes(&self) -> [Option<PlaneView<'_>>; 4];
}

/// Per-plane memory request handed to the allocator. The decoder
/// derives this from its internal pixel geometry; the allocator
/// never sees video semantics.
pub struct PlaneAllocation {
    pub plane: VideoPlane,
    /// Total bytes the decoder needs for this plane, already including
    /// stride padding and per-side border bytes.
    pub size_bytes: usize,
    /// Required base-pointer alignment in bytes.
    pub alignment: usize,
}

/// Full memory request for one frame.
pub struct BufferAllocation {
    pub planes: [Option<PlaneAllocation>; 4],
}

/// User-implemented backing storage for one frame. Responsibility is
/// memory provision and (optionally) pool bookkeeping on `Drop`.
pub trait FrameBuffer: Send + Sync {
    /// Raw pointer to the start of `plane`'s allocation, sized and
    /// aligned per the matching `PlaneAllocation` the decoder
    /// requested. `None` for planes not present in the request.
    fn plane_ptr(&self, plane: VideoPlane) -> Option<NonNull<u8>>;
}

pub trait VideoFrameAllocator: Send + Sync {
    /// Materialize the requested per-plane allocations. Return
    /// `UnsupportedAlignment` if a plane's alignment can't be honored.
    fn alloc_frame(
        &self,
        alloc: &BufferAllocation,
    ) -> Result<Box<dyn FrameBuffer>, AllocError>;
}

pub enum AllocError {
    UnsupportedAlignment,
    OutOfMemory,
}


// ===========================================================================
// 4. Asynchronous Notification & Errors
// ===========================================================================

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
pub enum DecoderError {
    /// Failed to initialize the decoder (e.g., invalid parameters, unsupported codec config).
    InitializationFailed(String),
    /// The compressed bitstream was corrupted or malformed. Non-fatal if future keyframes allow recovery.
    BitstreamCorrupted(String),
    /// The bitstream requires a codec feature not implemented by this decoder.
    FeatureNotSupported(String),
    /// The output queue is full. The caller must drain pictures to continue.
    QueueFull,
    /// The user-supplied frame allocator rejected an allocation request.
    Alloc(AllocError),
    /// An unrecoverable internal system error occurred (the decoder is now dead).
    Fatal(String),
}



// ===========================================================================
// 5. Main Decoder Trait
// ===========================================================================

/// Modes for flushing the decoder pipeline.
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

    /// Pull the next decoded picture from the output queue, or
    /// `Ok(None)` if the queue is empty.
    ///
    /// One `decode()` can yield zero or several pictures: B-frame
    /// reordering holds pictures back until their display order is
    /// resolved, then releases them in a batch. Pictures are emitted
    /// in display order. Callers should drain after each `decode()`:
    ///
    /// The queue holds `Arc<dyn VideoFrame>` clones, so undrained
    /// pictures keep their frame buffers alive in the user's
    /// allocator. A caller that stops draining will eventually see
    /// `QueueFull` from `decode()`.
    fn get_picture(&mut self) -> Result<Option<DecodedPicture>, DecoderError>;

    /// Flushes the decoder pipeline according to the specified `FlushMode`.
    fn flush(&mut self, mode: FlushMode) -> Result<(), DecoderError>;

    /// Dispatch a codec-specific command. The payload is downcast by
    /// the concrete decoder; unknown payload types should return
    /// `DecoderError::FeatureNotSupported`. Outputs are written back
    /// through `&mut` fields on the payload.
    fn control(&mut self, cmd: &mut ControlCmd) -> Result<(), DecoderError>;
}

/// Codec-specific control payload. Concrete decoders define their own
/// command structs (e.g. `Vp8SetReference`, `H264GetLastQuantizer`)
/// and downcast to them.
pub type ControlCmd = dyn Any;


// ===========================================================================
// 6. Decoder Creation & Configuration
// ===========================================================================

/// The primary entry point to instantiate any software video decoder.
/// Concrete implementations are enabled/disabled compile-time via Cargo features.
pub fn create_decoder(
    config: DecoderConfig,
    allocator: Arc<dyn VideoFrameAllocator>,
    callback: Arc<dyn VideoDecoderCallbacks>,
) -> Result<Box<dyn VideoDecoder>, DecoderError>;

// ===========================================================================
// 7. Codec-Specific Configurations
// ===========================================================================

/// H.264/AVC bitstream packaging formats.
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
    /// Optional out-of-band parameter sets. Either an ISO/IEC 14496-15
    /// `AVCDecoderConfigurationRecord` (avcC) or a concatenation of
    /// Annex-B-framed SPS+PPS NALs.
    pub extradata: Option<Vec<u8>>,
}

/// Control command (passed via `VideoDecoder::control`) that replaces
/// the decoder's parameter-set tables at runtime. `data` uses the same
/// blob format as `H264Config::extradata`.
pub struct H264SetExtradata {
    pub data: Vec<u8>,
}

// Usage:
//
//     let mut cmd = H264SetExtradata { data: avcc_blob };
//     decoder.control(&mut cmd)?;
```

---

## Memory Safety & Frame Lifecycle

Decoded pictures are retained inside the decoder's internal DPB as reference frames for future
temporal predictions, *and* surfaced to the caller for rendering. Both paths need read access
concurrently — the DPB while decoding the next frame, the renderer while displaying. That requires
shared ownership (`Arc`).

The challenge is that "shared" and "mutable" don't compose safely in Rust. The decoder needs
exclusive write access while reconstructing a picture (decoding slices, loop filtering, edge
replication); after that, multiple readers need concurrent access. Patterns like
`Arc<Mutex<Frame>>` or `Arc::get_mut` push the conflict to runtime — `get_mut` succeeds only when
refcount == 1, and any early DPB clone breaks it. Either route forces runtime checks (panics,
stalls) or `unsafe` raw pointers in user code.

### The Design: Decoder Owns the View; User Owns the Memory

The split: the user supplies raw backing memory via `FrameBuffer`; the decoder owns the
`VideoFrame` impl that interprets it.

1. **Allocation phase.** The decoder builds a `BufferAllocation` (per-plane size + alignment) and
   calls `VideoFrameAllocator::alloc_frame`. The user returns a `Box<dyn FrameBuffer>` that exposes
   raw `NonNull<u8>` pointers per plane. The user's surface ends here — no slice math, no
   knowledge of borders or visible-area geometry.

2. **Decode phase.** The decoder constructs decoder-internal bordered views over the buffer's raw
   pointers and writes through them: visible-area samples, edge-replicate border, reads from
   reference frames. All bordered access lives in decoder-internal helpers, never visible to user code.

3. **Publish phase.** When the picture is final, the decoder wraps the `Box<dyn FrameBuffer>` and
   its pixel geometry in a decoder-internal `VideoFrame` impl, hands the caller an
   `Arc<dyn VideoFrame>`. The `Box` is consumed and gone; no further mutation is possible because
   `VideoFrame` has no `&mut self` methods. Multiple readers (DPB, renderer) clone the `Arc` freely.
