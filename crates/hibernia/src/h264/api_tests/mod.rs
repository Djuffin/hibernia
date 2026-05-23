//! Tests for the public `VideoDecoder` API as implemented by the
//! H.264 decoder. Organized by API surface area; helpers live in
//! [`support`].

mod support;

mod data_path;
mod framing;
mod queue;
