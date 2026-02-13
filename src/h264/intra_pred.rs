use std::cmp::Ordering;

use log::info;
use v_frame::plane::{Plane, PlaneOffset};

use super::macroblock::{
    get_4x4chroma_block_location, get_4x4luma_block_location, get_4x4luma_block_neighbor, IMb,
    Intra_16x16_SamplePredMode, Intra_4x4_SamplePredMode, Intra_Chroma_Pred_Mode, MbAddr,
    MbNeighborName,
};
use super::residual::Block4x4;
use super::slice::Slice;
use super::tables::{MB_HEIGHT, MB_WIDTH};
use super::Point;

#[inline]
pub fn point_to_plane_offset(p: Point) -> PlaneOffset {
    PlaneOffset { x: p.x as isize, y: p.y as isize }
}

#[derive(Default)]
struct Surroundings4x4 {
    pub offset: PlaneOffset,
    pub top_row: [u8; 9],
    pub left_column: [u8; 5],
}

impl Surroundings4x4 {
    pub fn load(&mut self, plane: &Plane<u8>, blk_loc: Point, substitute_right: bool) {
        let mut offset = point_to_plane_offset(blk_loc);
        offset.x -= 1;
        offset.y -= 1;
        let mut target_slice = plane.slice(offset);

        if offset.y > 0 {
            if substitute_right {
                // Section 8.3.1.2 Intra_4x4 sample prediction
                // When samples p[ x, −1 ], with x = 4..7, are marked as "not available" ...
                self.top_row[0..5].copy_from_slice(&target_slice[0][0..5]);
                let filler = self.top_row[4];
                self.top_row[5..9].fill(filler);
            } else {
                self.top_row.copy_from_slice(&target_slice[0][0..9]);
            }
        } else {
            self.top_row.fill(0);
        }

        self.left_column[0] = self.top_row[0];
        offset.y += 1;
        target_slice = plane.slice(offset);
        if offset.x > 0 {
            for (idx, v) in target_slice.rows_iter().take(4).enumerate() {
                self.left_column[idx + 1] = v[0];
            }
        } else {
            self.left_column[1..5].fill(0);
        }

        offset.x += 1;
        self.offset = offset;
    }

    #[inline]
    pub fn top4(&self) -> &[u8] {
        &self.top_row[1..5]
    }

    #[inline]
    pub fn left4(&self) -> &[u8] {
        &self.left_column[1..5]
    }
}

