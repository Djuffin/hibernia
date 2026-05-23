use std::alloc::{self, Layout};
use std::ptr::NonNull;

use super::color::VideoPlane;
use super::frame::{
    AllocError, BufferAllocation, FrameBuffer, PlaneAllocation, VideoFrameAllocator,
};

/// A stock `VideoFrameAllocator` that backs each plane with an
/// aligned heap allocation. Provided for callers who don't need a
/// pool or custom storage.
#[derive(Default, Clone, Copy)]
pub struct DefaultAllocator;

impl DefaultAllocator {
    pub fn new() -> Self {
        Self
    }
}

impl VideoFrameAllocator for DefaultAllocator {
    fn alloc_frame(
        &self,
        alloc: &BufferAllocation,
    ) -> Result<Box<dyn FrameBuffer>, AllocError> {
        let mut buffers: [Option<PlaneBuffer>; 4] = [None, None, None, None];
        for (slot, plane_alloc) in buffers.iter_mut().zip(alloc.planes.iter()) {
            if let Some(pa) = plane_alloc {
                *slot = Some(PlaneBuffer::new(pa)?);
            }
        }
        Ok(Box::new(DefaultFrameBuffer { buffers }))
    }
}

struct DefaultFrameBuffer {
    buffers: [Option<PlaneBuffer>; 4],
}

impl FrameBuffer for DefaultFrameBuffer {
    fn plane_ptr(&self, plane: VideoPlane) -> Option<NonNull<u8>> {
        self.buffers.iter().flatten().find(|b| b.plane == plane).map(|b| b.ptr)
    }
}

struct PlaneBuffer {
    plane: VideoPlane,
    ptr: NonNull<u8>,
    layout: Layout,
}

impl PlaneBuffer {
    fn new(req: &PlaneAllocation) -> Result<Self, AllocError> {
        if req.size_bytes == 0 {
            return Err(AllocError::OutOfMemory);
        }
        let layout = Layout::from_size_align(req.size_bytes, req.alignment)
            .map_err(|_| AllocError::UnsupportedAlignment)?;
        let raw = unsafe { alloc::alloc_zeroed(layout) };
        let ptr = NonNull::new(raw).ok_or(AllocError::OutOfMemory)?;
        Ok(Self { plane: req.plane, ptr, layout })
    }
}

impl Drop for PlaneBuffer {
    fn drop(&mut self) {
        unsafe { alloc::dealloc(self.ptr.as_ptr(), self.layout) };
    }
}

// PlaneBuffer holds a raw pointer to an exclusive heap allocation;
// transferring it across threads is sound. Concurrent reads through
// shared references are sound because the decoder guarantees no
// further mutation once a FrameBuffer is published in a VideoFrame.
unsafe impl Send for PlaneBuffer {}
unsafe impl Sync for PlaneBuffer {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocates_aligned_per_plane() {
        let req = BufferAllocation {
            planes: [
                Some(PlaneAllocation { plane: VideoPlane::Y, size_bytes: 1024, alignment: 16 }),
                Some(PlaneAllocation { plane: VideoPlane::U, size_bytes: 256, alignment: 16 }),
                Some(PlaneAllocation { plane: VideoPlane::V, size_bytes: 256, alignment: 16 }),
                None,
            ],
        };
        let buf = DefaultAllocator.alloc_frame(&req).expect("alloc");
        let y = buf.plane_ptr(VideoPlane::Y).expect("Y present");
        let u = buf.plane_ptr(VideoPlane::U).expect("U present");
        let v = buf.plane_ptr(VideoPlane::V).expect("V present");
        assert!(buf.plane_ptr(VideoPlane::Alpha).is_none());
        assert_eq!(y.as_ptr() as usize % 16, 0);
        assert_eq!(u.as_ptr() as usize % 16, 0);
        assert_eq!(v.as_ptr() as usize % 16, 0);
    }

    #[test]
    fn rejects_non_power_of_two_alignment() {
        let req = BufferAllocation {
            planes: [
                Some(PlaneAllocation { plane: VideoPlane::Y, size_bytes: 256, alignment: 3 }),
                None,
                None,
                None,
            ],
        };
        assert_eq!(
            DefaultAllocator.alloc_frame(&req).map(|_| ()),
            Err(AllocError::UnsupportedAlignment)
        );
    }

    #[test]
    fn zeroed_initial_bytes() {
        let req = BufferAllocation {
            planes: [
                Some(PlaneAllocation { plane: VideoPlane::Y, size_bytes: 64, alignment: 8 }),
                None,
                None,
                None,
            ],
        };
        let buf = DefaultAllocator.alloc_frame(&req).expect("alloc");
        let ptr = buf.plane_ptr(VideoPlane::Y).unwrap();
        let bytes = unsafe { std::slice::from_raw_parts(ptr.as_ptr(), 64) };
        assert!(bytes.iter().all(|&b| b == 0));
    }
}
