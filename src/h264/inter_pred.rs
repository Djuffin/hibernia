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
}