// Section 8.3.1.1 Derivation process for Intra4x4PredMode
pub fn render_luma_4x4_intra_prediction(
    slice: &Slice,
    mb_addr: MbAddr,
    mb: &IMb,
    mb_loc: Point,
    target: &mut Plane<u8>,
    residuals: &[Block4x4],
) {
    // Equations 8-53, 8-54, etc.: (A + 2*B + C + 2) >> 2
    #[inline]
    fn weighted_avg(double: u8, single_a: u8, single_b: u8) -> u8 {
        ((2 * (double as u16) + (single_a as u16) + (single_b as u16) + 2) >> 2) as u8
    }

    // Standard average: (A + B + 1) >> 1
    #[inline]
    fn avg(a: u8, b: u8) -> u8 {
        (((a as u16) + (b as u16) + 1) >> 1) as u8
    }

    let mut ctx = Surroundings4x4::default();
    let has_c_mb_neighbor = slice.has_mb_neighbor(mb_addr, MbNeighborName::C);
    for blk_idx in 0..16 {
        let mut blk_loc = get_4x4luma_block_location(blk_idx);
        blk_loc.x += mb_loc.x;
        blk_loc.y += mb_loc.y;
        let substitute_right = match blk_idx {
            3 | 7 | 11 | 13 | 15 => true,
            5 => !has_c_mb_neighbor,
            _ => false,
        };
        ctx.load(target, blk_loc, substitute_right);
        let mut target_slice = target.mut_slice(ctx.offset);

        let mode = mb.rem_intra4x4_pred_mode[blk_idx as usize];
        match mode {
            Intra_4x4_SamplePredMode::Vertical => {
                // Section 8.3.1.2.1 Specification of Intra_4x4_Vertical prediction mode
                // Equation 8-46: pred4x4L[x, y] = p[x, -1]
                let src = ctx.top4();
                for row in target_slice.rows_iter_mut().take(4) {
                    row[0..4].copy_from_slice(src);
                }
            }
            Intra_4x4_SamplePredMode::Horizontal => {
                // Section 8.3.1.2.2 Specification of Intra_4x4_Horizontal prediction mode
                // Equation 8-47: pred4x4L[x, y] = p[-1, y]
                let src = ctx.left4();
                for (idx, row) in target_slice.rows_iter_mut().take(4).enumerate() {
                    row[0..4].fill(src[idx]);
                }
            }
            Intra_4x4_SamplePredMode::DC => {
                // Section 8.3.1.2.3 Specification of Intra_4x4_DC prediction mode
                // Calculate the sum of all the values at the left of the current macroblock
                let same_mb = get_4x4luma_block_neighbor(blk_idx, MbNeighborName::A).1.is_none();
                let sum_a = if same_mb || slice.has_mb_neighbor(mb_addr, MbNeighborName::A) {
                    Some(ctx.left4().iter().map(|v| *v as u32).sum::<u32>())
                } else {
                    None
                };

                // Calculate the sum of all the values at the top of the current macroblock
                let same_mb = get_4x4luma_block_neighbor(blk_idx, MbNeighborName::B).1.is_none();
                let sum_b = if same_mb || slice.has_mb_neighbor(mb_addr, MbNeighborName::B) {
                    Some(ctx.top4().iter().map(|v| *v as u32).sum::<u32>())
                } else {
                    None
                };

                // Equations 8-48 to 8-51: averaging top and/or left neighbors
                let mut sum = sum_a.unwrap_or(0) + sum_b.unwrap_or(0);
                if sum_a.is_some() && sum_b.is_some() {
                    sum = (sum + 4) >> 3;
                } else if sum_a.is_some() != sum_b.is_some() {
                    sum = (sum + 2) >> 2;
                } else {
                    sum = 1 << 7; // Default 128 for 8-bit
                }

                for row in target_slice.rows_iter_mut().take(4) {
                    row[0..4].fill(sum as u8);
                }
            }
            Intra_4x4_SamplePredMode::Diagonal_Down_Left => {
                // Section 8.3.1.2.4 Specification of Intra_4x4_Diagonal_Down_Left prediction mode
                let top_row = &ctx.top_row[1..=8];
                for (y, row) in target_slice.rows_iter_mut().take(4).enumerate() {
                    for (x, value) in row.iter_mut().take(4).enumerate() {
                        let i = x + y;
                        *value = if i == 6 {
                            // Equation 8-52
                            weighted_avg(top_row[7], top_row[7], top_row[6])
                        } else {
                            // Equation 8-53
                            weighted_avg(top_row[i + 1], top_row[i], top_row[i + 2])
                        };
                    }
                }
            }
            Intra_4x4_SamplePredMode::Diagonal_Down_Right => {
                // Section 8.3.1.2.5 Specification of Intra_4x4_Diagonal_Down_Right prediction mode
                // Equations 8-54 to 8-56
                let top = &ctx.top_row;
                let left = &ctx.left_column;
                for (y, row) in target_slice.rows_iter_mut().take(4).enumerate() {
                    for (x, value) in row.iter_mut().take(4).enumerate() {
                        *value = match x.cmp(&y) {
                            Ordering::Greater => {
                                let i = 1 + x - y;
                                weighted_avg(top[i - 1], top[i - 2], top[i])
                            }
                            Ordering::Less => {
                                let i = 1 + y - x;
                                weighted_avg(left[i - 1], left[i - 2], left[i])
                            }
                            Ordering::Equal => weighted_avg(top[0], top[1], left[1]),
                        }
                    }
                }
            }
            Intra_4x4_SamplePredMode::Vertical_Right => {
                // Section 8.3.1.2.6 Specification of Intra_4x4_Vertical_Right prediction mode
                // Equations 8-57 to 8-60
                let left = &ctx.left_column;
                let top = &ctx.top_row;
                for (y, row) in target_slice.rows_iter_mut().take(4).enumerate() {
                    for (x, value) in row.iter_mut().take(4).enumerate() {
                        let z = 2 * (x as isize) - (y as isize);
                        let i = 1 + x - (y >> 1);
                        *value = match z {
                            0 | 2 | 4 | 6 => avg(top[i], top[i - 1]),
                            1 | 3 | 5 => weighted_avg(top[i - 1], top[i - 2], top[i]),
                            -1 => weighted_avg(top[0], top[1], left[1]),
                            _ => {
                                let y = y + 1;
                                weighted_avg(left[y - 2], left[y - 1], left[y - 3])
                            }
                        };
                    }
                }
            }
            Intra_4x4_SamplePredMode::Horizontal_Down => {
                // Section 8.3.1.2.7 Specification of Intra_4x4_Horizontal_Down prediction mode
                // Equations 8-61 to 8-64
                let left = &ctx.left_column;
                let top = &ctx.top_row;
                for (y, row) in target_slice.rows_iter_mut().take(4).enumerate() {
                    for (x, value) in row.iter_mut().take(4).enumerate() {
                        let z = 2 * (y as isize) - (x as isize);
                        let i = 1 + y - (x >> 1);
                        *value = match z {
                            0 | 2 | 4 | 6 => avg(left[i - 1], left[i]),
                            1 | 3 | 5 => weighted_avg(left[i - 1], left[i - 2], left[i]),
                            -1 => weighted_avg(top[0], left[1], top[1]),
                            _ => weighted_avg(top[x - 1], top[x], top[x - 2]),
                        };
                    }
                }
            }
            Intra_4x4_SamplePredMode::Vertical_Left => {
                // Section 8.3.1.2.8 Specification of Intra_4x4_Vertical_Left prediction mode
                // Equations 8-65, 8-66
                let top_row = &ctx.top_row[1..];
                for (y, row) in target_slice.rows_iter_mut().take(4).enumerate() {
                    for (x, value) in row.iter_mut().take(4).enumerate() {
                        let i = x + (y >> 1);
                        *value = if y % 2 == 0 {
                            avg(top_row[i], top_row[i + 1])
                        } else {
                            weighted_avg(top_row[i + 1], top_row[i + 2], top_row[i])
                        };
                    }
                }
            }
            Intra_4x4_SamplePredMode::Horizontal_Up => {
                // Section 8.3.1.2.9 Specification of Intra_4x4_Horizontal_Up prediction mode
                // Equations 8-67 to 8-70
                let left = ctx.left4();
                for (y, row) in target_slice.rows_iter_mut().take(4).enumerate() {
                    for (x, value) in row.iter_mut().take(4).enumerate() {
                        let z = x + 2 * y;
                        let i = y + (x >> 1);
                        *value = match z {
                            0 | 2 | 4 => avg(left[i], left[i + 1]),
                            1 | 3 => weighted_avg(left[i + 1], left[i], left[i + 2]),
                            5 => weighted_avg(left[3], left[3], left[2]),
                            _ => left[3],
                        };
                    }
                }
            }
        }

        if let Some(residual) = residuals.get(blk_idx as usize) {
            for (y, row) in target_slice.rows_iter_mut().take(4).enumerate() {
                for (x, pixel) in row.iter_mut().take(4).enumerate() {
                    *pixel = (*pixel as i32 + residual.samples[y][x]).clamp(0, 255) as u8;
                }
            }
        }
    }
}

