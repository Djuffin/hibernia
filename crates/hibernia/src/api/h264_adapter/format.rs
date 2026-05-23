//! Compat shim. The format mapping has moved to `crate::h264::format`.
//! This re-export keeps the adapter's old call sites compiling during
//! the collapse of `h264_adapter` into `h264::Decoder`.

pub use crate::h264::format::stream_format_from_sps;
