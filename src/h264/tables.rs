use super::cavlc;
use super::macroblock;

use cavlc::BitPattern;
use log::trace;
use macroblock::{CodedBlockPattern, IMbType, Intra_16x16_SamplePredMode};

pub const MB_WIDTH: usize = 16;
pub const MB_HEIGHT: usize = 16;
pub const BIT_DEPTH: usize = 8;

// Table 7-3 – Specification of default scaling lists Default_4x4_Intra and Default_4x4_Inter
pub const DEFAULT_SCALING_LIST_4X4_INTRA: [u8; 16] =
    [6, 13, 13, 20, 20, 20, 28, 28, 28, 28, 32, 32, 32, 37, 37, 42];
pub const DEFAULT_SCALING_LIST_4X4_INTER: [u8; 16] =
    [10, 14, 14, 20, 20, 20, 24, 24, 24, 24, 27, 27, 27, 30, 30, 34];

// Table 9-4 – Assignment of codeNum to values of coded_block_pattern for
// macroblock prediction modes
const CODE_NUM_TO_INTRA_CODED_BLOCK_PATTERN: [u8; 48] = [
    47, 31, 15, 0, 23, 27, 29, 30, 7, 11, 13, 14, 39, 43, 45, 46, 16, 3, 5, 10, 12, 19, 21, 26, 28,
    35, 37, 42, 44, 1, 2, 4, 8, 17, 18, 20, 24, 6, 9, 22, 25, 32, 33, 34, 36, 40, 38, 41,
];

const CODE_NUM_TO_INTER_CODED_BLOCK_PATTERN: [u8; 48] = [
    0, 16, 1, 2, 4, 8, 32, 3, 5, 10, 12, 15, 47, 7, 11, 13, 14, 6, 9, 31, 35, 37, 42, 44, 33, 34,
    36, 40, 39, 43, 45, 46, 17, 18, 20, 24, 19, 21, 26, 28, 23, 27, 29, 30, 22, 25, 38, 41,
];

#[inline]
pub fn code_num_to_intra_coded_block_pattern(x: u8) -> Option<CodedBlockPattern> {
    let result =
        CODE_NUM_TO_INTRA_CODED_BLOCK_PATTERN.get(x as usize).map(|x| CodedBlockPattern(*x));
    trace!("intra coded_block_pattern num: {} pattern: {:?}", x, result);
    result
}

#[inline]
pub fn code_num_to_inter_coded_block_pattern(x: u8) -> Option<CodedBlockPattern> {
    let result =
        CODE_NUM_TO_INTER_CODED_BLOCK_PATTERN.get(x as usize).map(|x| CodedBlockPattern(*x));
    trace!("inter coded_block_pattern num: {} pattern: {:?}", x, result);
    result
}

// Table 7-11 – Macroblock types for I slices
pub const fn mb_type_to_coded_block_pattern(mb_type: IMbType) -> Option<CodedBlockPattern> {
    match mb_type {
        IMbType::I_16x16_0_0_0 => Some(CodedBlockPattern::new(0, 0)),
        IMbType::I_16x16_1_0_0 => Some(CodedBlockPattern::new(0, 0)),
        IMbType::I_16x16_2_0_0 => Some(CodedBlockPattern::new(0, 0)),
        IMbType::I_16x16_3_0_0 => Some(CodedBlockPattern::new(0, 0)),
        IMbType::I_16x16_0_1_0 => Some(CodedBlockPattern::new(1, 0)),
        IMbType::I_16x16_1_1_0 => Some(CodedBlockPattern::new(1, 0)),
        IMbType::I_16x16_2_1_0 => Some(CodedBlockPattern::new(1, 0)),
        IMbType::I_16x16_3_1_0 => Some(CodedBlockPattern::new(1, 0)),
        IMbType::I_16x16_0_2_0 => Some(CodedBlockPattern::new(2, 0)),
        IMbType::I_16x16_1_2_0 => Some(CodedBlockPattern::new(2, 0)),
        IMbType::I_16x16_2_2_0 => Some(CodedBlockPattern::new(2, 0)),
        IMbType::I_16x16_3_2_0 => Some(CodedBlockPattern::new(2, 0)),
        IMbType::I_16x16_0_0_1 => Some(CodedBlockPattern::new(0, 15)),
        IMbType::I_16x16_1_0_1 => Some(CodedBlockPattern::new(0, 15)),
        IMbType::I_16x16_2_0_1 => Some(CodedBlockPattern::new(0, 15)),
        IMbType::I_16x16_3_0_1 => Some(CodedBlockPattern::new(0, 15)),
        IMbType::I_16x16_0_1_1 => Some(CodedBlockPattern::new(1, 15)),
        IMbType::I_16x16_1_1_1 => Some(CodedBlockPattern::new(1, 15)),
        IMbType::I_16x16_2_1_1 => Some(CodedBlockPattern::new(1, 15)),
        IMbType::I_16x16_3_1_1 => Some(CodedBlockPattern::new(1, 15)),
        IMbType::I_16x16_0_2_1 => Some(CodedBlockPattern::new(2, 15)),
        IMbType::I_16x16_1_2_1 => Some(CodedBlockPattern::new(2, 15)),
        IMbType::I_16x16_2_2_1 => Some(CodedBlockPattern::new(2, 15)),
        IMbType::I_16x16_3_2_1 => Some(CodedBlockPattern::new(2, 15)),
        _ => None,
    }
}

