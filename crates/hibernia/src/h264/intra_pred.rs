use std::cmp::Ordering;

use v_frame::plane::{Plane, PlaneOffset};

use super::macroblock::{
    get_4x4chroma_block_location, get_4x4luma_block_location, get_4x4luma_block_neighbor,
    get_8x8luma_block_location, IMb, Intra_16x16_SamplePredMode, Intra_4x4_SamplePredMode,
    Intra_8x8_SamplePredMode, Intra_Chroma_Pred_Mode, MbAddr, MbNeighborName,
};
use super::residual::{add_residual_4x4, Block4x4};
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
                // When samples p[ x, -1 ], with x = 4..7, are marked as "not available" ...
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

// Section 8.3.1.2 Intra_4x4 sample prediction. Mode selection per Intra4x4PredMode
// is the parser's job (8.3.1.1); this routine consumes the resolved mode and
// produces the predicted luma samples.
pub fn render_luma_4x4_intra_prediction(
    slice: &Slice,
    mb_addr: MbAddr,
    mb: &IMb,
    mb_loc: Point,
    target: &mut Plane<u8>,
    residuals: &[Block4x4],
) {
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
                            weighted_avg3(top_row[6], top_row[7], top_row[7])
                        } else {
                            // Equation 8-53
                            weighted_avg3(top_row[i], top_row[i + 1], top_row[i + 2])
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
                                weighted_avg3(top[i - 2], top[i - 1], top[i])
                            }
                            Ordering::Less => {
                                let i = 1 + y - x;
                                weighted_avg3(left[i - 2], left[i - 1], left[i])
                            }
                            Ordering::Equal => weighted_avg3(top[1], top[0], left[1]),
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
                            1 | 3 | 5 => weighted_avg3(top[i - 2], top[i - 1], top[i]),
                            -1 => weighted_avg3(top[1], top[0], left[1]),
                            _ => {
                                let y = y + 1;
                                weighted_avg3(left[y - 3], left[y - 2], left[y - 1])
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
                            1 | 3 | 5 => weighted_avg3(left[i - 2], left[i - 1], left[i]),
                            -1 => weighted_avg3(left[1], top[0], top[1]),
                            _ => weighted_avg3(top[x - 2], top[x - 1], top[x]),
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
                            weighted_avg3(top_row[i], top_row[i + 1], top_row[i + 2])
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
                            1 | 3 => weighted_avg3(left[i], left[i + 1], left[i + 2]),
                            5 => weighted_avg3(left[2], left[3], left[3]),
                            _ => left[3],
                        };
                    }
                }
            }
        }

        if let Some(residual) = residuals.get(blk_idx as usize) {
            let stride = target.cfg.stride;
            let origin = ctx.offset.y as usize * stride + ctx.offset.x as usize;
            add_residual_4x4(target.data_origin_mut(), origin, stride, residual);
        }
    }
}

// (A + 2*B + C + 2) >> 2 -- 3-tap low-pass used throughout Intra 4x4/8x8.
#[inline]
fn weighted_avg3(a: u8, b: u8, c: u8) -> u8 {
    (((a as u16) + 2 * (b as u16) + (c as u16) + 2) >> 2) as u8
}

// (A + B + 1) >> 1 -- 2-tap average used throughout Intra 4x4 / 8x8.
#[inline]
fn avg(a: u8, b: u8) -> u8 {
    (((a as u16) + (b as u16) + 1) >> 1) as u8
}

// Section 8.3.2.2 reference-sample buffer for a single 8x8 luma block.
// After loading, `p_top[0..16]`, `p_left[0..8]`, `p_corner` hold the (possibly
// low-pass filtered) reference samples used by the mode formulas. Indexing is
// direct: `p_top[i]` is spec's p'[i, -1]; `p_left[y]` is p'[-1, y]; `p_corner`
// is p'[-1, -1].
#[derive(Default)]
struct Surroundings8x8 {
    offset: PlaneOffset,
    p_top: [u8; 16],
    p_left: [u8; 8],
    p_corner: u8,
    top_available: bool,
    left_available: bool,
    corner_available: bool,
}

