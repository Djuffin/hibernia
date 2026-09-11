use crate::api::DecoderError;

use super::decoder::VideoFrame;
use super::dpb::DpbPicture;
use super::macroblock::{self, BMb, MbPredictionMode, MotionVector, PMb, PMbType, PartitionInfo};
use super::plane::Plane;
use super::residual::{DequantTables, Residual};
use super::slice::{Slice, SliceType};
use super::{ColorPlane, Point};

/// Section 8.4.2.2.1 Luma sample interpolation process.
/// This function interpolates a block of luma samples with quarter-sample accuracy.
///
/// Inputs:
/// - `ref_plane`: Reference picture luma plane.
/// - `mb_x`, `mb_y`: Macroblock coordinates.
/// - `blk_x`, `blk_y`: Block coordinates within the macroblock.
/// - `width`, `height`: Block dimensions.
/// - `mv`: Motion vector (in quarter-sample units).
/// - `dst`: Destination buffer.
/// - `dst_stride`: Destination stride.
/// - `buffer`: Scratch buffer for interpolation.
#[allow(clippy::too_many_arguments)]
pub fn interpolate_luma(
    ref_plane: Plane<'_>,
    mb_x: u32,
    mb_y: u32,
    blk_x: u8,
    blk_y: u8,
    width: u8,
    height: u8,
    mv: MotionVector,
    dst: &mut [u8],
    dst_stride: usize,
    buffer: &mut InterpolationBuffer,
) {
    match (width, height) {
        (16, 16) => interpolate_luma_impl::<16, 16>(
            ref_plane, mb_x, mb_y, blk_x, blk_y, mv, dst, dst_stride, buffer,
        ),
        (16, 8) => interpolate_luma_impl::<16, 8>(
            ref_plane, mb_x, mb_y, blk_x, blk_y, mv, dst, dst_stride, buffer,
        ),
        (8, 16) => interpolate_luma_impl::<8, 16>(
            ref_plane, mb_x, mb_y, blk_x, blk_y, mv, dst, dst_stride, buffer,
        ),
        (8, 8) => interpolate_luma_impl::<8, 8>(
            ref_plane, mb_x, mb_y, blk_x, blk_y, mv, dst, dst_stride, buffer,
        ),
        (8, 4) => interpolate_luma_impl::<8, 4>(
            ref_plane, mb_x, mb_y, blk_x, blk_y, mv, dst, dst_stride, buffer,
        ),
        (4, 8) => interpolate_luma_impl::<4, 8>(
            ref_plane, mb_x, mb_y, blk_x, blk_y, mv, dst, dst_stride, buffer,
        ),
        (4, 4) => interpolate_luma_impl::<4, 4>(
            ref_plane, mb_x, mb_y, blk_x, blk_y, mv, dst, dst_stride, buffer,
        ),
        (4, 12) => interpolate_luma_impl::<4, 12>(
            ref_plane, mb_x, mb_y, blk_x, blk_y, mv, dst, dst_stride, buffer,
        ),
        (4, 16) => interpolate_luma_impl::<4, 16>(
            ref_plane, mb_x, mb_y, blk_x, blk_y, mv, dst, dst_stride, buffer,
        ),
        (8, 12) => interpolate_luma_impl::<8, 12>(
            ref_plane, mb_x, mb_y, blk_x, blk_y, mv, dst, dst_stride, buffer,
        ),
        (12, 4) => interpolate_luma_impl::<12, 4>(
            ref_plane, mb_x, mb_y, blk_x, blk_y, mv, dst, dst_stride, buffer,
        ),
        (12, 8) => interpolate_luma_impl::<12, 8>(
            ref_plane, mb_x, mb_y, blk_x, blk_y, mv, dst, dst_stride, buffer,
        ),
        (12, 12) => interpolate_luma_impl::<12, 12>(
            ref_plane, mb_x, mb_y, blk_x, blk_y, mv, dst, dst_stride, buffer,
        ),
        (12, 16) => interpolate_luma_impl::<12, 16>(
            ref_plane, mb_x, mb_y, blk_x, blk_y, mv, dst, dst_stride, buffer,
        ),
        (16, 4) => interpolate_luma_impl::<16, 4>(
            ref_plane, mb_x, mb_y, blk_x, blk_y, mv, dst, dst_stride, buffer,
        ),
        (16, 12) => interpolate_luma_impl::<16, 12>(
            ref_plane, mb_x, mb_y, blk_x, blk_y, mv, dst, dst_stride, buffer,
        ),
        _ => unreachable!("unsupported block size {}x{}", width, height),
    }
}

macro_rules! horiz_6tap_val {
    ($row:expr, $x:expr) => {
        ($row[$x] as i32) - 5 * ($row[$x + 1] as i32) + 20 * ($row[$x + 2] as i32)
            + 20 * ($row[$x + 3] as i32) - 5 * ($row[$x + 4] as i32) + ($row[$x + 5] as i32)
    };
}

macro_rules! vert_6tap_val {
    ($r0:expr, $r1:expr, $r2:expr, $r3:expr, $r4:expr, $r5:expr, $x:expr) => {
        ($r0[$x] as i32) - 5 * ($r1[$x] as i32) + 20 * ($r2[$x] as i32)
            + 20 * ($r3[$x] as i32) - 5 * ($r4[$x] as i32) + ($r5[$x] as i32)
    };
}

macro_rules! avg_u8 {
    ($a:expr, $b:expr $(,)?) => {
        (($a as u16 + $b as u16 + 1) >> 1) as u8
    };
}

