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
    // Equation 8-223, 8-224: Full sample units (Int) and Equation 8-225, 8-226: Fractional units (Frac)
    let x_int = (mb_x as i32) + (blk_x as i32) + (mv.x >> 2) as i32;
    let y_int = (mb_y as i32) + (blk_y as i32) + (mv.y >> 2) as i32;
    let x_frac = (mv.x & 3) as i8;
    let y_frac = (mv.y & 3) as i8;

    // To implement the 6-tap filter for a block of size (width x height),
    // we need a window of (width + 5) x (height + 5) integer pixels.
    // Specifically, for 6-tap filter at pos G (integer), we need E, F, G, H, I, J (1D view).
    // That is 2 pixels to the left/top and 3 pixels to the right/bottom relative to the top-left 'G'.
    let buf_w = (width as usize) + 5;
    let buf_h = (height as usize) + 5;

    let plane_height = ref_plane.cfg.height as i32;
    let plane_width = ref_plane.cfg.width as i32;

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
            for (d, s) in buffer.data[y][..buf_w].iter_mut().zip(src) {
                *d = *s as i16;
            }
        }
    } else {
        // Slow path: clamping for boundary pixels (Section 8.4.2.2.1, RefLayerFrame behavior)
        for y in 0..buf_h {
            let cy = (y_int + (y as i32) - 2).clamp(0, plane_height - 1);
            let row = ref_plane.row(cy as isize);
            for x in 0..buf_w {
                let cx = (x_int + (x as i32) - 2).clamp(0, plane_width - 1);
                buffer.data[y][x] = row[cx as usize] as i16;
            }
        }
    }

    match (x_frac, y_frac) {
        (0, 0) => {
            // Integer positions (G)
            for y in 0..height as usize {
                let d = &mut dst[y * dst_stride..y * dst_stride + width as usize];
                for x in 0..width as usize {
                    // Offset +2 to skip the padding (E, F)
                    d[x] = buffer.data[y + 2][x + 2] as u8;
                }
            }
        }
        _ => {
            macro_rules! interpolate {
                (|$x:ident, $y:ident| $calc:expr) => {
                    for y in 0..height as usize {
                        let $y = y;
                        let d = &mut dst[y * dst_stride..y * dst_stride + width as usize];
                        for x in 0..width as usize {
                            let $x = x;
                            d[x] = $calc;
                        }
                    }
                };
            }

            // Accessors for integer and half-pel positions
            let data: &[[i16; 21]; 21] = &buffer.data;
            // G, H, M are integer samples at different offsets
            macro_rules! G { ($x:expr, $y:expr) => { data[$y + 2][$x + 2] as u16 } }
            macro_rules! H { ($x:expr, $y:expr) => { data[$y + 2][$x + 3] as u16 } }
            macro_rules! M { ($x:expr, $y:expr) => { data[$y + 3][$x + 2] as u16 } }
            
            // Half-sample interpolation using 6-tap filter (Equation 8-241, 8-243)
            // b = (E - 5F + 20G + 20H - 5I + J + 16) >> 5
            macro_rules! b { ($x:expr, $y:expr) => { filter_6tap_and_clip(&data[$y + 2][$x..$x + 6]) as u16 } }
            // s is 'b' but for the next row (vertical shift) - wait, no, s is vertical filtering of M-line?
            // Actually: 
            // 'b' is horizontal interpolation at y (samples E..J at y)
            // 'h' is vertical interpolation at x (samples A..U at x)
            // 's' is horizontal interpolation at y+1 (samples E..J at y+1) -> used for 'p', 'r'
            // 'm' is vertical interpolation at x+1 (samples A..U at x+1) -> used for 'g', 'k'
            macro_rules! s { ($x:expr, $y:expr) => { filter_6tap_and_clip(&data[$y + 3][$x..$x + 6]) as u16 } }
            
            // h = (A - 5C + 20G + 20M - 5R + T + 16) >> 5 (Vertical filtering)
            macro_rules! h { ($x:expr, $y:expr) => { filter_6tap_vertical_and_clip(buffer, $x, $y) as u16 } }
            macro_rules! m { ($x:expr, $y:expr) => { filter_6tap_vertical_and_clip(buffer, $x + 1, $y) as u16 } }
            
            // Averaging for quarter-sample positions (Equation 8-250 to 8-252)
            macro_rules! avg { ($val1:expr, $val2:expr) => { (($val1 + $val2 + 1) >> 1) as u8 } }

            match (x_frac, y_frac) {
                // Half-pel positions (except j)
                (2, 0) => interpolate!(|x, y| b!(x, y) as u8), // b
                (0, 2) => interpolate!(|x, y| h!(x, y) as u8), // h
                
                // Quarter-pel positions (Table 8-12)
                (1, 0) => interpolate!(|x, y| avg!(G!(x, y), b!(x, y))), // a
                (3, 0) => interpolate!(|x, y| avg!(H!(x, y), b!(x, y))), // c
                (0, 1) => interpolate!(|x, y| avg!(G!(x, y), h!(x, y))), // d
                (0, 3) => interpolate!(|x, y| avg!(M!(x, y), h!(x, y))), // n
                (1, 1) => interpolate!(|x, y| avg!(b!(x, y), h!(x, y))), // e
                (3, 1) => interpolate!(|x, y| avg!(b!(x, y), m!(x, y))), // g
                (1, 3) => interpolate!(|x, y| avg!(h!(x, y), s!(x, y))), // p
                (3, 3) => interpolate!(|x, y| avg!(m!(x, y), s!(x, y))), // r

                // Cases needing j (center half-sample)
                // j is generated by applying the 6-tap filter vertically to the result 
                // of applying the 6-tap filter horizontally (or vice versa). (Equation 8-247)
                (2, 2) | (2, 1) | (2, 3) | (1, 2) | (3, 2) => {
                    let mut intermediate = [0i32; 21 * 21];
                    for y in 0..buf_h {
                        let row = &buffer.data[y];
                        for x in 0..width as usize {
                            // Compute horizontal filter first (unclipped)
                            intermediate[y * 21 + x] = filter_6tap(&row[x..x + 6]);
                        }
                    }

                    // Compute j by filtering 'intermediate' vertically.
                    // Also clips 'b' and 's' from 'intermediate' for efficiency.
                    macro_rules! j { ($x:expr, $y:expr) => { get_j_from_intermediate(&intermediate, $x, $y) as u16 } }
                    macro_rules! b_opt { ($x:expr, $y:expr) => { clip_intermediate(intermediate[($y + 2) * 21 + $x]) as u16 } }
                    macro_rules! s_opt { ($x:expr, $y:expr) => { clip_intermediate(intermediate[($y + 3) * 21 + $x]) as u16 } }

                    match (x_frac, y_frac) {
                        (2, 2) => interpolate!(|x, y| j!(x, y) as u8),           // j
                        (2, 1) => interpolate!(|x, y| avg!(b_opt!(x, y), j!(x, y))), // f
                        (2, 3) => interpolate!(|x, y| avg!(j!(x, y), s_opt!(x, y))), // q
                        (1, 2) => interpolate!(|x, y| avg!(h!(x, y), j!(x, y))),     // i
                        (3, 2) => interpolate!(|x, y| avg!(j!(x, y), m!(x, y))),     // k
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!("x_frac={}, y_frac={}", x_frac, y_frac),
            }
        }
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
        && x_int + width as i32 + 1 <= plane_width
        && y_int >= 0
        && y_int + height as i32 + 1 <= plane_height
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

/// Applies the 6-tap filter specified in Equation 8-246.
/// Filter coefficients: [1, -5, 20, 20, -5, 1].
/// This returns the unscaled/unclipped intermediate value.
#[inline(always)]
fn filter_6tap(p: &[i16]) -> i32 {
    (p[0] as i32) - 5 * (p[1] as i32) + 20 * (p[2] as i32) + 20 * (p[3] as i32) - 5 * (p[4] as i32)
        + (p[5] as i32)
}

/// Applies the 6-tap filter and clips the result to 8-bit range [0, 255].
/// Corresponds to Equations 8-243, 8-244, 8-248, 8-249 (final clipping for half-sample values).
#[inline(always)]
fn filter_6tap_and_clip(p: &[i16]) -> u8 {
    let val = filter_6tap(p);
    ((val + 16) >> 5).clamp(0, 255) as u8
}

/// Helper to apply vertical 6-tap filter on the interpolation buffer.
#[inline(always)]
fn filter_6tap_vertical_and_clip(buffer: &InterpolationBuffer, x: usize, y: usize) -> u8 {
    let col = x + 2;
    let val = (buffer.data[y][col] as i32)
        - 5 * (buffer.data[y + 1][col] as i32)
        + 20 * (buffer.data[y + 2][col] as i32)
        + 20 * (buffer.data[y + 3][col] as i32)
        - 5 * (buffer.data[y + 4][col] as i32)
        + (buffer.data[y + 5][col] as i32);
    ((val + 16) >> 5).clamp(0, 255) as u8
}

/// Computes the 'j' sample (center half-pel) using the intermediate horizontal filter results.
/// This corresponds to Equation 8-247, but operating on the intermediate values
/// (which are essentially the result of the first 6-tap filter pass).
/// Note the shift is >> 10 because it accumulates two >> 5 equivalent scalings (minus one >> 5).
/// Actually, intermediate is unscaled (sum of 6 taps).
/// Vertical filter is also sum of 6 taps.
/// Total weight is 32*32 = 1024. So >> 10 is correct.
/// Rounding offset is 512 (which is 1024/2).
#[inline(always)]
fn get_j_from_intermediate(intermediate: &[i32], x: usize, y: usize) -> u8 {
    // 21 is the width of intermediate buffer (InterploationBuffer::data width)
    let stride = 21;
    let val = intermediate[y * stride + x]
        - 5 * intermediate[(y + 1) * stride + x]
        + 20 * intermediate[(y + 2) * stride + x]
        + 20 * intermediate[(y + 3) * stride + x]
        - 5 * intermediate[(y + 4) * stride + x]
        + intermediate[(y + 5) * stride + x];
    ((val + 512) >> 10).clamp(0, 255) as u8
}

/// Clips an intermediate 6-tap filter result to 8-bit.
/// Used when we have precomputed horizontal filter results and need 'b' or 's' (horizontal half-pels).
#[inline(always)]
fn clip_intermediate(val: i32) -> u8 {
    ((val + 16) >> 5).clamp(0, 255) as u8
}

/// Buffer for storing integer pixels with padding for 6-tap filtering.
/// The size is 21x21 to accommodate a 16x16 block with 2 pixels padding on top/left
/// and 3 pixels padding on bottom/right (required for 6-tap filter).
pub struct InterpolationBuffer {
    data: [[i16; 21]; 21],
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
        interpolate_luma(&plane, 0, 0, 0, 0, 4, 4, MotionVector { x: 0, y: 0 }, &mut dst, 4, &mut buffer);
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
        let mut dst = [0u8; 4];
        let mut buffer = InterpolationBuffer::new();
        interpolate_luma(&plane, 2, 2, 0, 0, 4, 1, MotionVector { x: 2, y: 0 }, &mut dst, 4, &mut buffer);
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
        let mut dst = [0u8; 4];
        let mut buffer = InterpolationBuffer::new();
        interpolate_luma(&plane, 2, 2, 0, 0, 4, 1, MotionVector { x: 1, y: 0 }, &mut dst, 4, &mut buffer);
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