pub fn render_luma_16x16_intra_prediction(
    slice: &Slice,
    mb_addr: MbAddr,
    loc: Point,
    target: &mut Plane<u8>,
    mode: Intra_16x16_SamplePredMode,
    residuals: &[Block4x4],
) {
    let x = loc.x as usize;
    let y = loc.y as usize;
    let offset = point_to_plane_offset(loc);
    info!("luma 16x16 prediction: {mode:?}");
    match mode {
        Intra_16x16_SamplePredMode::Intra_16x16_Vertical => {
            // Section 8.3.3.1 Specification of Intra_16x16_Vertical prediction mode
            // Equation 8-116: predL[x, y] = p[x, -1]
            let mut src_row = [0; 16];
            src_row.copy_from_slice(&target.row(y as isize - 1)[x..(x + 16)]);
            let mut target_slice = target.mut_slice(offset);
            for row in target_slice.rows_iter_mut().take(16) {
                row[0..16].copy_from_slice(&src_row);
            }
        }
        Intra_16x16_SamplePredMode::Intra_16x16_Horizontal => {
            // Section 8.3.3.2 Specification of Intra_16x16_Horizontal prediction mode
            // Equation 8-117: predL[x, y] = p[-1, y]
            let mut target_slice = target.mut_slice(PlaneOffset { x: offset.x - 1, ..offset });
            for row in target_slice.rows_iter_mut().take(16) {
                let src = row[0];
                row[1..=16].fill(src);
            }
        }
        Intra_16x16_SamplePredMode::Intra_16x16_DC => {
            // Section 8.3.3.3 Specification of Intra_16x16_DC prediction mode
            let sum_a = if slice.has_mb_neighbor(mb_addr, MbNeighborName::A) {
                let target_slice = target.slice(PlaneOffset { x: offset.x - 1, ..offset });
                Some(target_slice.rows_iter().take(16).map(|r| r[0] as u32).sum::<u32>())
            } else {
                None
            };

            // Calculate the sum of all the values at the top of the current macroblock
            let sum_b = if slice.has_mb_neighbor(mb_addr, MbNeighborName::B) {
                let row = &target.row(y as isize - 1)[x..(x + 16)];
                Some(row.iter().map(|r| *r as u32).sum::<u32>())
            } else {
                None
            };

            // Equations 8-118 to 8-121
            let mut sum = sum_a.unwrap_or(0) + sum_b.unwrap_or(0);
            if sum_a.is_some() && sum_b.is_some() {
                sum = (sum + 16) >> 5;
            } else if sum_a.is_some() != sum_b.is_some() {
                sum = (sum + 8) >> 4;
            } else {
                sum = 1 << 7;
            }

            let mut target_slice = target.mut_slice(offset);
            for row in target_slice.rows_iter_mut().take(16) {
                row[0..16].fill(sum as u8);
            }
        }
        Intra_16x16_SamplePredMode::Intra_16x16_Plane => {
            // Section 8.3.3.4 Specification of Intra_16x16_Plane prediction mode
            let slice = target.slice(PlaneOffset { x: offset.x - 1, y: offset.y - 1 });
            let mut top = [0; 17];
            top.copy_from_slice(&slice[0][0..17]);
            let mut left = [0; 17];
            for (idx, row) in slice.rows_iter().take(17).enumerate() {
                left[idx] = row[0];
            }

            // Equations 8-126, 8-127
            let mut h = 0i32;
            let mut v = 0i32;
            for i in 1..=8 {
                h += (i as i32) * (top[i + 8] as i32 - top[8 - i] as i32);
                v += (i as i32) * (left[i + 8] as i32 - left[8 - i] as i32);
            }
            // Equation 8-123: a
            let a = (top[16] as i32 + left[16] as i32) * 16;
            // Equation 8-124: b
            let b = (5 * h + 32) >> 6;
            // Equation 8-125: c
            let c = (5 * v + 32) >> 6;

            let mut target_slice = target.mut_slice(offset);
            for (y, row) in target_slice.rows_iter_mut().take(16).enumerate() {
                for (x, pixel) in row.iter_mut().take(16).enumerate() {
                    let x = x as i32;
                    let y = y as i32;
                    // Equation 8-122
                    let value = (a + b * (x - 7) + c * (y - 7) + 16) >> 5;
                    *pixel = value.clamp(0, 255) as u8;
                }
            }
        }
    }

    for (blk_idx, blk) in residuals.iter().enumerate() {
        let mut blk_loc = get_4x4luma_block_location(blk_idx as u8);
        blk_loc.x += loc.x;
        blk_loc.y += loc.y;

        let mut plane_slice = target.mut_slice(point_to_plane_offset(blk_loc));
        for (y, row) in plane_slice.rows_iter_mut().take(4).enumerate() {
            for (x, pixel) in row.iter_mut().take(4).enumerate() {
                *pixel = (*pixel as i32 + blk.samples[y][x]).clamp(0, 255) as u8;
            }
        }
    }
}