macro_rules! load_6_rows {
    ($data:expr, $y:expr, $W:expr) => {
        (
            &$data[$y][..$W + 5],
            &$data[$y + 1][..$W + 5],
            &$data[$y + 2][..$W + 5],
            &$data[$y + 3][..$W + 5],
            &$data[$y + 4][..$W + 5],
            &$data[$y + 5][..$W + 5],
        )
    };
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn interpolate_luma_impl<const W: usize, const H: usize>(
    ref_plane: Plane<'_>,
    mb_x: u32,
    mb_y: u32,
    blk_x: u8,
    blk_y: u8,
    mv: MotionVector,
    dst: &mut [u8],
    dst_stride: usize,
    buffer: &mut InterpolationBuffer,
) {
    // Equation 8-223, 8-224: Full sample units (Int) and Equation 8-225, 8-226: Fractional units (Frac)
    let x_int = (mb_x as i32) + (blk_x as i32) + (mv.x >> 2) as i32;
    let y_int = (mb_y as i32) + (blk_y as i32) + (mv.y >> 2) as i32;
    let x_frac = (mv.x & 3) as i8;
    let y_frac = (mv.y & 3) as i8;

    let plane_height = ref_plane.cfg.height as i32;
    let plane_width = ref_plane.cfg.width as i32;

    if x_frac == 0 && y_frac == 0 {
        if x_int >= 0
            && x_int + (W as i32) <= plane_width
            && y_int >= 0
            && y_int + (H as i32) <= plane_height
        {
            // Fast path: direct copy
            let stride = ref_plane.cfg.stride;
            let data = ref_plane.data_origin();
            let base = (y_int as usize) * stride + (x_int as usize);
            assert!(base + (H - 1) * stride + W <= data.len());
            for y in 0..H {
                let off = base + y * stride;
                let d = &mut dst[y * dst_stride..y * dst_stride + W];
                d.copy_from_slice(&data[off..off + W]);
            }
        } else {
            // Slow path: clamping
            for y in 0..H {
                let cy = (y_int + (y as i32)).clamp(0, plane_height - 1);
                let row = ref_plane.row(cy as isize);
                let d = &mut dst[y * dst_stride..y * dst_stride + W];
                for x in 0..W {
                    let cx = (x_int + (x as i32)).clamp(0, plane_width - 1);
                    d[x] = row[cx as usize];
                }
            }
        }
        return;
    }

    // To implement the 6-tap filter for a block of size (W x H),
    // we need a window of (W + 5) x (H + 5) integer pixels.
    // Specifically, for 6-tap filter at pos G (integer), we need E, F, G, H, I, J (1D view).
    // That is 2 pixels to the left/top and 3 pixels to the right/bottom relative to the top-left 'G'.
    let buf_w = W + 5;
    let buf_h = H + 5;

    // Load integer samples into the scratch buffer with boundary checks.
    // Corresponds to fetching samples A through U (in 2D) for the filter process.
    if x_int >= 2
        && x_int + (buf_w as i32) - 2 <= plane_width
        && y_int >= 2
        && y_int + (buf_h as i32) - 2 <= plane_height
    {
        let stride = ref_plane.cfg.stride;
        let data = ref_plane.data_origin();
        let base = ((y_int - 2) as usize) * stride + ((x_int - 2) as usize);
        assert!(base + (buf_h - 1) * stride + buf_w <= data.len());
        for y in 0..buf_h {
            let off = base + y * stride;
            buffer.data[y][..buf_w].copy_from_slice(&data[off..off + buf_w]);
        }
    } else {
        // Slow path: clamping for boundary pixels (Section 8.4.2.2.1, RefLayerFrame behavior)
        for y in 0..buf_h {
            let cy = (y_int + (y as i32) - 2).clamp(0, plane_height - 1);
            let row = ref_plane.row(cy as isize);
            let b = &mut buffer.data[y][..buf_w];
            for x in 0..buf_w {
                let cx = (x_int + (x as i32) - 2).clamp(0, plane_width - 1);
                b[x] = row[cx as usize];
            }
        }
    }

    let data = &buffer.data;

    match (x_frac, y_frac) {
        // Half-pel positions (Equations 8-241, 8-243)
        (2, 0) => {
            // b: horizontal 6-tap
            for y in 0..H {
                let row = &data[y + 2][..W + 5];
                let d = &mut dst[y * dst_stride..y * dst_stride + W];
                for x in 0..W {
                    d[x] = ((horiz_6tap_val!(row, x) + 16) >> 5).clamp(0, 255) as u8;
                }
            }
        }
        (0, 2) => {
            // h: vertical 6-tap
            for y in 0..H {
                let (r0, r1, r2, r3, r4, r5) = load_6_rows!(data, y, W);
                let d = &mut dst[y * dst_stride..y * dst_stride + W];
                for x in 0..W {
                    d[x] = ((vert_6tap_val!(r0, r1, r2, r3, r4, r5, x + 2) + 16) >> 5).clamp(0, 255) as u8;
                }
            }
        }
        // Quarter-pel positions (Table 8-12, Equations 8-250 to 8-252)
        (1, 0) => {
            // a = avg(G, b)
            for y in 0..H {
                let row = &data[y + 2][..W + 5];
                let d = &mut dst[y * dst_stride..y * dst_stride + W];
                for x in 0..W {
                    let b_val = ((horiz_6tap_val!(row, x) + 16) >> 5).clamp(0, 255) as u8;
                    d[x] = avg_u8!(row[x + 2], b_val);
                }
            }
        }
        (3, 0) => {
            // c = avg(H, b)
            for y in 0..H {
                let row = &data[y + 2][..W + 5];
                let d = &mut dst[y * dst_stride..y * dst_stride + W];
                for x in 0..W {
                    let b_val = ((horiz_6tap_val!(row, x) + 16) >> 5).clamp(0, 255) as u8;
                    d[x] = avg_u8!(row[x + 3], b_val);
                }
            }
        }
        (0, 1) => {
            // d = avg(G, h)
            for y in 0..H {
                let (r0, r1, r2, r3, r4, r5) = load_6_rows!(data, y, W);
                let d = &mut dst[y * dst_stride..y * dst_stride + W];
                for x in 0..W {
                    let h_val = ((vert_6tap_val!(r0, r1, r2, r3, r4, r5, x + 2) + 16) >> 5).clamp(0, 255) as u8;
                    d[x] = avg_u8!(r2[x + 2], h_val);
                }
            }
        }
        (0, 3) => {
            // n = avg(M, h)
            for y in 0..H {
                let (r0, r1, r2, r3, r4, r5) = load_6_rows!(data, y, W);
                let d = &mut dst[y * dst_stride..y * dst_stride + W];
                for x in 0..W {
                    let h_val = ((vert_6tap_val!(r0, r1, r2, r3, r4, r5, x + 2) + 16) >> 5).clamp(0, 255) as u8;
                    d[x] = avg_u8!(r3[x + 2], h_val);
                }
            }
        }
        (1, 1) => {
            // e = avg(b, h)
            for y in 0..H {
                let (r0, r1, r2, r3, r4, r5) = load_6_rows!(data, y, W);
                let d = &mut dst[y * dst_stride..y * dst_stride + W];
                for x in 0..W {
                    let b_val = ((horiz_6tap_val!(r2, x) + 16) >> 5).clamp(0, 255) as u8;
                    let h_val = ((vert_6tap_val!(r0, r1, r2, r3, r4, r5, x + 2) + 16) >> 5).clamp(0, 255) as u8;
                    d[x] = avg_u8!(b_val, h_val);
                }
            }
        }
        (3, 1) => {
            // g = avg(b, m)
            for y in 0..H {
                let (r0, r1, r2, r3, r4, r5) = load_6_rows!(data, y, W);
                let d = &mut dst[y * dst_stride..y * dst_stride + W];
                for x in 0..W {
                    let b_val = ((horiz_6tap_val!(r2, x) + 16) >> 5).clamp(0, 255) as u8;
                    let m_val = ((vert_6tap_val!(r0, r1, r2, r3, r4, r5, x + 3) + 16) >> 5).clamp(0, 255) as u8;
                    d[x] = avg_u8!(b_val, m_val);
                }
            }
        }
        (1, 3) => {
            // p = avg(h, s)
            for y in 0..H {
                let (r0, r1, r2, r3, r4, r5) = load_6_rows!(data, y, W);
                let d = &mut dst[y * dst_stride..y * dst_stride + W];
                for x in 0..W {
                    let h_val = ((vert_6tap_val!(r0, r1, r2, r3, r4, r5, x + 2) + 16) >> 5).clamp(0, 255) as u8;
                    let s_val = ((horiz_6tap_val!(r3, x) + 16) >> 5).clamp(0, 255) as u8;
                    d[x] = avg_u8!(h_val, s_val);
                }
            }
        }
        (3, 3) => {
            // r = avg(m, s)
            for y in 0..H {
                let (r0, r1, r2, r3, r4, r5) = load_6_rows!(data, y, W);
                let d = &mut dst[y * dst_stride..y * dst_stride + W];
                for x in 0..W {
                    let m_val = ((vert_6tap_val!(r0, r1, r2, r3, r4, r5, x + 3) + 16) >> 5).clamp(0, 255) as u8;
                    let s_val = ((horiz_6tap_val!(r3, x) + 16) >> 5).clamp(0, 255) as u8;
                    d[x] = avg_u8!(m_val, s_val);
                }
            }
        }
        // Cases needing j (center half-sample, Equation 8-247)
        // j requires two-pass filtering: horizontal into unclipped intermediate,
        // then vertical on the intermediate results.
        (2, 2) | (2, 1) | (2, 3) | (1, 2) | (3, 2) => {
            let mut intermediate = [[0i32; 21]; 21];
            for y in 0..buf_h {
                let r = &data[y][..W + 5];
                let out = &mut intermediate[y][..W];
                for x in 0..W {
                    out[x] = horiz_6tap_val!(r, x);
                }
            }

            match (x_frac, y_frac) {
                (2, 2) => {
                    // j: two-pass 6-tap
                    for y in 0..H {
                        let (r0, r1, r2, r3, r4, r5) = load_6_rows!(intermediate, y, W);
                        let d = &mut dst[y * dst_stride..y * dst_stride + W];
                        for x in 0..W {
                            d[x] = ((vert_6tap_val!(r0, r1, r2, r3, r4, r5, x) + 512) >> 10).clamp(0, 255) as u8;
                        }
                    }
                }
                (2, 1) => {
                    // f = avg(b, j)
                    for y in 0..H {
                        let (r0, r1, r2, r3, r4, r5) = load_6_rows!(intermediate, y, W);
                        let d = &mut dst[y * dst_stride..y * dst_stride + W];
                        for x in 0..W {
                            let j_val = ((vert_6tap_val!(r0, r1, r2, r3, r4, r5, x) + 512) >> 10).clamp(0, 255) as u8;
                            let b_val = ((r2[x] + 16) >> 5).clamp(0, 255) as u8;
                            d[x] = avg_u8!(b_val, j_val);
                        }
                    }
                }
                (2, 3) => {
                    // q = avg(j, s)
                    for y in 0..H {
                        let (r0, r1, r2, r3, r4, r5) = load_6_rows!(intermediate, y, W);
                        let d = &mut dst[y * dst_stride..y * dst_stride + W];
                        for x in 0..W {
                            let j_val = ((vert_6tap_val!(r0, r1, r2, r3, r4, r5, x) + 512) >> 10).clamp(0, 255) as u8;
                            let s_val = ((r3[x] + 16) >> 5).clamp(0, 255) as u8;
                            d[x] = avg_u8!(j_val, s_val);
                        }
                    }
                }
                (1, 2) => {
                    // i = avg(h, j)
                    for y in 0..H {
                        let (d0, d1, d2, d3, d4, d5) = load_6_rows!(data, y, W);
                        let (r0, r1, r2, r3, r4, r5) = load_6_rows!(intermediate, y, W);
                        let d = &mut dst[y * dst_stride..y * dst_stride + W];
                        for x in 0..W {
                            let h_val = ((vert_6tap_val!(d0, d1, d2, d3, d4, d5, x + 2) + 16) >> 5).clamp(0, 255) as u8;
                            let j_val = ((vert_6tap_val!(r0, r1, r2, r3, r4, r5, x) + 512) >> 10).clamp(0, 255) as u8;
                            d[x] = avg_u8!(h_val, j_val);
                        }
                    }
                }
                (3, 2) => {
                    // k = avg(j, m)
                    for y in 0..H {
                        let (d0, d1, d2, d3, d4, d5) = load_6_rows!(data, y, W);
                        let (r0, r1, r2, r3, r4, r5) = load_6_rows!(intermediate, y, W);
                        let d = &mut dst[y * dst_stride..y * dst_stride + W];
                        for x in 0..W {
                            let m_val = ((vert_6tap_val!(d0, d1, d2, d3, d4, d5, x + 3) + 16) >> 5).clamp(0, 255) as u8;
                            let j_val = ((vert_6tap_val!(r0, r1, r2, r3, r4, r5, x) + 512) >> 10).clamp(0, 255) as u8;
                            d[x] = avg_u8!(j_val, m_val);
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!("x_frac={}, y_frac={}", x_frac, y_frac),
    }
}

/// Section 8.4.2.2.2 Chroma sample interpolation process.
/// Assumes 4:2:0 chroma sampling (ChromaArrayType == 1).
///
/// # Arguments
/// * `ref_plane` - The reference chroma plane (Cb or Cr).
/// * `mb_x` - The x-coordinate of the macroblock in chroma samples.
/// * `mb_y` - The y-coordinate of the macroblock in chroma samples.
/// * `blk_x` - The x-offset of the block within the macroblock in chroma samples.
/// * `blk_y` - The y-offset of the block within the macroblock in chroma samples.
/// * `width` - The width of the block in chroma samples.
/// * `height` - The height of the block in chroma samples.
/// * `mv` - The luma motion vector (in 1/4-luma-sample units).
/// * `dst` - The destination buffer.
/// * `dst_stride` - The stride of the destination buffer.
#[allow(clippy::too_many_arguments)]
pub fn interpolate_chroma(
    ref_plane: Plane<'_>,
    mb_x: u32,
    mb_y: u32,
    blk_x: u8,
    blk_y: u8,
    width: u8,
    height: u8,
    mv: MotionVector,
    dst: &mut [u8],
    dst_stride: usize,
) {
    // Eq 8-227..8-230 (4:2:0). `mv` is in quarter-luma-sample units; one
    // chroma sample spans two luma samples, so quarter-luma == 1/8 chroma.
    // That makes `mv.x` directly an offset in 1/8-chroma units, split by
    // `>> 3` (integer chroma samples) and `& 7` (fractional eighths).
    let x_int = (mb_x as i32) + (blk_x as i32) + (mv.x >> 3) as i32;
    let y_int = (mb_y as i32) + (blk_y as i32) + (mv.y >> 3) as i32;
    let x_frac = (mv.x & 7) as i16;
    let y_frac = (mv.y & 7) as i16;

    let plane_width = ref_plane.cfg.width as i32;
    let plane_height = ref_plane.cfg.height as i32;
    // Eq. 8-270 gives the sample right of (below) each block sample weight 0
    // when the horizontal (vertical) fraction is 0, and the fast path then
    // doesn't read it; so that column (row) only has to be inside the picture
    // when its fraction is non-zero.
    let reach_x = i32::from(width) + i32::from(x_frac != 0);
    let reach_y = i32::from(height) + i32::from(y_frac != 0);
    if x_int >= 0 && x_int + reach_x <= plane_width && y_int >= 0 && y_int + reach_y <= plane_height
    {
        let stride = ref_plane.cfg.stride;
        let base = (y_int as usize) * stride + (x_int as usize);
        let src = &ref_plane.data_origin()[base..];
        let (xf, yf) = (x_frac.unsigned_abs(), y_frac.unsigned_abs());
        // A compile-time block size lets LLVM unroll and vectorize each case.
        match (width, height) {
            (8, 8) => chroma_block::<8, 8>(src, stride, xf, yf, dst, dst_stride),
            (8, 6) => chroma_block::<8, 6>(src, stride, xf, yf, dst, dst_stride),
            (8, 4) => chroma_block::<8, 4>(src, stride, xf, yf, dst, dst_stride),
            (8, 2) => chroma_block::<8, 2>(src, stride, xf, yf, dst, dst_stride),
            (6, 8) => chroma_block::<6, 8>(src, stride, xf, yf, dst, dst_stride),
            (6, 6) => chroma_block::<6, 6>(src, stride, xf, yf, dst, dst_stride),
            (6, 4) => chroma_block::<6, 4>(src, stride, xf, yf, dst, dst_stride),
            (6, 2) => chroma_block::<6, 2>(src, stride, xf, yf, dst, dst_stride),
            (4, 8) => chroma_block::<4, 8>(src, stride, xf, yf, dst, dst_stride),
            (4, 6) => chroma_block::<4, 6>(src, stride, xf, yf, dst, dst_stride),
            (4, 4) => chroma_block::<4, 4>(src, stride, xf, yf, dst, dst_stride),
            (4, 2) => chroma_block::<4, 2>(src, stride, xf, yf, dst, dst_stride),
            (2, 8) => chroma_block::<2, 8>(src, stride, xf, yf, dst, dst_stride),
            (2, 6) => chroma_block::<2, 6>(src, stride, xf, yf, dst, dst_stride),
            (2, 4) => chroma_block::<2, 4>(src, stride, xf, yf, dst, dst_stride),
            (2, 2) => chroma_block::<2, 2>(src, stride, xf, yf, dst, dst_stride),
            _ => interpolate_chroma_clamped(
                ref_plane, x_int, y_int, x_frac, y_frac, width, height, dst, dst_stride,
            ),
        }
    } else {
        interpolate_chroma_clamped(
            ref_plane, x_int, y_int, x_frac, y_frac, width, height, dst, dst_stride,
        );
    }
}

/// Section 8.4.2.2.2, Eq. 8-270: bilinear interpolation of a `W`x`H` chroma
/// block whose samples, and the neighbours that its non-zero fractions
/// weight, all lie inside the reference picture. `src` starts at the block's
/// top-left integer sample; `x_frac` and `y_frac` are in eighths.
#[allow(clippy::inline_always)]
#[inline(always)]
fn chroma_block<const W: usize, const H: usize>(
    src: &[u8],
    stride: usize,
    x_frac: u16,
    y_frac: u16,
    dst: &mut [u8],
    dst_stride: usize,
) {
    match (x_frac, y_frac) {
        (0, 0) => {
            for y in 0..H {
                dst[y * dst_stride..][..W].copy_from_slice(&src[y * stride..][..W]);
            }
        }
        (_, 0) => {
            let (wa, wb) = (8 - x_frac, x_frac);
            for y in 0..H {
                let s = &src[y * stride..][..=W];
                for (x, d) in dst[y * dst_stride..][..W].iter_mut().enumerate() {
                    *d = low_byte((wa * u16::from(s[x]) + wb * u16::from(s[x + 1]) + 4) >> 3);
                }
            }
        }
        (0, _) => {
            let (wa, wc) = (8 - y_frac, y_frac);
            for y in 0..H {
                let (s0, s1) = (&src[y * stride..][..W], &src[(y + 1) * stride..][..W]);
                for (x, d) in dst[y * dst_stride..][..W].iter_mut().enumerate() {
                    *d = low_byte((wa * u16::from(s0[x]) + wc * u16::from(s1[x]) + 4) >> 3);
                }
            }
        }
        _ => {
            let (xa, xb, ya, yb) = (8 - x_frac, x_frac, 8 - y_frac, y_frac);
            let (w00, w10, w01, w11) = (xa * ya, xb * ya, xa * yb, xb * yb);
            for y in 0..H {
                let (s0, s1) = (&src[y * stride..][..=W], &src[(y + 1) * stride..][..=W]);
                for (x, d) in dst[y * dst_stride..][..W].iter_mut().enumerate() {
                    let top = w00 * u16::from(s0[x]) + w10 * u16::from(s0[x + 1]);
                    let bottom = w01 * u16::from(s1[x]) + w11 * u16::from(s1[x + 1]);
                    *d = low_byte((top + bottom + 32) >> 6);
                }
            }
        }
    }
}

/// A weighted average of 8-bit samples, which always fits in 8 bits.
#[inline(always)]
#[allow(clippy::inline_always)]
fn low_byte(value: u16) -> u8 {
    u8::try_from(value).unwrap_or(u8::MAX)
}

/// Section 8.4.2.2.2: chroma interpolation for blocks that reach outside the
/// reference picture. Every sample coordinate is clamped into the picture
/// before Eq. 8-270 is applied.
#[allow(clippy::too_many_arguments)]
fn interpolate_chroma_clamped(
    ref_plane: Plane<'_>,
    x_int: i32,
    y_int: i32,
    x_frac: i16,
    y_frac: i16,
    width: u8,
    height: u8,
    dst: &mut [u8],
    dst_stride: usize,
) {
    let plane_width = i32::try_from(ref_plane.cfg.width).unwrap_or(i32::MAX);
    let plane_height = i32::try_from(ref_plane.cfg.height).unwrap_or(i32::MAX);
    // Handle boundary conditions by clamping sample coordinates
    let w00 = (8 - x_frac) * (8 - y_frac);
    let w10 = x_frac * (8 - y_frac);
    let w01 = (8 - x_frac) * y_frac;
    let w11 = x_frac * y_frac;

    for y in 0..height as usize {
        let cy = (y_int + y as i32).clamp(0, plane_height - 1);
        let cy1 = (y_int + y as i32 + 1).clamp(0, plane_height - 1);
        let row = ref_plane.row(cy as isize);
        let row1 = ref_plane.row(cy1 as isize);

        let cx_start = x_int.clamp(0, plane_width - 1);
        let mut val_a = row[cx_start as usize] as i16;
        let mut val_c = row1[cx_start as usize] as i16;

        for x in 0..width as usize {
            let cx1 = (x_int + x as i32 + 1).clamp(0, plane_width - 1);

            let val_b = row[cx1 as usize] as i16;
            let val_d = row1[cx1 as usize] as i16;

            let prediction = (w00 * val_a + w10 * val_b + w01 * val_c + w11 * val_d + 32) >> 6;

            dst[y * dst_stride + x] = prediction as u8;

            val_a = val_b;
            val_c = val_d;
        }
    }
}

/// Buffer for storing integer pixels with padding for 6-tap filtering.
/// The size is 21x21 to accommodate a 16x16 block with 2 pixels padding on top/left
/// and 3 pixels padding on bottom/right (required for 6-tap filter).
pub struct InterpolationBuffer {
    data: [[u8; 21]; 21],
}

impl Default for InterpolationBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl InterpolationBuffer {
    pub fn new() -> Self {
        Self { data: [[0; 21]; 21] }
    }
}

// Section 8.4.2.3: Weighted prediction mode for the current slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WeightedPredMode {
    Default,
    Explicit,
    Implicit,
}

// Resolved weighting parameters for one prediction direction (Section 8.4.3 outputs).
#[derive(Copy, Clone)]
pub(crate) struct WeightParams {
    log_wd: u32,
    w0: i32,
    o0: i32,
    w1: i32,
    o1: i32,
}

/// Determine the weighted prediction mode for the current slice (Section 8.4.2.3).
pub(crate) fn get_weighted_pred_mode(slice: &Slice) -> WeightedPredMode {
    match slice.header.slice_type {
        SliceType::P | SliceType::SP => {
            if slice.pps.weighted_pred_flag {
                WeightedPredMode::Explicit
            } else {
                WeightedPredMode::Default
            }
        }
        SliceType::B => match slice.pps.weighted_bipred_idc {
            1 => WeightedPredMode::Explicit,
            2 => WeightedPredMode::Implicit,
            _ => WeightedPredMode::Default,
        },
        _ => WeightedPredMode::Default,
    }
}

// Section 8.4.2.3.2, Eq 8-274/8-275: Weighted sample prediction for uni-prediction.
#[inline]
fn weighted_uni_pred(pred: u8, w: i32, o: i32, log_wd: u32) -> u8 {
    let val = if log_wd >= 1 {
        ((i32::from(pred) * w + (1 << (log_wd - 1))) >> log_wd) + o
    } else {
        i32::from(pred) * w + o
    };
    val.clamp(0, 255) as u8
}

// Section 8.4.2.3.2, Eq 8-276: Weighted sample prediction for bi-prediction.
#[inline]
fn weighted_bi_pred(pred_l0: u8, pred_l1: u8, wp: &WeightParams) -> u8 {
    let val = ((i32::from(pred_l0) * wp.w0 + i32::from(pred_l1) * wp.w1 + (1 << wp.log_wd))
        >> (wp.log_wd + 1))
        + ((wp.o0 + wp.o1 + 1) >> 1);
    val.clamp(0, 255) as u8
}

// Section 8.4.3: Derive explicit luma weighting parameters from pred_weight_table.
fn get_explicit_luma_weights(slice: &Slice, ref_idx_l0: usize, ref_idx_l1: usize) -> WeightParams {
    let table = slice.header.pred_weight_table.as_ref().unwrap();
    let log_wd = table.luma_log2_weight_denom;
    let (w0, o0) =
        table.list0.get(ref_idx_l0).map_or((1 << log_wd, 0), |f| (f.luma_weight, f.luma_offset));
    let (w1, o1) =
        table.list1.get(ref_idx_l1).map_or((1 << log_wd, 0), |f| (f.luma_weight, f.luma_offset));
    WeightParams { log_wd, w0, o0, w1, o1 }
}

// Section 8.4.3: Derive explicit chroma weighting parameters from pred_weight_table.
// chroma_idx: 0 = Cb, 1 = Cr.
fn get_explicit_chroma_weights(
    slice: &Slice,
    ref_idx_l0: usize,
    ref_idx_l1: usize,
    chroma_idx: usize,
) -> WeightParams {
    let table = slice.header.pred_weight_table.as_ref().unwrap();
    let log_wd = table.chroma_log2_weight_denom;
    let (w0, o0) = table
        .list0
        .get(ref_idx_l0)
        .map_or((1 << log_wd, 0), |f| (f.chroma_weights[chroma_idx], f.chroma_offsets[chroma_idx]));
    let (w1, o1) = table
        .list1
        .get(ref_idx_l1)
        .map_or((1 << log_wd, 0), |f| (f.chroma_weights[chroma_idx], f.chroma_offsets[chroma_idx]));
    WeightParams { log_wd, w0, o0, w1, o1 }
}

// Section 8.4.3: Derive implicit weighting parameters from POC distances (Eq 8-277 to 8-283).
// Same weights are used for luma and chroma in implicit mode.
fn get_implicit_weights(
    ref_pic_l0: &DpbPicture,
    ref_pic_l1: &DpbPicture,
    current_poc: i32,
) -> WeightParams {
    let log_wd: u32 = 5;
    let default = WeightParams { log_wd, w0: 32, o0: 0, w1: 32, o1: 0 };

    // Fallback if either reference is long-term (Eq 8-280)
    if ref_pic_l0.marking.is_long_term() || ref_pic_l1.marking.is_long_term() {
        return default;
    }

    let poc_l0 = ref_pic_l0.picture.pic_order_cnt;
    let poc_l1 = ref_pic_l1.picture.pic_order_cnt;

    // DiffPicOrderCnt(pic1, pic0) -- Eq 8-197/8-198
    let diff_poc_l1_l0 = poc_l1 - poc_l0;
    if diff_poc_l1_l0 == 0 {
        return default;
    }

    // Eq 8-201, 8-202: DistScaleFactor
    let td = (poc_l1 - poc_l0).clamp(-128, 127);
    let tb = (current_poc - poc_l0).clamp(-128, 127);
    let tx = (16384 + (td.abs() >> 1)) / td;
    let dist_scale_factor = ((tb * tx + 32) >> 6).clamp(-1024, 1023);

    let w1 = dist_scale_factor >> 2;
    if w1 < -64 || w1 > 128 {
        return default;
    }
    let w0 = 64 - w1;

    WeightParams { log_wd, w0, o0: 0, w1, o1: 0 }
}

const DEFAULT_IMPLICIT_WEIGHT: WeightParams = WeightParams { log_wd: 5, w0: 32, o0: 0, w1: 32, o1: 0 };

pub(crate) type ImplicitWeightTable = [[WeightParams; 16]; 16];

pub(crate) fn build_implicit_weight_table(
    ref_pics_l0: &[&DpbPicture],
    ref_pics_l1: &[&DpbPicture],
    current_poc: i32,
) -> ImplicitWeightTable {
    let mut table = [[DEFAULT_IMPLICIT_WEIGHT; 16]; 16];
    for (i, l0) in ref_pics_l0.iter().enumerate().take(16) {
        for (j, l1) in ref_pics_l1.iter().enumerate().take(16) {
            table[i][j] = get_implicit_weights(l0, l1, current_poc);
        }
    }
    table
}

/// Quantisation parameters of one macroblock (8.5.8): QP'Y for luma and QP'C
/// for each chroma plane.
#[derive(Clone, Copy, Debug)]
pub(crate) struct MbQp {
    pub(crate) luma: u8,
    pub(crate) cb: u8,
    pub(crate) cr: u8,
}

/// The inputs of inter-macroblock reconstruction that are fixed for a slice.
/// The caller resolves them once per slice.
pub(crate) struct InterSliceRefs<'a> {
    pub(crate) wp_mode: WeightedPredMode,
    pub(crate) ref_pics_l0: &'a [&'a DpbPicture],
    pub(crate) ref_pics_l1: &'a [&'a DpbPicture],
    pub(crate) implicit_weights: &'a ImplicitWeightTable,
    pub(crate) dequant: &'a DequantTables,
}

/// Sections 8.4 and 8.5: reconstructs a P macroblock straight into `frame`.
pub(crate) fn reconstruct_p_macroblock(
    slice: &Slice,
    refs: &InterSliceRefs<'_>,
    mb: &PMb,
    mb_loc: Point,
    qp: MbQp,
    frame: &mut VideoFrame,
    buffer: &mut InterpolationBuffer,
) -> Result<(), DecoderError> {
    let rects = PredRects::for_p(mb);
    let residual = mb.residual.as_deref();
    reconstruct_inter_macroblock(slice, refs, &rects, residual, mb_loc, qp, frame, buffer)
}

/// Sections 8.4 and 8.5: reconstructs a B macroblock straight into `frame`.
pub(crate) fn reconstruct_b_macroblock(
    slice: &Slice,
    refs: &InterSliceRefs<'_>,
    mb: &BMb,
    mb_loc: Point,
    qp: MbQp,
    frame: &mut VideoFrame,
    buffer: &mut InterpolationBuffer,
) -> Result<(), DecoderError> {
    let rects = PredRects::for_b(&mb.motion);
    let residual = mb.residual.as_deref();
    reconstruct_inter_macroblock(slice, refs, &rects, residual, mb_loc, qp, frame, buffer)
}

/// Reconstructs an inter macroblock one plane at a time (luma, Cb, Cr), so
/// only one plane is borrowed mutably at once. Each prediction rectangle is
/// predicted straight into the frame (8.4.2); then the residual of the coded
/// blocks is added (8.5.12 to 8.5.14).
fn reconstruct_inter_macroblock(
    slice: &Slice,
    refs: &InterSliceRefs<'_>,
    rects: &PredRects,
    residual: Option<&Residual>,
    mb_loc: Point,
    qp: MbQp,
    frame: &mut VideoFrame,
    buffer: &mut InterpolationBuffer,
) -> Result<(), DecoderError> {
    for plane in [ColorPlane::Y, ColorPlane::Cb, ColorPlane::Cr] {
        let mut samples = frame.plane_mut(plane);
        let stride = samples.cfg.stride;
        let origin = mb_origin(plane, mb_loc, stride);
        let data = samples.data_origin_mut();
        for rect in rects.as_slice() {
            let area = rect.area(plane);
            let dst = &mut data[origin + area.offset(stride)..];
            predict_rect(slice, refs, &rect.key, plane, mb_loc, area, dst, stride, buffer)?;
        }
        if let Some(residual) = residual {
            add_plane_residual(residual, plane, qp, refs.dequant, data, origin, stride);
        }
    }
    Ok(())
}

/// Section 8.4.2: predicts one rectangle of `plane` into `dst`, which starts
/// at the rectangle's top-left sample and has row stride `stride`.
///
/// Uni-prediction interpolates straight into `dst` and, for explicit weighted
/// prediction, then weights it in place; implicit weighting doesn't apply to
/// uni-prediction (8.4.2.3). Bi-prediction interpolates both lists into
/// buffers on the stack and combines them row by row into `dst`.
fn predict_rect(
    slice: &Slice,
    refs: &InterSliceRefs<'_>,
    key: &PredKey,
    plane: ColorPlane,
    mb_loc: Point,
    area: Area,
    dst: &mut [u8],
    stride: usize,
    buffer: &mut InterpolationBuffer,
) -> Result<(), DecoderError> {
    let explicit = refs.wp_mode == WeightedPredMode::Explicit;
    match key.pred_mode {
        MbPredictionMode::Pred_L0 => {
            let reference = ref_plane(refs.ref_pics_l0, key.ref_idx_l0, "l0", plane)?;
            interpolate_block(reference, plane, mb_loc, area, key.mv_l0, dst, stride, buffer);
            if explicit {
                let wp = explicit_weights(slice, plane, key.ref_idx_l0, key.ref_idx_l1);
                weight_uni_in_place(dst, stride, area, wp.w0, wp.o0, wp.log_wd);
            }
        }
        MbPredictionMode::Pred_L1 => {
            let reference = ref_plane(refs.ref_pics_l1, key.ref_idx_l1, "l1", plane)?;
            interpolate_block(reference, plane, mb_loc, area, key.mv_l1, dst, stride, buffer);
            if explicit {
                let wp = explicit_weights(slice, plane, key.ref_idx_l0, key.ref_idx_l1);
                weight_uni_in_place(dst, stride, area, wp.w1, wp.o1, wp.log_wd);
            }
        }
        MbPredictionMode::BiPred => {
            // Rectangles are at most 16 luma or 8 chroma samples wide.
            let tmp_stride = if plane == ColorPlane::Y { 16 } else { 8 };
            let (mut pred_l0, mut pred_l1) = ([0u8; 256], [0u8; 256]);
            let reference = ref_plane(refs.ref_pics_l0, key.ref_idx_l0, "l0", plane)?;
            interpolate_block(
                reference,
                plane,
                mb_loc,
                area,
                key.mv_l0,
                &mut pred_l0,
                tmp_stride,
                buffer,
            );
            let reference = ref_plane(refs.ref_pics_l1, key.ref_idx_l1, "l1", plane)?;
            interpolate_block(
                reference,
                plane,
                mb_loc,
                area,
                key.mv_l1,
                &mut pred_l1,
                tmp_stride,
                buffer,
            );
            let (l0, l1) = (&pred_l0[..], &pred_l1[..]);
            match refs.wp_mode {
                WeightedPredMode::Default => average_into(dst, stride, l0, l1, tmp_stride, area),
                WeightedPredMode::Implicit => {
                    let (i, j) = (usize::from(key.ref_idx_l0), usize::from(key.ref_idx_l1));
                    let wp = &refs.implicit_weights[i][j];
                    weight_bi_into(dst, stride, l0, l1, tmp_stride, area, wp);
                }
                WeightedPredMode::Explicit => {
                    let wp = explicit_weights(slice, plane, key.ref_idx_l0, key.ref_idx_l1);
                    weight_bi_into(dst, stride, l0, l1, tmp_stride, area, &wp);
                }
            }
        }
        _ => {
            // P cells are keyed as L0 and the parser gives every B cell L0,
            // L1 or both, so no rectangle gets here. If one did, it would get
            // the previous behaviour: a prediction of zeros.
            debug_assert!(
                matches!(key.pred_mode, MbPredictionMode::Pred_L0 | MbPredictionMode::Pred_L1),
                "inter rectangle uses neither reference list ({:?})",
                key.pred_mode
            );
            fill_rect(dst, stride, area, 0);
        }
    }
    Ok(())
}

/// Section 8.4.2.1: the `plane` of the picture that `ref_idx` selects from a
/// reference picture list.
fn ref_plane<'a>(
    list: &[&'a DpbPicture],
    ref_idx: u8,
    list_name: &str,
    plane: ColorPlane,
) -> Result<Plane<'a>, DecoderError> {
    Ok(ref_picture(list, ref_idx, list_name)?.picture.frame.plane(plane))
}

/// Section 8.4.2.3.1: default weighted bi-prediction, the average of the two
/// predictions rounded up, written to `dst`.
fn average_into(
    dst: &mut [u8],
    dst_stride: usize,
    l0: &[u8],
    l1: &[u8],
    src_stride: usize,
    area: Area,
) {
    combine_into(dst, dst_stride, l0, l1, src_stride, area, |a, b| {
        u8::try_from((u16::from(a) + u16::from(b) + 1) >> 1).unwrap_or(u8::MAX)
    });
}

/// Section 8.4.2.3.2, Eq. 8-276: bi-prediction with the weights `wp`, written
/// to `dst`.
fn weight_bi_into(
    dst: &mut [u8],
    dst_stride: usize,
    l0: &[u8],
    l1: &[u8],
    src_stride: usize,
    area: Area,
    wp: &WeightParams,
) {
    combine_into(dst, dst_stride, l0, l1, src_stride, area, |a, b| weighted_bi_pred(a, b, wp));
}

/// Combines two predictions (row stride `src_stride`) sample by sample into
/// the rectangle at the start of `dst` (row stride `dst_stride`).
fn combine_into(
    dst: &mut [u8],
    dst_stride: usize,
    l0: &[u8],
    l1: &[u8],
    src_stride: usize,
    area: Area,
    combine: impl Fn(u8, u8) -> u8 + Copy,
) {
    let rows = usize::from(area.h);
    // Dispatching on the width gives each inlined copy of the row loop a
    // constant length, which lets LLVM vectorize it without a remainder.
    let (d, ds, s) = (dst_stride, src_stride, rows);
    match area.w {
        16 => combine_rows(dst, d, l0, l1, ds, 16, s, combine),
        12 => combine_rows(dst, d, l0, l1, ds, 12, s, combine),
        8 => combine_rows(dst, d, l0, l1, ds, 8, s, combine),
        6 => combine_rows(dst, d, l0, l1, ds, 6, s, combine),
        4 => combine_rows(dst, d, l0, l1, ds, 4, s, combine),
        2 => combine_rows(dst, d, l0, l1, ds, 2, s, combine),
        w => combine_rows(dst, d, l0, l1, ds, usize::from(w), s, combine),
    }
}

// Always inlined, so that each call in `combine_into` has a constant width.
#[allow(clippy::inline_always)]
#[inline(always)]
fn combine_rows(
    dst: &mut [u8],
    dst_stride: usize,
    l0: &[u8],
    l1: &[u8],
    src_stride: usize,
    width: usize,
    rows: usize,
    combine: impl Fn(u8, u8) -> u8,
) {
    for y in 0..rows {
        let out = &mut dst[y * dst_stride..][..width];
        let a = &l0[y * src_stride..][..width];
        let b = &l1[y * src_stride..][..width];
        for (sample, (&a, &b)) in out.iter_mut().zip(a.iter().zip(b)) {
            *sample = combine(a, b);
        }
    }
}

/// Sets every sample of the rectangle at the start of `dst` to `value`.
fn fill_rect(dst: &mut [u8], stride: usize, area: Area, value: u8) {
    for row in dst.chunks_mut(stride).take(usize::from(area.h)) {
        row[..usize::from(area.w)].fill(value);
    }
}

/// Position and size of a prediction rectangle within its macroblock, in
/// samples of one plane.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Area {
    x: u8,
    y: u8,
    w: u8,
    h: u8,
}

