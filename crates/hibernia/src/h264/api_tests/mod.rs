//! Tests for the public `VideoDecoder` API as implemented by the
//! H.264 decoder. Organized by API surface area; helpers live in
//! [`support`].

mod support;

mod allocator;
mod data_path;
mod format;
mod frame;
mod framing;
mod opaque;
mod queue;
