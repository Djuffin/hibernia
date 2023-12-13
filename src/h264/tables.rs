use super::cavlc;
use super::macroblock;

use cavlc::CoeffTokenPattern;
use macroblock::{CodedBlockPattern, IMacroblockType};

// Table 9-4 – Assignment of codeNum to values of coded_block_pattern for
// macroblock prediction modes
const CODE_NUM_TO_INTRA_CODED_BLOCK_PATTERN: [u8; 48] = [
    47, 31, 15, 0, 23, 27, 29, 30, 7, 11, 13, 14, 39, 43, 45, 46, 16, 3, 5, 10, 12, 19, 21, 26, 28,
    35, 37, 42, 44, 1, 2, 4, 8, 17, 18, 20, 24, 6, 9, 22, 25, 32, 33, 34, 36, 40, 38, 41,
];

#[inline]
pub fn code_num_to_intra_coded_block_pattern(x: u8) -> Option<CodedBlockPattern> {
    CODE_NUM_TO_INTRA_CODED_BLOCK_PATTERN.get(x as usize).map(|x| CodedBlockPattern(*x))
}

// Table 7-11 – Macroblock types for I slices
pub const fn mb_type_to_coded_block_pattern(mb_type: IMacroblockType) -> Option<CodedBlockPattern> {
    match mb_type {
        IMacroblockType::I_16x16_0_0_0 => Some(CodedBlockPattern::new(0, 0)),
        IMacroblockType::I_16x16_1_0_0 => Some(CodedBlockPattern::new(0, 0)),
        IMacroblockType::I_16x16_2_0_0 => Some(CodedBlockPattern::new(0, 0)),
        IMacroblockType::I_16x16_3_0_0 => Some(CodedBlockPattern::new(0, 0)),
        IMacroblockType::I_16x16_0_1_0 => Some(CodedBlockPattern::new(1, 0)),
        IMacroblockType::I_16x16_1_1_0 => Some(CodedBlockPattern::new(1, 0)),
        IMacroblockType::I_16x16_2_1_0 => Some(CodedBlockPattern::new(1, 0)),
        IMacroblockType::I_16x16_3_1_0 => Some(CodedBlockPattern::new(1, 0)),
        IMacroblockType::I_16x16_0_2_0 => Some(CodedBlockPattern::new(2, 0)),
        IMacroblockType::I_16x16_1_2_0 => Some(CodedBlockPattern::new(2, 0)),
        IMacroblockType::I_16x16_2_2_0 => Some(CodedBlockPattern::new(2, 0)),
        IMacroblockType::I_16x16_3_2_0 => Some(CodedBlockPattern::new(2, 0)),
        IMacroblockType::I_16x16_0_0_1 => Some(CodedBlockPattern::new(0, 15)),
        IMacroblockType::I_16x16_1_0_1 => Some(CodedBlockPattern::new(0, 15)),
        IMacroblockType::I_16x16_2_0_1 => Some(CodedBlockPattern::new(0, 15)),
        IMacroblockType::I_16x16_3_0_1 => Some(CodedBlockPattern::new(0, 15)),
        IMacroblockType::I_16x16_0_1_1 => Some(CodedBlockPattern::new(1, 15)),
        IMacroblockType::I_16x16_1_1_1 => Some(CodedBlockPattern::new(1, 15)),
        IMacroblockType::I_16x16_2_1_1 => Some(CodedBlockPattern::new(1, 15)),
        IMacroblockType::I_16x16_3_1_1 => Some(CodedBlockPattern::new(1, 15)),
        IMacroblockType::I_16x16_0_2_1 => Some(CodedBlockPattern::new(2, 15)),
        IMacroblockType::I_16x16_1_2_1 => Some(CodedBlockPattern::new(2, 15)),
        IMacroblockType::I_16x16_2_2_1 => Some(CodedBlockPattern::new(2, 15)),
        IMacroblockType::I_16x16_3_2_1 => Some(CodedBlockPattern::new(2, 15)),
        _ => None,
    }
}

pub const TABLE95: [(
    (u8, u8),
    CoeffTokenPattern,
    CoeffTokenPattern,
    CoeffTokenPattern,
    CoeffTokenPattern,
    CoeffTokenPattern,
    CoeffTokenPattern,
); 62] = include!("table_9-5.rs");