impl Area {
    /// Offset of the rectangle's top-left sample from the macroblock's.
    fn offset(self, stride: usize) -> usize {
        usize::from(self.y) * stride + usize::from(self.x)
    }
}

/// Index of the macroblock's top-left sample of `plane` in a plane buffer
/// with row stride `stride`. Chroma is 4:2:0, at half the luma position.
fn mb_origin(plane: ColorPlane, mb_loc: Point, stride: usize) -> usize {
    let shift = u32::from(plane != ColorPlane::Y);
    (mb_loc.y >> shift) as usize * stride + (mb_loc.x >> shift) as usize
}

/// Section 8.4.2.1: the picture that `ref_idx` selects from a reference
/// picture list.
fn ref_picture<'a>(
    list: &[&'a DpbPicture],
    ref_idx: u8,
    list_name: &str,
) -> Result<&'a DpbPicture, DecoderError> {
    list.get(usize::from(ref_idx)).copied().ok_or_else(|| {
        DecoderError::ReferenceNotFound(format!(
            "ref_idx_{list_name} {ref_idx} out of bounds (list length {})",
            list.len()
        ))
    })
}

/// Section 8.4.2.2: fractional sample interpolation of one rectangle of
/// `plane` from `ref_plane`, written to `dst` with row stride `dst_stride`.
fn interpolate_block(
    ref_plane: Plane<'_>,
    plane: ColorPlane,
    mb_loc: Point,
    area: Area,
    mv: MotionVector,
    dst: &mut [u8],
    dst_stride: usize,
    buffer: &mut InterpolationBuffer,
) {
    let Area { x, y, w, h } = area;
    if plane == ColorPlane::Y {
        interpolate_luma(ref_plane, mb_loc.x, mb_loc.y, x, y, w, h, mv, dst, dst_stride, buffer);
    } else {
        // 4:2:0: the chroma macroblock sits at half the luma position.
        let (mb_x, mb_y) = (mb_loc.x >> 1, mb_loc.y >> 1);
        interpolate_chroma(ref_plane, mb_x, mb_y, x, y, w, h, mv, dst, dst_stride);
    }
}

