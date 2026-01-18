use super::decoder::VideoFrame;
use super::macroblock::MotionVector;
use super::Point;
use v_frame::plane::Plane;

/// Section 8.4.2.2.1 Luma sample interpolation process.
/// This function interpolates a block of luma samples with quarter-sample accuracy.
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
) {
    let x_int = (mb_x as i32) + (blk_x as i32) + (mv.x >> 2) as i32;
    let y_int = (mb_y as i32) + (blk_y as i32) + (mv.y >> 2) as i32;
    let x_frac = (mv.x & 3) as i8;
    let y_frac = (mv.y & 3) as i8;

    // To implement the 6-tap filter for a block of size (width x height),
    // we need a window of (width + 5) x (height + 5) integer pixels.
    // Specifically, for 6-tap filter at pos G, we need E, F, G, H, I, J.
    // That is 2 pixels to the left/top and 3 pixels to the right/bottom.
    let mut tmp = [[0i16; 21]; 21]; // Max block size 16x16 + 5 padding
    let buf_w = (width as usize) + 5;
    let buf_h = (height as usize) + 5;

    for y in 0..buf_h {
        for x in 0..buf_w {
            tmp[y][x] =
                get_clamped_pixel(ref_plane, x_int + (x as i32) - 2, y_int + (y as i32) - 2) as i16;
        }
    }

    match (x_frac, y_frac) {
        (0, 0) => {
            // Integer positions
            for y in 0..height as usize {
                for x in 0..width as usize {
                    dst[y * dst_stride + x] = tmp[y + 2][x + 2] as u8;
                }
            }
        }
        (2, 0) => {
            // Half-pel horizontal (b)
            for y in 0..height as usize {
                for x in 0..width as usize {
                    dst[y * dst_stride + x] = filter_6tap_and_clip(&tmp[y + 2][x..x + 6]);
                }
            }
        }
        (0, 2) => {
            // Half-pel vertical (h)
            for y in 0..height as usize {
                for x in 0..width as usize {
                    let col = [
                        tmp[y][x + 2],
                        tmp[y + 1][x + 2],
                        tmp[y + 2][x + 2],
                        tmp[y + 3][x + 2],
                        tmp[y + 4][x + 2],
                        tmp[y + 5][x + 2],
                    ];
                    dst[y * dst_stride + x] = filter_6tap_and_clip(&col);
                }
            }
        }
        (2, 2) => {
            // Half-pel center (j)
            // First calculate intermediate horizontal 6-tap results for 6 vertical positions
            for y in 0..height as usize {
                for x in 0..width as usize {
                    let mut row_results = [0i32; 6];
                    for i in 0..6 {
                        row_results[i] = filter_6tap(&tmp[y + i][x..x + 6]);
                    }
                    dst[y * dst_stride + x] = filter_6tap_center_and_clip(&row_results);
                }
            }
        }
        _ => {
            // Quarter-pel positions
            // These are linear averages of integer and half-pel positions.
            // We'll calculate the needed components and average them.
            for y in 0..height as usize {
                for x in 0..width as usize {
                    dst[y * dst_stride + x] = interpolate_quarter_pel(&tmp, x, y, x_frac, y_frac);
                }
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

    // Integer part of the position (in chroma samples)
    let x_int = (mb_x as i32) + (blk_x as i32) + (mv.x >> 3) as i32;
    let y_int = (mb_y as i32) + (blk_y as i32) + (mv.y >> 3) as i32;

    // Fractional part (0..7), representing 1/8th chroma sample intervals
    let x_frac = (mv.x & 7) as i32;
    let y_frac = (mv.y & 7) as i32;

    for y in 0..height as usize {
        for x in 0..width as usize {
            let cx = x_int + x as i32;
            let cy = y_int + y as i32;

            // 2. Fetch Neighbors
            // Section 8.4.2.2.2: A, B, C, D samples
            let val_a = get_clamped_pixel(ref_plane, cx, cy) as i32;
            let val_b = get_clamped_pixel(ref_plane, cx + 1, cy) as i32;
            let val_c = get_clamped_pixel(ref_plane, cx, cy + 1) as i32;
            let val_d = get_clamped_pixel(ref_plane, cx + 1, cy + 1) as i32;

            // 3. Bilinear Interpolation
            // Equation 8-266
            // pred = ( (8 - xFrac) * (8 - yFrac) * A + xFrac * (8 - yFrac) * B +
            //          (8 - xFrac) * yFrac * C + xFrac * yFrac * D + 32 ) >> 6
            let w00 = (8 - x_frac) * (8 - y_frac);
            let w10 = x_frac * (8 - y_frac);
            let w01 = (8 - x_frac) * y_frac;
            let w11 = x_frac * y_frac;

            let prediction = (w00 * val_a + w10 * val_b + w01 * val_c + w11 * val_d + 32) >> 6;

            dst[y * dst_stride + x] = prediction.clamp(0, 255) as u8;
        }
    }
}

/// Section 8.4.2.1: Clamping for "Unrestricted Motion Vector"
#[inline]
fn get_clamped_pixel(plane: &Plane<u8>, x: i32, y: i32) -> u8 {
    let width = plane.cfg.width as i32;
    let height = plane.cfg.height as i32;
    let cx = x.clamp(0, width - 1);
    let cy = y.clamp(0, height - 1);
    plane.p(cx as usize, cy as usize)
}

#[inline]
fn filter_6tap(p: &[i16]) -> i32 {
    (p[0] as i32) - 5 * (p[1] as i32) + 20 * (p[2] as i32) + 20 * (p[3] as i32) - 5 * (p[4] as i32)
        + (p[5] as i32)
}

#[inline]
fn filter_6tap_and_clip(p: &[i16]) -> u8 {
    let val = filter_6tap(p);
    ((val + 16) >> 5).clamp(0, 255) as u8
}

#[inline]
fn filter_6tap_center_and_clip(p: &[i32]) -> u8 {
    let val = p[0] - 5 * p[1] + 20 * p[2] + 20 * p[3] - 5 * p[4] + p[5];
    ((val + 512) >> 10).clamp(0, 255) as u8
}

fn interpolate_quarter_pel(
    tmp: &[[i16; 21]; 21],
    x: usize,
    y: usize,
    x_frac: i8,
    y_frac: i8,
) -> u8 {
    // Refer to Figure 8-4 for positions a-s
    // x_frac, y_frac: 0=0, 1=1/4, 2=1/2, 3=3/4

    let get_h = |tx: usize, ty: usize| {
        let col = [
            tmp[ty][tx + 2],
            tmp[ty + 1][tx + 2],
            tmp[ty + 2][tx + 2],
            tmp[ty + 3][tx + 2],
            tmp[ty + 4][tx + 2],
            tmp[ty + 5][tx + 2],
        ];
        filter_6tap_and_clip(&col)
    };

    let get_b = |tx: usize, ty: usize| filter_6tap_and_clip(&tmp[ty + 2][tx..tx + 6]);

    let get_j = |tx: usize, ty: usize| {
        let mut row_results = [0i32; 6];
        for i in 0..6 {
            row_results[i] = filter_6tap(&tmp[ty + i][tx..tx + 6]);
        }
        filter_6tap_center_and_clip(&row_results)
    };

    let g = tmp[y + 2][x + 2] as u8;

    match (x_frac, y_frac) {
        (1, 0) => {
            // a = (G + b + 1) >> 1
            let b = get_b(x, y);
            ((g as u16 + b as u16 + 1) >> 1) as u8
        }
        (3, 0) => {
            // c = (H + b + 1) >> 1
            let b = get_b(x + 1, y);
            let h_int = tmp[y + 2][x + 3] as u8;
            ((h_int as u16 + b as u16 + 1) >> 1) as u8
        }
        (0, 1) => {
            // d = (G + h + 1) >> 1
            let h = get_h(x, y);
            ((g as u16 + h as u16 + 1) >> 1) as u8
        }
        (0, 3) => {
            // n = (M + h + 1) >> 1
            let h = get_h(x, y + 1);
            let m_int = tmp[y + 3][x + 2] as u8;
            ((m_int as u16 + h as u16 + 1) >> 1) as u8
        }
        (2, 1) => {
            // f = (b + j + 1) >> 1
            let b = get_b(x, y);
            let j = get_j(x, y);
            ((b as u16 + j as u16 + 1) >> 1) as u8
        }
        (2, 3) => {
            // q = (b + j + 1) >> 1 (using j at next vertical half-pel)
            let b = get_b(x, y + 1);
            let j = get_j(x, y);
            ((b as u16 + j as u16 + 1) >> 1) as u8
        }
        (1, 2) => {
            // i = (h + j + 1) >> 1
            let h = get_h(x, y);
            let j = get_j(x, y);
            ((h as u16 + j as u16 + 1) >> 1) as u8
        }
        (3, 2) => {
            // k = (h + j + 1) >> 1 (using j at next horizontal half-pel)
            let h = get_h(x + 1, y);
            let j = get_j(x, y);
            ((h as u16 + j as u16 + 1) >> 1) as u8
        }
        (1, 1) => {
            // e = (a + d + 1) >> 1 (Wait, spec says e is average of b and h?)
            // Section 8.4.2.2.1:
            // e = ( b + h + 1 ) >> 1
            let b = get_b(x, y);
            let h = get_h(x, y);
            ((b as u16 + h as u16 + 1) >> 1) as u8
        }
        (3, 1) => {
            // g = ( b + h + 1 ) >> 1 (b from (2,0), h from (0,0) offset)
            let b = get_b(x, y);
            let h = get_h(x + 1, y);
            ((b as u16 + h as u16 + 1) >> 1) as u8
        }
        (1, 3) => {
            // p = ( b + h + 1 ) >> 1
            let b = get_b(x, y + 1);
            let h = get_h(x, y);
            ((b as u16 + h as u16 + 1) >> 1) as u8
        }
        (3, 3) => {
            // r = ( b + h + 1 ) >> 1
            let b = get_b(x, y + 1);
            let h = get_h(x + 1, y);
            ((b as u16 + h as u16 + 1) >> 1) as u8
        }
        _ => unreachable!("x_frac={}, y_frac={}", x_frac, y_frac),
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
        interpolate_luma(&plane, 0, 0, 0, 0, 4, 4, MotionVector { x: 0, y: 0 }, &mut dst, 4);
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
        // At x=2, G=100, H=200, I=100, J=200...
        // x_int = 2, x_frac = 2 (half-pel)
        // b should be roughly average of 100 and 200
        interpolate_luma(&plane, 2, 2, 0, 0, 4, 1, MotionVector { x: 2, y: 0 }, &mut dst, 4);
        // For a constant gradient or alternating pattern, 6-tap might not be exactly 150
        // but it should be close.
        // E=100, F=200, G=100, H=200, I=100, J=200
        // val = 100 - 5*200 + 20*100 + 20*200 - 5*100 + 200
        // val = 100 - 1000 + 2000 + 4000 - 500 + 200 = 4800
        // (4800 + 16) >> 5 = 4816 >> 5 = 150
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
        // x_int = 2, x_frac = 1 (quarter-pel a)
        // G = 100, b = 150 (from previous test)
        // a = (100 + 150 + 1) >> 1 = 125
        interpolate_luma(&plane, 2, 2, 0, 0, 4, 1, MotionVector { x: 1, y: 0 }, &mut dst, 4);
        assert_eq!(dst[0], 125);
    }

    #[test]
    fn test_interpolate_chroma_integer() {
        let plane = create_test_plane(16, 16, 50);
        let mut dst = [0u8; 4];
        // mv = (0, 0) -> x_frac=0, y_frac=0. Result should be A (50)
        interpolate_chroma(
            &plane,
            0,
            0,
            0,
            0,
            2,
            2,
            MotionVector { x: 0, y: 0 },
            &mut dst,
            2,
        );
        assert_eq!(dst, [50; 4]);
    }

    #[test]
    fn test_interpolate_chroma_half() {
        // Create a plane with alternating 100 and 200
        let mut plane = Plane::new(16, 16, 0, 0, 16, 16);
        plane.data.fill(0);
        for y in 0..16 + 32 {
            let stride = plane.cfg.stride;
            if y * stride >= plane.data.len() { break; }
            for x in 0..stride {
                plane.data[y * stride + x] = if x % 2 == 0 { 100 } else { 200 };
            }
        }
        
        let mut dst = [0u8; 4];
        // We want half-pixel interpolation between 100 and 200.
        // In chroma logic:
        // x_int = 0
        // We need x_frac = 4 (which is 4/8 = 1/2 chroma pixel)
        // mv.x should be: (0 << 3) + 4 = 4.
        // Wait, mv is in 1/4 luma units.
        // mv.x = 4 means 1 luma pixel shift.
        // 1 luma pixel = 0.5 chroma pixel.
        // So mv.x = 4 should indeed give x_frac = 4.
        
        interpolate_chroma(
            &plane,
            0,
            0,
            0,
            0,
            2,
            2,
            MotionVector { x: 4, y: 0 },
            &mut dst,
            2,
        );
        
        // A=100, B=200. frac=4/8=0.5.
        // Res = 0.5*100 + 0.5*200 = 150.
        assert_eq!(dst[0], 150);
        assert_eq!(dst[1], 150); // Next pixel is B=200, C=100 -> avg 150
    }

    #[test]
    fn test_interpolate_chroma_eighth() {
        let mut plane = Plane::new(16, 16, 0, 0, 16, 16);
        plane.data.fill(100);

        {
            let mut slice = plane.mut_slice(v_frame::plane::PlaneOffset { x: 0, y: 0 });
            // Set (1,0) to 164 using the slice API which handles offsets correctly
            let row0 = &mut slice.rows_iter_mut().next().unwrap();
            row0[1] = 164;
        }
        
        // Verify our setup (p uses the same logic as slice usually)
        assert_eq!(plane.p(1, 0), 164, "Setup failed");
        
        let mut dst = [0u8; 1];
        
        // We want x_frac = 1 (1/8 chroma pixel).
        // mv.x = 1 (1/4 luma pixel = 1/8 chroma pixel).
        // x_int = 0.
        // Needs A=(0,0) [col xpad] -> 100
        // Needs B=(1,0) [col xpad+1] -> 164
        interpolate_chroma(
            &plane,
            0,
            0,
            0,
            0,
            1,
            1,
            MotionVector { x: 1, y: 0 },
            &mut dst,
            1,
        );
        
        // A=100, B=164. x_frac=1. y_frac=0.
        // Eq: ( (7*8*A + 1*8*B) + 32 ) >> 6
        // = ( 56*100 + 8*164 + 32 ) / 64
        // = 108
        
        assert_eq!(dst[0], 108);
    }
}
