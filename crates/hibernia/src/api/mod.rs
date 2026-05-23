//! Codec-agnostic video decoder API.
//!
//! The user-facing surface is composed of:
//! - `Codec`, `DecoderConfig`, `LatencyMode` for configuration ([`config`]).
//! - `ColorSpace` and `PixelFormat` for color signaling ([`color`]).
//! - `StreamFormat` describing the active stream geometry ([`format`]).
//! - `EncodedPacket` and `DecodedPicture` for the I/O data path ([`packet`]).
//! - `VideoFrame`, `FrameBuffer`, and `VideoFrameAllocator` for zero-copy
//!   user-supplied frame memory ([`frame`]).
//! - `VideoDecoderCallbacks` and `DecoderError` ([`callbacks`]).
//!
//! Concrete decoder traits and the `create_decoder` factory are added in
//! later phases.

pub mod bitstream;
pub mod callbacks;
pub mod color;
pub mod config;
pub mod default_allocator;
pub mod format;
pub mod frame;
pub mod h264_adapter;
pub mod packet;

pub use bitstream::{AnnexBSplitter, AvcBitstreamFormat, AvcSplitter, H264Config};
pub use callbacks::{DecoderError, VideoDecoderCallbacks};
pub use default_allocator::DefaultAllocator;
pub use color::{
    ColorPrimaries, ColorRange, ColorSpace, MatrixCoefficients, PixelFormat,
    TransferCharacteristics, VideoPlane,
};
pub use config::{Codec, DecoderConfig, LatencyMode};
pub use format::StreamFormat;
pub use frame::{
    AllocError, BufferAllocation, FrameBuffer, PlaneAllocation, PlaneView, VideoFrame,
    VideoFrameAllocator,
};
pub use packet::{DecodedPicture, EncodedPacket};