// Table 7-11 – Macroblock types for I slices
pub const fn mb_type_to_16x16_pred_mode(mb_type: IMbType) -> Option<Intra_16x16_SamplePredMode> {
    match mb_type {
        IMbType::I_16x16_0_0_0 => Some(Intra_16x16_SamplePredMode::Intra_16x16_Vertical),
        IMbType::I_16x16_1_0_0 => Some(Intra_16x16_SamplePredMode::Intra_16x16_Horizontal),
        IMbType::I_16x16_2_0_0 => Some(Intra_16x16_SamplePredMode::Intra_16x16_DC),
        IMbType::I_16x16_3_0_0 => Some(Intra_16x16_SamplePredMode::Intra_16x16_Plane),
        IMbType::I_16x16_0_1_0 => Some(Intra_16x16_SamplePredMode::Intra_16x16_Vertical),
        IMbType::I_16x16_1_1_0 => Some(Intra_16x16_SamplePredMode::Intra_16x16_Horizontal),
        IMbType::I_16x16_2_1_0 => Some(Intra_16x16_SamplePredMode::Intra_16x16_DC),
        IMbType::I_16x16_3_1_0 => Some(Intra_16x16_SamplePredMode::Intra_16x16_Plane),
        IMbType::I_16x16_0_2_0 => Some(Intra_16x16_SamplePredMode::Intra_16x16_Vertical),
        IMbType::I_16x16_1_2_0 => Some(Intra_16x16_SamplePredMode::Intra_16x16_Horizontal),
        IMbType::I_16x16_2_2_0 => Some(Intra_16x16_SamplePredMode::Intra_16x16_DC),
        IMbType::I_16x16_3_2_0 => Some(Intra_16x16_SamplePredMode::Intra_16x16_Plane),
        IMbType::I_16x16_0_0_1 => Some(Intra_16x16_SamplePredMode::Intra_16x16_Vertical),
        IMbType::I_16x16_1_0_1 => Some(Intra_16x16_SamplePredMode::Intra_16x16_Horizontal),
        IMbType::I_16x16_2_0_1 => Some(Intra_16x16_SamplePredMode::Intra_16x16_DC),
        IMbType::I_16x16_3_0_1 => Some(Intra_16x16_SamplePredMode::Intra_16x16_Plane),
        IMbType::I_16x16_0_1_1 => Some(Intra_16x16_SamplePredMode::Intra_16x16_Vertical),
        IMbType::I_16x16_1_1_1 => Some(Intra_16x16_SamplePredMode::Intra_16x16_Horizontal),
        IMbType::I_16x16_2_1_1 => Some(Intra_16x16_SamplePredMode::Intra_16x16_DC),
        IMbType::I_16x16_3_1_1 => Some(Intra_16x16_SamplePredMode::Intra_16x16_Plane),
        IMbType::I_16x16_0_2_1 => Some(Intra_16x16_SamplePredMode::Intra_16x16_Vertical),
        IMbType::I_16x16_1_2_1 => Some(Intra_16x16_SamplePredMode::Intra_16x16_Horizontal),
        IMbType::I_16x16_2_2_1 => Some(Intra_16x16_SamplePredMode::Intra_16x16_DC),
        IMbType::I_16x16_3_2_1 => Some(Intra_16x16_SamplePredMode::Intra_16x16_Plane),
        _ => None,
    }
}

#[allow(clippy::all)]
pub const TABLE95: [(
    (u8, u8),
    BitPattern,
    BitPattern,
    BitPattern,
    BitPattern,
    BitPattern,
    BitPattern,
); 62] = include!("table_9-5.rs");

#[allow(clippy::all)]
pub const TABLE9_7AND8: [(
    u8,
    BitPattern,
    BitPattern,
    BitPattern,
    BitPattern,
    BitPattern,
    BitPattern,
    BitPattern,
    BitPattern,
    BitPattern,
    BitPattern,
    BitPattern,
    BitPattern,
    BitPattern,
    BitPattern,
    BitPattern,
); 16] = include!("tables_9-7and8.rs");

#[allow(clippy::all)]
pub const TABLE9_9A: [(u8, BitPattern, BitPattern, BitPattern); 4] = include!("table_9-9_a.rs");

#[allow(clippy::all)]
pub const TABLE9_9B: [(
    u8,
    BitPattern,
    BitPattern,
    BitPattern,
    BitPattern,
    BitPattern,
    BitPattern,
    BitPattern,
); 8] = include!("table_9-9_b.rs");

#[allow(clippy::all)]
pub const TABLE9_10: [(
    u8,
    BitPattern,
    BitPattern,
    BitPattern,
    BitPattern,
    BitPattern,
    BitPattern,
    BitPattern,
); 15] = include!("table_9-10.rs");
