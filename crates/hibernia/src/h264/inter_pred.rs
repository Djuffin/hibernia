use super::decoder::{DecodingError, VideoFrame};
use super::dpb::DpbPicture;
use super::macroblock::{
    self, get_4x4chroma_block_location, BMb, MbPredictionMode, MotionVector, PMb, PartitionInfo,
};
use super::slice::{Slice, SliceType};
use super::residual::{add_residual_4x4, Block4x4};
use super::{ColorPlane, Point};
use v_frame::plane::Plane;

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
    ref_plane: &Plane<u8>,
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
    ref_plane: &Plane<u8>,
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
            let mut intermediate = [[0i32; 16]; 21];
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
    ref_plane: &Plane<u8>,
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
    if x_int >= 0
        && x_int + (width as i32) < plane_width
        && y_int >= 0
        && y_int + (height as i32) < plane_height
    {
        let stride = ref_plane.cfg.stride;
        let data = ref_plane.data_origin();
        let base = (y_int as usize) * stride + (x_int as usize);
        // Worst case is the (_, _) bilinear arm, which reads up through row
        // (y_int + height) at column (x_int + width) inclusive -- slice end
        // = base + height*stride + width + 1.
        assert!(base + (height as usize) * stride + (width as usize) + 1 <= data.len());
        if height > 0 {
            assert!(
                (height as usize - 1) * dst_stride + (width as usize) <= dst.len()
            );
        }

        macro_rules! interpolate_chroma_impl {
            ($w:expr) => {
                match (x_frac, y_frac) {
                    (0, 0) => {
                        for y in 0..height as usize {
                            let off = base + y * stride;
                            let d = &mut dst[y * dst_stride..y * dst_stride + $w];
                            d.copy_from_slice(&data[off..off + $w]);
                        }
                    }
                    (_, 0) => {
                        let w00 = 8 - x_frac;
                        let w10 = x_frac;
                        for y in 0..height as usize {
                            let off = base + y * stride;
                            let src = &data[off..off + $w + 1];
                            let d = &mut dst[y * dst_stride..y * dst_stride + $w];
                            for x in 0..$w {
                                let val_a = src[x] as i16;
                                let val_b = src[x + 1] as i16;
                                d[x] = ((w00 * val_a + w10 * val_b + 4) >> 3) as u8;
                            }
                        }
                    }
                    (0, _) => {
                        let w00 = 8 - y_frac;
                        let w01 = y_frac;
                        for y in 0..height as usize {
                            let off0 = base + y * stride;
                            let off1 = off0 + stride;
                            let src0 = &data[off0..off0 + $w];
                            let src1 = &data[off1..off1 + $w];
                            let d = &mut dst[y * dst_stride..y * dst_stride + $w];
                            for x in 0..$w {
                                let val_a = src0[x] as i16;
                                let val_c = src1[x] as i16;
                                d[x] = ((w00 * val_a + w01 * val_c + 4) >> 3) as u8;
                            }
                        }
                    }
                    _ => {
                        // Equation 8-270: Bilinear interpolation
                        // The weights (8-xFrac) and xFrac are used for linear interpolation.
                        let w00 = (8 - x_frac) * (8 - y_frac);
                        let w10 = x_frac * (8 - y_frac);
                        let w01 = (8 - x_frac) * y_frac;
                        let w11 = x_frac * y_frac;

                        for y in 0..height as usize {
                            let off0 = base + y * stride;
                            let off1 = off0 + stride;
                            let src0 = &data[off0..off0 + $w + 1];
                            let src1 = &data[off1..off1 + $w + 1];
                            let d = &mut dst[y * dst_stride..y * dst_stride + $w];

                            for x in 0..$w {
                                let val_a = src0[x] as i16;
                                let val_b = src0[x + 1] as i16;
                                let val_c = src1[x] as i16;
                                let val_d = src1[x + 1] as i16;

                                // Equation 8-270:
                                // predPartLXC[x, y] = ( (8-xFrac)*(8-yFrac)*A + xFrac*(8-yFrac)*B + ... + 32 ) >> 6
                                let prediction = (w00 * val_a + w10 * val_b + w01 * val_c + w11 * val_d + 32) >> 6;
                                d[x] = prediction as u8;
                            }
                        }
                    }
                }
            };
        }

        match width {
            2 => interpolate_chroma_impl!(2),
            4 => interpolate_chroma_impl!(4),
            8 => interpolate_chroma_impl!(8),
            _ => interpolate_chroma_impl!(width as usize),
        }
    } else {
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
enum WeightedPredMode {
    Default,
    Explicit,
    Implicit,
}

// Resolved weighting parameters for one prediction direction (Section 8.4.3 outputs).
struct WeightParams {
    log_wd: u32,
    w0: i32,
    o0: i32,
    w1: i32,
    o1: i32,
}

/// Determine the weighted prediction mode for the current slice (Section 8.4.2.3).
fn get_weighted_pred_mode(slice: &Slice) -> WeightedPredMode {
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

pub fn render_luma_inter_prediction(
    slice: &Slice,
    mb: &PMb,
    mb_loc: Point,
    frame: &mut VideoFrame,
    residuals: &[Block4x4],
    ref_pics_l0: &[&DpbPicture],
    buffer: &mut InterpolationBuffer,
) -> Result<(), DecodingError> {
    let y_plane = &mut frame.planes[0];
    let wp_mode = get_weighted_pred_mode(slice);

    let y_stride = y_plane.cfg.stride;
    let mb_origin = (mb_loc.y as usize) * y_stride + (mb_loc.x as usize);
    let y_data = y_plane.data_origin_mut();
    assert!(mb_origin + 15 * y_stride + 16 <= y_data.len());

    for raster_idx in 0..16 {
        let (grid_x, grid_y) = (raster_idx % 4, raster_idx / 4);
        let partition = mb.motion.partitions[grid_y as usize][grid_x as usize];

        let ref_idx = partition.ref_idx_l0;
        let mv = partition.mv_l0;

        let ref_pic = *ref_pics_l0.get(ref_idx as usize).ok_or_else(|| {
            DecodingError::ReferenceNotFound(format!(
                "ref_idx_l0 {} out of bounds (list length {})",
                ref_idx,
                ref_pics_l0.len()
            ))
        })?;
        let ref_plane = &ref_pic.picture.frame.planes[0];

        let blk_x = grid_x * 4;
        let blk_y = grid_y * 4;

        let mut dst = [0u8; 16]; // 4x4 block

        interpolate_luma(
            ref_plane,
            mb_loc.x,
            mb_loc.y,
            blk_x as u8,
            blk_y as u8,
            4,
            4,
            mv,
            &mut dst,
            4, // stride for 4x4 block buffer
            buffer,
        );

        // Section 8.4.2.3: Apply weighted prediction before residual addition
        if wp_mode == WeightedPredMode::Explicit {
            let wp = get_explicit_luma_weights(slice, ref_idx as usize, 0);
            for sample in &mut dst {
                *sample = weighted_uni_pred(*sample, wp.w0, wp.o0, wp.log_wd);
            }
        }

        // Add residual
        let blk_idx =
            macroblock::get_4x4luma_block_index(Point { x: blk_x as u32, y: blk_y as u32 });
        if let Some(residual_blk) = residuals.get(blk_idx as usize) {
            add_residual_4x4(&mut dst, 0, 4, residual_blk);
        }

        // Copy to frame
        let cell_base = mb_origin + (blk_y as usize) * y_stride + (blk_x as usize);
        for y in 0..4 {
            let row_base = cell_base + y * y_stride;
            y_data[row_base..row_base + 4].copy_from_slice(&dst[y * 4..y * 4 + 4]);
        }
    }
    Ok(())
}

/// One rectangular region of a 16x16 macroblock's 4x4 motion partition grid
/// that shares a single (ref_idx, mv) for one prediction direction. Sizes and
/// positions are in 4x4-grid units (so a full MB is grid_h=grid_w=4).
#[derive(Clone, Copy, Default)]
struct PartitionRect {
    grid_y: u8,
    grid_x: u8,
    grid_h: u8,
    grid_w: u8,
    ref_idx: u8,
    mv: MotionVector,
}

/// Greedy-rectangle scan of the 4x4 motion partition grid. `classify` returns
/// `Some((ref_idx, mv))` for cells where the requested direction is active and
/// `None` for cells to skip. Adjacent cells with identical keys are merged
/// into one maximal rectangle.
///
/// The parser fills the grid by replicating the same `PartitionInfo` across
/// every cell covered by an H.264 partition, so a left-to-right / top-to-bottom
/// greedy walk recovers the original partition shapes (16x16, 16x8, 8x16, 8x8,
/// 8x4, 4x8, 4x4) exactly. It also coalesces across partition boundaries when
/// they happen to share the requested direction's (ref_idx, mv).
fn collect_pred_rects(
    partitions: &[[PartitionInfo; 4]; 4],
    classify: impl Fn(&PartitionInfo) -> Option<(u8, MotionVector)>,
    out: &mut [PartitionRect; 16],
) -> usize {
    let mut visited = [[false; 4]; 4];
    let mut count = 0;
    for gy in 0..4 {
        for gx in 0..4 {
            if visited[gy][gx] {
                continue;
            }
            let key = match classify(&partitions[gy][gx]) {
                Some(k) => k,
                None => {
                    visited[gy][gx] = true;
                    continue;
                }
            };

            // Extend right while the next column at row gy matches.
            let mut w = 1;
            while gx + w < 4 && classify(&partitions[gy][gx + w]) == Some(key) {
                w += 1;
            }

            // Extend down while every cell in the next row matches across [gx, gx+w).
            let mut h = 1;
            'extend_down: while gy + h < 4 {
                for dx in 0..w {
                    if classify(&partitions[gy + h][gx + dx]) != Some(key) {
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
                ref_idx: key.0,
                mv: key.1,
            };
            count += 1;
        }
    }
    count
}

/// Classifier for B-frame L0 prediction: active iff `pred_mode` includes L0.
fn classify_b_l0(p: &PartitionInfo) -> Option<(u8, MotionVector)> {
    matches!(p.pred_mode, MbPredictionMode::Pred_L0 | MbPredictionMode::BiPred)
        .then_some((p.ref_idx_l0, p.mv_l0))
}

/// Classifier for B-frame L1 prediction: active iff `pred_mode` includes L1.
fn classify_b_l1(p: &PartitionInfo) -> Option<(u8, MotionVector)> {
    matches!(p.pred_mode, MbPredictionMode::Pred_L1 | MbPredictionMode::BiPred)
        .then_some((p.ref_idx_l1, p.mv_l1))
}

pub fn render_chroma_inter_prediction(
    slice: &Slice,
    mb: &PMb,
    mb_loc: Point,
    plane: ColorPlane,
    frame: &mut VideoFrame,
    residuals: &[Block4x4],
    ref_pics_l0: &[&DpbPicture],
) -> Result<(), DecodingError> {
    let chroma_plane = &mut frame.planes[plane as usize];
    let mb_x_chroma = mb_loc.x >> 1;
    let mb_y_chroma = mb_loc.y >> 1;
    let wp_mode = get_weighted_pred_mode(slice);
    let chroma_idx = plane as usize - 1; // Cb=0, Cr=1

    // 1. Prediction. Coalesce the 4x4 motion partition grid into maximal
    // rectangles sharing the same (ref_idx_l0, mv_l0) and call
    // interpolate_chroma once per partition (16x reduction in the common case
    // of a single 16x16 P partition).
    let mut pred_buf = [0u8; 64]; // 8x8 chroma block, row-major, stride 8
    let mut rects = [PartitionRect::default(); 16];
    let n_rects = collect_pred_rects(
        &mb.motion.partitions,
        |p| Some((p.ref_idx_l0, p.mv_l0)),
        &mut rects,
    );
    for rect in &rects[..n_rects] {
        let ref_pic = *ref_pics_l0.get(rect.ref_idx as usize).ok_or_else(|| {
            DecodingError::ReferenceNotFound(format!(
                "ref_idx_l0 {} out of bounds (list length {})",
                rect.ref_idx,
                ref_pics_l0.len()
            ))
        })?;
        let ref_plane = &ref_pic.picture.frame.planes[plane as usize];
        let cx = rect.grid_x as usize * 2;
        let cy = rect.grid_y as usize * 2;
        let cw = rect.grid_w as usize * 2;
        let ch = rect.grid_h as usize * 2;
        interpolate_chroma(
            ref_plane,
            mb_x_chroma,
            mb_y_chroma,
            cx as u8,
            cy as u8,
            cw as u8,
            ch as u8,
            rect.mv,
            &mut pred_buf[cy * 8 + cx..],
            8,
        );
    }

    let chroma_stride = chroma_plane.cfg.stride;
    let mb_origin = (mb_y_chroma as usize) * chroma_stride + (mb_x_chroma as usize);
    let chroma_data = chroma_plane.data_origin_mut();
    // One worst-case bound check for the 8x8 chroma MB; the per-pixel stores
    // below stay within it and LLVM elides their checks.
    assert!(mb_origin + 7 * chroma_stride + 8 <= chroma_data.len());

    // Per-2x2-cell weighted prediction and write-back from the staged 8x8 buffer.
    for blk_idx in 0..16 {
        let (grid_x, grid_y) = (blk_idx % 4, blk_idx / 4);
        let partition = mb.motion.partitions[grid_y as usize][grid_x as usize];
        let ref_idx = partition.ref_idx_l0;

        let blk_x = (grid_x * 4) >> 1;
        let blk_y = (grid_y * 4) >> 1;
        let buf_off = blk_y * 8 + blk_x;

        let mut dst = [
            pred_buf[buf_off],
            pred_buf[buf_off + 1],
            pred_buf[buf_off + 8],
            pred_buf[buf_off + 9],
        ];

        // Section 8.4.2.3: Apply weighted prediction
        if wp_mode == WeightedPredMode::Explicit {
            let wp = get_explicit_chroma_weights(slice, ref_idx as usize, 0, chroma_idx);
            for sample in &mut dst {
                *sample = weighted_uni_pred(*sample, wp.w0, wp.o0, wp.log_wd);
            }
        }

        let cell_base = mb_origin + blk_y * chroma_stride + blk_x;
        chroma_data[cell_base] = dst[0];
        chroma_data[cell_base + 1] = dst[1];
        chroma_data[cell_base + chroma_stride] = dst[2];
        chroma_data[cell_base + chroma_stride + 1] = dst[3];
    }

    // 2. Residuals (Block by block 4x4)
    for (blk_idx, residual_blk) in residuals.iter().enumerate() {
        let blk_loc = get_4x4chroma_block_location(blk_idx as u8);
        let blk_base =
            mb_origin + (blk_loc.y as usize) * chroma_stride + (blk_loc.x as usize);
        add_residual_4x4(chroma_data, blk_base, chroma_stride, residual_blk);
    }
    Ok(())
}

pub fn render_luma_inter_prediction_b(
    slice: &Slice,
    mb: &BMb,
    mb_loc: Point,
    frame: &mut VideoFrame,
    residuals: &[Block4x4],
    ref_pics_l0: &[&DpbPicture],
    ref_pics_l1: &[&DpbPicture],
    buffer: &mut InterpolationBuffer,
) -> Result<(), DecodingError> {
    let y_plane = &mut frame.planes[0];
    let wp_mode = get_weighted_pred_mode(slice);

    let y_stride = y_plane.cfg.stride;
    let mb_origin = (mb_loc.y as usize) * y_stride + (mb_loc.x as usize);
    let y_data = y_plane.data_origin_mut();
    assert!(mb_origin + 15 * y_stride + 16 <= y_data.len());

    for raster_idx in 0..16 {
        let (grid_x, grid_y) = (raster_idx % 4, raster_idx / 4);
        let partition = mb.motion.partitions[grid_y as usize][grid_x as usize];
        let pred_mode = partition.pred_mode;

        let blk_x = grid_x * 4;
        let blk_y = grid_y * 4;

        let mut dst = [0u8; 16];

        let has_l0 =
            pred_mode == MbPredictionMode::Pred_L0 || pred_mode == MbPredictionMode::BiPred;
        let has_l1 =
            pred_mode == MbPredictionMode::Pred_L1 || pred_mode == MbPredictionMode::BiPred;

        let mut pred_l0 = [0u8; 16];
        let mut pred_l1 = [0u8; 16];

        if has_l0 {
            let ref_pic = ref_pics_l0.get(partition.ref_idx_l0 as usize).ok_or_else(|| {
                DecodingError::ReferenceNotFound(format!(
                    "ref_idx_l0 {} out of bounds (list length {})",
                    partition.ref_idx_l0,
                    ref_pics_l0.len()
                ))
            })?;
            let ref_plane = &ref_pic.picture.frame.planes[0];
            interpolate_luma(
                ref_plane,
                mb_loc.x,
                mb_loc.y,
                blk_x as u8,
                blk_y as u8,
                4,
                4,
                partition.mv_l0,
                &mut pred_l0,
                4,
                buffer,
            );
        }

        if has_l1 {
            let ref_pic = ref_pics_l1.get(partition.ref_idx_l1 as usize).ok_or_else(|| {
                DecodingError::ReferenceNotFound(format!(
                    "ref_idx_l1 {} out of bounds (list length {})",
                    partition.ref_idx_l1,
                    ref_pics_l1.len()
                ))
            })?;
            let ref_plane = &ref_pic.picture.frame.planes[0];
            interpolate_luma(
                ref_plane,
                mb_loc.x,
                mb_loc.y,
                blk_x as u8,
                blk_y as u8,
                4,
                4,
                partition.mv_l1,
                &mut pred_l1,
                4,
                buffer,
            );
        }

        // Section 8.4.2.3: Combine predictions according to weighted prediction mode
        match wp_mode {
            WeightedPredMode::Explicit => {
                let wp = get_explicit_luma_weights(
                    slice,
                    partition.ref_idx_l0 as usize,
                    partition.ref_idx_l1 as usize,
                );
                if has_l0 && has_l1 {
                    for i in 0..16 {
                        dst[i] = weighted_bi_pred(pred_l0[i], pred_l1[i], &wp);
                    }
                } else if has_l0 {
                    for i in 0..16 {
                        dst[i] = weighted_uni_pred(pred_l0[i], wp.w0, wp.o0, wp.log_wd);
                    }
                } else if has_l1 {
                    for i in 0..16 {
                        dst[i] = weighted_uni_pred(pred_l1[i], wp.w1, wp.o1, wp.log_wd);
                    }
                }
            }
            WeightedPredMode::Implicit => {
                if has_l0 && has_l1 {
                    let ref_l0 =
                        ref_pics_l0.get(partition.ref_idx_l0 as usize).ok_or_else(|| {
                            DecodingError::ReferenceNotFound(format!(
                                "ref_idx_l0 {} out of bounds (list length {})",
                                partition.ref_idx_l0,
                                ref_pics_l0.len()
                            ))
                        })?;
                    let ref_l1 =
                        ref_pics_l1.get(partition.ref_idx_l1 as usize).ok_or_else(|| {
                            DecodingError::ReferenceNotFound(format!(
                                "ref_idx_l1 {} out of bounds (list length {})",
                                partition.ref_idx_l1,
                                ref_pics_l1.len()
                            ))
                        })?;
                    let wp = get_implicit_weights(ref_l0, ref_l1, slice.current_pic_poc);
                    for i in 0..16 {
                        dst[i] = weighted_bi_pred(pred_l0[i], pred_l1[i], &wp);
                    }
                } else if has_l0 {
                    // Section 8.4.2.3: implicit mode with only one list falls back to default
                    dst = pred_l0;
                } else if has_l1 {
                    dst = pred_l1;
                }
            }
            WeightedPredMode::Default => {
                if has_l0 && has_l1 {
                    for i in 0..16 {
                        dst[i] = ((pred_l0[i] as u16 + pred_l1[i] as u16 + 1) >> 1) as u8;
                    }
                } else if has_l0 {
                    dst = pred_l0;
                } else if has_l1 {
                    dst = pred_l1;
                }
            }
        }

        // Add residual
        let blk_idx =
            macroblock::get_4x4luma_block_index(Point { x: blk_x as u32, y: blk_y as u32 });
        if let Some(residual_blk) = residuals.get(blk_idx as usize) {
            add_residual_4x4(&mut dst, 0, 4, residual_blk);
        }

        // Copy to frame
        let cell_base = mb_origin + (blk_y as usize) * y_stride + (blk_x as usize);
        for y in 0..4 {
            let row_base = cell_base + y * y_stride;
            y_data[row_base..row_base + 4].copy_from_slice(&dst[y * 4..y * 4 + 4]);
        }
    }
    Ok(())
}

pub fn render_chroma_inter_prediction_b(
    slice: &Slice,
    mb: &BMb,
    mb_loc: Point,
    plane: ColorPlane,
    frame: &mut VideoFrame,
    residuals: &[Block4x4],
    ref_pics_l0: &[&DpbPicture],
    ref_pics_l1: &[&DpbPicture],
) -> Result<(), DecodingError> {
    let chroma_plane = &mut frame.planes[plane as usize];
    let mb_x_chroma = mb_loc.x >> 1;
    let mb_y_chroma = mb_loc.y >> 1;
    let wp_mode = get_weighted_pred_mode(slice);
    let chroma_idx = plane as usize - 1; // Cb=0, Cr=1

    // 1. Prediction. Coalesce the 4x4 motion partition grid into maximal
    // rectangles per direction and call interpolate_chroma once per partition.
    // Predictions are staged into 8x8 row-major buffers (stride 8) so the
    // per-cell weighted-prediction loop below can read each 2x2 patch from a
    // fixed offset.
    let mut pred_l0_buf = [0u8; 64];
    let mut pred_l1_buf = [0u8; 64];
    let mut rects = [PartitionRect::default(); 16];

    let n_l0 = collect_pred_rects(&mb.motion.partitions, classify_b_l0, &mut rects);
    for rect in &rects[..n_l0] {
        let ref_pic = ref_pics_l0.get(rect.ref_idx as usize).ok_or_else(|| {
            DecodingError::ReferenceNotFound(format!(
                "ref_idx_l0 {} out of bounds (list length {})",
                rect.ref_idx,
                ref_pics_l0.len()
            ))
        })?;
        let ref_plane = &ref_pic.picture.frame.planes[plane as usize];
        let cx = rect.grid_x as usize * 2;
        let cy = rect.grid_y as usize * 2;
        let cw = rect.grid_w as usize * 2;
        let ch = rect.grid_h as usize * 2;
        interpolate_chroma(
            ref_plane,
            mb_x_chroma,
            mb_y_chroma,
            cx as u8,
            cy as u8,
            cw as u8,
            ch as u8,
            rect.mv,
            &mut pred_l0_buf[cy * 8 + cx..],
            8,
        );
    }

    let n_l1 = collect_pred_rects(&mb.motion.partitions, classify_b_l1, &mut rects);
    for rect in &rects[..n_l1] {
        let ref_pic = ref_pics_l1.get(rect.ref_idx as usize).ok_or_else(|| {
            DecodingError::ReferenceNotFound(format!(
                "ref_idx_l1 {} out of bounds (list length {})",
                rect.ref_idx,
                ref_pics_l1.len()
            ))
        })?;
        let ref_plane = &ref_pic.picture.frame.planes[plane as usize];
        let cx = rect.grid_x as usize * 2;
        let cy = rect.grid_y as usize * 2;
        let cw = rect.grid_w as usize * 2;
        let ch = rect.grid_h as usize * 2;
        interpolate_chroma(
            ref_plane,
            mb_x_chroma,
            mb_y_chroma,
            cx as u8,
            cy as u8,
            cw as u8,
            ch as u8,
            rect.mv,
            &mut pred_l1_buf[cy * 8 + cx..],
            8,
        );
    }

    let chroma_stride = chroma_plane.cfg.stride;
    let mb_origin = (mb_y_chroma as usize) * chroma_stride + (mb_x_chroma as usize);
    let chroma_data = chroma_plane.data_origin_mut();
    // Both passes write within the 8x8 chroma MB at mb_origin, so a single
    // worst-case assert dominates every per-pixel store below.
    assert!(mb_origin + 7 * chroma_stride + 8 <= chroma_data.len());

    for blk_idx in 0..16 {
        let (grid_x, grid_y) = (blk_idx % 4, blk_idx / 4);
        let partition = mb.motion.partitions[grid_y as usize][grid_x as usize];
        let pred_mode = partition.pred_mode;

        let has_l0 =
            pred_mode == MbPredictionMode::Pred_L0 || pred_mode == MbPredictionMode::BiPred;
        let has_l1 =
            pred_mode == MbPredictionMode::Pred_L1 || pred_mode == MbPredictionMode::BiPred;

        let blk_x = (grid_x * 4) >> 1;
        let blk_y = (grid_y * 4) >> 1;
        let buf_off = blk_y * 8 + blk_x;

        let mut pred_l0 = [0u8; 4];
        let mut pred_l1 = [0u8; 4];
        if has_l0 {
            pred_l0 = [
                pred_l0_buf[buf_off],
                pred_l0_buf[buf_off + 1],
                pred_l0_buf[buf_off + 8],
                pred_l0_buf[buf_off + 9],
            ];
        }
        if has_l1 {
            pred_l1 = [
                pred_l1_buf[buf_off],
                pred_l1_buf[buf_off + 1],
                pred_l1_buf[buf_off + 8],
                pred_l1_buf[buf_off + 9],
            ];
        }

        // Section 8.4.2.3: Combine predictions according to weighted prediction mode
        let mut dst = [0u8; 4];
        match wp_mode {
            WeightedPredMode::Explicit => {
                let wp = get_explicit_chroma_weights(
                    slice,
                    partition.ref_idx_l0 as usize,
                    partition.ref_idx_l1 as usize,
                    chroma_idx,
                );
                if has_l0 && has_l1 {
                    for i in 0..4 {
                        dst[i] = weighted_bi_pred(pred_l0[i], pred_l1[i], &wp);
                    }
                } else if has_l0 {
                    for i in 0..4 {
                        dst[i] = weighted_uni_pred(pred_l0[i], wp.w0, wp.o0, wp.log_wd);
                    }
                } else if has_l1 {
                    for i in 0..4 {
                        dst[i] = weighted_uni_pred(pred_l1[i], wp.w1, wp.o1, wp.log_wd);
                    }
                }
            }
            WeightedPredMode::Implicit => {
                if has_l0 && has_l1 {
                    // Implicit mode uses same weights for luma and chroma
                    let ref_l0 =
                        ref_pics_l0.get(partition.ref_idx_l0 as usize).ok_or_else(|| {
                            DecodingError::ReferenceNotFound(format!(
                                "ref_idx_l0 {} out of bounds (list length {})",
                                partition.ref_idx_l0,
                                ref_pics_l0.len()
                            ))
                        })?;
                    let ref_l1 =
                        ref_pics_l1.get(partition.ref_idx_l1 as usize).ok_or_else(|| {
                            DecodingError::ReferenceNotFound(format!(
                                "ref_idx_l1 {} out of bounds (list length {})",
                                partition.ref_idx_l1,
                                ref_pics_l1.len()
                            ))
                        })?;
                    let wp = get_implicit_weights(ref_l0, ref_l1, slice.current_pic_poc);
                    for i in 0..4 {
                        dst[i] = weighted_bi_pred(pred_l0[i], pred_l1[i], &wp);
                    }
                } else if has_l0 {
                    dst = pred_l0;
                } else if has_l1 {
                    dst = pred_l1;
                }
            }
            WeightedPredMode::Default => {
                if has_l0 && has_l1 {
                    for i in 0..4 {
                        dst[i] = ((pred_l0[i] as u16 + pred_l1[i] as u16 + 1) >> 1) as u8;
                    }
                } else if has_l0 {
                    dst = pred_l0;
                } else if has_l1 {
                    dst = pred_l1;
                }
            }
        }

        // Write to frame: 2x2 block at chroma sample (mb_x_chroma + blk_x,
        // mb_y_chroma + blk_y) addressed directly via the hoisted base.
        let cell_base = mb_origin + blk_y * chroma_stride + blk_x;
        chroma_data[cell_base] = dst[0];
        chroma_data[cell_base + 1] = dst[1];
        chroma_data[cell_base + chroma_stride] = dst[2];
        chroma_data[cell_base + chroma_stride + 1] = dst[3];
    }

    // 2. Residuals
    for (blk_idx, residual_blk) in residuals.iter().enumerate() {
        let blk_loc = get_4x4chroma_block_location(blk_idx as u8);
        let blk_base =
            mb_origin + (blk_loc.y as usize) * chroma_stride + (blk_loc.x as usize);
        add_residual_4x4(chroma_data, blk_base, chroma_stride, residual_blk);
    }
    Ok(())
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

    fn create_test_plane(width: usize, height: usize, fill: u8) -> Plane<u8> {
        let mut p = Plane::new(width, height, 0, 0, 16, 16);
        p.data.fill(fill);
        p
    }

    #[test]
    fn test_interpolate_integer() {
        let plane = create_test_plane(32, 32, 100);
        let mut dst = [0u8; 16];
        let mut buffer = InterpolationBuffer::new();
        interpolate_luma(
            &plane,
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
        // Create a plane with alternating 100 and 200
        let mut plane = Plane::new(32, 32, 0, 0, 16, 16);
        plane.data.fill(0); // Clear padding too
        for y in 0..32 + 32 {
            // Fill enough to cover padding and visible
            let stride = plane.cfg.stride;
            if y * stride >= plane.data.len() {
                break;
            }
            let row_start = y * stride;
            for x in 0..stride {
                if row_start + x < plane.data.len() {
                    plane.data[row_start + x] = if x % 2 == 0 { 100 } else { 200 };
                }
            }
        }
        let mut dst = [0u8; 16];
        let mut buffer = InterpolationBuffer::new();
        interpolate_luma(
            &plane,
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
        // Create a plane with alternating 100 and 200
        let mut plane = Plane::new(32, 32, 0, 0, 16, 16);
        plane.data.fill(0);
        for y in 0..32 + 32 {
            let stride = plane.cfg.stride;
            if y * stride >= plane.data.len() {
                break;
            }
            for x in 0..stride {
                plane.data[y * stride + x] = if x % 2 == 0 { 100 } else { 200 };
            }
        }
        let mut dst = [0u8; 16];
        let mut buffer = InterpolationBuffer::new();
        interpolate_luma(
            &plane,
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
        let plane = create_test_plane(16, 16, 50);
        let mut dst = [0u8; 4];
        interpolate_chroma(&plane, 0, 0, 0, 0, 2, 2, MotionVector { x: 0, y: 0 }, &mut dst, 2);
        assert_eq!(dst, [50; 4]);
    }

    #[test]
    fn test_interpolate_chroma_half() {
        // Create a plane with alternating 100 and 200
        let mut plane = Plane::new(16, 16, 0, 0, 16, 16);
        plane.data.fill(0);
        for y in 0..16 + 32 {
            let stride = plane.cfg.stride;
            if y * stride >= plane.data.len() {
                break;
            }
            for x in 0..stride {
                plane.data[y * stride + x] = if x % 2 == 0 { 100 } else { 200 };
            }
        }
        let mut dst = [0u8; 4];
        interpolate_chroma(&plane, 0, 0, 0, 0, 2, 2, MotionVector { x: 4, y: 0 }, &mut dst, 2);
        assert_eq!(dst[0], 150);
        assert_eq!(dst[1], 150); // Next pixel is B=200, C=100 -> avg 150
    }

    #[test]
    fn test_interpolate_chroma_eighth() {
        let mut plane = Plane::new(16, 16, 0, 0, 16, 16);
        plane.data.fill(100);
        {
            let mut slice = plane.mut_slice(v_frame::plane::PlaneOffset { x: 0, y: 0 });
            let row0 = &mut slice.rows_iter_mut().next().unwrap();
            row0[1] = 164;
        }
        assert_eq!(plane.p(1, 0), 164, "Setup failed");
        let mut dst = [0u8; 1];
        interpolate_chroma(&plane, 0, 0, 0, 0, 1, 1, MotionVector { x: 1, y: 0 }, &mut dst, 1);
        assert_eq!(dst[0], 108);
    }
}
