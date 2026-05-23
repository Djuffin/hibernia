//! Internal, lifetime-bound plane views.
//!
//! Mirrors the subset of `v_frame::plane` actually consulted by the
//! prediction / deblocking code, but holds borrowed slices instead of
//! owned `Vec<u8>`. Frame memory is provided by the user via
//! `api::FrameBuffer`; the decoder borrows into it through these views.

use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct PlaneOffset {
    pub x: isize,
    pub y: isize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PlaneConfig {
    /// Row stride in bytes (visible width plus per-side horizontal padding).
    pub stride: usize,
    /// Visible width in samples.
    pub width: usize,
    /// Visible height in samples.
    pub height: usize,
    /// Horizontal offset from buffer start to the first visible column.
    pub xorigin: usize,
    /// Vertical offset (in rows) from buffer start to the first visible row.
    pub yorigin: usize,
    /// Total allocation height in rows (visible plus top+bottom borders).
    pub alloc_height: usize,
}

impl PlaneConfig {
    #[inline]
    pub fn total_bytes(&self) -> usize {
        self.stride * self.alloc_height
    }

    #[inline]
    fn origin_offset(&self) -> usize {
        self.yorigin * self.stride + self.xorigin
    }
}

/// Read-only view of a single plane's full bordered allocation.
#[derive(Copy, Clone)]
pub struct Plane<'a> {
    pub data: &'a [u8],
    pub cfg: PlaneConfig,
}

/// Read/write view of a single plane's full bordered allocation.
pub struct PlaneMut<'a> {
    pub data: &'a mut [u8],
    pub cfg: PlaneConfig,
}

impl<'a> Plane<'a> {
    #[inline]
    pub fn data_origin(&self) -> &[u8] {
        &self.data[self.cfg.origin_offset()..]
    }

    /// Cropped row at `y` rows below the visible origin (matches
    /// `v_frame::Plane::row`).
    pub fn row(&self, y: isize) -> &[u8] {
        let base = row_range_cropped(&self.cfg, 0, y);
        &self.data[base]
    }

    /// Region anchored at the visible origin offset by `po`. Lifetime
    /// extends to the underlying buffer.
    pub fn slice(&self, po: PlaneOffset) -> PlaneSlice<'a> {
        PlaneSlice { data: self.data, cfg: self.cfg, x: po.x, y: po.y }
    }
}

impl<'a> PlaneMut<'a> {
    #[inline]
    pub fn data_origin(&self) -> &[u8] {
        &self.data[self.cfg.origin_offset()..]
    }

    #[inline]
    pub fn data_origin_mut(&mut self) -> &mut [u8] {
        let off = self.cfg.origin_offset();
        &mut self.data[off..]
    }

    pub fn row(&self, y: isize) -> &[u8] {
        let base = row_range_cropped(&self.cfg, 0, y);
        &self.data[base]
    }

    pub fn slice(&self, po: PlaneOffset) -> PlaneSlice<'_> {
        PlaneSlice { data: self.data, cfg: self.cfg, x: po.x, y: po.y }
    }

    pub fn mut_slice(&mut self, po: PlaneOffset) -> PlaneMutSlice<'_> {
        PlaneMutSlice { data: &mut *self.data, cfg: self.cfg, x: po.x, y: po.y }
    }

    pub fn as_ref(&self) -> Plane<'_> {
        Plane { data: &*self.data, cfg: self.cfg }
    }
}

fn row_range_cropped(cfg: &PlaneConfig, x: isize, y: isize) -> std::ops::Range<usize> {
    debug_assert!(cfg.yorigin as isize + y >= 0);
    debug_assert!(cfg.xorigin as isize + x >= 0);
    let base_y = (cfg.yorigin as isize + y) as usize;
    let base_x = (cfg.xorigin as isize + x) as usize;
    let base = base_y * cfg.stride + base_x;
    let width = (cfg.width as isize - x) as usize;
    base..base + width
}

fn row_range_full(cfg: &PlaneConfig, x: isize, y: isize) -> std::ops::Range<usize> {
    debug_assert!(cfg.yorigin as isize + y >= 0);
    debug_assert!(cfg.xorigin as isize + x >= 0);
    let base_y = (cfg.yorigin as isize + y) as usize;
    let base_x = (cfg.xorigin as isize + x) as usize;
    let base = base_y * cfg.stride + base_x;
    let width = cfg.stride - base_x;
    base..base + width
}

#[derive(Copy, Clone)]
pub struct PlaneSlice<'a> {
    data: &'a [u8],
    cfg: PlaneConfig,
    x: isize,
    y: isize,
}

impl<'a> PlaneSlice<'a> {
    pub fn rows_iter(&self) -> RowsIter<'a> {
        RowsIter { data: self.data, cfg: self.cfg, x: self.x, y: self.y }
    }
}

impl Index<usize> for PlaneSlice<'_> {
    type Output = [u8];

    fn index(&self, index: usize) -> &Self::Output {
        let range = row_range_full(&self.cfg, self.x, self.y + index as isize);
        &self.data[range]
    }
}

pub struct PlaneMutSlice<'a> {
    data: &'a mut [u8],
    cfg: PlaneConfig,
    x: isize,
    y: isize,
}