impl Surroundings8x8 {
    // Reference-sample availability is slice-scoped (Section 6.4.8): a neighbor
    // sample whose containing MB is in a different slice is "not available for
    // Intra_8x8 prediction" even though it sits at a valid plane offset. The
    // caller passes `has_top` / `has_left` / `has_corner` already reduced
    // against the slice's `has_mb_neighbor` map; here we only consume them.
    //
    // `has_top_right` covers the p[x, -1] samples for x = 8..15. For block 0
    // they live within neighbor MB B's bottom row; for block 1 they require
    // neighbor MB C (above-right); for block 2 they come from within the
    // current MB (block 1's bottom row, always decoded); for block 3 they lie
    // to the right of the current MB and are never available (substituted from
    // p[7, -1]).
    pub fn load(
        &mut self,
        plane: &Plane<u8>,
        blk_loc: Point,
        has_top: bool,
        has_left: bool,
        has_corner: bool,
        has_top_right: bool,
    ) {
        let offset = point_to_plane_offset(blk_loc);
        let top_available = has_top && offset.y > 0;
        let left_available = has_left && offset.x > 0;
        let corner_available = has_corner && offset.y > 0 && offset.x > 0;

        let mut top_row_raw = [0u8; 16];
        let mut left_col_raw = [0u8; 8];
        let mut corner_raw = 0u8;

        if top_available {
            let top_slice = plane.slice(PlaneOffset { x: offset.x, y: offset.y - 1 });
            if has_top_right {
                top_row_raw.copy_from_slice(&top_slice[0][0..16]);
            } else {
                top_row_raw[0..8].copy_from_slice(&top_slice[0][0..8]);
                // Substitution per Clause 8.3.2.2: when p[x, -1] with x = 8..15
                // is unavailable but p[7, -1] is available, replicate p[7, -1].
                let fill = top_row_raw[7];
                top_row_raw[8..16].fill(fill);
            }
        }

        if left_available {
            let left_slice = plane.slice(PlaneOffset { x: offset.x - 1, y: offset.y });
            for (idx, row) in left_slice.rows_iter().take(8).enumerate() {
                left_col_raw[idx] = row[0];
            }
        }

        if corner_available {
            let corner_slice = plane.slice(PlaneOffset { x: offset.x - 1, y: offset.y - 1 });
            corner_raw = corner_slice[0][0];
        }

        self.top_available = top_available;
        self.left_available = left_available;
        self.corner_available = corner_available;

        // Clause 8.3.2.2.1 reference-sample filter. Applied only when the
        // relevant samples are available.
        if top_available {
            // Eq. 8-78 / 8-79
            self.p_top[0] = if corner_available {
                weighted_avg3(corner_raw, top_row_raw[0], top_row_raw[1])
            } else {
                (((3 * top_row_raw[0] as u16) + top_row_raw[1] as u16 + 2) >> 2) as u8
            };
            // Eq. 8-80
            for x in 1..=14 {
                self.p_top[x] = weighted_avg3(top_row_raw[x - 1], top_row_raw[x], top_row_raw[x + 1]);
            }
            // Eq. 8-81
            self.p_top[15] =
                ((top_row_raw[14] as u16 + 3 * top_row_raw[15] as u16 + 2) >> 2) as u8;
        } else {
            self.p_top = top_row_raw;
        }

        if left_available {
            // Eq. 8-85 / 8-86
            self.p_left[0] = if corner_available {
                weighted_avg3(corner_raw, left_col_raw[0], left_col_raw[1])
            } else {
                (((3 * left_col_raw[0] as u16) + left_col_raw[1] as u16 + 2) >> 2) as u8
            };
            // Eq. 8-87
            for y in 1..=6 {
                self.p_left[y] =
                    weighted_avg3(left_col_raw[y - 1], left_col_raw[y], left_col_raw[y + 1]);
            }
            // Eq. 8-88
            self.p_left[7] =
                ((left_col_raw[6] as u16 + 3 * left_col_raw[7] as u16 + 2) >> 2) as u8;
        } else {
            self.p_left = left_col_raw;
        }

        // Corner filter: Eq. 8-82 / 8-83 / 8-84.
        self.p_corner = if corner_available {
            // With our simplification, corner_available implies both top and
            // left are available, so only Eq. 8-84 applies.
            weighted_avg3(top_row_raw[0], corner_raw, left_col_raw[0])
        } else {
            0
        };

        self.offset = offset;
    }
}

