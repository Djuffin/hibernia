use super::decoder::VideoFrame;
use super::macroblock::MotionVector;
use super::Point;
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
///
/// The process produces fractional sample values 'a' through 'r' (and integer samples 'G')
/// depending on the fractional part of the motion vector, as described in Table 8-10.
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

macro_rules! vert_6tap_clip {
    ($data:expr, $y:expr, $col:expr) => {{
        let val = $data[$y][$col] as i32 - 5 * $data[$y + 1][$col] as i32
            + 20 * $data[$y + 2][$col] as i32
            + 20 * $data[$y + 3][$col] as i32
            - 5 * $data[$y + 4][$col] as i32
            + $data[$y + 5][$col] as i32;
        ((val + 16) >> 5).clamp(0, 255) as u8
    }};
}

macro_rules! vert_6tap_j {
    ($intermediate:expr, $y:expr, $x:expr) => {{
        let val = $intermediate[$y][$x] - 5 * $intermediate[$y + 1][$x]
            + 20 * $intermediate[$y + 2][$x]
            + 20 * $intermediate[$y + 3][$x]
            - 5 * $intermediate[$y + 4][$x]
            + $intermediate[$y + 5][$x];
        ((val + 512) >> 10).clamp(0, 255) as u8
    }};
}

macro_rules! clip_i32 {
    ($val:expr) => {
        ((($val) + 16) >> 5).clamp(0, 255) as u8
    };
}

macro_rules! filter_6tap {
    ($p:expr) => {{
        let p = &$p[..6];
        (p[0] as i32) - 5 * (p[1] as i32) + 20 * (p[2] as i32) + 20 * (p[3] as i32)
            - 5 * (p[4] as i32)
            + (p[5] as i32)
    }};
}

macro_rules! horiz_6tap_clip {
    ($row:expr, $x:expr) => {{
        let val = filter_6tap!(&$row[$x..$x + 6]);
        ((val + 16) >> 5).clamp(0, 255) as u8
    }};
}