/// Section 8.4.3: the explicit weights of `plane` for a pair of reference
/// indices. The index of an unused list misses the weight table and gets the
/// default weight, which the uni-predicted formulas then ignore.
fn explicit_weights(
    slice: &Slice,
    plane: ColorPlane,
    ref_idx_l0: u8,
    ref_idx_l1: u8,
) -> WeightParams {
    let (l0, l1) = (usize::from(ref_idx_l0), usize::from(ref_idx_l1));
    match plane {
        ColorPlane::Y => get_explicit_luma_weights(slice, l0, l1),
        ColorPlane::Cb => get_explicit_chroma_weights(slice, l0, l1, 0),
        ColorPlane::Cr => get_explicit_chroma_weights(slice, l0, l1, 1),
    }
}

/// Section 8.4.2.3.2, Eqs. 8-274 and 8-275: explicit weighting of a
/// uni-predicted rectangle in place. `dst` starts at its top-left sample.
fn weight_uni_in_place(
    dst: &mut [u8],
    stride: usize,
    area: Area,
    weight: i32,
    offset: i32,
    log_wd: u32,
) {
    for row in dst.chunks_mut(stride).take(usize::from(area.h)) {
        for sample in &mut row[..usize::from(area.w)] {
            *sample = weighted_uni_pred(*sample, weight, offset, log_wd);
        }
    }
}

