//! H.264 decoder-internal frame storage.
//!
//! Holds a user-supplied `FrameBuffer` plus per-plane geometry, and
//! hands out `Plane`/`PlaneMut` views into the bordered allocation.

use std::ptr::NonNull;
use std::sync::Arc;

use crate::api::frame::{
    AllocError, BufferAllocation, FrameBuffer, PlaneAllocation, VideoFrame, VideoFrameAllocator,
};
use crate::api::{PlaneView, VideoPlane};

use super::plane::{Plane, PlaneConfig, PlaneMut};
use super::ColorPlane;

/// Per-plane geometry + total allocation byte count, paired with the
/// `VideoPlane` channel the user-supplied `FrameBuffer` keys on.
#[derive(Copy, Clone, Debug)]
struct PlaneSlot {
    plane: VideoPlane,
    cfg: PlaneConfig,
    /// The plane's memory, as returned by `FrameBuffer::plane_ptr` at
    /// allocation and checked to hold at least `cfg.total_bytes()`. Cached
    /// because `plane_ptr` is a dynamic call (a linear search, for
    /// `DefaultAllocator`) that would otherwise run on every plane access.
    ptr: NonNull<[u8]>,
}

/// Border padding (in samples per side) required by the H.264
/// reference-picture interpolation filter. The 6-tap luma filter
/// reads up to 3 samples beyond block edges and the chroma bilinear
/// up to 1; we always allocate enough for 1/4-pel motion across a
/// 16x16 macroblock, which is 16 samples per side.
pub const BORDER_PX: usize = 16;

pub struct BorderedFrame {
    /// Owns the memory behind the slots' cached plane pointers. Planes are
    /// reached through those pointers, so this is only held until `Drop`.
    #[allow(dead_code)]
    buffer: Box<dyn FrameBuffer>,
    luma: PlaneSlot,
    chroma_cb: Option<PlaneSlot>,
    chroma_cr: Option<PlaneSlot>,
}

// SAFETY: The plane pointers cached in the slots are the only fields that
// stop `Send` and `Sync` from being derived. They point into memory owned by
// `buffer`, a `Box<dyn FrameBuffer>` that is itself `Send + Sync` and lives as
// long as the frame, and the `FrameBuffer::plane_ptr` contract keeps that
// memory valid and in place until the buffer is dropped. Moving a frame to
// another thread therefore moves the memory's owner along with the pointers.
// Writes happen only through `plane_mut`, which takes `&mut self`.
unsafe impl Send for BorderedFrame {}
// SAFETY: See `Send` above. A shared `&BorderedFrame` only hands out
// read-only views, so access through it from several threads only reads.
unsafe impl Sync for BorderedFrame {}

impl BorderedFrame {
    /// Allocate a 4:2:0 frame: luma plus Cb/Cr at half resolution.
    /// Each plane gets its own allocation request with `BORDER_PX`
    /// border on every side.
    pub fn alloc_4_2_0(
        alloc: &dyn VideoFrameAllocator,
        luma_width: usize,
        luma_height: usize,
    ) -> Result<Self, AllocError> {
        let luma_cfg = bordered_plane_cfg(luma_width, luma_height);
        let chroma_cfg = bordered_plane_cfg(luma_width / 2, luma_height / 2);
        let req = BufferAllocation {
            planes: [
                Some(PlaneAllocation {
                    plane: VideoPlane::Y,
                    size_bytes: luma_cfg.total_bytes(),
                    alignment: BORDER_PX,
                }),
                Some(PlaneAllocation {
                    plane: VideoPlane::U,
                    size_bytes: chroma_cfg.total_bytes(),
                    alignment: BORDER_PX,
                }),
                Some(PlaneAllocation {
                    plane: VideoPlane::V,
                    size_bytes: chroma_cfg.total_bytes(),
                    alignment: BORDER_PX,
                }),
                None,
            ],
        };
        let buffer = alloc.alloc_frame(&req)?;
        let y_ptr = buffer.plane_ptr(VideoPlane::Y).ok_or(AllocError::OutOfMemory)?;
        let u_ptr = buffer.plane_ptr(VideoPlane::U).ok_or(AllocError::OutOfMemory)?;
        let v_ptr = buffer.plane_ptr(VideoPlane::V).ok_or(AllocError::OutOfMemory)?;

        assert!(y_ptr.len() >= luma_cfg.total_bytes(), "Allocator returned undersized Y plane");
        assert!(u_ptr.len() >= chroma_cfg.total_bytes(), "Allocator returned undersized U plane");
        assert!(v_ptr.len() >= chroma_cfg.total_bytes(), "Allocator returned undersized V plane");

        Ok(Self {
            buffer,
            luma: PlaneSlot { plane: VideoPlane::Y, cfg: luma_cfg, ptr: y_ptr },
            chroma_cb: Some(PlaneSlot { plane: VideoPlane::U, cfg: chroma_cfg, ptr: u_ptr }),
            chroma_cr: Some(PlaneSlot { plane: VideoPlane::V, cfg: chroma_cfg, ptr: v_ptr }),
        })
    }

