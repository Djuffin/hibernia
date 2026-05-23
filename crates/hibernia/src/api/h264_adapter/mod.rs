//! H.264 implementation of the codec-agnostic `VideoDecoder` API.
//! Built up across phases 5..8.

pub mod format;

pub use format::stream_format_from_sps;