/// Sections 8.5.12 to 8.5.14: adds the residual of `plane` to its prediction.
fn add_plane_residual(
    residual: &Residual,
    plane: ColorPlane,
    qp: MbQp,
    dequant: &DequantTables,
    data: &mut [u8],
    origin: usize,
    stride: usize,
) {
    match plane {
        ColorPlane::Y => residual.add_luma_to(qp.luma, dequant, data, origin, stride),
        ColorPlane::Cb => residual.add_chroma_to(plane, qp.cb, dequant, data, origin, stride),
        ColorPlane::Cr => residual.add_chroma_to(plane, qp.cr, dequant, data, origin, stride),
    }
}

/// The fields of a `PartitionInfo` that inter prediction depends on (8.4.2):
/// the lists a cell predicts from, and each list's reference index and motion
/// vector. Cells with equal keys predict identically, so they can share one
/// interpolation call.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct PredKey {
    pred_mode: MbPredictionMode,
    ref_idx_l0: u8,
    mv_l0: MotionVector,
    ref_idx_l1: u8,
    mv_l1: MotionVector,
}

impl PredKey {
    /// A P macroblock cell. The parser leaves its `pred_mode` at None, but
    /// every P cell predicts from L0 alone, so the key is L0 with the L1
    /// fields zeroed: cells merge on `(ref_idx_l0, mv_l0)` only.
    fn p(p: &PartitionInfo) -> Self {
        Self {
            pred_mode: MbPredictionMode::Pred_L0,
            ref_idx_l0: p.ref_idx_l0,
            mv_l0: p.mv_l0,
            ..Self::default()
        }
    }

