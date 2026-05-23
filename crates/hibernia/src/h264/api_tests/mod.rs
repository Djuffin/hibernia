//! Tests for the public `VideoDecoder` API as implemented by the
//! H.264 decoder. Organized by API surface area; helpers live in
//! [`support`].

mod support;

mod allocator;
mod callbacks;
mod construction;
mod control;
mod data_path;
mod errors;
mod flush;
mod format;
mod frame;
mod framing;
mod opaque;
mod queue;
