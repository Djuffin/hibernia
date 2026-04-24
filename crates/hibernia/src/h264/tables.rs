use super::cavlc;
use super::macroblock;
use super::slice::SliceType;

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
    result
}

#[inline]
pub fn code_num_to_inter_coded_block_pattern(x: u8) -> Option<CodedBlockPattern> {
    let result =
        CODE_NUM_TO_INTER_CODED_BLOCK_PATTERN.get(x as usize).map(|x| CodedBlockPattern(*x));
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

/// Table 9-45 – State transition table
#[rustfmt::skip]
#[allow(clippy::all)]
pub const TRANS_IDX_LPS: [u8; 64] = include!("table_9-45_lps.rs");

/// Table 9-45 – State transition table
#[rustfmt::skip]
#[allow(clippy::all)]
pub const TRANS_IDX_MPS: [u8; 64] = include!("table_9-45_mps.rs");

/// Table 9-44 – Specification of rangeTabLPS depending on pStateIdx and qCodIRangeIdx
#[rustfmt::skip]
#[allow(clippy::all)]
pub const RANGE_TAB_LPS: [[u8; 4]; 64] = include!("table_9-44.rs");

/// Initialization values for I slices (Tables 9-12 to 9-33)
pub const INIT_CTX_I: [(i8, i8); 1024] = include!("tables_9-12to33_i.rs");
/// Initialization values for P/B slices with cabac_init_idc = 0 (Tables 9-12 to 9-33)
pub const INIT_CTX_PB_0: [(i8, i8); 1024] = include!("tables_9-12to33_pb0.rs");
/// Initialization values for P/B slices with cabac_init_idc = 1 (Tables 9-12 to 9-33)
pub const INIT_CTX_PB_1: [(i8, i8); 1024] = include!("tables_9-12to33_pb1.rs");
/// Initialization values for P/B slices with cabac_init_idc = 2 (Tables 9-12 to 9-33)
pub const INIT_CTX_PB_2: [(i8, i8); 1024] = include!("tables_9-12to33_pb2.rs");

pub fn get_init_table(slice_type: SliceType, cabac_init_idc: u8) -> &'static [(i8, i8); 1024] {
    if slice_type.is_intra() {
        &INIT_CTX_I
    } else {
        match cabac_init_idc {
            0 => &INIT_CTX_PB_0,
            1 => &INIT_CTX_PB_1,
            _ => &INIT_CTX_PB_2,
        }
    }
}

/// Per-`ctxBlockCat` ctxIdxOffset for `coded_block_flag`. H.264 Table 9-34 / 9-40.
/// Indexed by `ctx_block_cat` in 0..=13. Out-of-range cats fall back to 85 in the caller.
pub const CBF_OFFSETS: [u32; 14] =
    [85, 89, 93, 97, 101, 1012, 460, 464, 468, 1016, 472, 476, 480, 1020];

/// Per-`ctxBlockCat` ctxIdxOffset for `significant_coeff_flag`. Table 9-34 / 9-40.
pub const SIG_COEFF_OFFSETS: [u32; 14] =
    [105, 120, 134, 149, 152, 402, 484, 499, 513, 660, 528, 543, 557, 718];

/// Per-`ctxBlockCat` ctxIdxOffset for `last_significant_coeff_flag`. Table 9-34 / 9-40.
pub const LAST_SIG_COEFF_OFFSETS: [u32; 14] =
    [166, 181, 195, 210, 213, 417, 572, 587, 601, 690, 616, 631, 645, 748];

/// Per-`ctxBlockCat` ctxIdxOffset for `coeff_abs_level_minus1`. Table 9-34 / 9-40.
pub const COEFF_ABS_OFFSETS: [u32; 14] =
    [227, 237, 247, 257, 266, 426, 952, 962, 972, 708, 982, 992, 1002, 766];

/// Table 9-43 — ctxIdxInc for `significant_coeff_flag` in 8x8 luma/Cb/Cr blocks
/// (ctxBlockCat 5, 9, 13) with frame-coded macroblocks. Indexed by levelListIdx
/// in 0..=62.
pub const SIG_COEFF_FLAG_CTX_IDX_INC_8X8_FRAME: [u8; 63] = [
    0, 1, 2, 3, 4, 5, 5, 4, 4, 3, 3, 4, 4, 4, 5, 5,
    4, 4, 4, 4, 3, 3, 6, 7, 7, 7, 8, 9, 10, 9, 8, 7,
    7, 6, 11, 12, 13, 11, 6, 7, 8, 9, 14, 10, 9, 8, 6, 11,
    12, 13, 11, 6, 9, 14, 10, 9, 11, 12, 13, 11, 14, 10, 12,
];

/// Table 9-43 — ctxIdxInc for `last_significant_coeff_flag` in 8x8 luma/Cb/Cr
/// blocks (ctxBlockCat 5, 9, 13). The frame/field columns collapse to the same
/// mapping. Indexed by levelListIdx in 0..=62.
pub const LAST_SIG_COEFF_FLAG_CTX_IDX_INC_8X8: [u8; 63] = [
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4,
    5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8,
];
