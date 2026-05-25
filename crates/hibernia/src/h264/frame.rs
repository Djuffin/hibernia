//! H.264 decoder-internal frame storage.
//!
//! Holds a user-supplied `FrameBuffer` plus per-plane geometry, and
//! hands out `Plane`/`PlaneMut` views into the bordered allocation.

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
}

/// Border padding (in samples per side) required by the H.264
/// reference-picture interpolation filter. The 6-tap luma filter
/// reads up to 3 samples beyond block edges and the chroma bilinear
/// up to 1; we always allocate enough for 1/4-pel motion across a
/// 16x16 macroblock, which is 16 samples per side.
pub const BORDER_PX: usize = 16;

pub struct BorderedFrame {
    buffer: Box<dyn FrameBuffer>,
    luma: PlaneSlot,
    chroma_cb: Option<PlaneSlot>,
    chroma_cr: Option<PlaneSlot>,
}

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
            luma: PlaneSlot { plane: VideoPlane::Y, cfg: luma_cfg },
            chroma_cb: Some(PlaneSlot { plane: VideoPlane::U, cfg: chroma_cfg }),
            chroma_cr: Some(PlaneSlot { plane: VideoPlane::V, cfg: chroma_cfg }),
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
        let ptr = self.buffer.plane_ptr(slot.plane).expect("buffer has plane");
        // SAFETY: ptr is a fat slice pointer owned by buffer. We already verified 
        // ptr.len() is large enough at allocation time. The returned reference 
        // borrows securely from &self.
        let data = unsafe { ptr.as_ref() };
        Plane { data, cfg: slot.cfg }
    }

    /// Mutable view of a plane.
    pub fn plane_mut(&mut self, plane: ColorPlane) -> PlaneMut<'_> {
        let slot = *self.slot(plane).expect("plane present");
        let mut ptr = self.buffer.plane_ptr(slot.plane).expect("buffer has plane");
        // SAFETY: Same as above, with &mut self enforcing exclusive borrow.
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
        let ptr = self.inner.buffer.plane_ptr(slot.plane)?;
        let cfg = slot.cfg;
        let visible_bytes = (cfg.height.saturating_sub(1)) * cfg.stride + cfg.width;
        let origin = cfg.yorigin * cfg.stride + cfg.xorigin;
        
        let slice_ref = unsafe { ptr.as_ref() };
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
}