    /// A B macroblock cell, on the full key. The `mvd` fields don't affect
    /// prediction, so they are left out.
    fn b(p: &PartitionInfo) -> Self {
        Self {
            pred_mode: p.pred_mode,
            ref_idx_l0: p.ref_idx_l0,
            mv_l0: p.mv_l0,
            ref_idx_l1: p.ref_idx_l1,
            mv_l1: p.mv_l1,
        }
    }
}

/// One rectangular region of a macroblock's 4x4 motion grid whose cells share
/// a `PredKey`. Positions and sizes are in grid cells (a whole macroblock is
/// 4x4 cells).
#[derive(Clone, Copy, Default)]
struct PartitionRect {
    grid_y: u8,
    grid_x: u8,
    grid_h: u8,
    grid_w: u8,
    key: PredKey,
}

impl PartitionRect {
    /// The rectangle in samples of `plane`: a grid cell covers 4x4 luma
    /// samples, or 2x2 chroma samples in 4:2:0.
    fn area(&self, plane: ColorPlane) -> Area {
        let scale = if plane == ColorPlane::Y { 4 } else { 2 };
        Area {
            x: self.grid_x * scale,
            y: self.grid_y * scale,
            w: self.grid_w * scale,
            h: self.grid_h * scale,
        }
    }
}

/// Greedy-rectangle scan of the 4x4 motion grid: adjacent cells with equal
/// `merge_key`s are merged into maximal rectangles, each keyed by the
/// `pred_key` of its cells. Equal merge keys must imply equal prediction keys:
/// P cells merge on their L0 motion alone, B cells on the whole `PredKey`.
///
/// The parser fills the grid by replicating the same `PartitionInfo` across
/// every cell covered by an H.264 partition, so a left-to-right / top-to-bottom
/// greedy walk recovers the original partition shapes (16x16, 16x8, 8x16, 8x8,
/// 8x4, 4x8, 4x4) exactly. It also coalesces across partition boundaries when
/// neighbouring partitions happen to share a key.
fn collect_pred_rects<K: PartialEq>(
    partitions: &[[PartitionInfo; 4]; 4],
    merge_key: impl Fn(&PartitionInfo) -> K,
    pred_key: impl Fn(&PartitionInfo) -> PredKey,
    out: &mut [PartitionRect; 16],
) -> usize {
    // Fast path: the whole grid has one key (16x16 partitions, P_Skip, and
    // skipped or direct B macroblocks with uniform motion). The greedy walk
    // below would produce the same single rectangle.
    let first = merge_key(&partitions[0][0]);
    if partitions.iter().flatten().skip(1).all(|p| merge_key(p) == first) {
        let key = pred_key(&partitions[0][0]);
        out[0] = PartitionRect { grid_y: 0, grid_x: 0, grid_h: 4, grid_w: 4, key };
        return 1;
    }

    let mut visited = [[false; 4]; 4];
    let mut count = 0;
    for gy in 0..4 {
        for gx in 0..4 {
            if visited[gy][gx] {
                continue;
            }
            let key = merge_key(&partitions[gy][gx]);

            // Extend right while the next column at row gy matches.
            let mut w = 1;
            while gx + w < 4 && merge_key(&partitions[gy][gx + w]) == key {
                w += 1;
            }

            // Extend down while every cell in the next row matches across [gx, gx+w).
            let mut h = 1;
            'extend_down: while gy + h < 4 {
                for dx in 0..w {
                    if merge_key(&partitions[gy + h][gx + dx]) != key {
                        break 'extend_down;
                    }
                }
                h += 1;
            }

            for dy in 0..h {
                for dx in 0..w {
                    visited[gy + dy][gx + dx] = true;
                }
            }

            out[count] = PartitionRect {
                grid_y: gy as u8,
                grid_x: gx as u8,
                grid_h: h as u8,
                grid_w: w as u8,
                key: pred_key(&partitions[gy][gx]),
            };
            count += 1;
        }
    }
    count
}

/// The prediction rectangles of a macroblock, as merged by
/// `collect_pred_rects`. Built once per macroblock and shared by luma and both
/// chroma planes: chroma motion compensation walks the same 4x4 grid, with
/// sample offsets scaled by 2 instead of 4 (8.4.2.2.2).
struct PredRects {
    rects: [PartitionRect; 16],
    len: usize,
}

impl PredRects {
    /// The rectangles of a P macroblock, all predicted from L0. Partitions
    /// other than 8x8 come straight from `mb_type` (Table 7-13), whose cells
    /// the parser fills with one `PartitionInfo` each. The sub-macroblock
    /// types of `P_8x8` aren't kept, so those macroblocks scan the grid.
    fn for_p(mb: &PMb) -> Self {
        let cells = &mb.motion.partitions;
        let part = |grid_y: u8, grid_x: u8, grid_h: u8, grid_w: u8| PartitionRect {
            grid_y,
            grid_x,
            grid_h,
            grid_w,
            key: PredKey::p(&cells[usize::from(grid_y)][usize::from(grid_x)]),
        };
        let mut rects = [PartitionRect::default(); 16];
        let len = match mb.mb_type {
            PMbType::P_Skip | PMbType::P_L0_16x16 => {
                rects[0] = part(0, 0, 4, 4);
                1
            }
            PMbType::P_L0_L0_16x8 => {
                rects[0] = part(0, 0, 2, 4);
                rects[1] = part(2, 0, 2, 4);
                2
            }
            PMbType::P_L0_L0_8x16 => {
                rects[0] = part(0, 0, 4, 2);
                rects[1] = part(0, 2, 4, 2);
                2
            }
            PMbType::P_8x8 | PMbType::P_8x8ref0 => return Self::scan_p(&mb.motion),
        };
        Self { rects, len }
    }

    /// The rectangles of a P macroblock from a scan of its 4x4 motion grid.
    /// Merging on the small `(ref_idx_l0, mv_l0)` key keeps the scan cheap.
    fn scan_p(motion: &macroblock::MbMotion) -> Self {
        let mut rects = [PartitionRect::default(); 16];
        let merge_key = |p: &PartitionInfo| (p.ref_idx_l0, p.mv_l0);
        let len = collect_pred_rects(&motion.partitions, merge_key, PredKey::p, &mut rects);
        Self { rects, len }
    }

    /// The rectangles of a B macroblock, each L0-only, L1-only or
    /// bi-predicted with fixed references and motion vectors.
    fn for_b(motion: &macroblock::MbMotion) -> Self {
        let mut rects = [PartitionRect::default(); 16];
        let len = collect_pred_rects(&motion.partitions, PredKey::b, PredKey::b, &mut rects);
        Self { rects, len }
    }

    fn as_slice(&self) -> &[PartitionRect] {
        &self.rects[..self.len]
    }
}

#[cfg(test)]
mod weighted_pred_tests {
    use super::*;

    // --- weighted_uni_pred tests (Eq 8-274/8-275) ---

    #[test]
    fn uni_pred_identity() {
        // w = 1 << log_wd, o = 0 should produce the same value (default weight)
        for log_wd in 0..8u32 {
            let w = 1i32 << log_wd;
            for pred in [0u8, 1, 127, 128, 254, 255] {
                assert_eq!(
                    weighted_uni_pred(pred, w, 0, log_wd),
                    pred,
                    "identity failed for pred={pred}, log_wd={log_wd}"
                );
            }
        }
    }

    #[test]
    fn uni_pred_log_wd_zero() {
        // log_wd = 0: result = pred * w + o, clamped
        assert_eq!(weighted_uni_pred(100, 2, 10, 0), 210);
        assert_eq!(weighted_uni_pred(200, 2, 0, 0), 255); // clamped
        assert_eq!(weighted_uni_pred(10, -1, 0, 0), 0); // clamped negative
    }

    #[test]
    fn uni_pred_with_offset() {
        // log_wd = 7, w = 128 (=1<<7), o = 10 => pred + 10
        assert_eq!(weighted_uni_pred(100, 128, 10, 7), 110);
        // Clamping to 255
        assert_eq!(weighted_uni_pred(250, 128, 10, 7), 255);
    }

    #[test]
    fn uni_pred_clamping() {
        // Should clamp to [0, 255]
        assert_eq!(weighted_uni_pred(0, 128, -50, 7), 0);
        assert_eq!(weighted_uni_pred(255, 256, 100, 7), 255);
    }

    // --- weighted_bi_pred tests (Eq 8-276) ---

    #[test]
    fn bi_pred_equal_weights() {
        // w0=w1=32, log_wd=5, o0=o1=0 => same as default (l0+l1+1)>>1
        let wp = WeightParams { log_wd: 5, w0: 32, o0: 0, w1: 32, o1: 0 };
        assert_eq!(weighted_bi_pred(100, 200, &wp), 150);
        assert_eq!(weighted_bi_pred(0, 0, &wp), 0);
        assert_eq!(weighted_bi_pred(255, 255, &wp), 255);
        assert_eq!(weighted_bi_pred(1, 0, &wp), 1); // (1*32 + 0*32 + 32) >> 6 = 64 >> 6 = 1
    }

