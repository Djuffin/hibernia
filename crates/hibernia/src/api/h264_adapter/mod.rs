//! H.264 implementation of the codec-agnostic `VideoDecoder` API.

pub mod decoder;
pub mod format;

pub use decoder::H264VideoDecoder;
pub use format::stream_format_from_sps;

#[cfg(test)]
mod tests;
