use std::result;

use super::{
    macroblock::{self, CodedBlockPattern, MbPredictionMode},
    tables, ColorPlane,
};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Block4x4 {
    pub samples: [[i32; 4]; 4],
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Residual {
    pub prediction_mode: MbPredictionMode,
    pub coded_block_pattern: CodedBlockPattern,
    pub qp: i32,

    pub dc_level16x16: [i32; 16],
    pub ac_level16x16: [[i32; 15]; 16],
    pub ac_level16x16_nc: [u8; 16],
    pub luma_level4x4: [[i32; 16]; 16],
    pub luma_level4x4_nc: [u8; 16],

    pub chroma_cb_dc_level: [i32; 4],
    pub chroma_cr_dc_level: [i32; 4],

    pub chroma_cb_ac_level: [[i32; 15]; 4],
    pub chroma_cr_ac_level: [[i32; 15]; 4],

    pub chroma_cb_level4x4_nc: [u8; 4],
    pub chroma_cr_level4x4_nc: [u8; 4],
}

impl Residual {
    pub fn get_dc_levels_for(&mut self, plane: ColorPlane) -> &mut [i32] {
        let nc: &mut u8;
        match plane {
            ColorPlane::Y => {
                if self.has_separate_luma_dc() {
                    self.dc_level16x16.as_mut_slice()
                } else {
                    panic!("No separate DC levels in this prediction mode");
                }
            }
            ColorPlane::Cb => self.chroma_cb_dc_level.as_mut_slice(),
            ColorPlane::Cr => self.chroma_cr_dc_level.as_mut_slice(),
        }
    }

    pub fn get_ac_levels_for(&mut self, blk_idx: u8, plane: ColorPlane) -> (&mut [i32], &mut u8) {
        let levels: &mut [i32];
        let nc: &mut u8;
        let blk_idx = blk_idx as usize;
        match plane {
            ColorPlane::Y => {
                if self.has_separate_luma_dc() {
                    levels = self.ac_level16x16[blk_idx].as_mut_slice();
                    nc = &mut self.ac_level16x16_nc[blk_idx];
                } else {
                    levels = self.luma_level4x4[blk_idx].as_mut_slice();
                    nc = &mut self.luma_level4x4_nc[blk_idx];
                }
            }
            ColorPlane::Cb => {
                levels = self.chroma_cb_ac_level[blk_idx].as_mut_slice();
                nc = &mut self.chroma_cb_level4x4_nc[blk_idx];
            }
            ColorPlane::Cr => {
                levels = self.chroma_cr_ac_level[blk_idx].as_mut_slice();
                nc = &mut self.chroma_cr_level4x4_nc[blk_idx];
            }
        }
        (levels, nc)
    }

    // Calculates nC for the block withing the macroblock
    pub fn get_nc(&self, blk_idx: u8, plane: ColorPlane) -> u8 {
        let blk_idx = blk_idx as usize;
        match plane {
            ColorPlane::Y => match self.prediction_mode {
                MbPredictionMode::None => todo!(),
                MbPredictionMode::Intra_8x8 => todo!(),
                MbPredictionMode::Intra_4x4 => self.luma_level4x4_nc[blk_idx],
                MbPredictionMode::Intra_16x16 => self.ac_level16x16_nc[blk_idx],
                MbPredictionMode::Pred_L0 => todo!(),
                MbPredictionMode::Pred_L1 => todo!(),
            },

            ColorPlane::Cb => self.chroma_cb_level4x4_nc[blk_idx],
            ColorPlane::Cr => self.chroma_cr_level4x4_nc[blk_idx],
        }
    }

    pub fn has_separate_luma_dc(&self) -> bool {
        self.prediction_mode == MbPredictionMode::Intra_16x16
    }

    pub fn restore(&self, plane: ColorPlane, qp: u8) -> Vec<Block4x4> {
        let mut result = Vec::new();
        result.reserve(if plane.is_luma() { 16 } else { 4 });

        match plane {
            ColorPlane::Y => {
                if self.has_separate_luma_dc() {
                    let mut dcs = self.dc_level16x16;
                    dc_scale_4x4_block(&mut dcs, qp);
                    let mut dcs_block = unscan_block_4x4(&dcs);
                    dcs_block = transform_dc(&mut dcs_block);

                    for blk_idx in 0..16 {
                        let mut idct_coefficients = [0i32; 16];
                        let (dc_row, dc_column) = un_zig_zag_4x4(blk_idx);
                        idct_coefficients[0] = dcs_block.samples[dc_row][dc_column];
                        idct_coefficients[1..].copy_from_slice(&self.ac_level16x16[blk_idx]);
                        level_scale_4x4_block(
                            &mut idct_coefficients,
                            self.prediction_mode.is_inter(),
                            true,
                            qp,
                        );
                        let idct_4x4_block = unzip_block_4x4(&idct_coefficients);
                        let block = transform_4x4(&idct_4x4_block);
                        result.push(block);
                    }
                } else {
                    for blk_idx in 0..16 {
                        let mut idct_coefficients = [0i32; 16];
                        idct_coefficients.copy_from_slice(&self.luma_level4x4[blk_idx]);
                        level_scale_4x4_block(
                            &mut idct_coefficients,
                            self.prediction_mode.is_inter(),
                            false,
                            qp,
                        );
                        let idct_4x4_block = unzip_block_4x4(&idct_coefficients);
                        let block = transform_4x4(&idct_4x4_block);
                        result.push(block);
                    }
                }
            }
            ColorPlane::Cb => todo!(),
            ColorPlane::Cr => todo!(),
        }

        result
    }
}

// Table 8-13 – Specification of mapping of idx to Cij for zig-zag scan
#[inline]
pub const fn un_zig_zag_4x4(idx: usize) -> (/* row */ usize, /* column */ usize) {
    match idx {
        0 => (0, 0),
        1 => (0, 1),
        2 => (1, 0),
        3 => (2, 0),
        4 => (1, 1),
        5 => (0, 2),
        6 => (0, 3),
        7 => (1, 2),
        8 => (2, 1),
        9 => (3, 0),
        10 => (3, 1),
        11 => (2, 2),
        12 => (1, 3),
        13 => (2, 3),
        14 => (3, 2),
        15 => (3, 3),
        _ => panic!("Out of bounds zig-zag index"),
    }
}

// Get index of zig-zag walk for a given coordinates in 4x4 block
#[inline]
pub const fn zig_zag_4x4(row: usize, column: usize) -> usize {
    match (row, column) {
        (0, 0) => 0,
        (0, 1) => 1,
        (1, 0) => 2,
        (2, 0) => 3,
        (1, 1) => 4,
        (0, 2) => 5,
        (0, 3) => 6,
        (1, 2) => 7,
        (2, 1) => 8,
        (3, 0) => 9,
        (3, 1) => 10,
        (2, 2) => 11,
        (1, 3) => 12,
        (2, 3) => 13,
        (3, 2) => 14,
        (3, 3) => 15,
        _ => panic!("Out of bounds zig-zag coordinates"),
    }
}

#[inline]
const fn norm_adjust_4x4(m: u8, idx: usize) -> u8 {
    const IDX_TO_V_COLUMN: [u8; 16] = [0, 2, 2, 0, 1, 0, 2, 2, 2, 2, 1, 0, 1, 2, 2, 1];
    const V: [[u8; 3]; 6] =
        [[10, 16, 13], [11, 18, 14], [13, 20, 16], [14, 23, 18], [16, 25, 20], [18, 29, 23]];
    V[m as usize][IDX_TO_V_COLUMN[idx] as usize]
}

// Section 8.5.9 Derivation process for scaling functions
#[inline]
pub const fn level_scale_4x4(is_inter: bool, m: u8, idx: usize) -> i32 {
    let scaling_list = if is_inter {
        tables::DEFAULT_SCALING_LIST_4X4_INTER
    } else {
        tables::DEFAULT_SCALING_LIST_4X4_INTRA
    };

    (scaling_list[idx] as i32) * (norm_adjust_4x4(m, idx) as i32)
}

// Section 8.5.12.1 Scaling process for residual 4x4 blocks
pub fn level_scale_4x4_block(block: &mut [i32], is_inter: bool, skip_dc: bool, qp: u8) {
    let m = qp % 6;
    for (idx, c) in &mut block.iter_mut().enumerate() {
        let d = if skip_dc && idx == 0 {
            *c
        } else if qp >= 24 {
            (*c * level_scale_4x4(is_inter, m, idx)) << (qp / 6 - 4)
        } else {
            (*c * level_scale_4x4(is_inter, m, idx) + (1 << (3 - qp / 6))) >> (4 - qp / 6)
        };
        *c = d;
    }
}

// Section 8.5.10 Scaling and transformation process for DC transform coefficients for Intra_16x16
pub fn dc_scale_4x4_block(block: &mut [i32], qp: u8) {
    let m = qp % 6;
    let is_inter = false;
    for c in &mut block.iter_mut() {
        let d = if qp >= 36 {
            (*c * level_scale_4x4(is_inter, m, 0)) << (qp / 6 - 6)
        } else {
            (*c * level_scale_4x4(is_inter, m, 0) + (1 << (5 - qp / 6))) >> (6 - qp / 6)
        };
        *c = d;
    }
}

// Section 8.5.10 Scaling and transformation process for DC transform coefficients for Intra_16x16
pub fn transform_dc(block: &Block4x4) -> Block4x4 {
    const T: Block4x4 =
        Block4x4 { samples: [[1, 1, 1, 1], [1, 1, -1, -1], [1, -1, -1, 1], [1, -1, 1, -1]] };
    let f = matrix_mul(&T, block);
    matrix_mul(&f, &T)
}

pub fn unzip_block_4x4(block: &[i32]) -> Block4x4 {
    assert_eq!(block.len(), 16);
    let mut result = Block4x4::default();
    for (idx, value) in block.iter().enumerate() {
        let (i, j) = un_zig_zag_4x4(idx);
        result.samples[i][j] = *value;
    }
    result
}

pub fn unscan_block_4x4(block: &[i32]) -> Block4x4 {
    assert_eq!(block.len(), 16);
    let mut result = Block4x4::default();
    for (idx, value) in block.iter().enumerate() {
        let p = macroblock::get_4x4luma_block_location(idx as u8);
        result.samples[p.y as usize / 4][p.x as usize / 4] = *value;
    }
    result
}

pub fn matrix_mul(m1: &Block4x4, m2: &Block4x4) -> Block4x4 {
    let mut result = Block4x4::default();
    let a = &m1.samples;
    let b = &m2.samples;
    let r = &mut result.samples;
    for i in 0..4 {
        for j in 0..4 {
            let mut v = 0;
            for k in 0..4 {
                v += a[i][k] * b[k][j];
            }
            r[i][j] = v;
        }
    }
    result
}

// Section 8.5.12.2 Transformation process for residual 4x4 blocks
pub fn transform_4x4(block: &Block4x4) -> Block4x4 {
    let d = block;
    let mut e = Block4x4::default();
    for i in 0..4 {
        // (8-338)
        e.samples[i][0] = d.samples[i][0] + d.samples[i][2];
        // (8-339)
        e.samples[i][1] = d.samples[i][0] - d.samples[i][2];
        // (8-340)
        e.samples[i][2] = d.samples[i][1] / 2 - d.samples[i][3];
        // (8-341)
        e.samples[i][3] = d.samples[i][1] + d.samples[i][3] / 2;
    }

    let mut f = Block4x4::default();
    for i in 0..4 {
        // (8-342)
        f.samples[i][0] = e.samples[i][0] + e.samples[i][3];
        // (8-343)
        f.samples[i][1] = e.samples[i][1] + e.samples[i][2];
        // (8-344)
        f.samples[i][2] = e.samples[i][1] - e.samples[i][2];
        // (8-345)
        f.samples[i][3] = e.samples[i][0] - e.samples[i][3];
    }

    let mut g = Block4x4::default();
    for j in 0..4 {
        // (8-346)
        g.samples[0][j] = f.samples[0][j] + f.samples[2][j];
        // (8-347)
        g.samples[1][j] = f.samples[0][j] - f.samples[2][j];
        // (8-348)
        g.samples[2][j] = f.samples[1][j] / 2 - f.samples[3][j];
        // (8-349)
        g.samples[3][j] = f.samples[1][j] + f.samples[3][j] / 2;
    }

    let mut h = Block4x4::default();
    for j in 0..4 {
        // (8-350)
        h.samples[0][j] = g.samples[0][j] + g.samples[3][j];
        // (8-351)
        h.samples[1][j] = g.samples[1][j] + g.samples[2][j];
        // (8-352)
        h.samples[2][j] = g.samples[1][j] - g.samples[2][j];
        // (8-353)
        h.samples[3][j] = g.samples[0][j] - g.samples[3][j];
    }

    let mut r = Block4x4::default();
    for i in 0..4 {
        for j in 0..4 {
            r.samples[i][j] = (h.samples[i][j] + 32) >> 6;
        }
    }

    r
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
        for i in 0..3 {
            for j in 0..3 {
                let idx = zig_zag_4x4(i, j);
                assert!(idx < 16);
                let (y, x) = un_zig_zag_4x4(idx);
                assert_eq!((i, j), (y, x));
            }
        }
    }

    #[test]
    pub fn test_transform_4x4() {
        let coefficients = [[10, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
        let qp = 1;
        let mut block = [0i32; 16];
        for i in 0..3 {
            for j in 0..3 {
                block[zig_zag_4x4(i, j)] = coefficients[i][j];
            }
        }

        level_scale_4x4_block(&mut block, false, false, qp);
        let output = transform_4x4(&unzip_block_4x4(&block));
        for row in output.samples {
            for col in row {
                assert_eq!(col, 1);
            }
        }
    }

    #[test]
    pub fn test_unscan_block_4x4() {
        let input = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let block =
            Block4x4 { samples: [[0, 1, 4, 5], [2, 3, 6, 7], [8, 9, 12, 13], [10, 11, 14, 15]] };
        assert_eq!(block, unscan_block_4x4(&input));
    }

    #[test]
    pub fn test_matrix_mut() {
        let zero = Block4x4::default();
        let identity =
            Block4x4 { samples: [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]] };
        let m1 = Block4x4 {
            samples: [[12, 32, 56, 17], [45, -34, 56, 21], [-8, -45, 3, -99], [0, -1, 8, 17]],
        };

        assert_eq!(m1, matrix_mul(&m1, &identity));
        assert_eq!(m1, matrix_mul(&identity, &m1));
        assert_eq!(zero, matrix_mul(&zero, &m1));
    }
}