pub fn render_chroma_intra_prediction(
    slice: &Slice,
    mb_addr: MbAddr,
    loc: Point,
    target: &mut Plane<u8>,
    mode: Intra_Chroma_Pred_Mode,
    residuals: &[Block4x4],
) {
    // Section 8.3.4 Intra prediction process for chroma samples
    let chroma_shift = slice.sps.ChromaArrayType().get_chroma_shift();
    let loc = Point { x: loc.x >> chroma_shift.width, y: loc.y >> chroma_shift.width };
    let mb_width = MB_WIDTH >> chroma_shift.width;
    let mb_height = MB_HEIGHT >> chroma_shift.height;
    let offset = point_to_plane_offset(loc);

    #[inline]
    fn sum(slice: &[u8]) -> u32 {
        slice.iter().map(|v| *v as u32).sum::<u32>()
    }

    match mode {
        Intra_Chroma_Pred_Mode::Vertical => {
            // Section 8.3.4.3 Specification of Intra_Chroma_Vertical prediction mode
            // Equation 8-143
            let x = loc.x as usize;
            let y = loc.y as usize;
            let mut src_row = [0; 16];
            src_row[0..mb_width].copy_from_slice(&target.row(y as isize - 1)[x..(x + mb_width)]);
            let mut target_slice = target.mut_slice(offset);
            for row in target_slice.rows_iter_mut().take(mb_height) {
                row[0..mb_width].copy_from_slice(&src_row[0..mb_width]);
            }
        }
        Intra_Chroma_Pred_Mode::Horizontal => {
            // Section 8.3.4.2 Specification of Intra_Chroma_Horizontal prediction mode
            // Equation 8-142
            let mut target_slice = target.mut_slice(PlaneOffset { x: offset.x - 1, ..offset });
            for row in target_slice.rows_iter_mut().take(mb_height) {
                let src = row[0];
                row[1..=mb_width].fill(src);
            }
        }
        Intra_Chroma_Pred_Mode::DC => {
            // Section 8.3.4.1 Specification of Intra_Chroma_DC prediction mode

            // Calculate the sum of all the values at the top of the current block
            let mut top_left = None;
            let mut top_right = None;
            if slice.has_mb_neighbor(mb_addr, MbNeighborName::B) {
                let target_slice = target.slice(PlaneOffset { y: offset.y - 1, ..offset });
                top_left = Some(sum(&target_slice[0][0..4]));
                top_right = Some(sum(&target_slice[0][4..8]));
            }

            // Calculate the sum of all the values at the left of the current block
            let mut left_top = None;
            let mut left_bottom = None;
            if slice.has_mb_neighbor(mb_addr, MbNeighborName::A) {
                let target_slice = target.slice(PlaneOffset { x: offset.x - 1, ..offset });
                let mut left_column = [0u8; 8];
                for (idx, row) in target_slice.rows_iter().take(8).enumerate() {
                    left_column[idx] = row[0];
                }
                left_top = Some(sum(&left_column[0..4]));
                left_bottom = Some(sum(&left_column[4..8]));
            }

            for blk_idx in 0..4 {
                // Equations 8-132 to 8-141 (derivation of prediction values based on availability)
                const DEFAULT_VALUE: u32 = 1 << 7; // = 1 << ( BitDepthC − 1 )
                let result = match blk_idx {
                    0 => {
                        // If ( xO, yO ) is equal to ( 0, 0 ) or xO and yO are greater than 0
                        if let (Some(left), Some(top)) = (left_top, top_left) {
                            (left + top + 4) >> 3
                        } else if let Some(s) = top_left {
                            (s + 2) >> 2
                        } else if let Some(s) = left_top {
                            (s + 2) >> 2
                        } else {
                            DEFAULT_VALUE
                        }
                    }
                    1 => {
                        // If xO is greater than 0 and yO is equal to 0
                        if let Some(s) = top_right {
                            (s + 2) >> 2
                        } else if let Some(s) = left_top {
                            (s + 2) >> 2
                        } else {
                            DEFAULT_VALUE
                        }
                    }
                    2 => {
                        // If xO is equal to 0 and yO is greater than 0
                        if let Some(s) = left_bottom {
                            (s + 2) >> 2
                        } else if let Some(s) = top_left {
                            (s + 2) >> 2
                        } else {
                            DEFAULT_VALUE
                        }
                    }
                    3 => {
                        if let (Some(left), Some(top)) = (left_bottom, top_right) {
                            (left + top + 4) >> 3
                        } else if let Some(s) = top_right {
                            (s + 2) >> 2
                        } else if let Some(s) = left_bottom {
                            (s + 2) >> 2
                        } else {
                            DEFAULT_VALUE
                        }
                    }
                    _ => unreachable!(),
                };

                let mut blk_loc = get_4x4chroma_block_location(blk_idx);
                blk_loc.x += loc.x;
                blk_loc.y += loc.y;
                let mut target_slice = target.mut_slice(point_to_plane_offset(blk_loc));
                for row in target_slice.rows_iter_mut().take(4) {
                    row[0..4].fill(result as u8);
                }
            }
        }
        Intra_Chroma_Pred_Mode::Plane => {
            // Section 8.3.4.4 Specification of Intra_Chroma_Plane prediction mode
            // yCF = 0 and xCF = 0
            let target_slice = target.slice(PlaneOffset { x: offset.x - 1, y: offset.y - 1 });
            let mut h = 0;
            let mut top_row = [0u8; 9];
            top_row.copy_from_slice(&target_slice[0][0..9]);
            // Equation 8-148: H
            for x in 0..4usize {
                h += (x as isize + 1) * (top_row[4 + 1 + x] as isize - top_row[2 + 1 - x] as isize);
            }

            let mut v = 0;
            let mut left_column = [0u8; 9];
            for (idx, row) in target_slice.rows_iter().take(9).enumerate() {
                left_column[idx] = row[0];
            }
            // Equation 8-149: V
            for y in 0..4usize {
                v += (y as isize + 1)
                    * (left_column[4 + 1 + y] as isize - left_column[2 + 1 - y] as isize);
            }

            // Equation 8-145: a
            let a = 16 * (left_column[8] as isize + top_row[8] as isize);
            // Equation 8-146: b
            let b = (34 * h + 32) >> 6;
            // Equation 8-147: c
            let c = (34 * v + 32) >> 6;

            let mut target_slice = target.mut_slice(offset);
            for (y, row) in target_slice.rows_iter_mut().take(mb_height).enumerate() {
                for (x, pixel) in row.iter_mut().take(mb_width).enumerate() {
                    let x = x as isize;
                    let y = y as isize;
                    // Equation 8-144
                    *pixel = ((a + b * (x - 3) + c * (y - 3) + 16) >> 5) as u8;
                }
            }
        }
    }

    for (blk_idx, residual) in residuals.iter().enumerate() {
        let mut blk_loc = get_4x4chroma_block_location(blk_idx as u8);
        blk_loc.x += loc.x;
        blk_loc.y += loc.y;
        let mut target_slice = target.mut_slice(point_to_plane_offset(blk_loc));
        for (y, row) in target_slice.rows_iter_mut().take(4).enumerate() {
            for (x, pixel) in row.iter_mut().take(4).enumerate() {
                *pixel = (*pixel as i32 + residual.samples[y][x]).clamp(0, 255) as u8;
            }
        }
    }
}
