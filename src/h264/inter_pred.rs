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
    let mut buffer = InterpolationBuffer::new(); // Max block size 16x16 + 5 padding
    let buf_w = (width as usize) + 5;
    let buf_h = (height as usize) + 5;

    for y in 0..buf_h {
        for x in 0..buf_w {
            buffer.data[y][x] =
                get_clamped_pixel(ref_plane, x_int + (x as i32) - 2, y_int + (y as i32) - 2) as i16;
        }
    }

    match (x_frac, y_frac) {
        (0, 0) => {
            // Integer positions
            for y in 0..height as usize {
                for x in 0..width as usize {
                    dst[y * dst_stride + x] = buffer.get_integer(x, y) as u8;
                }
            }
        }
        (2, 0) => {
            // Half-pel horizontal (b)
            for y in 0..height as usize {
                for x in 0..width as usize {
                    dst[y * dst_stride + x] =
                        filter_6tap_and_clip(buffer.get_horizontal_window(x, y));
                }
            }
        }
        (0, 2) => {
            // Half-pel vertical (h)
            for y in 0..height as usize {
                for x in 0..width as usize {
                    let col = buffer.get_vertical_window(x, y);
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
                        row_results[i] = filter_6tap(&buffer.data[y + i][x..x + 6]);
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
                    dst[y * dst_stride + x] =
                        interpolate_quarter_pel(&buffer, x, y, x_frac, y_frac);
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

struct InterpolationBuffer {
    data: [[i16; 21]; 21],
}

impl InterpolationBuffer {
    const PADDING: usize = 2;

    fn new() -> Self {
        Self { data: [[0; 21]; 21] }
    }

    // Returns the horizontal window for the 6-tap filter centered at (x, y) (half-pel position b)
    // The window covers integer pixels (x-2, y) to (x+3, y).
    fn get_horizontal_window(&self, x: usize, y: usize) -> &[i16] {
        &self.data[y + Self::PADDING][x..x + 6]
    }

    // Returns the vertical window for the 6-tap filter centered at (x, y) (half-pel position h)
    // The window covers integer pixels (x, y-2) to (x, y+3).
    fn get_vertical_window(&self, x: usize, y: usize) -> [i16; 6] {
        let col = x + Self::PADDING;
        [
            self.data[y][col],
            self.data[y + 1][col],
            self.data[y + 2][col],
            self.data[y + 3][col],
            self.data[y + 4][col],
            self.data[y + 5][col],
        ]
    }

    // Returns integer sample at (x, y)
    fn get_integer(&self, x: usize, y: usize) -> i16 {
        self.data[y + Self::PADDING][x + Self::PADDING]
    }
}

/*
 * Luma sample interpolation for quarter-pixel positions (Section 8.4.2.2.1, Figure 8-4).
 * Coordinates x_frac, y_frac are in 1/4th pixel units (0..3).
 * Anchor samples: integer (G, H, M), half-pel (b, h, j, m, s).
 */
fn interpolate_quarter_pel(
    buffer: &InterpolationBuffer,
    x: usize,
    y: usize,
    x_frac: i8,
    y_frac: i8,
) -> u8 {
    // get_h: Vertical half-sample 'h' at (x, y + 1/2)
    let get_h = |tx: usize, ty: usize| {
        let col = buffer.get_vertical_window(tx, ty);
        filter_6tap_and_clip(&col)
    };

    // get_b: Horizontal half-sample 'b' at (x + 1/2, y)
    let get_b = |tx: usize, ty: usize| filter_6tap_and_clip(buffer.get_horizontal_window(tx, ty));

    // get_j: Center half-sample 'j' at (x + 1/2, y + 1/2)
    let get_j = |tx: usize, ty: usize| {
        let mut row_results = [0i32; 6];
        for i in 0..6 {
            row_results[i] = filter_6tap(&buffer.data[ty + i][tx..tx + 6]);
        }
        filter_6tap_center_and_clip(&row_results)
    };

    // 'G' is the integer sample at (x, y)
    let g = buffer.get_integer(x, y) as u8;

    match (x_frac, y_frac) {
        (1, 0) => {
            // Position 'a' (1/4, 0)
            // Equation 8-250: a = (G + b + 1) >> 1
            let b = get_b(x, y);
            ((g as u16 + b as u16 + 1) >> 1) as u8
        }
        (3, 0) => {
            // Position 'c' (3/4, 0)
            // Equation 8-251: c = (H + b + 1) >> 1
            // 'H' is integer sample at (x + 1, y)
            let b = get_b(x, y);
            let h_int = buffer.get_integer(x + 1, y) as u8;
            ((h_int as u16 + b as u16 + 1) >> 1) as u8
        }
        (0, 1) => {
            // Position 'd' (0, 1/4)
            // Equation 8-252: d = (G + h + 1) >> 1
            let h = get_h(x, y);
            ((g as u16 + h as u16 + 1) >> 1) as u8
        }
        (0, 3) => {
            // Position 'n' (0, 3/4)
            // Equation 8-253: n = (M + h + 1) >> 1
            // 'M' is integer sample at (x, y + 1)
            let h = get_h(x, y);
            let m_int = buffer.get_integer(x, y + 1) as u8;
            ((m_int as u16 + h as u16 + 1) >> 1) as u8
        }
        (2, 1) => {
            // Position 'f' (1/2, 1/4)
            // Equation 8-254: f = (b + j + 1) >> 1
            let b = get_b(x, y);
            let j = get_j(x, y);
            ((b as u16 + j as u16 + 1) >> 1) as u8
        }
        (2, 3) => {
            // Position 'q' (1/2, 3/4)
            // Equation 8-257: q = (j + s + 1) >> 1
            // 's' is horizontal half-sample at (x + 1/2, y + 1) (i.e., 'b' at y+1)
            let s = get_b(x, y + 1);
            let j = get_j(x, y);
            ((s as u16 + j as u16 + 1) >> 1) as u8
        }
        (1, 2) => {
            // Position 'i' (1/4, 1/2)
            // Equation 8-255: i = (h + j + 1) >> 1
            let h = get_h(x, y);
            let j = get_j(x, y);
            ((h as u16 + j as u16 + 1) >> 1) as u8
        }
        (3, 2) => {
            // Position 'k' (3/4, 1/2)
            // Equation 8-256: k = (j + m + 1) >> 1
            // 'm' is vertical half-sample at (x + 1, y + 1/2) (i.e., 'h' at x+1)
            let m = get_h(x + 1, y);
            let j = get_j(x, y);
            ((m as u16 + j as u16 + 1) >> 1) as u8
        }
        (1, 1) => {
            // Position 'e' (1/4, 1/4)
            // Equation 8-258: e = (b + h + 1) >> 1
            let b = get_b(x, y);
            let h = get_h(x, y);
            ((b as u16 + h as u16 + 1) >> 1) as u8
        }
        (3, 1) => {
            // Position 'g' (3/4, 1/4)
            // Equation 8-259: g = (b + m + 1) >> 1
            // 'm' is vertical half-sample at (x + 1, y + 1/2) (i.e., 'h' at x+1)
            let b = get_b(x, y);
            let m = get_h(x + 1, y);
            ((b as u16 + m as u16 + 1) >> 1) as u8
        }
        (1, 3) => {
            // Position 'p' (1/4, 3/4)
            // Equation 8-260: p = (h + s + 1) >> 1
            // 's' is horizontal half-sample at (x + 1/2, y + 1) (i.e., 'b' at y+1)
            let s = get_b(x, y + 1);
            let h = get_h(x, y);
            ((s as u16 + h as u16 + 1) >> 1) as u8
        }
        (3, 3) => {
            // Position 'r' (3/4, 3/4)
            // Equation 8-261: r = (m + s + 1) >> 1
            // 's' is horizontal half-sample at (x + 1/2, y + 1) (i.e., 'b' at y+1)
            // 'm' is vertical half-sample at (x + 1, y + 1/2) (i.e., 'h' at x+1)
            let s = get_b(x, y + 1);
            let m = get_h(x + 1, y);
            ((m as u16 + s as u16 + 1) >> 1) as u8
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
        interpolate_luma(&plane, 2, 2, 0, 0, 4, 1, MotionVector { x: 2, y: 0 }, &mut dst, 4);
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
        interpolate_luma(&plane, 2, 2, 0, 0, 4, 1, MotionVector { x: 1, y: 0 }, &mut dst, 4);
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
