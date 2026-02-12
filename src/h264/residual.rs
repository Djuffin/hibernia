use std::result;

use log::info;
use smallvec::SmallVec;

use super::{
    macroblock::{self, CodedBlockPattern, MbPredictionMode},
    tables, ColorPlane,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Block4x4 {
    pub samples: [[i32; 4]; 4],
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Block2x2 {
    pub samples: [[i32; 2]; 2],
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
                MbPredictionMode::Intra_16x16 => self.ac_level16x16_nc[blk_idx],
                MbPredictionMode::Intra_4x4
                | MbPredictionMode::Pred_L0
                | MbPredictionMode::Pred_L1
                | MbPredictionMode::None => self.luma_level4x4_nc[blk_idx],
                MbPredictionMode::Intra_8x8 => todo!(),
            },

            ColorPlane::Cb => self.chroma_cb_level4x4_nc[blk_idx],
            ColorPlane::Cr => self.chroma_cr_level4x4_nc[blk_idx],
        }
    }

    pub fn has_separate_luma_dc(&self) -> bool {
        self.prediction_mode == MbPredictionMode::Intra_16x16
    }

    pub fn restore(&self, plane: ColorPlane, qp: u8) -> SmallVec<[Block4x4; 16]> {
        let mut result = SmallVec::new();

        if plane == ColorPlane::Y {
            if self.has_separate_luma_dc() {
                // Section 8.5.2 Specification of transform decoding process for luma samples
                // of Intra_16x16 macroblock prediction mode
                let mut dcs_block = unzip_block_4x4(&self.dc_level16x16);
                dcs_block = transform_dc(&dcs_block);
                dc_scale_4x4_block(&mut dcs_block, qp);

                for blk_idx in 0..16 {
                    let mut idct_coefficients = [0i32; 16];
                    let (dc_row, dc_column) = unscan_4x4(blk_idx);
                    idct_coefficients[0] = dcs_block.samples[dc_row][dc_column];
                    idct_coefficients[1..].copy_from_slice(&self.ac_level16x16[blk_idx]);
                    level_scale_4x4_block(
                        &mut idct_coefficients,
                        self.prediction_mode.is_inter(),
                        true,
                        qp,
                    );
                    let mut block = unzip_block_4x4(&idct_coefficients);
                    transform_4x4(&mut block);
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
                    let mut block = unzip_block_4x4(&idct_coefficients);
                    transform_4x4(&mut block);
                    result.push(block);
                }
            }
        } else {
            // Section 8.5.8, 8.5.11 Specification of transform decoding process for chroma samples
            let dcs = match plane {
                ColorPlane::Cb => &self.chroma_cb_dc_level,
                ColorPlane::Cr => &self.chroma_cr_dc_level,
                _ => unreachable!(),
            };
            let mut dcs_block = Block2x2 { samples: [[dcs[0], dcs[1]], [dcs[2], dcs[3]]] };
            dcs_block = transform_chroma_dc(&dcs_block);
            dc_scale_2x2_block(&mut dcs_block, qp);

            for blk_idx in 0..4 {
                let acs = match plane {
                    ColorPlane::Cb => &self.chroma_cb_ac_level[blk_idx],
                    ColorPlane::Cr => &self.chroma_cr_ac_level[blk_idx],
                    _ => unreachable!(),
                };
                let mut idct_coefficients = [0i32; 16];
                let (dc_row, dc_column) = unscan_2x2(blk_idx);
                idct_coefficients[0] = dcs_block.samples[dc_row][dc_column];
                idct_coefficients[1..].copy_from_slice(acs);
                level_scale_4x4_block(
                    &mut idct_coefficients,
                    self.prediction_mode.is_inter(),
                    true,
                    qp,
                );
                let mut block = unzip_block_4x4(&idct_coefficients);
                transform_4x4(&mut block);
                result.push(block);
            }
        }

        result
    }
}

// Figure 8-6 – Assignment of the indices of dcY to luma4x4BlkIdx
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

// Figure 8-7 – Assignment of the indices of dcC to chroma4x4BlkIdx
#[inline]
pub const fn unscan_2x2(idx: usize) -> (/* row */ usize, /* column */ usize) {
    const TABLE: [(usize, usize); 4] = [(0, 0), (0, 1), (1, 0), (1, 1)];
    TABLE[idx]
}

// Table 8-13 – Specification of mapping of idx to Cij for zig-zag scan
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
    const TABLE: [usize; 16] =
        [0, 1, 5, 6, 2, 4, 7, 12, 3, 8, 11, 13, 9, 10, 14, 15];
    TABLE[row * 4 + column]
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
    // See seq_scaling_matrix_present_flag=0, all values in Flat_4x4_16 are equal 16.
    let scaling_list = 16;
    scaling_list * (norm_adjust_4x4(m, idx) as i32)
}

