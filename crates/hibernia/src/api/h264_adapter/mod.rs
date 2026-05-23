//! Compat shim. The adapter's content has been folded into
//! `crate::h264::decoder::Decoder`. This module exists for one more
//! phase so the `pub use` re-export in `api/mod.rs` keeps resolving;
//! Phase D removes the directory entirely.

pub mod format;

pub use format::stream_format_from_sps;