macro_rules! avg_u8 {
    ($a:expr, $b:expr $(,)?) => {
        (($a as u16 + $b as u16 + 1) >> 1) as u8
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
            for y in 0..H {
                let row = ref_plane.row((y_int + (y as i32)) as isize);
                let src = &row[x_int as usize..x_int as usize + W];
                let d = &mut dst[y * dst_stride..y * dst_stride + W];
                d.copy_from_slice(src);
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
        for y in 0..buf_h {
            let row = ref_plane.row((y_int + (y as i32) - 2) as isize);
            let src = &row[(x_int - 2) as usize..(x_int - 2 + buf_w as i32) as usize];
            buffer.data[y][..buf_w].copy_from_slice(src);
        }
    } else {
        // Slow path: clamping for boundary pixels (Section 8.4.2.2.1, RefLayerFrame behavior)
        for y in 0..buf_h {
            let cy = (y_int + (y as i32) - 2).clamp(0, plane_height - 1);
            let row = ref_plane.row(cy as isize);
            for x in 0..buf_w {
                let cx = (x_int + (x as i32) - 2).clamp(0, plane_width - 1);
                buffer.data[y][x] = row[cx as usize];
            }
        }
    }

    let data = &buffer.data;

    match (x_frac, y_frac) {
        // Half-pel positions (Equations 8-241, 8-243)
        (2, 0) => {
            // b: horizontal 6-tap
            for y in 0..H {
                let row = &data[y + 2];
                let d = &mut dst[y * dst_stride..y * dst_stride + W];
                for x in 0..W {
                    d[x] = horiz_6tap_clip!(row, x);
                }
            }
        }
        (0, 2) => {
            // h: vertical 6-tap
            for y in 0..H {
                let d = &mut dst[y * dst_stride..y * dst_stride + W];
                for x in 0..W {
                    d[x] = vert_6tap_clip!(data, y, x + 2);
                }
            }
        }

        // Quarter-pel positions (Table 8-12, Equations 8-250 to 8-252)
        (1, 0) => {
            // a = avg(G, b)
            for y in 0..H {
                let row = &data[y + 2];
                let d = &mut dst[y * dst_stride..y * dst_stride + W];
                for x in 0..W {
                    d[x] = avg_u8!(row[x + 2], horiz_6tap_clip!(row, x));
                }
            }
        }
        (3, 0) => {
            // c = avg(H, b)
            for y in 0..H {
                let row = &data[y + 2];
                let d = &mut dst[y * dst_stride..y * dst_stride + W];
                for x in 0..W {
                    d[x] = avg_u8!(row[x + 3], horiz_6tap_clip!(row, x));
                }
            }
        }
        (0, 1) => {
            // d = avg(G, h)
            for y in 0..H {
                let d = &mut dst[y * dst_stride..y * dst_stride + W];
                for x in 0..W {
                    d[x] = avg_u8!(data[y + 2][x + 2], vert_6tap_clip!(data, y, x + 2));
                }
            }
        }
        (0, 3) => {
            // n = avg(M, h)
            for y in 0..H {
                let d = &mut dst[y * dst_stride..y * dst_stride + W];
                for x in 0..W {
                    d[x] = avg_u8!(data[y + 3][x + 2], vert_6tap_clip!(data, y, x + 2));
                }
            }
        }
        (1, 1) => {
            // e = avg(b, h)
            for y in 0..H {
                let row = &data[y + 2];
                let d = &mut dst[y * dst_stride..y * dst_stride + W];
                for x in 0..W {
                    d[x] = avg_u8!(horiz_6tap_clip!(row, x), vert_6tap_clip!(data, y, x + 2),);
                }
            }
        }
        (3, 1) => {
            // g = avg(b, m)
            for y in 0..H {
                let row = &data[y + 2];
                let d = &mut dst[y * dst_stride..y * dst_stride + W];
                for x in 0..W {
                    d[x] = avg_u8!(horiz_6tap_clip!(row, x), vert_6tap_clip!(data, y, x + 3),);
                }
            }
        }
        (1, 3) => {
            // p = avg(h, s)
            for y in 0..H {
                let d = &mut dst[y * dst_stride..y * dst_stride + W];
                for x in 0..W {
                    d[x] =
                        avg_u8!(vert_6tap_clip!(data, y, x + 2), horiz_6tap_clip!(data[y + 3], x),);
                }
            }
        }
        (3, 3) => {
            // r = avg(m, s)
            for y in 0..H {
                let d = &mut dst[y * dst_stride..y * dst_stride + W];
                for x in 0..W {
                    d[x] =
                        avg_u8!(vert_6tap_clip!(data, y, x + 3), horiz_6tap_clip!(data[y + 3], x),);
                }
            }
        }

        // Cases needing j (center half-sample, Equation 8-247)
        // j requires two-pass filtering: horizontal into unclipped intermediate,
        // then vertical on the intermediate results.
        (2, 2) | (2, 1) | (2, 3) | (1, 2) | (3, 2) => {
            let mut intermediate = [[0i32; 16]; 21];
            for y in 0..buf_h {
                for x in 0..W {
                    intermediate[y][x] = filter_6tap!(&data[y][x..x + 6]);
                }
            }

            match (x_frac, y_frac) {
                (2, 2) => {
                    // j: two-pass 6-tap
                    for y in 0..H {
                        let d = &mut dst[y * dst_stride..y * dst_stride + W];
                        for x in 0..W {
                            d[x] = vert_6tap_j!(intermediate, y, x);
                        }
                    }
                }
                (2, 1) => {
                    // f = avg(b, j)
                    for y in 0..H {
                        let d = &mut dst[y * dst_stride..y * dst_stride + W];
                        for x in 0..W {
                            d[x] = avg_u8!(
                                clip_i32!(intermediate[y + 2][x]),
                                vert_6tap_j!(intermediate, y, x),
                            );
                        }
                    }
                }
                (2, 3) => {
                    // q = avg(j, s)
                    for y in 0..H {
                        let d = &mut dst[y * dst_stride..y * dst_stride + W];
                        for x in 0..W {
                            d[x] = avg_u8!(
                                vert_6tap_j!(intermediate, y, x),
                                clip_i32!(intermediate[y + 3][x]),
                            );
                        }
                    }
                }
                (1, 2) => {
                    // i = avg(h, j)
                    for y in 0..H {
                        let d = &mut dst[y * dst_stride..y * dst_stride + W];
                        for x in 0..W {
                            d[x] = avg_u8!(
                                vert_6tap_clip!(data, y, x + 2),
                                vert_6tap_j!(intermediate, y, x),
                            );
                        }
                    }
                }
                (3, 2) => {
                    // k = avg(j, m)
                    for y in 0..H {
                        let d = &mut dst[y * dst_stride..y * dst_stride + W];
                        for x in 0..W {
                            d[x] = avg_u8!(
                                vert_6tap_j!(intermediate, y, x),
                                vert_6tap_clip!(data, y, x + 3),
                            );
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
    // 1. Derive Chroma Motion Vector
    // For 4:2:0, the chroma motion vector is derived by scaling the luma motion vector.
    // Luma MV is in 1/4 pixel units.
    // 1 chroma pixel = 2 luma pixels.
    // So 1 unit of Luma MV = 1/4 luma pixel = 1/8 chroma pixel.
    // This corresponds to Equation 8-227, 8-228, 8-229, 8-230 for 4:2:0:
    // xIntC = ( xAL / SubWidthC ) + ( mvCLX[ 0 ] >> 3 ) + xC
    // yIntC = ( yAL / SubHeightC ) + ( mvCLX[ 1 ] >> 3 ) + yC
    // xFracC = mvCLX[ 0 ] & 7
    // yFracC = mvCLX[ 1 ] & 7

    // Integer part of the position (in chroma samples)
    let x_int = (mb_x as i32) + (blk_x as i32) + (mv.x >> 3) as i32;
    let y_int = (mb_y as i32) + (blk_y as i32) + (mv.y >> 3) as i32;

    // Fractional part (0..7), representing 1/8th chroma sample intervals
    let x_frac = (mv.x & 7) as i32;
    let y_frac = (mv.y & 7) as i32;

    let plane_width = ref_plane.cfg.width as i32;
    let plane_height = ref_plane.cfg.height as i32;
    if x_int >= 0
        && x_int + (width as i32) < plane_width
        && y_int >= 0
        && y_int + (height as i32) < plane_height
    {
        // Equation 8-270: Bilinear interpolation
        // The weights (8-xFrac) and xFrac are used for linear interpolation.
        let w00 = (8 - x_frac) * (8 - y_frac);
        let w10 = x_frac * (8 - y_frac);
        let w01 = (8 - x_frac) * y_frac;
        let w11 = x_frac * y_frac;

        for y in 0..height as usize {
            let row = ref_plane.row((y_int + y as i32) as isize);
            let row1 = ref_plane.row((y_int + y as i32 + 1) as isize);
            let d = &mut dst[y * dst_stride..y * dst_stride + width as usize];
            let x_start = x_int as usize;

            let mut val_a = row[x_start] as i32;
            let mut val_c = row1[x_start] as i32;

            for x in 0..width as usize {
                let val_b = row[x_start + x + 1] as i32;
                let val_d = row1[x_start + x + 1] as i32;

                // Equation 8-270:
                // predPartLXC[x, y] = ( (8-xFrac)*(8-yFrac)*A + xFrac*(8-yFrac)*B + ... + 32 ) >> 6
                let prediction = (w00 * val_a + w10 * val_b + w01 * val_c + w11 * val_d + 32) >> 6;
                d[x] = prediction as u8;

                val_a = val_b;
                val_c = val_d;
            }
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
            let mut val_a = row[cx_start as usize] as i32;
            let mut val_c = row1[cx_start as usize] as i32;

            for x in 0..width as usize {
                let cx1 = (x_int + x as i32 + 1).clamp(0, plane_width - 1);

                let val_b = row[cx1 as usize] as i32;
                let val_d = row1[cx1 as usize] as i32;

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

#[cfg(test)]
mod tests {
    use super::*;
    use v_frame::pixel::ChromaSampling;

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
            let row_end = (row_start + stride).min(plane.data.len());
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