// Section 8.5.12.1 Scaling process for residual 4x4 blocks
pub fn level_scale_4x4_block(block: &mut [i32], is_inter: bool, skip_dc: bool, qp: u8) {
    let m = qp % 6;
    for (idx, c) in &mut block.iter_mut().enumerate() {
        let d = if skip_dc && idx == 0 {
            *c
        } else if qp >= 24 {
            // Equation 8-336
            (*c * level_scale_4x4(is_inter, m, idx)) << (qp / 6 - 4)
        } else {
            // Equation 8-337
            (*c * level_scale_4x4(is_inter, m, idx) + (1 << (3 - qp / 6))) >> (4 - qp / 6)
        };
        *c = d;
    }
}

// Section 8.5.10 Scaling and transformation process for DC transform coefficients for Intra_16x16
pub fn dc_scale_4x4_block(block: &mut Block4x4, qp: u8) {
    let m = qp % 6;
    let is_inter = false;
    for row in block.samples.iter_mut() {
        for c in row.iter_mut() {
            let d = if qp >= 36 {
                // Equation 8-321
                (*c * level_scale_4x4(is_inter, m, 0)) << (qp / 6 - 6)
            } else {
                // Equation 8-322
                (*c * level_scale_4x4(is_inter, m, 0) + (1 << (5 - qp / 6))) >> (6 - qp / 6)
            };
            *c = d;
        }
    }
}

// Section 8.5.11.2 Scaling and transformation process for chroma DC transform coefficients
pub fn dc_scale_2x2_block(block: &mut Block2x2, qp: u8) {
    let m = qp % 6;
    let is_inter = false;
    for row in block.samples.iter_mut() {
        for c in row.iter_mut() {
            // Equation 8-326 (approximated)
            let d = (*c * level_scale_4x4(is_inter, m, 0) << (qp / 6)) >> 5;
            *c = d;
        }
    }
}

// Section 8.5.10 Scaling and transformation process for DC transform coefficients for Intra_16x16
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

    // This is a temporary storage for the intermediate matrix after first multiplication
    let mut f = [[0i32; 4]; 4];

    // Calculate the result of the first multiplication using only +/-.
    for j in 0..4 {
        let b0 = b[0][j];
        let b1 = b[1][j];
        let b2 = b[2][j];
        let b3 = b[3][j];

        f[0][j] = b0 + b1 + b2 + b3;
        f[1][j] = b0 + b1 - b2 - b3;
        f[2][j] = b0 - b1 - b2 + b3;
        f[3][j] = b0 - b1 + b2 - b3;
    }

    // Calculate the final result using the intermediate 'f' matrix.
    for i in 0..4 {
        let f0 = f[i][0];
        let f1 = f[i][1];
        let f2 = f[i][2];
        let f3 = f[i][3];

        r[i][0] = f0 + f1 + f2 + f3;
        r[i][1] = f0 + f1 - f2 - f3;
        r[i][2] = f0 - f1 - f2 + f3;
        r[i][3] = f0 - f1 + f2 - f3;
    }

    result
}

// Section 8.5.11.1 Transformation process for chroma DC transform coefficients
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

pub fn unzip_block_4x4(block: &[i32]) -> Block4x4 {
    assert_eq!(block.len(), 16);
    let mut result = Block4x4::default();
    for (idx, value) in block.iter().enumerate() {
        let (row, column) = un_zig_zag_4x4(idx);
        result.samples[row][column] = *value;
    }
    result
}

pub fn unscan_block_4x4(block: &[i32]) -> Block4x4 {
    let mut result = Block4x4::default();
    for (idx, value) in block.iter().enumerate() {
        let (row, column) = unscan_4x4(idx);
        result.samples[row][column] = *value;
    }
    result
}

// Section 8.5.12.2 Transformation process for residual 4x4 blocks
pub fn transform_4x4(block: &mut Block4x4) {
    let d = &mut block.samples;

    for i in 0..4 {
        // (8-338)
        let e0 = d[i][0] + d[i][2];
        // (8-339)
        let e1 = d[i][0] - d[i][2];
        // (8-340)
        let e2 = (d[i][1] >> 1) - d[i][3];
        // (8-341)
        let e3 = d[i][1] + (d[i][3] >> 1);

        // (8-342)
        d[i][0] = e0 + e3;
        // (8-343)
        d[i][1] = e1 + e2;
        // (8-344)
        d[i][2] = e1 - e2;
        // (8-345)
        d[i][3] = e0 - e3;
    }

    for j in 0..4 {
        // (8-346)
        let g0 = d[0][j] + d[2][j];
        // (8-347)
        let g1 = d[0][j] - d[2][j];
        // (8-348)
        let g2 = (d[1][j] >> 1) - d[3][j];
        // (8-349)
        let g3 = d[1][j] + (d[3][j] >> 1);

        // (8-350)
        let h0 = g0 + g3;
        // (8-351)
        let h1 = g1 + g2;
        // (8-352)
        let h2 = g1 - g2;
        // (8-353)
        let h3 = g0 - g3;

        d[0][j] = (h0 + 32) >> 6;
        d[1][j] = (h1 + 32) >> 6;
        d[2][j] = (h2 + 32) >> 6;
        d[3][j] = (h3 + 32) >> 6;
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

        level_scale_4x4_block(&mut block, false, false, qp);
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
}
