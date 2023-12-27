use super::tables;

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
    for (idx, c) in &mut block[1..].iter_mut().enumerate() {
        let d = if qp >= 24 {
            (*c * level_scale_4x4(is_inter, qp % 6, idx)) << (qp / 6 - 4)
        } else {
            (*c * level_scale_4x4(is_inter, qp % 6, idx) + (1 << (3 - qp / 6))) >> (4 - qp / 6)
        };
        *c = d;
    }
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
}
