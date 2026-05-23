use std::ptr::NonNull;

use super::color::VideoPlane;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlaneAllocation {
    pub plane: VideoPlane,
    /// Total bytes the decoder needs for this plane, already including
    /// stride padding and per-side border bytes.
    pub size_bytes: usize,
    /// Required base-pointer alignment in bytes.
    pub alignment: usize,
}

/// Full memory request for one frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllocError {
    UnsupportedAlignment,
    OutOfMemory,
}