// Section 8.3.2 Intra_8x8 sample prediction.
pub fn render_luma_8x8_intra_prediction(
    slice: &Slice,
    mb_addr: MbAddr,
    mb: &IMb,
    mb_loc: Point,
    target: &mut Plane<u8>,
    residuals: &[Block4x4],
) {
    let has_a = slice.has_mb_neighbor(mb_addr, MbNeighborName::A);
    let has_b = slice.has_mb_neighbor(mb_addr, MbNeighborName::B);
    let has_c = slice.has_mb_neighbor(mb_addr, MbNeighborName::C);
    let has_d = slice.has_mb_neighbor(mb_addr, MbNeighborName::D);
    let mut ctx = Surroundings8x8::default();

    for blk_idx in 0..4u8 {
        let mut blk_loc = get_8x8luma_block_location(blk_idx);
        blk_loc.x += mb_loc.x;
        blk_loc.y += mb_loc.y;
        // Per-block neighbor availability. Blocks 0/1 pull their top reference
        // from MB B; blocks 0/2 pull their left from MB A; block 0's corner is
        // MB D; block 1's corner sits in MB B; block 2's corner sits in MB A.
        // Within-MB block edges (blocks 2/3 top, blocks 1/3 left, block 3
        // corner) are always available because the source block is decoded
        // earlier in the same slice.
        let (has_top, has_left, has_corner, has_top_right) = match blk_idx {
            0 => (has_b, has_a, has_d, has_b),
            1 => (has_b, true, has_b, has_c),
            2 => (true, has_a, has_a, true),
            3 => (true, true, true, false),
            _ => unreachable!(),
        };
        ctx.load(target, blk_loc, has_top, has_left, has_corner, has_top_right);

        let mode = mb.rem_intra8x8_pred_mode[blk_idx as usize];
        let mut pred = [[0u8; 8]; 8];
        fill_predicted_8x8(&mut pred, &ctx, mode);

        // Write predicted samples and add 4x4 residuals that belong to this 8x8.
        let mut target_slice = target.mut_slice(ctx.offset);
        for (y, row) in target_slice.rows_iter_mut().take(8).enumerate() {
            for (x, pixel) in row.iter_mut().take(8).enumerate() {
                *pixel = pred[y][x];
            }
        }
        let stride = target.cfg.stride;
        let mb_origin = ctx.offset.y as usize * stride + ctx.offset.x as usize;
        let buf = target.data_origin_mut();
        for i4x4 in 0..4 {
            let sub_idx = (blk_idx as usize) * 4 + i4x4;
            let Some(residual) = residuals.get(sub_idx) else { continue };
            let sub_x = (i4x4 & 1) * 4;
            let sub_y = (i4x4 >> 1) * 4;
            add_residual_4x4(buf, mb_origin + sub_y * stride + sub_x, stride, residual);
        }
    }
}