    fn slot(&self, plane: ColorPlane) -> Option<&PlaneSlot> {
        match plane {
            ColorPlane::Y => Some(&self.luma),
            ColorPlane::Cb => self.chroma_cb.as_ref(),
            ColorPlane::Cr => self.chroma_cr.as_ref(),
        }
    }

    /// Read-only view of a plane.
    pub fn plane(&self, plane: ColorPlane) -> Plane<'_> {
        let slot = self.slot(plane).expect("plane present");
        // SAFETY: `slot.ptr` points into memory owned by `self.buffer`, and
        // `alloc_4_2_0` checked that it holds at least `cfg.total_bytes()`.
        // The view borrows `self`, so the memory outlives it, and no
        // `plane_mut` view can write to the plane while it lives.
        let data = unsafe { slot.ptr.as_ref() };
        Plane { data, cfg: slot.cfg }
    }

    /// Mutable view of a plane.
    pub fn plane_mut(&mut self, plane: ColorPlane) -> PlaneMut<'_> {
        let slot = *self.slot(plane).expect("plane present");
        let mut ptr = slot.ptr;
        // SAFETY: As in `plane`; in addition, `&mut self` makes this view
        // exclusive, so no other view of any plane exists while it lives.
        let data = unsafe { ptr.as_mut() };
        PlaneMut { data, cfg: slot.cfg }
    }
}

impl std::fmt::Debug for BorderedFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BorderedFrame")
            .field("luma", &self.luma)
            .field("chroma_cb", &self.chroma_cb)
            .field("chroma_cr", &self.chroma_cr)
            .finish()
    }
}

/// Public read-only `VideoFrame` view over a shared `BorderedFrame`.
/// Constructed at publish time when the decoder hands a finished
/// picture to the caller.
pub(crate) struct PublishedFrame {
    inner: Arc<BorderedFrame>,
}

impl PublishedFrame {
    pub(crate) fn new(inner: Arc<BorderedFrame>) -> Self {
        Self { inner }
    }
}

impl VideoFrame for PublishedFrame {
    fn plane(&self, plane: VideoPlane) -> Option<PlaneView<'_>> {
        let slot = match plane {
            VideoPlane::Y => Some(self.inner.luma),
            VideoPlane::U => self.inner.chroma_cb,
            VideoPlane::V => self.inner.chroma_cr,
            _ => None,
        }?;
        let cfg = slot.cfg;
        let visible_bytes = (cfg.height.saturating_sub(1)) * cfg.stride + cfg.width;
        let origin = cfg.yorigin * cfg.stride + cfg.xorigin;