    #[test]
    fn bi_pred_arithmetic() {
        // Manual calculation: log_wd=5, w0=32, w1=32, o0=0, o1=0
        // (100*32 + 200*32 + 32) >> 6 + 0 = (3200 + 6400 + 32) >> 6 = 9632 >> 6 = 150
        let wp = WeightParams { log_wd: 5, w0: 32, o0: 0, w1: 32, o1: 0 };
        assert_eq!(weighted_bi_pred(100, 200, &wp), 150);

        // w0=64, w1=0 => effectively uni-pred from L0
        // (100*64 + 200*0 + 32) >> 6 + 0 = 6432 >> 6 = 100
        let wp2 = WeightParams { log_wd: 5, w0: 64, o0: 0, w1: 0, o1: 0 };
        assert_eq!(weighted_bi_pred(100, 200, &wp2), 100);
    }

    #[test]
    fn bi_pred_with_offsets() {
        // log_wd=5, w0=32, w1=32, o0=20, o1=10
        // (100*32 + 100*32 + 32) >> 6 + (20+10+1)>>1 = (6400+32)>>6 + 15 = 100 + 15 = 115
        let wp = WeightParams { log_wd: 5, w0: 32, o0: 20, w1: 32, o1: 10 };
        assert_eq!(weighted_bi_pred(100, 100, &wp), 115);
    }

    #[test]
    fn bi_pred_clamping() {
        let wp = WeightParams { log_wd: 5, w0: 64, o0: 127, w1: 64, o1: 127 };
        assert_eq!(weighted_bi_pred(255, 255, &wp), 255); // clamped
        let wp2 = WeightParams { log_wd: 5, w0: 64, o0: -128, w1: 64, o1: -128 };
        assert_eq!(weighted_bi_pred(0, 0, &wp2), 0); // clamped
    }

    // --- Default weight identity ---

