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
    buffer: &mut InterpolationBuffer,
) {
    let x_int = (mb_x as i32) + (blk_x as i32) + (mv.x >> 2) as i32;
    let y_int = (mb_y as i32) + (blk_y as i32) + (mv.y >> 2) as i32;
    let x_frac = (mv.x & 3) as i8;
    let y_frac = (mv.y & 3) as i8;

    // To implement the 6-tap filter for a block of size (width x height),
    // we need a window of (width + 5) x (height + 5) integer pixels.
    // Specifically, for 6-tap filter at pos G, we need E, F, G, H, I, J.
    // That is 2 pixels to the left/top and 3 pixels to the right/bottom.
    let buf_w = (width as usize) + 5;
    let buf_h = (height as usize) + 5;

    let plane_height = ref_plane.cfg.height as i32;
    let plane_width = ref_plane.cfg.width as i32;

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
            // Integer positions
            for y in 0..height as usize {
                let d = &mut dst[y * dst_stride..y * dst_stride + width as usize];
                for x in 0..width as usize {
                    d[x] = buffer.data[y + 2][x + 2] as u8;
                }
            }
        }
        (2, 0) => {
            // Half-pel horizontal (b)
            for y in 0..height as usize {
                let d = &mut dst[y * dst_stride..y * dst_stride + width as usize];
                let src_row = &buffer.data[y + 2];
                for x in 0..width as usize {
                    d[x] = filter_6tap_and_clip(&src_row[x..x + 6]);
                }
            }
        }
        (0, 2) => {
            // Half-pel vertical (h)
            for y in 0..height as usize {
                let d = &mut dst[y * dst_stride..y * dst_stride + width as usize];
                for x in 0..width as usize {
                    let col = [
                        buffer.data[y][x + 2],
                        buffer.data[y + 1][x + 2],
                        buffer.data[y + 2][x + 2],
                        buffer.data[y + 3][x + 2],
                        buffer.data[y + 4][x + 2],
                        buffer.data[y + 5][x + 2],
                    ];
                    d[x] = filter_6tap_and_clip(&col);
                }
            }
        }
        (2, 2) => {
            // Half-pel center (j)
            let mut intermediate = [0i32; 21 * 21];
            for y in 0..buf_h {
                let row = &buffer.data[y];
                for x in 0..width as usize {
                    intermediate[y * 21 + x] = filter_6tap(&row[x..x + 6]);
                }
            }
            for y in 0..height as usize {
                let d = &mut dst[y * dst_stride..y * dst_stride + width as usize];
                for x in 0..width as usize {
                    let val = intermediate[y * 21 + x] - 5 * intermediate[(y + 1) * 21 + x]
                        + 20 * intermediate[(y + 2) * 21 + x]
                        + 20 * intermediate[(y + 3) * 21 + x]
                        - 5 * intermediate[(y + 4) * 21 + x]
                        + intermediate[(y + 5) * 21 + x];
                    d[x] = ((val + 512) >> 10).clamp(0, 255) as u8;
                }
            }
        }
        (1, 0) => {
            // Position 'a' (1/4, 0)
            // Equation 8-250: a = (G + b + 1) >> 1
            for y in 0..height as usize {
                let d = &mut dst[y * dst_stride..y * dst_stride + width as usize];
                let src_row = &buffer.data[y + 2];
                for x in 0..width as usize {
                    let g = src_row[x + 2];
                    let b = filter_6tap_and_clip(&src_row[x..x + 6]);
                    d[x] = ((g as u16 + b as u16 + 1) >> 1) as u8;
                }
            }
        }
        (3, 0) => {
            // Position 'c' (3/4, 0)
            // Equation 8-251: c = (H + b + 1) >> 1
            for y in 0..height as usize {
                let d = &mut dst[y * dst_stride..y * dst_stride + width as usize];
                let src_row = &buffer.data[y + 2];
                for x in 0..width as usize {
                    let h = src_row[x + 3];
                    let b = filter_6tap_and_clip(&src_row[x..x + 6]);
                    d[x] = ((h as u16 + b as u16 + 1) >> 1) as u8;
                }
            }
        }
        (0, 1) => {
            // Position 'd' (0, 1/4)
            // Equation 8-252: d = (G + h + 1) >> 1
            for y in 0..height as usize {
                let d = &mut dst[y * dst_stride..y * dst_stride + width as usize];
                for x in 0..width as usize {
                    let g = buffer.data[y + 2][x + 2];
                    let h = filter_6tap_vertical_and_clip(buffer, x, y);
                    d[x] = ((g as u16 + h as u16 + 1) >> 1) as u8;
                }
            }
        }
        (0, 3) => {
            // Position 'n' (0, 3/4)
            // Equation 8-253: n = (M + h + 1) >> 1
            for y in 0..height as usize {
                let d = &mut dst[y * dst_stride..y * dst_stride + width as usize];
                for x in 0..width as usize {
                    let m = buffer.data[y + 3][x + 2];
                    let h = filter_6tap_vertical_and_clip(buffer, x, y);
                    d[x] = ((m as u16 + h as u16 + 1) >> 1) as u8;
                }
            }
        }
        (1, 1) => {
            // Position 'e' (1/4, 1/4)
            // Equation 8-258: e = (b + h + 1) >> 1
            for y in 0..height as usize {
                let d = &mut dst[y * dst_stride..y * dst_stride + width as usize];
                let src_row = &buffer.data[y + 2];
                for x in 0..width as usize {
                    let b = filter_6tap_and_clip(&src_row[x..x + 6]);
                    let h = filter_6tap_vertical_and_clip(buffer, x, y);
                    d[x] = ((b as u16 + h as u16 + 1) >> 1) as u8;
                }
            }
        }
        (3, 1) => {
            // Position 'g' (3/4, 1/4)
            // Equation 8-259: g = (b + m + 1) >> 1
            // 'm' is vertical half-sample at (x + 1, y + 1/2) (i.e., 'h' at x+1)
            for y in 0..height as usize {
                let d = &mut dst[y * dst_stride..y * dst_stride + width as usize];
                let src_row = &buffer.data[y + 2];
                for x in 0..width as usize {
                    let b = filter_6tap_and_clip(&src_row[x..x + 6]);
                    let m = filter_6tap_vertical_and_clip(buffer, x + 1, y);
                    d[x] = ((b as u16 + m as u16 + 1) >> 1) as u8;
                }
            }
        }
        (1, 3) => {
            // Position 'p' (1/4, 3/4)
            // Equation 8-260: p = (h + s + 1) >> 1
            // 's' is horizontal half-sample at (x + 1/2, y + 1) (i.e., 'b' at y+1)
            for y in 0..height as usize {
                let d = &mut dst[y * dst_stride..y * dst_stride + width as usize];
                let src_row_next = &buffer.data[y + 3];
                for x in 0..width as usize {
                    let h = filter_6tap_vertical_and_clip(buffer, x, y);
                    let s = filter_6tap_and_clip(&src_row_next[x..x + 6]);
                    d[x] = ((h as u16 + s as u16 + 1) >> 1) as u8;
                }
            }
        }
        (3, 3) => {
            // Position 'r' (3/4, 3/4)
            // Equation 8-261: r = (m + s + 1) >> 1
            for y in 0..height as usize {
                let d = &mut dst[y * dst_stride..y * dst_stride + width as usize];
                let src_row_next = &buffer.data[y + 3];
                for x in 0..width as usize {
                    let m = filter_6tap_vertical_and_clip(buffer, x + 1, y);
                    let s = filter_6tap_and_clip(&src_row_next[x..x + 6]);
                    d[x] = ((m as u16 + s as u16 + 1) >> 1) as u8;
                }
            }
        }
        // Cases needing j (center half-sample)
        (2, 1) | (2, 3) | (1, 2) | (3, 2) => {
            let mut intermediate = [0i32; 21 * 21];
            for y in 0..buf_h {
                let row = &buffer.data[y];
                for x in 0..width as usize {
                    intermediate[y * 21 + x] = filter_6tap(&row[x..x + 6]);
                }
            }

            match (x_frac, y_frac) {
                (2, 1) => {
                    // Position 'f' (1/2, 1/4)
                    // Equation 8-254: f = (b + j + 1) >> 1
                    for y in 0..height as usize {
                        let d = &mut dst[y * dst_stride..y * dst_stride + width as usize];
                        for x in 0..width as usize {
                            let b = clip_intermediate(intermediate[y * 21 + x]);
                            let j = get_j_from_intermediate(&intermediate, x, y);
                            d[x] = ((b as u16 + j as u16 + 1) >> 1) as u8;
                        }
                    }
                }
                (2, 3) => {
                    // Position 'q' (1/2, 3/4)
                    // Equation 8-257: q = (j + s + 1) >> 1
                    for y in 0..height as usize {
                        let d = &mut dst[y * dst_stride..y * dst_stride + width as usize];
                        for x in 0..width as usize {
                            let j = get_j_from_intermediate(&intermediate, x, y);
                            let s = clip_intermediate(intermediate[(y + 1) * 21 + x]);
                            d[x] = ((j as u16 + s as u16 + 1) >> 1) as u8;
                        }
                    }
                }
                (1, 2) => {
                    // Position 'i' (1/4, 1/2)
                    // Equation 8-255: i = (h + j + 1) >> 1
                    for y in 0..height as usize {
                        let d = &mut dst[y * dst_stride..y * dst_stride + width as usize];
                        for x in 0..width as usize {
                            let h = filter_6tap_vertical_and_clip(buffer, x, y);
                            let j = get_j_from_intermediate(&intermediate, x, y);
                            d[x] = ((h as u16 + j as u16 + 1) >> 1) as u8;
                        }
                    }
                }
                (3, 2) => {
                    // Position 'k' (3/4, 1/2)
                    // Equation 8-256: k = (j + m + 1) >> 1
                    for y in 0..height as usize {
                        let d = &mut dst[y * dst_stride..y * dst_stride + width as usize];
                        for x in 0..width as usize {
                            let j = get_j_from_intermediate(&intermediate, x, y);
                            let m = filter_6tap_vertical_and_clip(buffer, x + 1, y);
                            d[x] = ((j as u16 + m as u16 + 1) >> 1) as u8;
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

    let plane_width = ref_plane.cfg.width as i32;
    let plane_height = ref_plane.cfg.height as i32;
    if x_int >= 0
        && x_int + width as i32 + 1 <= plane_width
        && y_int >= 0
        && y_int + height as i32 + 1 <= plane_height
    {
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

                let prediction = (w00 * val_a + w10 * val_b + w01 * val_c + w11 * val_d + 32) >> 6;
                d[x] = prediction as u8;

                val_a = val_b;
                val_c = val_d;
            }
        }
    } else {
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

#[inline(always)]
fn filter_6tap(p: &[i16]) -> i32 {
    (p[0] as i32) - 5 * (p[1] as i32) + 20 * (p[2] as i32) + 20 * (p[3] as i32) - 5 * (p[4] as i32)
        + (p[5] as i32)
}

#[inline(always)]
fn filter_6tap_and_clip(p: &[i16]) -> u8 {
    let val = filter_6tap(p);
    ((val + 16) >> 5).clamp(0, 255) as u8
}

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