// Compute predicted samples for a single 8x8 block. `pred[y][x]` holds p8x8L[x, y].
fn fill_predicted_8x8(
    pred: &mut [[u8; 8]; 8],
    ctx: &Surroundings8x8,
    mode: Intra_8x8_SamplePredMode,
) {
    let t = &ctx.p_top;
    let l = &ctx.p_left;
    let c = ctx.p_corner;

    match mode {
        Intra_8x8_SamplePredMode::Vertical => {
            // Section 8.3.2.2.2 -- Eq. 8-89: pred8x8L[x, y] = p'[x, -1]
            for y in 0..8 {
                for x in 0..8 {
                    pred[y][x] = t[x];
                }
            }
        }
        Intra_8x8_SamplePredMode::Horizontal => {
            // Section 8.3.2.2.3 -- Eq. 8-90: pred8x8L[x, y] = p'[-1, y]
            for y in 0..8 {
                for x in 0..8 {
                    pred[y][x] = l[y];
                }
            }
        }
        Intra_8x8_SamplePredMode::DC => {
            // Section 8.3.2.2.4 -- Eqs. 8-91 to 8-94.
            let v = match (ctx.top_available, ctx.left_available) {
                (true, true) => {
                    let s: u32 = t[0..8].iter().map(|v| *v as u32).sum::<u32>()
                        + l[0..8].iter().map(|v| *v as u32).sum::<u32>();
                    ((s + 8) >> 4) as u8
                }
                (true, false) => {
                    let s: u32 = t[0..8].iter().map(|v| *v as u32).sum();
                    ((s + 4) >> 3) as u8
                }
                (false, true) => {
                    let s: u32 = l[0..8].iter().map(|v| *v as u32).sum();
                    ((s + 4) >> 3) as u8
                }
                (false, false) => 128, // 1 << (BitDepthY - 1) for 8-bit
            };
            for y in 0..8 {
                for x in 0..8 {
                    pred[y][x] = v;
                }
            }
        }
        Intra_8x8_SamplePredMode::Diagonal_Down_Left => {
            // Section 8.3.2.2.5 -- Eqs. 8-95, 8-96. Uses p'[0..=15, -1].
            for y in 0..8usize {
                for x in 0..8usize {
                    pred[y][x] = if x == 7 && y == 7 {
                        weighted_avg3(t[14], t[15], t[15])
                    } else {
                        weighted_avg3(t[x + y], t[x + y + 1], t[x + y + 2])
                    };
                }
            }
        }
        Intra_8x8_SamplePredMode::Diagonal_Down_Right => {
            // Section 8.3.2.2.6 -- Eqs. 8-97 to 8-99.
            for y in 0..8usize {
                for x in 0..8usize {
                    pred[y][x] = match x.cmp(&y) {
                        std::cmp::Ordering::Greater => {
                            // Eq. 8-97: p'[x-y-2, -1], p'[x-y-1, -1], p'[x-y, -1]
                            let i = x - y;
                            let a = if i >= 2 { t[i - 2] } else { c };
                            let b = if i >= 1 { t[i - 1] } else { c };
                            weighted_avg3(a, b, t[i])
                        }
                        std::cmp::Ordering::Less => {
                            // Eq. 8-98: p'[-1, y-x-2], p'[-1, y-x-1], p'[-1, y-x]
                            let i = y - x;
                            let a = if i >= 2 { l[i - 2] } else { c };
                            let b = if i >= 1 { l[i - 1] } else { c };
                            weighted_avg3(a, b, l[i])
                        }
                        std::cmp::Ordering::Equal => {
                            // Eq. 8-99: p'[0, -1], p'[-1, -1], p'[-1, 0]
                            weighted_avg3(t[0], c, l[0])
                        }
                    };
                }
            }
        }
        Intra_8x8_SamplePredMode::Vertical_Right => {
            // Section 8.3.2.2.7 -- Eqs. 8-100 to 8-103.
            for y in 0..8i32 {
                for x in 0..8i32 {
                    let z_vr = 2 * x - y;
                    // Base index i = x - (y >> 1) into the top row. Some negative
                    // offsets wrap back to the corner or left column per spec.
                    let get_t = |i: i32| -> u8 {
                        if i < 0 {
                            c
                        } else {
                            t[i as usize]
                        }
                    };
                    pred[y as usize][x as usize] = match z_vr {
                        0 | 2 | 4 | 6 | 8 | 10 | 12 | 14 => {
                            let i = x - (y >> 1);
                            avg(get_t(i - 1), get_t(i))
                        }
                        1 | 3 | 5 | 7 | 9 | 11 | 13 => {
                            let i = x - (y >> 1);
                            weighted_avg3(get_t(i - 2), get_t(i - 1), get_t(i))
                        }
                        -1 => weighted_avg3(l[0], c, t[0]),
                        _ => {
                            // z_vr in {-2..-7}: use left column
                            let idx = (y - 2 * x - 1) as usize;
                            let a = if idx >= 2 { l[idx - 2] } else { c };
                            let b = if idx >= 1 { l[idx - 1] } else { c };
                            weighted_avg3(a, b, l[idx])
                        }
                    };
                }
            }
        }
        Intra_8x8_SamplePredMode::Horizontal_Down => {
            // Section 8.3.2.2.8 -- Eqs. 8-104 to 8-107.
            for y in 0..8i32 {
                for x in 0..8i32 {
                    let z_hd = 2 * y - x;
                    let get_l = |i: i32| -> u8 {
                        if i < 0 {
                            c
                        } else {
                            l[i as usize]
                        }
                    };
                    pred[y as usize][x as usize] = match z_hd {
                        0 | 2 | 4 | 6 | 8 | 10 | 12 | 14 => {
                            let i = y - (x >> 1);
                            avg(get_l(i - 1), get_l(i))
                        }
                        1 | 3 | 5 | 7 | 9 | 11 | 13 => {
                            let i = y - (x >> 1);
                            weighted_avg3(get_l(i - 2), get_l(i - 1), get_l(i))
                        }
                        -1 => weighted_avg3(l[0], c, t[0]),
                        _ => {
                            // z_hd in {-2..-7}: use top row
                            let idx = (x - 2 * y - 1) as usize;
                            let a = if idx >= 2 { t[idx - 2] } else { c };
                            let b = if idx >= 1 { t[idx - 1] } else { c };
                            weighted_avg3(a, b, t[idx])
                        }
                    };
                }
            }
        }
        Intra_8x8_SamplePredMode::Vertical_Left => {
            // Section 8.3.2.2.9 -- Eqs. 8-108, 8-109. Uses p'[0..=15, -1].
            for y in 0..8usize {
                for x in 0..8usize {
                    let i = x + (y >> 1);
                    pred[y][x] = if y % 2 == 0 {
                        avg(t[i], t[i + 1])
                    } else {
                        weighted_avg3(t[i], t[i + 1], t[i + 2])
                    };
                }
            }
        }
        Intra_8x8_SamplePredMode::Horizontal_Up => {
            // Section 8.3.2.2.10 -- Eqs. 8-110 to 8-113.
            for y in 0..8usize {
                for x in 0..8usize {
                    let z_hu = x + 2 * y;
                    pred[y][x] = match z_hu {
                        0 | 2 | 4 | 6 | 8 | 10 | 12 => {
                            let i = y + (x >> 1);
                            avg(l[i], l[i + 1])
                        }
                        1 | 3 | 5 | 7 | 9 | 11 => {
                            let i = y + (x >> 1);
                            weighted_avg3(l[i], l[i + 1], l[i + 2])
                        }
                        13 => weighted_avg3(l[6], l[7], l[7]),
                        _ => l[7], // z_hu > 13
                    };
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
    let stride = target.cfg.stride;
    let mb_x = loc.x as usize;
    let mb_y = loc.y as usize;
    let mb_origin = mb_y * stride + mb_x;
    let data = target.data_origin_mut();

    match mode {
        Intra_16x16_SamplePredMode::Intra_16x16_Vertical => {
            // Section 8.3.3.1 Specification of Intra_16x16_Vertical prediction mode
            // Equation 8-116: predL[x, y] = p[x, -1]
            let top_base = mb_origin - stride;
            let mut src_row = [0u8; 16];
            src_row.copy_from_slice(&data[top_base..top_base + 16]);
            for y in 0..16 {
                let row_base = mb_origin + y * stride;
                data[row_base..row_base + 16].copy_from_slice(&src_row);
            }
        }
        Intra_16x16_SamplePredMode::Intra_16x16_Horizontal => {
            // Section 8.3.3.2 Specification of Intra_16x16_Horizontal prediction mode
            // Equation 8-117: predL[x, y] = p[-1, y]
            for y in 0..16 {
                let row_base = mb_origin + y * stride;
                let src = data[row_base - 1];
                data[row_base..row_base + 16].fill(src);
            }
        }
        Intra_16x16_SamplePredMode::Intra_16x16_DC => {
            // Section 8.3.3.3 Specification of Intra_16x16_DC prediction mode
            let sum_a = if slice.has_mb_neighbor(mb_addr, MbNeighborName::A) {
                Some((0..16usize).map(|y| data[mb_origin + y * stride - 1] as u32).sum::<u32>())
            } else {
                None
            };

            // Calculate the sum of all the values at the top of the current macroblock
            let sum_b = if slice.has_mb_neighbor(mb_addr, MbNeighborName::B) {
                let top_base = mb_origin - stride;
                Some(data[top_base..top_base + 16].iter().map(|r| *r as u32).sum::<u32>())
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
            let dc = sum as u8;

            for y in 0..16 {
                let row_base = mb_origin + y * stride;
                data[row_base..row_base + 16].fill(dc);
            }
        }
        Intra_16x16_SamplePredMode::Intra_16x16_Plane => {
            // Section 8.3.3.4 Specification of Intra_16x16_Plane prediction mode
            // Read 17 top samples (corner + 16 above) and 17 left samples
            // (corner + 16 to the left) directly from the hoisted `data`.
            let mut top = [0u8; 17];
            let top_base = mb_origin - stride - 1;
            top.copy_from_slice(&data[top_base..top_base + 17]);
            let mut left = [0u8; 17];
            for i in 0..17 {
                left[i] = data[(mb_origin - stride - 1) + i * stride];
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

            for py in 0..16 {
                let row_base = mb_origin + py * stride;
                let py_i = py as i32;
                for px in 0..16 {
                    let px_i = px as i32;
                    // Equation 8-122
                    let value = (a + b * (px_i - 7) + c * (py_i - 7) + 16) >> 5;
                    data[row_base + px] = value.clamp(0, 255) as u8;
                }
            }
        }
    }

    // Add residuals -- each 4x4 block at its known offset within the MB.
    for (blk_idx, blk) in residuals.iter().enumerate() {
        let blk_loc = get_4x4luma_block_location(blk_idx as u8);
        let blk_base =
            mb_origin + (blk_loc.y as usize) * stride + (blk_loc.x as usize);
        for ry in 0..4 {
            let row_base = blk_base + ry * stride;
            for rx in 0..4 {
                let v = data[row_base + rx] as i32 + blk.samples[ry][rx];
                data[row_base + rx] = v.clamp(0, 255) as u8;
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
    let loc = Point { x: loc.x >> chroma_shift.width, y: loc.y >> chroma_shift.height };
    let mb_width = MB_WIDTH >> chroma_shift.width;
    let mb_height = MB_HEIGHT >> chroma_shift.height;

    let stride = target.cfg.stride;
    let mb_x = loc.x as usize;
    let mb_y = loc.y as usize;
    let mb_origin = mb_y * stride + mb_x;
    let data = target.data_origin_mut();

    match mode {
        Intra_Chroma_Pred_Mode::Vertical => {
            // Section 8.3.4.3 Specification of Intra_Chroma_Vertical prediction mode
            // Equation 8-143
            let top_base = mb_origin - stride;
            let mut src_row = [0u8; 16];
            src_row[0..mb_width].copy_from_slice(&data[top_base..top_base + mb_width]);
            for y in 0..mb_height {
                let row_base = mb_origin + y * stride;
                data[row_base..row_base + mb_width].copy_from_slice(&src_row[0..mb_width]);
            }
        }
        Intra_Chroma_Pred_Mode::Horizontal => {
            // Section 8.3.4.2 Specification of Intra_Chroma_Horizontal prediction mode
            // Equation 8-142
            for y in 0..mb_height {
                let row_base = mb_origin + y * stride;
                let src = data[row_base - 1];
                data[row_base..row_base + mb_width].fill(src);
            }
        }
        Intra_Chroma_Pred_Mode::DC => {
            // Section 8.3.4.1 Specification of Intra_Chroma_DC prediction mode

            // Calculate the sum of all the values at the top of the current block
            let mut top_left = None;
            let mut top_right = None;
            if slice.has_mb_neighbor(mb_addr, MbNeighborName::B) {
                let top_base = mb_origin - stride;
                top_left = Some(data[top_base..top_base + 4].iter().map(|v| *v as u32).sum::<u32>());
                top_right =
                    Some(data[top_base + 4..top_base + 8].iter().map(|v| *v as u32).sum::<u32>());
            }

            // Calculate the sum of all the values at the left of the current block
            let mut left_top = None;
            let mut left_bottom = None;
            if slice.has_mb_neighbor(mb_addr, MbNeighborName::A) {
                let mut left_column = [0u8; 8];
                for i in 0..8 {
                    left_column[i] = data[mb_origin + i * stride - 1];
                }
                left_top = Some(left_column[0..4].iter().map(|v| *v as u32).sum::<u32>());
                left_bottom = Some(left_column[4..8].iter().map(|v| *v as u32).sum::<u32>());
            }

            for blk_idx in 0..4 {
                // Equations 8-132 to 8-141 (derivation of prediction values based on availability).
                // The "no neighbor available" branch substitutes 1 << (BitDepthC - 1)
                // per clause 8.3.4.1; the literal 7 is BitDepthC - 1 with BitDepthC = 8,
                // matching the u8 chroma plane the renderer writes into.
                const DEFAULT_VALUE: u32 = 1 << 7;
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
                let result = result as u8;

                let blk_loc = get_4x4chroma_block_location(blk_idx);
                let blk_base = mb_origin + (blk_loc.y as usize) * stride + (blk_loc.x as usize);
                for ry in 0..4 {
                    let row_base = blk_base + ry * stride;
                    data[row_base..row_base + 4].fill(result);
                }
            }
        }
        Intra_Chroma_Pred_Mode::Plane => {
            // Section 8.3.4.4 Specification of Intra_Chroma_Plane prediction mode
            // yCF = 0 and xCF = 0
            // Read 9 top samples (corner + 8 above) and 9 left samples
            // (corner + 8 to the left).
            let top_base = mb_origin - stride - 1;
            let mut top_row = [0u8; 9];
            top_row.copy_from_slice(&data[top_base..top_base + 9]);
            let mut h = 0isize;
            // Equation 8-148: H
            for x in 0..4usize {
                h += (x as isize + 1) * (top_row[4 + 1 + x] as isize - top_row[2 + 1 - x] as isize);
            }

            let mut left_column = [0u8; 9];
            for i in 0..9 {
                left_column[i] = data[(mb_origin - stride - 1) + i * stride];
            }
            let mut v = 0isize;
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

            for py in 0..mb_height {
                let row_base = mb_origin + py * stride;
                let py_i = py as isize;
                for px in 0..mb_width {
                    let px_i = px as isize;
                    // Equation 8-144
                    let value = (a + b * (px_i - 3) + c * (py_i - 3) + 16) >> 5;
                    data[row_base + px] = value.clamp(0, 255) as u8;
                }
            }
        }
    }

    for (blk_idx, residual) in residuals.iter().enumerate() {
        let blk_loc = get_4x4chroma_block_location(blk_idx as u8);
        let blk_base = mb_origin + (blk_loc.y as usize) * stride + (blk_loc.x as usize);
        add_residual_4x4(data, blk_base, stride, residual);
    }
}

#[cfg(test)]
mod tests_intra_8x8 {
    use super::*;

    fn ctx_with(top: [u8; 16], left: [u8; 8], corner: u8) -> Surroundings8x8 {
        Surroundings8x8 {
            offset: PlaneOffset::default(),
            p_top: top,
            p_left: left,
            p_corner: corner,
            top_available: true,
            left_available: true,
            corner_available: true,
        }
    }

    #[test]
    fn mode_vertical_copies_top_row() {
        let mut top = [0u8; 16];
        for i in 0..16 {
            top[i] = (10 + i) as u8;
        }
        let ctx = ctx_with(top, [0; 8], 0);
        let mut pred = [[0u8; 8]; 8];
        fill_predicted_8x8(&mut pred, &ctx, Intra_8x8_SamplePredMode::Vertical);
        for y in 0..8 {
            for x in 0..8 {
                assert_eq!(pred[y][x], top[x], "(x={x}, y={y})");
            }
        }
    }

    #[test]
    fn mode_horizontal_copies_left_column() {
        let left = [20u8, 21, 22, 23, 24, 25, 26, 27];
        let ctx = ctx_with([0; 16], left, 0);
        let mut pred = [[0u8; 8]; 8];
        fill_predicted_8x8(&mut pred, &ctx, Intra_8x8_SamplePredMode::Horizontal);
        for y in 0..8 {
            for x in 0..8 {
                assert_eq!(pred[y][x], left[y], "(x={x}, y={y})");
            }
        }
    }

    #[test]
    fn mode_dc_all_available_averages_sixteen() {
        // Sum of 0..=7 = 28 (top) + 28 (left) = 56; (56 + 8) >> 4 = 4.
        let top = [0, 1, 2, 3, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0];
        let left = [0, 1, 2, 3, 4, 5, 6, 7];
        let ctx = ctx_with(top, left, 0);
        let mut pred = [[0u8; 8]; 8];
        fill_predicted_8x8(&mut pred, &ctx, Intra_8x8_SamplePredMode::DC);
        for y in 0..8 {
            for x in 0..8 {
                assert_eq!(pred[y][x], 4);
            }
        }
    }

    #[test]
    fn mode_dc_only_top() {
        let top = [10u8; 16];
        let mut ctx = ctx_with(top, [0; 8], 0);
        ctx.left_available = false;
        ctx.corner_available = false;
        let mut pred = [[0u8; 8]; 8];
        fill_predicted_8x8(&mut pred, &ctx, Intra_8x8_SamplePredMode::DC);
        // sum = 80, (80 + 4) >> 3 = 10.
        for row in pred {
            for v in row {
                assert_eq!(v, 10);
            }
        }
    }

    #[test]
    fn mode_dc_only_left() {
        let left = [20u8; 8];
        let mut ctx = ctx_with([0; 16], left, 0);
        ctx.top_available = false;
        ctx.corner_available = false;
        let mut pred = [[0u8; 8]; 8];
        fill_predicted_8x8(&mut pred, &ctx, Intra_8x8_SamplePredMode::DC);
        for row in pred {
            for v in row {
                assert_eq!(v, 20);
            }
        }
    }

    #[test]
    fn mode_dc_no_neighbors_is_128() {
        let mut ctx = ctx_with([0; 16], [0; 8], 0);
        ctx.top_available = false;
        ctx.left_available = false;
        ctx.corner_available = false;
        let mut pred = [[0u8; 8]; 8];
        fill_predicted_8x8(&mut pred, &ctx, Intra_8x8_SamplePredMode::DC);
        for row in pred {
            for v in row {
                assert_eq!(v, 128);
            }
        }
    }

    #[test]
    fn flat_input_produces_flat_output_all_modes() {
        // If every reference sample is the same constant K, every prediction mode
        // must produce a block filled with K -- the weighted averages and 2-tap
        // means are all identity on a constant, and the diagonal/vertical modes
        // read only from the flat reference row/column.
        const K: u8 = 42;
        let ctx = ctx_with([K; 16], [K; 8], K);

        let modes = [
            Intra_8x8_SamplePredMode::Vertical,
            Intra_8x8_SamplePredMode::Horizontal,
            Intra_8x8_SamplePredMode::DC,
            Intra_8x8_SamplePredMode::Diagonal_Down_Left,
            Intra_8x8_SamplePredMode::Diagonal_Down_Right,
            Intra_8x8_SamplePredMode::Vertical_Right,
            Intra_8x8_SamplePredMode::Horizontal_Down,
            Intra_8x8_SamplePredMode::Vertical_Left,
            Intra_8x8_SamplePredMode::Horizontal_Up,
        ];
        for mode in modes {
            let mut pred = [[0u8; 8]; 8];
            fill_predicted_8x8(&mut pred, &ctx, mode);
            for y in 0..8 {
                for x in 0..8 {
                    assert_eq!(pred[y][x], K, "mode={mode:?} x={x} y={y}");
                }
            }
        }
    }

    #[test]
    fn diagonal_down_left_top_right_corner_uses_edge_formula() {
        // Eq. 8-96: pred[7][7] = (t[14] + 3*t[15] + 2) >> 2.
        let mut top = [0u8; 16];
        top[14] = 0;
        top[15] = 100;
        let ctx = ctx_with(top, [0; 8], 0);
        let mut pred = [[0u8; 8]; 8];
        fill_predicted_8x8(&mut pred, &ctx, Intra_8x8_SamplePredMode::Diagonal_Down_Left);
        // (0 + 300 + 2) >> 2 = 75.
        assert_eq!(pred[7][7], 75);
    }
}