    #[test]
    fn default_weight_is_identity() {
        // Explicit weights with w = 1 << log_wd, o = 0 should match unweighted copy
        for log_wd in 0..8u32 {
            let w = 1i32 << log_wd;
            for pred in [0u8, 64, 128, 200, 255] {
                let weighted = weighted_uni_pred(pred, w, 0, log_wd);
                assert_eq!(
                    weighted, pred,
                    "default weight not identity: pred={pred}, log_wd={log_wd}"
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::h264::plane::{PlaneConfig, PlaneMut, PlaneOffset};

    fn test_plane_cfg(width: usize, height: usize) -> PlaneConfig {
        PlaneConfig {
            stride: width + 32,
            width,
            height,
            xorigin: 16,
            yorigin: 16,
            alloc_height: height + 32,
        }
    }

    fn create_test_plane(width: usize, height: usize, fill: u8) -> (Vec<u8>, PlaneConfig) {
        let cfg = test_plane_cfg(width, height);
        let buf = vec![fill; cfg.total_bytes()];
        (buf, cfg)
    }

    fn alternating_plane(width: usize, height: usize) -> (Vec<u8>, PlaneConfig) {
        let cfg = test_plane_cfg(width, height);
        let mut buf = vec![0u8; cfg.total_bytes()];
        for y in 0..cfg.alloc_height {
            for x in 0..cfg.stride {
                buf[y * cfg.stride + x] = if x % 2 == 0 { 100 } else { 200 };
            }
        }
        (buf, cfg)
    }

    #[test]
    fn test_interpolate_integer() {
        let (buf, cfg) = create_test_plane(32, 32, 100);
        let plane = Plane { data: &buf, cfg };
        let mut dst = [0u8; 16];
        let mut buffer = InterpolationBuffer::new();
        interpolate_luma(
            plane,
            0,
            0,
            0,
            0,
            4,
            4,
            MotionVector { x: 0, y: 0 },
            &mut dst,
            4,
            &mut buffer,
        );
        assert_eq!(dst, [100; 16]);
    }

    #[test]
    fn test_interpolate_half_pel_horizontal() {
        let (buf, cfg) = alternating_plane(32, 32);
        let plane = Plane { data: &buf, cfg };
        let mut dst = [0u8; 16];
        let mut buffer = InterpolationBuffer::new();
        interpolate_luma(
            plane,
            2,
            2,
            0,
            0,
            4,
            4,
            MotionVector { x: 2, y: 0 },
            &mut dst,
            4,
            &mut buffer,
        );
        assert_eq!(dst[0], 150);
    }

    #[test]
    fn test_interpolate_quarter_pel_a() {
        let (buf, cfg) = alternating_plane(32, 32);
        let plane = Plane { data: &buf, cfg };
        let mut dst = [0u8; 16];
        let mut buffer = InterpolationBuffer::new();
        interpolate_luma(
            plane,
            2,
            2,
            0,
            0,
            4,
            4,
            MotionVector { x: 1, y: 0 },
            &mut dst,
            4,
            &mut buffer,
        );
        assert_eq!(dst[0], 125);
    }

    #[test]
    fn test_interpolate_chroma_integer() {
        let (buf, cfg) = create_test_plane(16, 16, 50);
        let plane = Plane { data: &buf, cfg };
        let mut dst = [0u8; 4];
        interpolate_chroma(plane, 0, 0, 0, 0, 2, 2, MotionVector { x: 0, y: 0 }, &mut dst, 2);
        assert_eq!(dst, [50; 4]);
    }

    #[test]
    fn test_interpolate_chroma_half() {
        let (buf, cfg) = alternating_plane(16, 16);
        let plane = Plane { data: &buf, cfg };
        let mut dst = [0u8; 4];
        interpolate_chroma(plane, 0, 0, 0, 0, 2, 2, MotionVector { x: 4, y: 0 }, &mut dst, 2);
        assert_eq!(dst[0], 150);
        assert_eq!(dst[1], 150);
    }

    #[test]
    fn test_interpolate_chroma_eighth() {
        let cfg = test_plane_cfg(16, 16);
        let mut buf = vec![100u8; cfg.total_bytes()];
        // Set visible (1, 0) = 164.
        let one_off = cfg.yorigin * cfg.stride + cfg.xorigin + 1;
        buf[one_off] = 164;
        let plane = Plane { data: &buf, cfg };
        let mut dst = [0u8; 1];
        interpolate_chroma(plane, 0, 0, 0, 0, 1, 1, MotionVector { x: 1, y: 0 }, &mut dst, 1);
        assert_eq!(dst[0], 108);
    }

    /// `(grid_y, grid_x, grid_h, grid_w, key)` of each rectangle, in order.
    fn rect_list(rects: &PredRects) -> Vec<(u8, u8, u8, u8, PredKey)> {
        rects.as_slice().iter().map(|r| (r.grid_y, r.grid_x, r.grid_h, r.grid_w, r.key)).collect()
    }

    fn uniform_motion(info: PartitionInfo) -> macroblock::MbMotion {
        macroblock::MbMotion { partitions: [[info; 4]; 4], decoded_mask: 0xFFFF }
    }

    /// The key of P cells with the given L0 motion.
    fn l0_key(ref_idx: u8, mv: MotionVector) -> PredKey {
        PredKey {
            pred_mode: MbPredictionMode::Pred_L0,
            ref_idx_l0: ref_idx,
            mv_l0: mv,
            ..PredKey::default()
        }
    }

    /// `PredRects::for_p` of a P macroblock with the given type and motion.
    fn p_rects(mb_type: PMbType, motion: macroblock::MbMotion) -> Vec<(u8, u8, u8, u8, PredKey)> {
        rect_list(&PredRects::for_p(&PMb { mb_type, motion, ..Default::default() }))
    }

    #[test]
    fn test_pred_rects_p_16x16_is_one_rect() {
        let mv = MotionVector { x: 3, y: -2 };
        let motion =
            uniform_motion(PartitionInfo { ref_idx_l0: 1, mv_l0: mv, ..Default::default() });
        for mb_type in [PMbType::P_Skip, PMbType::P_L0_16x16] {
            assert_eq!(p_rects(mb_type, motion.clone()), vec![(0, 0, 4, 4, l0_key(1, mv))]);
        }
    }

    #[test]
    fn test_pred_rects_p_16x8_and_8x8() {
        // 16x8: the two halves have different MVs.
        let top_mv = MotionVector { x: 1, y: 0 };
        let bottom_mv = MotionVector { x: 2, y: 0 };
        let mut motion = uniform_motion(PartitionInfo { mv_l0: top_mv, ..Default::default() });
        for row in &mut motion.partitions[2..] {
            for cell in row.iter_mut() {
                cell.mv_l0 = bottom_mv;
            }
        }
        assert_eq!(
            p_rects(PMbType::P_L0_L0_16x8, motion),
            vec![(0, 0, 2, 4, l0_key(0, top_mv)), (2, 0, 2, 4, l0_key(0, bottom_mv))]
        );

        // 8x8: each quadrant has its own reference index.
        let refs = [[0, 0, 1, 1], [0, 0, 1, 1], [2, 2, 3, 3], [2, 2, 3, 3]];
        let mut motion = uniform_motion(PartitionInfo::default());
        for (row, row_refs) in motion.partitions.iter_mut().zip(refs) {
            for (cell, ref_idx) in row.iter_mut().zip(row_refs) {
                cell.ref_idx_l0 = ref_idx;
            }
        }
        let mv = MotionVector::default();
        assert_eq!(
            p_rects(PMbType::P_8x8, motion),
            vec![
                (0, 0, 2, 2, l0_key(0, mv)),
                (0, 2, 2, 2, l0_key(1, mv)),
                (2, 0, 2, 2, l0_key(2, mv)),
                (2, 2, 2, 2, l0_key(3, mv))
            ]
        );
    }

    #[test]
    fn test_pred_rects_p_ignores_l1_and_mvd() {
        // A P key covers only (ref_idx_l0, mv_l0): stale L1 fields and mvds
        // must not split a partition.
        let mut motion = uniform_motion(PartitionInfo { ref_idx_l0: 2, ..Default::default() });
        motion.partitions[1][2].ref_idx_l1 = 7;
        motion.partitions[3][0].mvd_l0 = MotionVector { x: 5, y: 5 };
        let mv = MotionVector::default();
        assert_eq!(p_rects(PMbType::P_8x8, motion), vec![(0, 0, 4, 4, l0_key(2, mv))]);
    }

    #[test]
    fn test_pred_rects_b_split_on_full_key() {
        // Top half Pred_L0; bottom-left BiPred; bottom-right Pred_L1 with the
        // same L1 motion as the BiPred cells. Each region is one rectangle
        // with fixed lists, references and motion vectors.
        let l0 = PartitionInfo {
            pred_mode: MbPredictionMode::Pred_L0,
            mv_l0: MotionVector { x: 4, y: 4 },
            ref_idx_l1: u8::MAX,
            ..Default::default()
        };
        let bi = PartitionInfo {
            pred_mode: MbPredictionMode::BiPred,
            ref_idx_l0: 1,
            mv_l0: MotionVector { x: -4, y: 0 },
            mv_l1: MotionVector { x: 8, y: 0 },
            ..Default::default()
        };
        let l1 = PartitionInfo {
            pred_mode: MbPredictionMode::Pred_L1,
            ref_idx_l0: u8::MAX,
            mv_l1: bi.mv_l1,
            ..Default::default()
        };
        let mut motion = uniform_motion(l0);
        for row in &mut motion.partitions[2..] {
            row[..2].fill(bi);
            row[2..].fill(l1);
        }
        assert_eq!(
            rect_list(&PredRects::for_b(&motion)),
            vec![
                (0, 0, 2, 4, PredKey::b(&l0)),
                (2, 0, 2, 2, PredKey::b(&bi)),
                (2, 2, 2, 2, PredKey::b(&l1))
            ]
        );

        // The mvds don't affect prediction, so they don't split rectangles.
        let mut motion = uniform_motion(bi);
        motion.partitions[1][1].mvd_l1 = MotionVector { x: 3, y: 3 };
        assert_eq!(rect_list(&PredRects::for_b(&motion)), vec![(0, 0, 4, 4, PredKey::b(&bi))]);
    }

    #[test]
    fn test_pred_rects_fast_path_falls_back() {
        // Only the last cell differs, so the greedy walk must run.
        let mut motion = uniform_motion(PartitionInfo::default());
        motion.partitions[3][3].ref_idx_l0 = 1;
        let mv = MotionVector::default();
        assert_eq!(
            p_rects(PMbType::P_8x8, motion),
            vec![
                (0, 0, 3, 4, l0_key(0, mv)),
                (3, 0, 1, 3, l0_key(0, mv)),
                (3, 3, 1, 1, l0_key(1, mv))
            ]
        );
    }

    #[test]
    fn test_pred_rects_p_from_mb_type() {
        // 8x16: left and right halves with different motion.
        let (left, right) = (MotionVector { x: 1, y: 2 }, MotionVector { x: -3, y: 0 });
        let mut motion = uniform_motion(PartitionInfo { mv_l0: left, ..Default::default() });
        for row in &mut motion.partitions {
            row[2..].fill(PartitionInfo { mv_l0: right, ..Default::default() });
        }
        assert_eq!(
            p_rects(PMbType::P_L0_L0_8x16, motion),
            vec![(0, 0, 4, 2, l0_key(0, left)), (0, 2, 4, 2, l0_key(0, right))]
        );

        // The partitions come from mb_type, so two 16x8 halves with the same
        // motion stay two rectangles, where the grid scan would merge them.
        // Either way the prediction is the same.
        let motion = uniform_motion(PartitionInfo { mv_l0: left, ..Default::default() });
        assert_eq!(
            p_rects(PMbType::P_L0_L0_16x8, motion),
            vec![(0, 0, 2, 4, l0_key(0, left)), (2, 0, 2, 4, l0_key(0, left))]
        );
    }

    #[test]
    fn test_bi_combine_matches_per_sample() {
        let wp = WeightParams { log_wd: 5, w0: 40, o0: -3, w1: 24, o1: 7 };
        let l0: Vec<u8> = (0..256).map(|i| u8::try_from(i * 7 % 256).expect("fits")).collect();
        let l1: Vec<u8> =
            (0..256).map(|i| u8::try_from((i * 13 + 5) % 256).expect("fits")).collect();
        let stride = 24;
        // Every rectangle width, placed at row 1, column 1 of the destination.
        for (w, h) in [(2, 2), (4, 8), (6, 4), (8, 8), (12, 4), (16, 16), (16, 8)] {
            let area = Area { x: 0, y: 0, w, h };
            let mut avg = vec![9u8; stride * 18];
            average_into(&mut avg[stride + 1..], stride, &l0, &l1, 16, area);
            let mut weighted = vec![9u8; stride * 18];
            weight_bi_into(&mut weighted[stride + 1..], stride, &l0, &l1, 16, area, &wp);
            for i in 0..stride * 18 {
                let (row, col) = (i / stride, i % stride);
                if (1..=usize::from(h)).contains(&row) && (1..=usize::from(w)).contains(&col) {
                    let src = (row - 1) * 16 + col - 1;
                    let (a, b) = (l0[src], l1[src]);
                    let mean = (u16::from(a) + u16::from(b)).div_ceil(2);
                    assert_eq!(u16::from(avg[i]), mean, "average at ({row}, {col}), {w}x{h}");
                    let want = weighted_bi_pred(a, b, &wp);
                    assert_eq!(weighted[i], want, "weighted at ({row}, {col}), {w}x{h}");
                } else {
                    assert_eq!((avg[i], weighted[i]), (9, 9), "outside at ({row}, {col}), {w}x{h}");
                }
            }
        }
    }

    #[test]
    fn test_fill_rect_stays_inside() {
        let stride = 10;
        let mut buf = vec![1u8; stride * 6];
        fill_rect(&mut buf[stride + 2..], stride, Area { x: 0, y: 0, w: 4, h: 3 }, 0);
        for (i, &value) in buf.iter().enumerate() {
            let (row, col) = (i / stride, i % stride);
            let inside = (1..4).contains(&row) && (2..6).contains(&col);
            assert_eq!(value, u8::from(!inside), "({row}, {col})");
        }
    }

    #[test]
    fn test_rect_area_and_origins() {
        let rect =
            PartitionRect { grid_y: 2, grid_x: 1, grid_h: 2, grid_w: 3, ..Default::default() };
        assert_eq!(rect.area(ColorPlane::Y), Area { x: 4, y: 8, w: 12, h: 8 });
        assert_eq!(rect.area(ColorPlane::Cr), Area { x: 2, y: 4, w: 6, h: 4 });
        assert_eq!(rect.area(ColorPlane::Y).offset(100), 8 * 100 + 4);
        let mb_loc = Point { x: 32, y: 48 };
        assert_eq!(mb_origin(ColorPlane::Y, mb_loc, 200), 48 * 200 + 32);
        assert_eq!(mb_origin(ColorPlane::Cb, mb_loc, 100), 24 * 100 + 16);
    }

    #[test]
    fn test_weight_uni_in_place_matches_per_sample() {
        let stride = 20;
        let area = Area { x: 0, y: 0, w: 12, h: 4 };
        for (weight, offset, log_wd) in [(64, 0, 6), (-3, 10, 2), (5, -20, 0), (200, 127, 7)] {
            let original: Vec<u8> =
                (0..stride * 6).map(|i| u8::try_from(i * 37 % 256).expect("fits")).collect();
            let mut dst = original.clone();
            // The rectangle starts at row 1, column 3.
            weight_uni_in_place(&mut dst[stride + 3..], stride, area, weight, offset, log_wd);
            for (i, (&got, &was)) in dst.iter().zip(&original).enumerate() {
                let (row, col) = (i / stride, i % stride);
                let inside = (1..5).contains(&row) && (3..15).contains(&col);
                let want =
                    if inside { weighted_uni_pred(was, weight, offset, log_wd) } else { was };
                assert_eq!(got, want, "sample ({row}, {col}), weight {weight}, log_wd {log_wd}");
            }
        }
    }

    /// `interpolate_chroma` against the clamped reference, for one block.
    fn check_chroma_block(
        buf: &[u8],
        cfg: PlaneConfig,
        (w, h): (u8, u8),
        (x_int, y_int): (i32, i32),
        (x_frac, y_frac): (i16, i16),
    ) {
        let mv = MotionVector {
            x: i16::try_from(x_int * 8).expect("fits") + x_frac,
            y: i16::try_from(y_int * 8).expect("fits") + y_frac,
        };
        let (mut fast, mut clamped) = ([0u8; 64], [0u8; 64]);
        interpolate_chroma(Plane { data: buf, cfg }, 0, 0, 0, 0, w, h, mv, &mut fast, 8);
        let plane = Plane { data: buf, cfg };
        interpolate_chroma_clamped(plane, x_int, y_int, x_frac, y_frac, w, h, &mut clamped, 8);
        assert_eq!(fast, clamped, "{w}x{h} at ({x_int}, {y_int}), fraction ({x_frac}, {y_frac})");
    }

    #[test]
    fn test_chroma_fast_path_matches_clamped() {
        // Every block size and fraction, at positions inside, on and beyond
        // each edge of a 24x16 plane: wherever interpolate_chroma takes its
        // fast path, it must match the clamped reference exactly.
        let cfg = test_plane_cfg(24, 16);
        let mut state = 0x9E37_79B9_7F4A_7C15_u64;
        let buf: Vec<u8> = (0..cfg.total_bytes())
            .map(|_| {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                state.to_be_bytes()[0]
            })
            .collect();
        let positions = |len: i32, size: u8| {
            let size = i32::from(size);
            [-3, -1, 0, 1, 5, len - size - 1, len - size, len - size + 1, len - 1]
        };
        for (w, h) in [2u8, 4, 6, 8].iter().flat_map(|&w| [2u8, 4, 6, 8].map(move |h| (w, h))) {
            for x_int in positions(24, w) {
                for y_int in positions(16, h) {
                    for x_frac in 0..8 {
                        for y_frac in 0..8 {
                            let (int, frac) = ((x_int, y_int), (x_frac, y_frac));
                            check_chroma_block(&buf, cfg, (w, h), int, frac);
                        }
                    }
                }
            }
        }
    }
}
