use super::tables::{self, zz, zz_idx_to_yx};

#[inline]
fn norm_adjust_4x4(m: u8, idx: usize) -> u8 {
    const IDX_TO_V_COLUMN: [u8; 16] = [0, 2, 2, 0, 1, 0, 2, 2, 2, 2, 1, 0, 1, 2, 2, 1];
    const V: [[u8; 3]; 6] =
        [[10, 16, 13], [11, 18, 14], [13, 20, 16], [14, 23, 18], [16, 25, 20], [18, 29, 23]];
    return V[m as usize][IDX_TO_V_COLUMN[idx] as usize];
}

// Section 8.5.9 Derivation process for scaling functions
#[inline]
pub fn level_scale_4x4(is_inter: bool, m: u8, idx: usize) -> i32 {
    let scaling_list = if is_inter {
        tables::DEFAULT_SCALING_LIST_4X4_INTER
    } else {
        tables::DEFAULT_SCALING_LIST_4X4_INTRA
    };

    (scaling_list[idx] as i32) * (norm_adjust_4x4(m, idx) as i32)
}

// Section 8.5.12.1 Scaling process for residual 4x4 blocks
pub fn level_scale_4x4_block(block: &mut [i32], is_inter: bool, qp: u8) {
    let m = qp % 6;
    for (idx, c) in &mut block.iter_mut().enumerate() {
        let d = if qp >= 24 {
            (*c * level_scale_4x4(is_inter, m, idx)) << (qp / 6 - 4)
        } else {
            (*c * level_scale_4x4(is_inter, m, idx) + (1 << (3 - qp / 6))) >> (4 - qp / 6)
        };
        *c = d;
    }
}

// Section 8.5.12.2 Transformation process for residual 4x4 blocks
pub fn transform_4x4(block: &[i32]) -> [[u8; 4]; 4] {
    debug_assert_eq!(block.len(), 16);
    let mut r: [[u8; 4]; 4] = [[0; 4]; 4];
    let mut d: [[i32; 4]; 4] = [[0; 4]; 4];
    let mut e: [[i32; 4]; 4] = [[0; 4]; 4];
    let mut f: [[i32; 4]; 4] = [[0; 4]; 4];
    let mut g: [[i32; 4]; 4] = [[0; 4]; 4];
    let mut h: [[i32; 4]; 4] = [[0; 4]; 4];

    for idx in 0..16 {
        let (i, j) = zz_idx_to_yx(idx);
        d[i][j] = block[idx];
    }

    for i in 0..3 {
        // (8-338)
        e[i][0] = d[i][0] + d[i][2];
        // (8-339)
        e[i][1] = d[i][0] - d[i][2];
        // (8-340)
        e[i][2] = d[i][1] / 2 - d[i][3];
        // (8-341)
        e[i][2] = d[i][1] + d[i][3] / 2;
    }

    for i in 0..3 {
        // (8-342)
        f[i][0] = e[i][0] + e[i][3];
        // (8-343)
        f[i][1] = e[i][1] + e[i][2];
        // (8-344)
        f[i][2] = e[i][1] - e[i][2];
        // (8-345)
        f[i][3] = e[i][0] - e[i][3];
    }

    for j in 0..3 {
        // (8-346)
        g[0][j] = f[0][j] + f[2][j];
        // (8-347)
        g[1][j] = f[0][j] - f[2][j];
        // (8-348)
        g[2][j] = f[1][j] / 2 - f[3][j];
        // (8-349)
        g[3][j] = f[1][j] + f[3][j] / 2;
    }

    for j in 0..3 {
        // (8-350)
        h[0][j] = g[0][j] + g[3][j];
        // (8-351)
        h[1][j] = g[1][j] + g[2][j];
        // (8-352)
        h[2][j] = g[1][j] - g[2][j];
        // (8-353)
        h[3][j] = g[0][j] - g[3][j];
    }

    for i in 0..3 {
        for j in 0..3 {
            r[i][j] = ((h[i][j] + 1 << 5) >> 6) as u8;
        }
    }

    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_norm_adjust_4x4() {
        assert_eq!(norm_adjust_4x4(0, 0), 10);
        assert_eq!(norm_adjust_4x4(3, 15), 23);
        assert_eq!(norm_adjust_4x4(2, 1), 16);
    }

    #[test]
    pub fn test_zig_zag() {
        for i in 0..3 {
            for j in 0..3 {
                let idx = zz(i, j);
                assert!(idx < 16);
                let (y, x) = zz_idx_to_yx(idx);
                assert_eq!((i, j), (y, x));
            }
        }
    }

    #[test]
    pub fn test_transform_4x4() {
        // let y = [[48, -1, 0, -1], [-1, 1, -1, -2], [0, 0, 0, 0], [0, 1, 0, 0]];
        // let qp = 18;
        let y = [[12, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
        let qp = 30;
        let mut block = [0i32; 16];
        for i in 0..3 {
            for j in 0..3 {
                block[zz(i, j)] = y[i][j];
            }
        }

        level_scale_4x4_block(&mut block, true, qp);
        let r = transform_4x4(&block);
        println!("{:?}", r);
    }
}