        // SAFETY: `slot.ptr` points into memory owned by `self.inner.buffer`,
        // which the `Arc` keeps alive while this view borrows `self`, and
        // `alloc_4_2_0` checked that it holds at least `cfg.total_bytes()`.
        // Writes need `&mut BorderedFrame`, which can't coexist with this
        // shared `Arc`, so the plane is only read while the view lives.
        let slice_ref = unsafe { slot.ptr.as_ref() };
        let data = &slice_ref[origin..(origin + visible_bytes)];

        Some(PlaneView {
            plane: slot.plane,
            data,
            stride: cfg.stride,
            width: cfg.width,
            height: cfg.height,
        })
    }

    fn planes(&self) -> [Option<PlaneView<'_>>; 4] {
        [
            self.plane(VideoPlane::Y),
            self.plane(VideoPlane::U),
            self.plane(VideoPlane::V),
            None,
        ]
    }
}

/// Build a plane config with `BORDER_PX` border on every side and a
/// stride rounded up to a multiple of `BORDER_PX`.
fn bordered_plane_cfg(visible_w: usize, visible_h: usize) -> PlaneConfig {
    let stride = round_up(visible_w + 2 * BORDER_PX, BORDER_PX);
    let alloc_height = visible_h + 2 * BORDER_PX;
    PlaneConfig {
        stride,
        width: visible_w,
        height: visible_h,
        xorigin: BORDER_PX,
        yorigin: BORDER_PX,
        alloc_height,
    }
}

#[inline]
fn round_up(value: usize, align: usize) -> usize {
    value.div_ceil(align) * align
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::default_allocator::DefaultAllocator;

    #[test]
    fn alloc_4_2_0_planes_have_expected_geometry() {
        let f = BorderedFrame::alloc_4_2_0(&DefaultAllocator, 320, 240).expect("alloc");
        let y = f.plane(ColorPlane::Y);
        assert_eq!(y.cfg.width, 320);
        assert_eq!(y.cfg.height, 240);
        assert!(y.cfg.stride >= 320 + 2 * BORDER_PX);
        let cb = f.plane(ColorPlane::Cb);
        assert_eq!(cb.cfg.width, 160);
        assert_eq!(cb.cfg.height, 120);
    }

    #[test]
    fn writes_through_plane_mut_persist() {
        let mut f = BorderedFrame::alloc_4_2_0(&DefaultAllocator, 32, 32).expect("alloc");
        {
            let mut y = f.plane_mut(ColorPlane::Y);
            y.data_origin_mut()[0] = 77;
        }
        let y = f.plane(ColorPlane::Y);
        assert_eq!(y.data_origin()[0], 77);
    }

    #[test]
    fn bordered_frame_is_send_and_sync() {
        // `VideoFrame: Send + Sync` and `VideoDecoder: Send` depend on this.
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<BorderedFrame>();
    }

    #[test]
    fn cached_plane_pointers_match_the_buffer() {
        let f = BorderedFrame::alloc_4_2_0(&DefaultAllocator, 64, 32).expect("alloc");
        for (plane, video_plane) in [
            (ColorPlane::Y, VideoPlane::Y),
            (ColorPlane::Cb, VideoPlane::U),
            (ColorPlane::Cr, VideoPlane::V),
        ] {
            let view = f.plane(plane);
            let expected = f.buffer.plane_ptr(video_plane).expect("buffer has plane");
            assert_eq!(view.data.as_ptr(), expected.cast::<u8>().as_ptr().cast_const());
            assert_eq!(view.data.len(), expected.len());
        }
    }

    #[test]
    fn published_frame_reads_through_cached_pointers() {
        let mut f = BorderedFrame::alloc_4_2_0(&DefaultAllocator, 32, 16).expect("alloc");
        f.plane_mut(ColorPlane::Cb).data_origin_mut()[0] = 42;
        let published = PublishedFrame::new(Arc::new(f));
        let view = published.plane(VideoPlane::U).expect("U plane");
        assert_eq!(view.data[0], 42);
        assert_eq!((view.width, view.height), (16, 8));
    }
}
