use std::any::Any;
use std::sync::Arc;

use super::format::StreamFormat;
use super::frame::VideoFrame;

/// An encoded packet containing compressed bitstream data and optional metadata.
pub struct EncodedPacket {
    /// The compressed bitstream bytes. Any type that implements
    /// [`AsRef<[u8]>`] works -- `Vec<u8>`, `Box<[u8]>`, or a user-defined
    /// wrapper around mapped files, network ring buffers, etc. -- so the
    /// decoder can ingest existing storage without copying.
    pub data: Arc<dyn AsRef<[u8]> + Send + Sync>,
    /// Opaque user metadata (e.g., PTS, DTS, custom IDs) propagated through the decoder.
    pub opaque: Option<Box<dyn Any + Send>>,
}

impl EncodedPacket {
    /// Wrap a `Vec<u8>` payload without an opaque tag.
    pub fn from_vec(data: Vec<u8>) -> Self {
        Self { data: Arc::new(data), opaque: None }
    }

    /// Wrap a `Vec<u8>` payload with an opaque tag.
    pub fn from_vec_with_opaque<T: Any + Send>(data: Vec<u8>, opaque: T) -> Self {
        Self { data: Arc::new(data), opaque: Some(Box::new(opaque)) }
    }

    /// Borrow the bitstream bytes.
    pub fn bytes(&self) -> &[u8] {
        (*self.data).as_ref()
    }
}

/// A fully decoded picture containing pixel data and propagated metadata.
pub struct DecodedPicture {
    /// The decoded pixel data buffer (shared and read-only).
    pub frame: Arc<dyn VideoFrame>,
    /// The format of this decoded picture (resolution, cropping, color space).
    pub format: StreamFormat,
    /// Propagated metadata matching the original `EncodedPacket` (reordered if necessary).
    pub opaque: Option<Box<dyn Any + Send>>,
}
