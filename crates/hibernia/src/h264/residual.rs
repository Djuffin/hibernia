use std::result;

use log::info;
use smallvec::SmallVec;
use wide::i32x4;

use super::{
    macroblock::{self, CodedBlockPattern, MbPredictionMode},
    scaling_list::{weight_scale_8x8_2d, ResolvedScalingMatrix, FLAT_4X4_16, FLAT_8X8_16},
    tables, ColorPlane,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Block4x4 {
    pub samples: [[i32; 4]; 4],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Block8x8 {
    pub samples: [[i32; 8]; 8],
}

impl Default for Block8x8 {
    fn default() -> Self {
        Block8x8 { samples: [[0; 8]; 8] }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Block2x2 {
    pub samples: [[i32; 2]; 2],
}

// 64 zig-zag-order coefficients for a single 8x8 luma residual block.
// Wrapped in a struct because arrays larger than 32 do not auto-derive Default.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LumaLevel8x8(pub [i32; 64]);

impl Default for LumaLevel8x8 {
    fn default() -> Self {
        LumaLevel8x8([0; 64])
    }
}

/// Recycles `Box<Residual>` allocations across macroblocks. Allocating ~3.6 KB
/// per coded MB shows up as ~6% of decode time in flamegraphs; the pool drops
/// that to one allocation per peak high-water mark.
#[derive(Default)]
pub struct ResidualPool {
    boxes: Vec<Box<Residual>>,
}

impl ResidualPool {
    /// Pops a zeroed `Box<Residual>` from the pool, or allocates a fresh one.
    pub fn acquire(&mut self) -> Box<Residual> {
        match self.boxes.pop() {
            Some(mut b) => {
                *b = Residual::default();
                b
            }
            None => Box::default(),
        }
    }

    /// Returns a box to the pool. Drops it on the floor if the pool is full --
    /// bound prevents pathological streams from holding ~MB*frames of state.
    pub fn release(&mut self, b: Box<Residual>) {
        const MAX_POOLED: usize = 8192;
        if self.boxes.len() < MAX_POOLED {
            self.boxes.push(b);
        }
    }
}

/// Luma residual storage. The three coded variants are mutually exclusive,
/// selected by `prediction_mode` (Intra_16x16 vs the rest) and
/// `transform_size_8x8_flag`. Unioning them halves the size of `Residual`.
/// `Empty` is the post-acquire / default state, before the parser has chosen.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum LumaResidual {
    #[default]
    Empty,
    /// Intra_16x16: separate 4x4 DC block and 16 AC blocks of 15 coefficients each.
    Intra16x16 {
        dc: [i32; 16],
        ac: [[i32; 15]; 16],
        ac_nc: [u8; 16],
    },
    /// 4x4 transform (Intra_4x4 or Inter without 8x8 DCT): 16 blocks of 16 coefficients.
    Block4x4 {
        levels: [[i32; 16]; 16],
        nc: [u8; 16],
    },
    /// 8x8 transform (Intra_8x8 or Inter with 8x8 DCT enabled): 4 blocks of 64 coefficients.
    /// CAVLC decodes these as four 4x4 subsections and de-interleaves into
    /// `levels[i8x8][4*i + i4x4]` per Clause 7.3.5.3.1. The per-4x4 NC counts
    /// are kept for CAVLC neighbor-context derivation in subsequent blocks; the
    /// 8x8 inverse transform itself does not consult them.
    Block8x8 {
        levels: [LumaLevel8x8; 4],
        nc: [u8; 16],
    },
}

impl LumaResidual {
    /// Resets to the Intra_16x16 layout with zeroed coefficient arrays.
    pub fn init_intra_16x16(&mut self) {
        *self = LumaResidual::Intra16x16 {
            dc: [0; 16],
            ac: [[0; 15]; 16],
            ac_nc: [0; 16],
        };
    }

    /// Resets to the 4x4-transform layout with zeroed coefficient arrays.
    pub fn init_4x4(&mut self) {
        *self = LumaResidual::Block4x4 { levels: [[0; 16]; 16], nc: [0; 16] };
    }

    /// Resets to the 8x8-transform layout with zeroed coefficient arrays.
    pub fn init_8x8(&mut self) {
        *self = LumaResidual::Block8x8 { levels: [LumaLevel8x8::default(); 4], nc: [0; 16] };
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Residual {
    pub prediction_mode: MbPredictionMode,
    pub coded_block_pattern: CodedBlockPattern,
    // Clause 7.4.5 transform_size_8x8_flag, copied from the parent MB. Selects
    // the 8x8 transform/scaling path for both Intra_8x8 and Inter MBs with
    // 8x8 DCT enabled in the PPS.
    pub transform_size_8x8_flag: bool,

    pub luma: LumaResidual,

    pub chroma_cb_dc_level: [i32; 4],
    pub chroma_cr_dc_level: [i32; 4],

    pub chroma_cb_ac_level: [[i32; 15]; 4],
    pub chroma_cr_ac_level: [[i32; 15]; 4],

    pub chroma_cb_level4x4_nc: [u8; 4],
    pub chroma_cr_level4x4_nc: [u8; 4],
}

impl Residual {
    pub fn get_dc_levels_for(&mut self, plane: ColorPlane) -> &mut [i32] {
        match plane {
            ColorPlane::Y => match &mut self.luma {
                LumaResidual::Intra16x16 { dc, .. } => dc.as_mut_slice(),
                _ => panic!("No separate DC levels in this prediction mode"),
            },
            ColorPlane::Cb => self.chroma_cb_dc_level.as_mut_slice(),
            ColorPlane::Cr => self.chroma_cr_dc_level.as_mut_slice(),
        }
    }

    pub fn get_ac_levels_for(&mut self, blk_idx: u8, plane: ColorPlane) -> (&mut [i32], &mut u8) {
        let blk_idx = blk_idx as usize;
        match plane {
            ColorPlane::Y => match &mut self.luma {
                LumaResidual::Intra16x16 { ac, ac_nc, .. } => {
                    (ac[blk_idx].as_mut_slice(), &mut ac_nc[blk_idx])
                }
                LumaResidual::Block4x4 { levels, nc } => {
                    (levels[blk_idx].as_mut_slice(), &mut nc[blk_idx])
                }
                _ => panic!("get_ac_levels_for(Y): luma residual is not 4x4 or Intra_16x16"),
            },
            ColorPlane::Cb => (
                self.chroma_cb_ac_level[blk_idx].as_mut_slice(),
                &mut self.chroma_cb_level4x4_nc[blk_idx],
            ),
            ColorPlane::Cr => (
                self.chroma_cr_ac_level[blk_idx].as_mut_slice(),
                &mut self.chroma_cr_level4x4_nc[blk_idx],
            ),
        }
    }

    // Calculates nC for the block withing the macroblock
    pub fn get_nc(&self, blk_idx: u8, plane: ColorPlane) -> u8 {
        let blk_idx = blk_idx as usize;
        match plane {
            ColorPlane::Y => match &self.luma {
                LumaResidual::Intra16x16 { ac_nc, .. } => ac_nc[blk_idx],
                LumaResidual::Block4x4 { nc, .. } => nc[blk_idx],
                LumaResidual::Block8x8 { nc, .. } => nc[blk_idx],
                LumaResidual::Empty => 0,
            },

            ColorPlane::Cb => self.chroma_cb_level4x4_nc[blk_idx],
            ColorPlane::Cr => self.chroma_cr_level4x4_nc[blk_idx],
        }
    }

    pub fn has_separate_luma_dc(&self) -> bool {
        self.prediction_mode == MbPredictionMode::Intra_16x16
    }

    pub fn restore(
        &self,
        plane: ColorPlane,
        qp: u8,
        scaling: &ResolvedScalingMatrix,
    ) -> SmallVec<[Block4x4; 16]> {
        let mut result = SmallVec::new();
        let is_inter = self.prediction_mode.is_inter();

        if plane == ColorPlane::Y {
            let weight_scale_4x4 = scaling.list_4x4(is_inter, ColorPlane::Y);
            match &self.luma {
                LumaResidual::Intra16x16 { dc, ac, .. } => {
                    // Section 8.5.2 Specification of transform decoding process for luma samples
                    // of Intra_16x16 macroblock prediction mode
                    let mut dcs_block = unzip_block_4x4(dc);
                    dcs_block = transform_dc(&dcs_block);
                    dc_scale_4x4_block(&mut dcs_block, weight_scale_4x4[0], qp);

                    for blk_idx in 0..16 {
                        let mut coeffs = [0i32; 16];
                        let (dc_row, dc_column) = unscan_4x4(blk_idx);
                        coeffs[0] = dcs_block.samples[dc_row][dc_column];
                        coeffs[1..].copy_from_slice(&ac[blk_idx]);
                        level_scale_4x4_block(&mut coeffs, weight_scale_4x4, true, qp);
                        let mut block = unzip_block_4x4(&coeffs);
                        transform_4x4(&mut block);
                        result.push(block);
                    }
                }
                LumaResidual::Block8x8 { levels, .. } => {
                    // Section 8.5.13 Scaling and transformation process for residual
                    // 8x8 blocks. Used for Intra_8x8 and for Inter MBs when the PPS
                    // enables the 8x8 transform. Reconstruct each 8x8 block and split
                    // it into four Block4x4 in the renderer's expected sub-order
                    // (0,0), (4,0), (0,4), (4,4) so the existing 4x4 residual-add
                    // loop works.
                    let weight_scale_8x8 =
                        weight_scale_8x8_2d(scaling.list_8x8(is_inter, ColorPlane::Y));
                    for i8x8 in 0..4 {
                        let mut block = unzip_block_8x8(&levels[i8x8].0);
                        level_scale_8x8_block(&mut block, &weight_scale_8x8, qp);
                        transform_8x8(&mut block);
                        for (sub_y, sub_x) in [(0, 0), (0, 4), (4, 0), (4, 4)] {
                            let mut sub = Block4x4::default();
                            for y in 0..4 {
                                for x in 0..4 {
                                    sub.samples[y][x] = block.samples[sub_y + y][sub_x + x];
                                }
                            }
                            result.push(sub);
                        }
                    }
                }
                LumaResidual::Block4x4 { levels, .. } => {
                    for blk_idx in 0..16 {
                        let mut coeffs = [0i32; 16];
                        coeffs.copy_from_slice(&levels[blk_idx]);
                        level_scale_4x4_block(&mut coeffs, weight_scale_4x4, false, qp);
                        let mut block = unzip_block_4x4(&coeffs);
                        transform_4x4(&mut block);
                        result.push(block);
                    }
                }
                LumaResidual::Empty => {
                    // No luma residual data -- leave `result` empty so the caller
                    // adds nothing on top of the prediction.
                }
            }
        } else {
            // Section 8.5.8, 8.5.11 Specification of transform decoding process for chroma samples
            let weight_scale_4x4 = scaling.list_4x4(is_inter, plane);
            let dcs = match plane {
                ColorPlane::Cb => &self.chroma_cb_dc_level,
                ColorPlane::Cr => &self.chroma_cr_dc_level,
                _ => unreachable!(),
            };
            let mut dcs_block = Block2x2 { samples: [[dcs[0], dcs[1]], [dcs[2], dcs[3]]] };
            dcs_block = transform_chroma_dc(&dcs_block);
            dc_scale_2x2_block(&mut dcs_block, weight_scale_4x4[0], qp);

            for blk_idx in 0..4 {
                let acs = match plane {
                    ColorPlane::Cb => &self.chroma_cb_ac_level[blk_idx],
                    ColorPlane::Cr => &self.chroma_cr_ac_level[blk_idx],
                    _ => unreachable!(),
                };
                let mut coeffs = [0i32; 16];
                let (dc_row, dc_column) = unscan_2x2(blk_idx);
                coeffs[0] = dcs_block.samples[dc_row][dc_column];
                coeffs[1..].copy_from_slice(acs);
                level_scale_4x4_block(&mut coeffs, weight_scale_4x4, true, qp);
                let mut block = unzip_block_4x4(&coeffs);
                transform_4x4(&mut block);
                result.push(block);
            }
        }

        result
    }
}

// Figure 8-6 - Assignment of the indices of dcY to luma4x4BlkIdx
#[inline]
pub const fn unscan_4x4(idx: usize) -> (/* row */ usize, /* column */ usize) {
    const TABLE: [(usize, usize); 16] = [
        (0, 0),
        (0, 1),
        (1, 0),
        (1, 1),
        (0, 2),
        (0, 3),
        (1, 2),
        (1, 3),
        (2, 0),
        (2, 1),
        (3, 0),
        (3, 1),
        (2, 2),
        (2, 3),
        (3, 2),
        (3, 3),
    ];
    TABLE[idx]
}

#[inline]
pub const fn scan_4x4(row: usize, col: usize) -> usize {
    (row / 2) * 8 + (col / 2) * 4 + (row % 2) * 2 + (col % 2)
}

// Figure 8-7 - Assignment of the indices of dcC to chroma4x4BlkIdx
#[inline]
pub const fn unscan_2x2(idx: usize) -> (/* row */ usize, /* column */ usize) {
    const TABLE: [(usize, usize); 4] = [(0, 0), (0, 1), (1, 0), (1, 1)];
    TABLE[idx]
}

// Table 8-13 - Specification of mapping of idx to Cij for zig-zag scan
#[inline]
pub const fn un_zig_zag_4x4(idx: usize) -> (/* row */ usize, /* column */ usize) {
    const TABLE: [(usize, usize); 16] = [
        (0, 0),
        (0, 1),
        (1, 0),
        (2, 0),
        (1, 1),
        (0, 2),
        (0, 3),
        (1, 2),
        (2, 1),
        (3, 0),
        (3, 1),
        (2, 2),
        (1, 3),
        (2, 3),
        (3, 2),
        (3, 3),
    ];
    TABLE[idx]
}

// Get index of zig-zag walk for a given coordinates in 4x4 block
#[inline]
pub const fn zig_zag_4x4(row: usize, column: usize) -> usize {
    const TABLE: [usize; 16] = [0, 1, 5, 6, 2, 4, 7, 12, 3, 8, 11, 13, 9, 10, 14, 15];
    TABLE[row * 4 + column]
}

#[inline(always)]
const fn norm_adjust_4x4(m: u8, idx: usize) -> u8 {
    const IDX_TO_V_COLUMN: [u8; 16] = [0, 2, 2, 0, 1, 0, 2, 2, 2, 2, 1, 0, 1, 2, 2, 1];
    const V: [[u8; 3]; 6] =
        [[10, 16, 13], [11, 18, 14], [13, 20, 16], [14, 23, 18], [16, 25, 20], [18, 29, 23]];
    V[m as usize][IDX_TO_V_COLUMN[idx] as usize]
}

// Section 8.5.9 Derivation process for scaling functions. `weight_scale` is
// the value of the zig-zag-ordered scaling list at position `idx` (equivalently
// `weightScale4x4[i][j]` from the spec, after inverse scan).
#[inline(always)]
pub const fn level_scale_4x4(weight_scale: u8, m: u8, idx: usize) -> i32 {
    (weight_scale as i32) * (norm_adjust_4x4(m, idx) as i32)
}

// Section 8.5.12.1 Scaling process for residual 4x4 blocks. `weight_scale` is
// the 16-element, zig-zag-ordered scaling list active for this block.
#[inline(always)]
pub fn level_scale_4x4_block(
    block: &mut [i32],
    weight_scale: &[u8; 16],
    skip_dc: bool,
    qp: u8,
) {
    debug_assert!(block.len() == 16);
    let m = qp % 6;

    // Per-position scaling factors keyed on (weight_scale[i], m, i). Held as
    // four i32x4 rows aligned with the block's row layout below.
    let s0 = i32x4::new([
        level_scale_4x4(weight_scale[0], m, 0),
        level_scale_4x4(weight_scale[1], m, 1),
        level_scale_4x4(weight_scale[2], m, 2),
        level_scale_4x4(weight_scale[3], m, 3),
    ]);
    let s1 = i32x4::new([
        level_scale_4x4(weight_scale[4], m, 4),
        level_scale_4x4(weight_scale[5], m, 5),
        level_scale_4x4(weight_scale[6], m, 6),
        level_scale_4x4(weight_scale[7], m, 7),
    ]);
    let s2 = i32x4::new([
        level_scale_4x4(weight_scale[8], m, 8),
        level_scale_4x4(weight_scale[9], m, 9),
        level_scale_4x4(weight_scale[10], m, 10),
        level_scale_4x4(weight_scale[11], m, 11),
    ]);
    let s3 = i32x4::new([
        level_scale_4x4(weight_scale[12], m, 12),
        level_scale_4x4(weight_scale[13], m, 13),
        level_scale_4x4(weight_scale[14], m, 14),
        level_scale_4x4(weight_scale[15], m, 15),
    ]);

    let mut b0 = i32x4::new(block[0..4].try_into().unwrap());
    let mut b1 = i32x4::new(block[4..8].try_into().unwrap());
    let mut b2 = i32x4::new(block[8..12].try_into().unwrap());
    let mut b3 = i32x4::new(block[12..16].try_into().unwrap());

    // Preserve DC for `skip_dc` callers that already ran `dc_scale_4x4_block`.
    let dc_val = block[0];

    if qp >= 24 {
        // Equation 8-336: d = (c * scale) << (qp / 6 - 4)
        let shift = i32x4::splat((qp / 6 - 4) as i32);
        b0 = (b0 * s0) << shift;
        b1 = (b1 * s1) << shift;
        b2 = (b2 * s2) << shift;
        b3 = (b3 * s3) << shift;
    } else {
        // Equation 8-337: d = (c * scale + (1 << (3 - qp / 6))) >> (4 - qp / 6)
        let offset = i32x4::splat(1 << (3 - qp / 6));
        let shift = i32x4::splat((4 - qp / 6) as i32);
        b0 = (b0 * s0 + offset) >> shift;
        b1 = (b1 * s1 + offset) >> shift;
        b2 = (b2 * s2 + offset) >> shift;
        b3 = (b3 * s3 + offset) >> shift;
    }

    // Write back the scaled values to the original block array.
    let a0 = b0.to_array();
    block[0] = if skip_dc { dc_val } else { a0[0] };
    block[1] = a0[1];
    block[2] = a0[2];
    block[3] = a0[3];
    block[4..8].copy_from_slice(&b1.to_array());
    block[8..12].copy_from_slice(&b2.to_array());
    block[12..16].copy_from_slice(&b3.to_array());
}

// Section 8.5.10 Scaling and transformation process for DC transform coefficients for Intra_16x16.
// `weight_scale_dc` is position 0 of the active 4x4 scaling list.
#[inline(always)]
pub fn dc_scale_4x4_block(block: &mut Block4x4, weight_scale_dc: u8, qp: u8) {
    let m = qp % 6;
    for row in block.samples.iter_mut() {
        for c in row.iter_mut() {
            let d = if qp >= 36 {
                // Equation 8-321
                (*c * level_scale_4x4(weight_scale_dc, m, 0)) << (qp / 6 - 6)
            } else {
                // Equation 8-322
                (*c * level_scale_4x4(weight_scale_dc, m, 0) + (1 << (5 - qp / 6))) >> (6 - qp / 6)
            };
            *c = d;
        }
    }
}

// Section 8.5.11.2 Scaling process for chroma DC transform coefficients
// (the 2x2 transform itself lives in 8.5.11.1 / `transform_chroma_dc`).
// `weight_scale_dc` is position 0 of the active (Intra or Inter) chroma 4x4 scaling list.
#[inline(always)]
pub fn dc_scale_2x2_block(block: &mut Block2x2, weight_scale_dc: u8, qp: u8) {
    let m = qp % 6;
    for row in block.samples.iter_mut() {
        for c in row.iter_mut() {
            // Equation 8-326.
            let d = ((*c * level_scale_4x4(weight_scale_dc, m, 0)) << (qp / 6)) >> 5;
            *c = d;
        }
    }
}

// Section 8.5.10 Scaling and transformation process for DC transform coefficients for Intra_16x16
#[inline(always)]
pub fn transform_dc(block: &Block4x4) -> Block4x4 {
    let b = &block.samples;
    let mut result = Block4x4::default();
    let r = &mut result.samples;

    /*
    Equation 8-320 (Inverse Hadamard Transform)
    r = [ 1  1  1  1 ]   [ b00 b01 b02 b03 ]   [ 1  1  1  1 ]
        [ 1  1 -1 -1 ] * [ b10 b11 b12 b13 ] * [ 1  1 -1 -1 ]
        [ 1 -1 -1  1 ]   [ b20 b21 b22 b23 ]   [ 1 -1 -1  1 ]
        [ 1 -1  1 -1 ]   [ b30 b31 b32 b33 ]   [ 1 -1  1 -1 ]
    */

    let row0 = i32x4::new(b[0]);
    let row1 = i32x4::new(b[1]);
    let row2 = i32x4::new(b[2]);
    let row3 = i32x4::new(b[3]);

    let [mut c0, mut c1, mut c2, mut c3] = i32x4::transpose([row0, row1, row2, row3]);

    // First pass: 1D transform on the columns.
    let f0 = c0 + c1 + c2 + c3;
    let f1 = c0 + c1 - c2 - c3;
    let f2 = c0 - c1 - c2 + c3;
    let f3 = c0 - c1 + c2 - c3;

    c0 = f0;
    c1 = f1;
    c2 = f2;
    c3 = f3;

    let [r0, r1, r2, r3] = i32x4::transpose([c0, c1, c2, c3]);

    // Second pass: 1D transform on the rows.
    let h0 = r0 + r1 + r2 + r3;
    let h1 = r0 + r1 - r2 - r3;
    let h2 = r0 - r1 - r2 + r3;
    let h3 = r0 - r1 + r2 - r3;

    r[0] = h0.to_array();
    r[1] = h1.to_array();
    r[2] = h2.to_array();
    r[3] = h3.to_array();

    result
}

// Section 8.5.11.1 Transformation process for chroma DC transform coefficients
#[inline(always)]
pub fn transform_chroma_dc(block: &Block2x2) -> Block2x2 {
    let c = &block.samples;

    // This is a 2x2 Hadamard transform: f = H * c * H
    // Equation 8-324
    // H =  [1  1]
    //      [1 -1]
    let hc00 = c[0][0] + c[1][0];
    let hc01 = c[0][1] + c[1][1];
    let hc10 = c[0][0] - c[1][0];
    let hc11 = c[0][1] - c[1][1];

    Block2x2 { samples: [[hc00 + hc01, hc00 - hc01], [hc10 + hc11, hc10 - hc11]] }
}

#[inline(always)]
pub fn unzip_block_4x4(block: &[i32]) -> Block4x4 {
    assert_eq!(block.len(), 16);
    let mut result = Block4x4::default();
    for (idx, value) in block.iter().enumerate() {
        let (row, column) = un_zig_zag_4x4(idx);
        result.samples[row][column] = *value;
    }
    result
}

#[inline(always)]
pub fn unscan_block_4x4(block: &[i32]) -> Block4x4 {
    let mut result = Block4x4::default();
    for (idx, value) in block.iter().enumerate() {
        let (row, column) = unscan_4x4(idx);
        result.samples[row][column] = *value;
    }
    result
}

// Section 8.5.12.2 Transformation process for residual 4x4 blocks
#[inline(always)]
pub fn transform_4x4(block: &mut Block4x4) {
    let d = &mut block.samples;

    let row0 = i32x4::new(d[0]);
    let row1 = i32x4::new(d[1]);
    let row2 = i32x4::new(d[2]);
    let row3 = i32x4::new(d[3]);

    // The 4x4 inverse integer transform is separable. Run the column pass on
    // a transposed block so the SIMD lanes line up with columns, then
    // transpose back for the row pass.
    let [mut c0, mut c1, mut c2, mut c3] = i32x4::transpose([row0, row1, row2, row3]);

    // Eq 8-338..8-345 across the 4 columns.
    let e0 = c0 + c2;
    let e1 = c0 - c2;
    let e2 = (c1 >> 1) - c3;
    let e3 = c1 + (c3 >> 1);

    c0 = e0 + e3;
    c1 = e1 + e2;
    c2 = e1 - e2;
    c3 = e0 - e3;

    let [r0, r1, r2, r3] = i32x4::transpose([c0, c1, c2, c3]);

    // Eq 8-346..8-353 across the 4 rows.
    let g0 = r0 + r2;
    let g1 = r0 - r2;
    let g2 = (r1 >> 1) - r3;
    let g3 = r1 + (r3 >> 1);

    let h0 = g0 + g3;
    let h1 = g1 + g2;
    let h2 = g1 - g2;
    let h3 = g0 - g3;

    // Final +32 round, >>6 per spec.
    let h0_final: i32x4 = (h0 + i32x4::splat(32)) >> 6;
    let h1_final: i32x4 = (h1 + i32x4::splat(32)) >> 6;
    let h2_final: i32x4 = (h2 + i32x4::splat(32)) >> 6;
    let h3_final: i32x4 = (h3 + i32x4::splat(32)) >> 6;

    d[0] = h0_final.to_array();
    d[1] = h1_final.to_array();
    d[2] = h2_final.to_array();
    d[3] = h3_final.to_array();
}

// Table 8-14 -- 8x8 inverse zig-zag (frame) scan. Maps idx in 0..64 to (row, col).
// Field scan (for MBAFF / field-coded pictures) is not implemented here; the
// CABAC parser rejects field/MBAFF before it gets this far.
#[inline]
pub const fn un_zig_zag_8x8(idx: usize) -> (/* row */ usize, /* col */ usize) {
    const TABLE: [(usize, usize); 64] = [
        (0, 0), (0, 1), (1, 0), (2, 0), (1, 1), (0, 2), (0, 3), (1, 2),
        (2, 1), (3, 0), (4, 0), (3, 1), (2, 2), (1, 3), (0, 4), (0, 5),
        (1, 4), (2, 3), (3, 2), (4, 1), (5, 0), (6, 0), (5, 1), (4, 2),
        (3, 3), (2, 4), (1, 5), (0, 6), (0, 7), (1, 6), (2, 5), (3, 4),
        (4, 3), (5, 2), (6, 1), (7, 0), (7, 1), (6, 2), (5, 3), (4, 4),
        (3, 5), (2, 6), (1, 7), (2, 7), (3, 6), (4, 5), (5, 4), (6, 3),
        (7, 2), (7, 3), (6, 4), (5, 5), (4, 6), (3, 7), (4, 7), (5, 6),
        (6, 5), (7, 4), (7, 5), (6, 6), (5, 7), (6, 7), (7, 6), (7, 7),
    ];
    TABLE[idx]
}

#[inline]
pub fn unzip_block_8x8(block: &[i32; 64]) -> Block8x8 {
    let mut result = Block8x8::default();
    for (idx, v) in block.iter().enumerate() {
        let (row, col) = un_zig_zag_8x8(idx);
        result.samples[row][col] = *v;
    }
    result
}

// Eq. 8-318 -- V matrix for 8x8 inverse quantization. Row = qp % 6 (m). Column =
// position class (vm0..vm5). Column assignment per Eq. 8-317 partitions all 64
// (i, j) positions into 6 classes based on (i mod 4, j mod 4) / (i mod 2, j mod 2).
#[inline]
const fn norm_adjust_8x8(m: u8, i: usize, j: usize) -> u8 {
    const V: [[u8; 6]; 6] = [
        [20, 18, 32, 19, 25, 24],
        [22, 19, 35, 21, 28, 26],
        [26, 23, 42, 24, 33, 31],
        [28, 25, 45, 26, 35, 33],
        [32, 28, 51, 30, 40, 38],
        [36, 32, 58, 34, 46, 43],
    ];
    // Match Eq. 8-317 classification. Order matters: vm0/vm2 (mod-4 diagonals)
    // take priority over vm1 (mod-2 odd-odd), which takes priority over vm3/vm4.
    let class: usize = if i % 4 == 0 && j % 4 == 0 {
        0
    } else if i % 4 == 2 && j % 4 == 2 {
        2
    } else if i % 2 == 1 && j % 2 == 1 {
        1
    } else if (i % 4 == 0 && j % 4 == 2) || (i % 4 == 2 && j % 4 == 0) {
        4
    } else if (i % 4 == 0 && j % 2 == 1) || (i % 2 == 1 && j % 4 == 0) {
        3
    } else {
        5
    };
    V[m as usize][class]
}

// Eq. 8-316: LevelScale8x8(m, i, j) = weightScale8x8(i, j) * normAdjust8x8(m, i, j).
// `weight_scale` is `weightScale8x8[i][j]` from the spec (the 2D form, after
// inverse zig-zag from the stored 1D scaling list).
#[inline]
pub const fn level_scale_8x8(weight_scale: u8, m: u8, i: usize, j: usize) -> i32 {
    (weight_scale as i32) * (norm_adjust_8x8(m, i, j) as i32)
}

// Section 8.5.13.1 Scaling process for residual 8x8 blocks (Eqs. 8-356, 8-357).
// `weight_scale` is the 8x8 active scaling list in 2D form.
pub fn level_scale_8x8_block(block: &mut Block8x8, weight_scale: &[[u8; 8]; 8], qp: u8) {
    let m = qp % 6;
    if qp >= 36 {
        let shift = (qp / 6 - 6) as i32;
        for i in 0..8 {
            for j in 0..8 {
                block.samples[i][j] =
                    (block.samples[i][j] * level_scale_8x8(weight_scale[i][j], m, i, j)) << shift;
            }
        }
    } else {
        let shift = (6 - qp / 6) as i32;
        let offset = 1 << (5 - qp / 6);
        for i in 0..8 {
            for j in 0..8 {
                block.samples[i][j] = ((block.samples[i][j]
                    * level_scale_8x8(weight_scale[i][j], m, i, j))
                    + offset)
                    >> shift;
            }
        }
    }
}

// Section 8.5.13.2 Transformation process for residual 8x8 blocks. 1D 8-point
// inverse integer transform (Eqs. 8-358 through 8-381), applied horizontally
// then vertically (Eqs. 8-382 through 8-405 are the same equations for columns).
#[inline(always)]
fn transform_1d_8(d: [i32; 8]) -> [i32; 8] {
    // Eqs. 8-358 to 8-365
    let e = [
        d[0] + d[4],
        -d[3] + d[5] - d[7] - (d[7] >> 1),
        d[0] - d[4],
        d[1] + d[7] - d[3] - (d[3] >> 1),
        (d[2] >> 1) - d[6],
        -d[1] + d[7] + d[5] + (d[5] >> 1),
        d[2] + (d[6] >> 1),
        d[3] + d[5] + d[1] + (d[1] >> 1),
    ];
    // Eqs. 8-366 to 8-373
    let f = [
        e[0] + e[6],
        e[1] + (e[7] >> 2),
        e[2] + e[4],
        e[3] + (e[5] >> 2),
        e[2] - e[4],
        (e[3] >> 2) - e[5],
        e[0] - e[6],
        e[7] - (e[1] >> 2),
    ];
    // Eqs. 8-374 to 8-381
    [
        f[0] + f[7],
        f[2] + f[5],
        f[4] + f[3],
        f[6] + f[1],
        f[6] - f[1],
        f[4] - f[3],
        f[2] - f[5],
        f[0] - f[7],
    ]
}

pub fn transform_8x8(block: &mut Block8x8) {
    // Row pass (horizontal 1D).
    for i in 0..8 {
        block.samples[i] = transform_1d_8(block.samples[i]);
    }
    // Column pass (vertical 1D): transpose, apply, transpose back.
    let mut transposed = [[0i32; 8]; 8];
    for i in 0..8 {
        for j in 0..8 {
            transposed[j][i] = block.samples[i][j];
        }
    }
    for j in 0..8 {
        transposed[j] = transform_1d_8(transposed[j]);
    }
    // Eq. 8-406: final sample = (m + 32) >> 6.
    for i in 0..8 {
        for j in 0..8 {
            block.samples[i][j] = (transposed[j][i] + 32) >> 6;
        }
    }
}

/// Adds a 4x4 residual block to a flat sample buffer at `origin` with row stride
/// `stride`, saturating to u8 range. Section 8.5.14: residuals are added to
/// predicted samples after inverse transform/scaling.
#[inline]
pub(super) fn add_residual_4x4(buf: &mut [u8], origin: usize, stride: usize, residual: &Block4x4) {
    for y in 0..4 {
        let row_base = origin + y * stride;
        for x in 0..4 {
            let v = buf[row_base + x] as i32 + residual.samples[y][x];
            buf[row_base + x] = v.clamp(0, 255) as u8;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_norm_adjust_4x4() {
        let m_to_v = |m| match m {
            0 => [[10, 13, 10, 13], [13, 16, 13, 16], [10, 13, 10, 13], [13, 16, 13, 16]],
            1 => [[11, 14, 11, 14], [14, 18, 14, 18], [11, 14, 11, 14], [14, 18, 14, 18]],
            2 => [[13, 16, 13, 16], [16, 20, 16, 20], [13, 16, 13, 16], [16, 20, 16, 20]],
            3 => [[14, 18, 14, 18], [18, 23, 18, 23], [14, 18, 14, 18], [18, 23, 18, 23]],
            4 => [[16, 20, 16, 20], [20, 25, 20, 25], [16, 20, 16, 20], [20, 25, 20, 25]],
            5 => [[18, 23, 18, 23], [23, 29, 23, 29], [18, 23, 18, 23], [23, 29, 23, 29]],
            _ => panic!(),
        };

        for m in 0..6 {
            for i in 0..4 {
                for j in 0..4 {
                    let idx = zig_zag_4x4(i, j);
                    let v = m_to_v(m)[i][j];
                    assert_eq!(v, norm_adjust_4x4(m, idx));
                }
            }
        }
    }

    #[test]
    pub fn test_zig_zag() {
        for i in 0..4 {
            for j in 0..4 {
                let idx = zig_zag_4x4(i, j);
                assert!(idx < 16);
                let (y, x) = un_zig_zag_4x4(idx);
                assert_eq!((i, j), (y, x));
            }
        }
    }

    pub fn test_transform_4x4(input: Block4x4, expected: Block4x4, qp: u8) {
        let mut block = [0i32; 16];
        for i in 0..4 {
            for j in 0..4 {
                block[zig_zag_4x4(i, j)] = input.samples[i][j];
            }
        }

        level_scale_4x4_block(&mut block, &FLAT_4X4_16, false, qp);
        let mut output = unzip_block_4x4(&block);
        transform_4x4(&mut output);
        assert_eq!(output.samples, expected.samples);
    }

    #[test]
    pub fn test_transform_4x4_zeros() {
        test_transform_4x4(
            Block4x4 { samples: [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]] },
            Block4x4 { samples: [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]] },
            28,
        );
    }

    #[test]
    pub fn test_transform_4x4_ex1() {
        test_transform_4x4(
            Block4x4 { samples: [[192, -5, 3, -6], [-4, 5, -3, -8], [-3, 0, 3, 3], [1, 6, 0, 0]] },
            Block4x4 {
                samples: [[58, 63, 51, 59], [53, 64, 57, 66], [62, 63, 60, 64], [59, 52, 63, 68]],
            },
            6,
        );
    }

    #[test]
    pub fn test_transform_4x4_ex2() {
        test_transform_4x4(
            Block4x4 { samples: [[96, -2, 1, -3], [-2, 3, -2, -4], [-1, 0, 1, 1], [0, 3, 0, 0]] },
            Block4x4 {
                samples: [[57, 65, 51, 57], [53, 64, 57, 65], [62, 62, 59, 63], [59, 53, 64, 69]],
            },
            12,
        );
    }

    #[test]
    pub fn test_transform_4x4_ex3() {
        test_transform_4x4(
            Block4x4 { samples: [[48, -1, 0, -1], [-1, 1, -1, -2], [0, 0, 0, 0], [0, 1, 0, 0]] },
            Block4x4 {
                samples: [[55, 66, 54, 58], [54, 62, 58, 63], [61, 59, 61, 62], [60, 55, 65, 67]],
            },
            18,
        );
    }

    #[test]
    pub fn test_transform_4x4_ex4() {
        test_transform_4x4(
            Block4x4 { samples: [[12, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]] },
            Block4x4 {
                samples: [[60, 60, 60, 60], [60, 60, 60, 60], [60, 60, 60, 60], [60, 60, 60, 60]],
            },
            30,
        );
    }

    #[test]
    pub fn test_unscan_block_4x4() {
        let input = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let block =
            Block4x4 { samples: [[0, 1, 4, 5], [2, 3, 6, 7], [8, 9, 12, 13], [10, 11, 14, 15]] };
        assert_eq!(block, unscan_block_4x4(&input));
    }

    #[test]
    pub fn test_scan_unscan_4x4() {
        for i in 0..16 {
            let (r, c) = unscan_4x4(i);
            assert_eq!(scan_4x4(r, c), i);
        }
    }

    #[test]
    pub fn test_un_zig_zag_8x8_bijection() {
        let mut seen = [false; 64];
        for idx in 0..64 {
            let (r, c) = un_zig_zag_8x8(idx);
            assert!(r < 8 && c < 8);
            let flat = r * 8 + c;
            assert!(!seen[flat], "duplicate mapping at idx {idx} -> ({r},{c})");
            seen[flat] = true;
        }
        assert!(seen.iter().all(|b| *b));
    }

    #[test]
    pub fn test_norm_adjust_8x8_class_counts() {
        // Partition sanity: the 6 weight classes must cover all 64 positions
        // exactly once. Verify by counting distinct values per m=0; since the
        // V row at m=0 is [20, 18, 32, 19, 25, 24] (all distinct), each class
        // yields a distinct weight and the count per weight equals the class size.
        let mut counts = std::collections::HashMap::new();
        for i in 0..8 {
            for j in 0..8 {
                *counts.entry(norm_adjust_8x8(0, i, j)).or_insert(0usize) += 1;
            }
        }
        assert_eq!(counts[&20], 4); // vm0 class: (0,0), (0,4), (4,0), (4,4)
        assert_eq!(counts[&18], 16); // vm1: both odd
        assert_eq!(counts[&32], 4); // vm2: (2,2), (2,6), (6,2), (6,6)
        assert_eq!(counts[&19], 16); // vm3
        assert_eq!(counts[&25], 8); // vm4
        assert_eq!(counts[&24], 16); // vm5 (remainder)
    }

    #[test]
    pub fn test_transform_8x8_zeros() {
        let mut block = Block8x8::default();
        let w = weight_scale_8x8_2d(&FLAT_8X8_16);
        level_scale_8x8_block(&mut block, &w, 20);
        transform_8x8(&mut block);
        assert_eq!(block.samples, [[0i32; 8]; 8]);
    }

    #[test]
    pub fn test_transform_1d_8_dc_only() {
        // 1D transform of [x, 0, 0, 0, 0, 0, 0, 0] should produce [x; 8].
        let out = super::transform_1d_8([5, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(out, [5, 5, 5, 5, 5, 5, 5, 5]);
    }

    #[test]
    pub fn test_transform_8x8_dc_only_flat() {
        // 2D transform of a DC-only block produces a flat block whose value
        // is the DC after the final (v + 32) >> 6 shift.
        let mut block = Block8x8::default();
        block.samples[0][0] = 512; // deliberately chosen so (512 + 32) >> 6 = 8
        transform_8x8(&mut block);
        for row in block.samples {
            for v in row {
                assert_eq!(v, 8);
            }
        }
    }

    #[test]
    pub fn test_level_scale_8x8_qp_below_36() {
        // qp=24, m=0. For (0,0): normAdjust=20, scale=16*20=320. Expected:
        // (c * 320 + 2^(5 - 4)) >> (6 - 4) = (c * 320 + 2) >> 2.
        let mut block = Block8x8::default();
        block.samples[0][0] = 3;
        let w = weight_scale_8x8_2d(&FLAT_8X8_16);
        level_scale_8x8_block(&mut block, &w, 24);
        assert_eq!(block.samples[0][0], (3 * 320 + 2) >> 2);
    }

    #[test]
    pub fn test_unzip_block_8x8_respects_zigzag() {
        // The CAVLC-de-interleaved input is a flat [i32; 64] in zig-zag order,
        // and unzip_block_8x8 must place index idx at position un_zig_zag_8x8(idx).
        let mut input = [0i32; 64];
        for i in 0..64 {
            input[i] = i as i32 + 1;
        }
        let block = unzip_block_8x8(&input);
        for idx in 0..64 {
            let (r, c) = un_zig_zag_8x8(idx);
            assert_eq!(block.samples[r][c], input[idx], "idx={idx}");
        }
    }

    #[test]
    pub fn test_level_scale_8x8_qp_ge_36() {
        // qp=42, m=0 (42%6=0). normAdjust(0,0,0)=20, scale=320.
        // Expected: (c * 320) << (7 - 6) = c * 320 * 2.
        let mut block = Block8x8::default();
        block.samples[0][0] = 3;
        let w = weight_scale_8x8_2d(&FLAT_8X8_16);
        level_scale_8x8_block(&mut block, &w, 42);
        assert_eq!(block.samples[0][0], 3 * 320 * 2);
    }

    #[test]
    pub fn test_level_scale_4x4_custom_weight_scale() {
        // Flat_4x4_16 at qp=24 gives the same result as before (scaling=16*norm).
        // A custom list of all 32s should exactly double the output vs flat.
        let block_flat = {
            let mut b = [0i32; 16];
            for i in 0..16 {
                b[i] = 10 + i as i32;
            }
            level_scale_4x4_block(&mut b, &FLAT_4X4_16, false, 24);
            b
        };
        let block_doubled = {
            let mut b = [0i32; 16];
            for i in 0..16 {
                b[i] = 10 + i as i32;
            }
            let doubled = [32u8; 16];
            level_scale_4x4_block(&mut b, &doubled, false, 24);
            b
        };
        for i in 0..16 {
            assert_eq!(block_doubled[i], block_flat[i] * 2, "idx {i}");
        }
    }

    #[test]
    pub fn test_level_scale_8x8_custom_weight_scale() {
        // Scaling list of all 32s doubles vs flat 16 at the same qp.
        let flat_block = {
            let mut block = Block8x8::default();
            for i in 0..8 {
                for j in 0..8 {
                    block.samples[i][j] = 5;
                }
            }
            let w = weight_scale_8x8_2d(&FLAT_8X8_16);
            level_scale_8x8_block(&mut block, &w, 24);
            block
        };
        let doubled_block = {
            let mut block = Block8x8::default();
            for i in 0..8 {
                for j in 0..8 {
                    block.samples[i][j] = 5;
                }
            }
            let list = [32u8; 64];
            let w = weight_scale_8x8_2d(&list);
            level_scale_8x8_block(&mut block, &w, 24);
            block
        };
        for i in 0..8 {
            for j in 0..8 {
                assert_eq!(
                    doubled_block.samples[i][j],
                    flat_block.samples[i][j] * 2,
                    "({i},{j})"
                );
            }
        }
    }
}