impl<'a> PlaneMutSlice<'a> {
    pub fn rows_iter(&self) -> RowsIter<'_> {
        RowsIter { data: self.data, cfg: self.cfg, x: self.x, y: self.y }
    }

    pub fn rows_iter_mut(&mut self) -> RowsIterMut<'_> {
        let cfg = self.cfg;
        let x = self.x;
        let y = self.y;
        let height_left = (cfg.height as isize - y).max(0) as usize;
        let width_left = (cfg.width as isize - x).max(0) as usize;
        let start_y = (cfg.yorigin as isize + y) as usize;
        let start_x = (cfg.xorigin as isize + x) as usize;
        let start = start_y * cfg.stride + start_x;
        let total = height_left.saturating_mul(cfg.stride);
        let region = &mut self.data[start..start + total];
        RowsIterMut { region, stride: cfg.stride, width: width_left }
    }
}

impl Index<usize> for PlaneMutSlice<'_> {
    type Output = [u8];

    fn index(&self, index: usize) -> &Self::Output {
        let range = row_range_full(&self.cfg, self.x, self.y + index as isize);
        &self.data[range]
    }
}

impl IndexMut<usize> for PlaneMutSlice<'_> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let range = row_range_full(&self.cfg, self.x, self.y + index as isize);
        &mut self.data[range]
    }
}

pub struct RowsIter<'a> {
    data: &'a [u8],
    cfg: PlaneConfig,
    x: isize,
    y: isize,
}

impl<'a> Iterator for RowsIter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.cfg.height as isize > self.y {
            let range = row_range_cropped(&self.cfg, self.x, self.y);
            self.y += 1;
            Some(&self.data[range])
        } else {
            None
        }
    }
}

pub struct RowsIterMut<'a> {
    region: &'a mut [u8],
    stride: usize,
    width: usize,
}

impl<'a> Iterator for RowsIterMut<'a> {
    type Item = &'a mut [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.region.len() < self.stride {
            return None;
        }
        let region = std::mem::take(&mut self.region);
        let (row, rest) = region.split_at_mut(self.stride);
        self.region = rest;
        Some(&mut row[..self.width])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_cfg() -> PlaneConfig {
        PlaneConfig {
            stride: 24,
            width: 16,
            height: 8,
            xorigin: 4,
            yorigin: 2,
            alloc_height: 12,
        }
    }

    fn make_buffer(cfg: &PlaneConfig) -> Vec<u8> {
        let mut buf = vec![0u8; cfg.total_bytes()];
        // Write a known pattern in the visible area: data[y*stride + x] = y*100 + x.
        for vy in 0..cfg.height {
            for vx in 0..cfg.width {
                let row = cfg.yorigin + vy;
                let col = cfg.xorigin + vx;
                buf[row * cfg.stride + col] = (vy * 100 + vx) as u8;
            }
        }
        buf
    }

    #[test]
    fn data_origin_and_row() {
        let cfg = make_cfg();
        let buf = make_buffer(&cfg);
        let p = Plane { data: &buf, cfg };
        assert_eq!(p.data_origin()[0], 0);
        assert_eq!(p.row(0)[5], 5);
        assert_eq!(p.row(3)[7], (3 * 100 + 7) as u8);
        assert_eq!(p.row(0).len(), 16);
    }

    #[test]
    fn slice_indexing_returns_padded_row() {
        let cfg = make_cfg();
        let buf = make_buffer(&cfg);
        let p = Plane { data: &buf, cfg };
        let s = p.slice(PlaneOffset { x: 0, y: 0 });
        // Index returns full row including right-of-x padding.
        assert_eq!(s[0].len(), cfg.stride - cfg.xorigin);
        assert_eq!(s[2][3], (2 * 100 + 3) as u8);
    }

    #[test]
    fn rows_iter_yields_cropped_rows() {
        let cfg = make_cfg();
        let buf = make_buffer(&cfg);
        let p = Plane { data: &buf, cfg };
        let s = p.slice(PlaneOffset { x: 0, y: 0 });
        let rows: Vec<_> = s.rows_iter().collect();
        assert_eq!(rows.len(), cfg.height);
        for (vy, row) in rows.iter().enumerate() {
            assert_eq!(row.len(), cfg.width);
            assert_eq!(row[7], (vy * 100 + 7) as u8);
        }
    }

    #[test]
    fn rows_iter_mut_yields_disjoint_rows() {
        let cfg = make_cfg();
        let mut buf = make_buffer(&cfg);
        let mut p = PlaneMut { data: &mut buf, cfg };
        let mut s = p.mut_slice(PlaneOffset { x: 0, y: 0 });
        for (vy, row) in s.rows_iter_mut().take(cfg.height).enumerate() {
            row[0] = vy as u8 + 200;
        }
        for vy in 0..cfg.height {
            let off = (cfg.yorigin + vy) * cfg.stride + cfg.xorigin;
            assert_eq!(buf[off], vy as u8 + 200);
        }
    }

    #[test]
    fn slice_with_negative_offset_reads_border() {
        let mut cfg = make_cfg();
        cfg.xorigin = 8;
        cfg.yorigin = 4;
        let mut buf = vec![0u8; cfg.total_bytes()];
        // Set the byte one above-left of the visible origin to a marker.
        let marker_off = (cfg.yorigin - 1) * cfg.stride + (cfg.xorigin - 1);
        buf[marker_off] = 42;
        let p = Plane { data: &buf, cfg };
        let s = p.slice(PlaneOffset { x: -1, y: -1 });
        assert_eq!(s[0][0], 42);
    }
}
