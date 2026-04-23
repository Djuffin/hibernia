use super::macroblock::{
    get_4x4chroma_block_neighbor, get_4x4luma_block_neighbor, BMb, BMbType, BSubMacroblock,
    BSubMbType, IMb, IMbType, Intra_4x4_SamplePredMode, Macroblock, MbNeighborName,
    MbPredictionMode, MotionVector, PMb, PMbType, PartitionInfo, PcmMb, SubMacroblock, SubMbType,
};
use super::parser::{
    calc_prev_intra4x4_pred_mode, calculate_motion, calculate_motion_b, more_rbsp_data, BitReader,
    ParseResult,
};
use super::residual::Residual;
use super::slice::{Slice, SliceType};
use super::tables;
use super::ColorPlane;
use crate::{cast_or_error, read_value};
use log::trace;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct CoeffToken {
    pub total_coeffs: u8,
    pub trailing_ones: u8,
    pub pattern_len: u8,
}

impl CoeffToken {
    pub fn is_valid(&self) -> bool {
        self.pattern_len > 0
    }
}

pub(crate) type BitPattern = (/* bit pattern */ u16, /* length */ u8);

// Generates a 16KB Lookup Table (LUT) for `total_zeros` decoding at compile time.
//
// What is this LUT?
// It maps `(vlc_idx, top_9_bits_of_input)` to `(total_zeros, bit_length)`.
// `vlc_idx` (1..15) is determined by `total_coeffs`.
// The inner array has 512 entries, covering all possible 9-bit patterns (2^9 = 512).
//
// How it works:
// The table `TABLE9_7AND8` contains variable-length codes.
// Shorter codes are "expanded" to fill all 512-entry slots that start with that code's prefix.
// This allows O(1) lookup by simply indexing with the top 9 bits of the bitstream.
const fn init_total_zeros_lut() -> [[(u8, u8); 512]; 16] {
    let mut lut = [[(0, 0); 512]; 16];

    let mut vlc_idx = 1;
    while vlc_idx <= 15 {
        let mut row_idx = 0;
        while row_idx < 16 {
            let row = tables::TABLE9_7AND8[row_idx];
            let (pattern, len) = match vlc_idx {
                1 => row.1,
                2 => row.2,
                3 => row.3,
                4 => row.4,
                5 => row.5,
                6 => row.6,
                7 => row.7,
                8 => row.8,
                9 => row.9,
                10 => row.10,
                11 => row.11,
                12 => row.12,
                13 => row.13,
                14 => row.14,
                15 => row.15,
                _ => (0, 0),
            };

            if len > 0 {
                let total_zeros = row.0;
                let shift = 9 - len;
                let start = (pattern as usize) << shift;
                let end = ((pattern as usize) + 1) << shift;
                let mut i = start;
                while i < end {
                    lut[vlc_idx][i] = (total_zeros, len);
                    i += 1;
                }
            }
            row_idx += 1;
        }
        vlc_idx += 1;
    }
    lut
}

static TOTAL_ZEROS_LUT: [[(u8, u8); 512]; 16] = init_total_zeros_lut();

// Naive implementation of Tables 9-7, 9-8 lookup for total_zeros patterns
fn lookup_total_zeros(bits: u16, vlc_idx: u8) -> (u8, u8) {
    if vlc_idx == 0 || vlc_idx > 15 {
        return (0, 0);
    }
    let index = (bits >> 7) as usize;
    TOTAL_ZEROS_LUT[vlc_idx as usize][index]
}

// Generates a LUT for `total_zeros_chroma` decoding at compile time.
const fn init_total_zeros_chroma_lut() -> [[(u8, u8); 8]; 4] {
    let mut lut = [[(0, 0); 8]; 4];

    let mut vlc_idx = 1;
    while vlc_idx <= 3 {
        let mut row_idx = 0;
        while row_idx < 4 {
            let row = tables::TABLE9_9A[row_idx];
            let (pattern, len) = match vlc_idx {
                1 => row.1,
                2 => row.2,
                3 => row.3,
                _ => (0, 0),
            };

            if len > 0 {
                let total_zeros = row.0;
                let shift = 3 - len;
                let start = (pattern as usize) << shift;
                let end = ((pattern as usize) + 1) << shift;
                let mut i = start;
                while i < end {
                    lut[vlc_idx][i] = (total_zeros, len);
                    i += 1;
                }
            }
            row_idx += 1;
        }
        vlc_idx += 1;
    }
    lut
}

static TOTAL_ZEROS_CHROMA_LUT: [[(u8, u8); 8]; 4] = init_total_zeros_chroma_lut();

// Naive implementation of Tables 9-9 total_zeros patterns
fn lookup_total_zeros_chroma(bits: u16, vlc_idx: u8) -> (u8, u8) {
    if vlc_idx < 1 || vlc_idx > 3 {
        return (0, 0);
    }
    let index = (bits >> 13) as usize;
    TOTAL_ZEROS_CHROMA_LUT[vlc_idx as usize][index]
}

// Naive implementation of Table 9-10 – Tables for run_before
fn lookup_run_before(bits: u16, zeros_left: u8) -> (u8, u8) {
    for row in tables::TABLE9_10 {
        let (pattern, pattern_len) = match zeros_left {
            0 => (0, 0),
            1 => row.1,
            2 => row.2,
            3 => row.3,
            4 => row.4,
            5 => row.5,
            6 => row.6,
            7.. => row.7,
        };
        if pattern_len == 0 {
            break;
        }
        let shift = u16::BITS - pattern_len as u32;
        let meaningful_bits = bits >> shift;
        if meaningful_bits == pattern {
            return (row.0, pattern_len);
        }
    }
    (0, 0)
}

// Naive implementation of Table 9-5 lookup for coeff_token patterns
fn lookup_coeff_token(bits: u16, nc: i32) -> CoeffToken {
    for row in tables::TABLE95 {
        let (pattern, pattern_len) = match nc {
            0 | 1 => row.1,
            2 | 3 => row.2,
            4..=7 => row.3,
            8.. => row.4,
            -1 => row.5,
            -2 => row.6,
            _ => (0, 0),
        };
        if pattern_len == 0 {
            break;
        }
        let shift = u16::BITS - pattern_len as u32;
        let meaningful_bits = bits >> shift;
        if meaningful_bits == pattern {
            let (trailing_ones, total_coeffs) = row.0;
            return CoeffToken { total_coeffs, trailing_ones, pattern_len };
        }
    }
    CoeffToken::default()
}

// Section 9.2.2.1 Parsing process for level_prefix
fn parse_level_prefix(input: &mut BitReader) -> ParseResult<u32> {
    input.read_till_one().map_err(|_e| "leadingZeroBits".to_owned())
}

// Parses a block of residual coefficients into `coeff_level` and returns
// the number of non-zero coefficients.
pub fn parse_residual_block(
    input: &mut BitReader,
    coeff_level: &mut [i32],
    nc: i32,
) -> ParseResult<u8> {
    trace!("CAVLC residual_block: len: {} nc: {}", coeff_level.len(), nc);
    debug_assert!(matches!(coeff_level.len(), 4 | 15 | 16));
    let max_num_coeff = coeff_level.len();
    let next_16_bits = input.peek_or_pad16().map_err(|e| "coeff_token: ".to_owned() + &e)?;
    let coeff_token = lookup_coeff_token(next_16_bits, nc);
    if !coeff_token.is_valid() {
        return Err(format!("Unknown coeff_token value: {:#016b} nc:{}", next_16_bits, nc));
    }
    input.skip(coeff_token.pattern_len as u32)?;
    let total_coeffs = coeff_token.total_coeffs as usize;
    trace!(
        "coeff_token total_coeffs: {} t1s: {}, bits:{}",
        total_coeffs,
        coeff_token.trailing_ones,
        coeff_token.pattern_len
    );

    if total_coeffs == 0 {
        return Ok(0);
    }

    // Section 9.2.2 Parsing process for level information
    let mut levels = [0; 16];
    let (lower_levels, higher_levels) =
        levels[..total_coeffs].split_at_mut(coeff_token.trailing_ones as usize);
    for level in lower_levels {
        let trailing_ones_sign_flag: i32;
        read_value!(input, trailing_ones_sign_flag, u, 1);
        *level = 1 - 2 * trailing_ones_sign_flag;
    }

    let mut suffix_len = if total_coeffs > 10 && coeff_token.trailing_ones < 3 { 1 } else { 0 };
    for (i, level) in higher_levels.iter_mut().enumerate() {
        let level_prefix = parse_level_prefix(input)?;
        let level_suffix_size = if level_prefix == 14 && suffix_len == 0 {
            4
        } else if level_prefix >= 15 {
            level_prefix - 3
        } else {
            suffix_len
        };

        let mut level_suffix = 0u32;
        if level_suffix_size > 0 {
            read_value!(input, level_suffix, u, level_suffix_size as u8);
        }
        let mut level_code = (std::cmp::min(15, level_prefix) << suffix_len) + level_suffix;
        if level_prefix >= 15 && suffix_len == 0 {
            level_code += 15;
        }
        if level_prefix >= 16 {
            level_code += (1 << (level_prefix - 3)) - 4096;
        }
        if i == 0 && coeff_token.trailing_ones < 3 {
            level_code += 2;
        }
        if level_code % 2 == 0 {
            *level = (level_code as i32 + 2) >> 1;
        } else {
            *level = (-(level_code as i32) - 1) >> 1;
        }
        if suffix_len == 0 {
            suffix_len = 1;
        }
        if suffix_len < 6 && level.abs() > (3 << (suffix_len - 1)) {
            suffix_len += 1;
        }
        trace!(
            "i: {i} level_prefix: {level_prefix} level_suffix_size: {level_suffix_size} \
                suffix_len: {suffix_len} coeff_level: {}",
            *level
        );
    }

    // Section 9.2.3 Parsing process for run information
    let mut zeros_left = if total_coeffs < max_num_coeff {
        let next_16_bits = input.peek_or_pad16().map_err(|e| "total_zeros: ".to_owned() + &e)?;
        let tz_vlc_index = total_coeffs as u8;
        let lookup_tz =
            if max_num_coeff == 4 { lookup_total_zeros_chroma } else { lookup_total_zeros };
        let (total_zeros, bits) = lookup_tz(next_16_bits, tz_vlc_index);
        if bits == 0 {
            return Err(format!(
                "Unknown total_zeros value: {:#016b} tz_vlc_index:{}",
                next_16_bits, tz_vlc_index
            ));
        }
        input.skip(bits as u32)?;
        trace!("total_zeros: {} bits:{}", total_zeros, bits);
        total_zeros
    } else {
        0
    };

    let mut runs = [0; 16];
    for run in runs.iter_mut().take(total_coeffs - 1) {
        *run = if zeros_left > 0 {
            let next_16_bits = input.peek_or_pad16().map_err(|e| "run_before: ".to_owned() + &e)?;
            let (run_before, bits) = lookup_run_before(next_16_bits, zeros_left);
            if bits == 0 {
                return Err(format!(
                    "Unknown run_before value: {:#016b} zeros_left:{}",
                    next_16_bits, zeros_left
                ));
            }
            input.skip(bits as u32)?;
            zeros_left -= run_before;
            trace!("run_before: {} bits:{}", run_before, bits);
            run_before
        } else {
            0
        }
    }

    // Section 9.2.4 Combining level and run information
    runs[total_coeffs - 1] = zeros_left;

    let mut coeff_num = -1isize;
    for i in (0..total_coeffs).rev() {
        coeff_num += (runs[i] + 1) as isize;
        coeff_level[coeff_num as usize] = levels[i];
    }
    trace!("coeff_level: {:?}", coeff_level);
    Ok(coeff_token.total_coeffs)
}

fn calculate_nc(slice: &Slice, blk_idx: u8, residual: &Residual, plane: ColorPlane) -> i32 {
    let get_block_neighbor =
        if plane.is_luma() { get_4x4luma_block_neighbor } else { get_4x4chroma_block_neighbor };

    let mut total_nc = 0;
    let mut nc_counted = 0;
    let this_mb_addr = slice.get_next_mb_addr();
    for neighbor in [MbNeighborName::A, MbNeighborName::B] {
        let (block_neighbor_idx, mb_neighbor) = get_block_neighbor(blk_idx, neighbor);
        if let Some(mb_neighbor) = mb_neighbor {
            if let Some(mb) = slice.get_mb_neighbor(this_mb_addr, mb_neighbor) {
                let nc = mb.get_nc(block_neighbor_idx, plane) as i32;
                total_nc += nc;
                nc_counted += 1;
            }
        } else {
            let nc = residual.get_nc(block_neighbor_idx, plane) as i32;
            total_nc += nc;
            nc_counted += 1;
        }
    }
    if nc_counted == 2 {
        total_nc = (total_nc + 1) / 2;
    }
    total_nc
}

fn parse_residual_luma(
    input: &mut BitReader,
    slice: &Slice,
    residual: &mut Residual,
) -> ParseResult<()> {
    trace!("parse_residual_luma");
    let pred_mode = residual.prediction_mode;
    let coded_block_pattern = residual.coded_block_pattern;
    if pred_mode == MbPredictionMode::Intra_16x16 {
        trace!(" luma DC");
        let nc = calculate_nc(slice, 0, residual, ColorPlane::Y);
        let levels = residual.get_dc_levels_for(ColorPlane::Y);
        parse_residual_block(input, levels, nc)?;
    }
    for i8x8 in 0..4 {
        if coded_block_pattern.luma() & (1 << i8x8) != 0 {
            for i4x4 in 0..4 {
                let blk_idx = i8x8 * 4 + i4x4;
                trace!(" luma BK {}", blk_idx);
                let nc = calculate_nc(slice, blk_idx, residual, ColorPlane::Y);
                let (levels_ref, total_coeff_ref) =
                    residual.get_ac_levels_for(blk_idx, ColorPlane::Y);
                *total_coeff_ref = parse_residual_block(input, levels_ref, nc)?;
            }
        }
    }
    Ok(())
}

pub fn parse_residual(
    input: &mut BitReader,
    slice: &Slice,
    residual: &mut Residual,
) -> ParseResult<()> {
    trace!("parse_residual");
    let pred_mode = residual.prediction_mode;
    let coded_block_pattern = residual.coded_block_pattern;
    parse_residual_luma(input, slice, residual)?;
    if slice.sps.ChromaArrayType().is_chroma_subsampled() {
        if coded_block_pattern.chroma() & 3 != 0 {
            for plane in [ColorPlane::Cb, ColorPlane::Cr] {
                let levels = residual.get_dc_levels_for(plane);
                trace!(" chroma {:?} DC", plane);
                let nc = -1; // Section 9.2.1, If ChromaArrayType is 1, nC = −1,
                parse_residual_block(input, levels, nc)?;
            }
        }

        for plane in [ColorPlane::Cb, ColorPlane::Cr] {
            if coded_block_pattern.chroma() & 2 != 0 {
                for blk_idx in 0..4 {
                    let nc = calculate_nc(slice, blk_idx, residual, plane);
                    let (levels_ref, total_coeff_ref) = residual.get_ac_levels_for(blk_idx, plane);

                    trace!(" chroma {:?} BK {}", plane, blk_idx);
                    *total_coeff_ref = parse_residual_block(input, levels_ref, nc)?;
                }
            }
        }
    } else {
        return Err("YUV 4:4:4 residual parsing is not supported".into());
    }
    Ok(())
}

// Section 7.3.5 Macroblock layer syntax
pub fn parse_macroblock(input: &mut BitReader, slice: &Slice) -> ParseResult<Macroblock> {
    let mb_type_val: u32;
    read_value!(input, mb_type_val, ue);

    if matches!(slice.header.slice_type, SliceType::I | SliceType::SI) {
        let mb_type = IMbType::try_from(mb_type_val)?;
        parse_i_macroblock(input, slice, mb_type)
    } else if slice.header.slice_type == SliceType::B {
        // Table 7-14: B slice mb_type 0-22 are B types, 23-48 are I types (subtract 23)
        if mb_type_val >= 23 {
            let mb_type = IMbType::try_from(mb_type_val - 23)?;
            parse_i_macroblock(input, slice, mb_type)
        } else {
            let mb_type = BMbType::try_from(mb_type_val)?;
            parse_b_macroblock(input, slice, mb_type)
        }
    } else if mb_type_val >= 5 {
        // The macroblock types for P and SP slices are specified in Tables 7-13 and 7-11.
        // mb_type values 0 to 4 are specified in Table 7-13 and mb_type values 5 to 30 are
        // specified in Table 7-11, indexed by subtracting 5 from the value of mb_type.
        let mb_type = IMbType::try_from(mb_type_val - 5)?;
        parse_i_macroblock(input, slice, mb_type)
    } else {
        let mb_type = PMbType::try_from(mb_type_val)?;
        parse_p_macroblock(input, slice, mb_type)
    }
}

pub fn parse_p_macroblock(
    input: &mut BitReader,
    slice: &Slice,
    mb_type: PMbType,
) -> ParseResult<Macroblock> {
    let mut block = PMb { mb_type, transform_size_8x8_flag: false, qp: 0, ..PMb::default() };
    let this_mb_addr = slice.get_next_mb_addr();
    let num_mb_part = block.NumMbPart();
    let mb_part_pred_mode = block.MbPartPredMode(0);

    let mut partitions = [PartitionInfo::default(); 4];
    let mut sub_macroblocks = [SubMacroblock::default(); 4];

    if mb_part_pred_mode != MbPredictionMode::None {
        // P_L0_16x16, P_L0_L0_16x8, P_L0_L0_8x16
        assert!(num_mb_part <= partitions.len());

        if mb_part_pred_mode != MbPredictionMode::Pred_L1 {
            if slice.header.num_ref_idx_l0_active_minus1 > 0 {
                for i in 0..num_mb_part {
                    read_value!(
                        input,
                        partitions[i].ref_idx_l0,
                        te,
                        slice.header.num_ref_idx_l0_active_minus1
                    );
                }
            }

            for i in 0..num_mb_part {
                let mvd_x: i32;
                let mvd_y: i32;
                read_value!(input, mvd_x, se);
                read_value!(input, mvd_y, se);
                partitions[i].mvd_l0 = MotionVector { x: mvd_x as i16, y: mvd_y as i16 };
            }
        }
    } else {
        // P_8x8 or P_8x8ref0
        for i in 0..sub_macroblocks.len() {
            // 4 sub-macroblocks
            let sub_mb_type_val: u32;
            read_value!(input, sub_mb_type_val, ue);
            let sub_mb_type = SubMbType::try_from(sub_mb_type_val)?;
            sub_macroblocks[i].sub_mb_type = sub_mb_type;
        }

        let mut ref_idx_l0 = [0u8; 4];
        if slice.header.num_ref_idx_l0_active_minus1 > 0 {
            for i in 0..4 {
                if mb_type == PMbType::P_8x8ref0 {
                    ref_idx_l0[i] = 0;
                } else {
                    read_value!(
                        input,
                        ref_idx_l0[i],
                        te,
                        slice.header.num_ref_idx_l0_active_minus1
                    );
                }
            }
        }

        let mut mvd_l0 = [[MotionVector::default(); 4]; 4]; // For each sub-mb, for each partition
        for i in 0..sub_macroblocks.len() {
            // sub-macroblock index
            let num_sub_mb_part = sub_macroblocks[i].sub_mb_type.NumSubMbPart();
            for j in 0..num_sub_mb_part {
                // sub-macroblock partition index
                let mvd_x: i32;
                let mvd_y: i32;
                read_value!(input, mvd_x, se);
                read_value!(input, mvd_y, se);
                mvd_l0[i][j] = MotionVector { x: mvd_x as i16, y: mvd_y as i16 };
            }
        }

        for i in 0..sub_macroblocks.len() {
            let num_sub_mb_part = sub_macroblocks[i].sub_mb_type.NumSubMbPart();
            for j in 0..num_sub_mb_part {
                sub_macroblocks[i].partitions[j].ref_idx_l0 = ref_idx_l0[i];
                sub_macroblocks[i].partitions[j].mvd_l0 = mvd_l0[i][j];
            }
        }
    }

    block.motion = calculate_motion(slice, this_mb_addr, mb_type, &partitions, &sub_macroblocks);

    let coded_block_pattern_num: u8;
    read_value!(input, coded_block_pattern_num, ue, 8);
    block.coded_block_pattern =
        tables::code_num_to_inter_coded_block_pattern(coded_block_pattern_num)
            .ok_or("Invalid coded_block_pattern")?;

    if !block.coded_block_pattern.is_zero() {
        read_value!(input, block.mb_qp_delta, se);
        let mut residual = Box::<Residual>::default();
        residual.coded_block_pattern = block.coded_block_pattern;
        residual.prediction_mode = block.MbPartPredMode(0);
        parse_residual(input, slice, &mut residual)?;
        block.residual = Some(residual);
    }

    Ok(Macroblock::P(block))
}

pub fn parse_b_macroblock(
    input: &mut BitReader,
    slice: &Slice,
    mb_type: BMbType,
) -> ParseResult<Macroblock> {
    let mut block = BMb { mb_type, ..BMb::default() };
    let this_mb_addr = slice.get_next_mb_addr();
    let num_mb_part = mb_type.NumMbPart();

    let mut partitions = [PartitionInfo::default(); 4];
    let mut sub_macroblocks = [BSubMacroblock::default(); 4];

    if mb_type == BMbType::B_Direct_16x16 {
        // No ref_idx or mvd parsed — motion derived via direct prediction
    } else if mb_type == BMbType::B_8x8 {
        // Section 7.3.5.1: sub_mb_pred for B_8x8
        // Parse 4 sub-macroblock types
        for i in 0..4 {
            let sub_mb_type_val: u32;
            read_value!(input, sub_mb_type_val, ue);
            sub_macroblocks[i].sub_mb_type = BSubMbType::try_from(sub_mb_type_val)?;
        }

        // ref_idx_l0 for each 8x8 partition where SubMbPredMode != Pred_L1 and != Direct
        let mut ref_idx_l0 = [0u8; 4];
        for i in 0..4 {
            let mode = sub_macroblocks[i].sub_mb_type.SubMbPredMode();
            if mode != MbPredictionMode::Pred_L1 && mode != MbPredictionMode::Direct
                && slice.header.num_ref_idx_l0_active_minus1 > 0 {
                    read_value!(
                        input,
                        ref_idx_l0[i],
                        te,
                        slice.header.num_ref_idx_l0_active_minus1
                    );
                }
        }

        // ref_idx_l1 for each 8x8 partition where SubMbPredMode != Pred_L0 and != Direct
        let mut ref_idx_l1 = [0u8; 4];
        for i in 0..4 {
            let mode = sub_macroblocks[i].sub_mb_type.SubMbPredMode();
            if mode != MbPredictionMode::Pred_L0 && mode != MbPredictionMode::Direct
                && slice.header.num_ref_idx_l1_active_minus1 > 0 {
                    read_value!(
                        input,
                        ref_idx_l1[i],
                        te,
                        slice.header.num_ref_idx_l1_active_minus1
                    );
                }
        }

        // mvd_l0 for sub-partitions where mode != Pred_L1 and != Direct
        let mut mvd_l0 = [[MotionVector::default(); 4]; 4];
        for i in 0..4 {
            let mode = sub_macroblocks[i].sub_mb_type.SubMbPredMode();
            if mode != MbPredictionMode::Pred_L1 && mode != MbPredictionMode::Direct {
                let num_sub_mb_part = sub_macroblocks[i].sub_mb_type.NumSubMbPart();
                for j in 0..num_sub_mb_part {
                    let mvd_x: i32;
                    let mvd_y: i32;
                    read_value!(input, mvd_x, se);
                    read_value!(input, mvd_y, se);
                    mvd_l0[i][j] = MotionVector { x: mvd_x as i16, y: mvd_y as i16 };
                }
            }
        }

        // mvd_l1 for sub-partitions where mode != Pred_L0 and != Direct
        let mut mvd_l1 = [[MotionVector::default(); 4]; 4];
        for i in 0..4 {
            let mode = sub_macroblocks[i].sub_mb_type.SubMbPredMode();
            if mode != MbPredictionMode::Pred_L0 && mode != MbPredictionMode::Direct {
                let num_sub_mb_part = sub_macroblocks[i].sub_mb_type.NumSubMbPart();
                for j in 0..num_sub_mb_part {
                    let mvd_x: i32;
                    let mvd_y: i32;
                    read_value!(input, mvd_x, se);
                    read_value!(input, mvd_y, se);
                    mvd_l1[i][j] = MotionVector { x: mvd_x as i16, y: mvd_y as i16 };
                }
            }
        }

        // Fill sub-macroblock partition info
        for i in 0..4 {
            let num_sub_mb_part = sub_macroblocks[i].sub_mb_type.NumSubMbPart();
            for j in 0..num_sub_mb_part {
                sub_macroblocks[i].partitions[j].ref_idx_l0 = ref_idx_l0[i];
                sub_macroblocks[i].partitions[j].mvd_l0 = mvd_l0[i][j];
                sub_macroblocks[i].partitions[j].ref_idx_l1 = ref_idx_l1[i];
                sub_macroblocks[i].partitions[j].mvd_l1 = mvd_l1[i][j];
            }
        }
    } else {
        // Non-direct, non-8x8: 16x16, 16x8, 8x16
        // ref_idx_l0 for partitions where MbPartPredMode != Pred_L1
        for i in 0..num_mb_part {
            let mode = mb_type.MbPartPredMode(i);
            if mode != MbPredictionMode::Pred_L1
                && slice.header.num_ref_idx_l0_active_minus1 > 0 {
                    read_value!(
                        input,
                        partitions[i].ref_idx_l0,
                        te,
                        slice.header.num_ref_idx_l0_active_minus1
                    );
                }
        }

        // ref_idx_l1 for partitions where MbPartPredMode != Pred_L0
        for i in 0..num_mb_part {
            let mode = mb_type.MbPartPredMode(i);
            if mode != MbPredictionMode::Pred_L0
                && slice.header.num_ref_idx_l1_active_minus1 > 0 {
                    read_value!(
                        input,
                        partitions[i].ref_idx_l1,
                        te,
                        slice.header.num_ref_idx_l1_active_minus1
                    );
                }
        }

        // mvd_l0 for partitions where MbPartPredMode != Pred_L1
        for i in 0..num_mb_part {
            let mode = mb_type.MbPartPredMode(i);
            if mode != MbPredictionMode::Pred_L1 {
                let mvd_x: i32;
                let mvd_y: i32;
                read_value!(input, mvd_x, se);
                read_value!(input, mvd_y, se);
                partitions[i].mvd_l0 = MotionVector { x: mvd_x as i16, y: mvd_y as i16 };
            }
        }

        // mvd_l1 for partitions where MbPartPredMode != Pred_L0
        for i in 0..num_mb_part {
            let mode = mb_type.MbPartPredMode(i);
            if mode != MbPredictionMode::Pred_L0 {
                let mvd_x: i32;
                let mvd_y: i32;
                read_value!(input, mvd_x, se);
                read_value!(input, mvd_y, se);
                partitions[i].mvd_l1 = MotionVector { x: mvd_x as i16, y: mvd_y as i16 };
            }
        }
    }

    block.motion = calculate_motion_b(slice, this_mb_addr, mb_type, &partitions, &sub_macroblocks);

    let coded_block_pattern_num: u8;
    read_value!(input, coded_block_pattern_num, ue, 8);
    block.coded_block_pattern =
        tables::code_num_to_inter_coded_block_pattern(coded_block_pattern_num)
            .ok_or("Invalid coded_block_pattern")?;

    if !block.coded_block_pattern.is_zero() {
        read_value!(input, block.mb_qp_delta, se);
        let mut residual = Box::<Residual>::default();
        residual.coded_block_pattern = block.coded_block_pattern;
        residual.prediction_mode = block.mb_type.MbPartPredMode(0);
        parse_residual(input, slice, &mut residual)?;
        block.residual = Some(residual);
    }

    Ok(Macroblock::B(block))
}

pub fn parse_i_macroblock(
    input: &mut BitReader,
    slice: &Slice,
    mb_type: IMbType,
) -> ParseResult<Macroblock> {
    if mb_type == IMbType::I_PCM {
        let mut block = PcmMb { qp: 0, ..PcmMb::default() };
        input.align();

        let luma_samples =
            tables::MB_WIDTH * tables::MB_HEIGHT * tables::BIT_DEPTH / (u8::BITS as usize);
        let chroma_samples = luma_samples >> 2; // assuming i420
        block.pcm_sample_luma.reserve(luma_samples);
        for _ in 0..luma_samples {
            block.pcm_sample_luma.push(input.u(8)? as u8);
        }
        block.pcm_sample_chroma_cb.reserve(chroma_samples);
        for _ in 0..chroma_samples {
            block.pcm_sample_chroma_cb.push(input.u(8)? as u8);
        }
        for _ in 0..chroma_samples {
            block.pcm_sample_chroma_cr.push(input.u(8)? as u8);
        }
        Ok(Macroblock::PCM(block))
    } else {
        let mut block = IMb { mb_type, qp: 0, ..IMb::default() };
        let this_mb_addr = slice.get_next_mb_addr();
        if slice.pps.transform_8x8_mode_flag && block.mb_type == IMbType::I_NxN {
            read_value!(input, block.transform_size_8x8_flag, f);
        }
        match block.MbPartPredMode(0) {
            MbPredictionMode::Intra_4x4 => {
                trace!("4x4 luma predictions");
                for blk_idx in 0..block.rem_intra4x4_pred_mode.len() {
                    let prev_pred_mode =
                        calc_prev_intra4x4_pred_mode(slice, &block, this_mb_addr, blk_idx);

                    let prev_intra4x4_pred_mode_flag: bool;
                    read_value!(input, prev_intra4x4_pred_mode_flag, f);
                    let mode = &mut block.rem_intra4x4_pred_mode[blk_idx];
                    if prev_intra4x4_pred_mode_flag {
                        *mode = prev_pred_mode
                    } else {
                        let rem_intra4x4_pred_mode: Intra_4x4_SamplePredMode;
                        read_value!(input, rem_intra4x4_pred_mode, u, 3);
                        if rem_intra4x4_pred_mode < prev_pred_mode {
                            *mode = rem_intra4x4_pred_mode;
                        } else {
                            *mode = ((rem_intra4x4_pred_mode as u32) + 1)
                                .try_into()
                                .map_err(|e| "rem_intra4x4_pred_mode is too large".to_string())?;
                        }
                    }
                    trace!("  blk:{blk_idx} prev: {prev_pred_mode} actual: {}", *mode);
                }
            }
            MbPredictionMode::Intra_16x16 => {}
            _ => return Err("Intra_8x8 prediction is not supported".into()),
        };
        if slice.sps.ChromaArrayType().is_chroma_subsampled() {
            read_value!(input, block.intra_chroma_pred_mode, ue, 2);
        }
        if block.MbPartPredMode(0) == MbPredictionMode::Intra_16x16 {
            block.coded_block_pattern = tables::mb_type_to_coded_block_pattern(block.mb_type)
                .ok_or("Invalid coded_block_pattern")?;
        } else {
            let coded_block_pattern_num: u8;
            read_value!(input, coded_block_pattern_num, ue, 8);
            block.coded_block_pattern =
                tables::code_num_to_intra_coded_block_pattern(coded_block_pattern_num)
                    .ok_or("Invalid coded_block_pattern")?;
        }

        let mut result;
        if !block.coded_block_pattern.is_zero()
            || block.MbPartPredMode(0) == MbPredictionMode::Intra_16x16
        {
            read_value!(input, block.mb_qp_delta, se);
            let mut residual = Box::<Residual>::default();
            residual.coded_block_pattern = block.coded_block_pattern;
            residual.prediction_mode = block.MbPartPredMode(0);
            parse_residual(input, slice, &mut residual)?;
            result = Macroblock::I(block);
            result.set_residual(Some(residual));
        } else {
            result = Macroblock::I(block);
        }

        Ok(result)
    }
}

pub fn parse_slice_data_cavlc(input: &mut BitReader, slice: &mut Slice) -> ParseResult<()> {
    loop {
        let pic_size_in_mbs = slice.sps.pic_size_in_mbs();
        if slice.header.slice_type != SliceType::I && slice.header.slice_type != SliceType::SI {
            let mb_skip_run: usize;
            read_value!(input, mb_skip_run, ue);
            let blocks_left = pic_size_in_mbs - slice.get_next_mb_addr() as usize;
            if mb_skip_run > blocks_left {
                return Err(format!(
                    "Trying to skip {mb_skip_run} blocks, only {blocks_left} blocks left"
                ));
            }
            for _ in 0..mb_skip_run {
                let curr_mb_addr = slice.get_next_mb_addr();
                if slice.header.slice_type == SliceType::B {
                    let default_partitions = [PartitionInfo::default(); 4];
                    let default_sub_mbs = [BSubMacroblock::default(); 4];
                    let motion = calculate_motion_b(
                        slice,
                        curr_mb_addr,
                        BMbType::B_Skip,
                        &default_partitions,
                        &default_sub_mbs,
                    );
                    let block = Macroblock::B(BMb {
                        mb_type: BMbType::B_Skip,
                        motion,
                        ..Default::default()
                    });
                    slice.append_mb(block);
                } else {
                    let default_partitions = [PartitionInfo::default(); 4];
                    let default_sub_mbs = [SubMacroblock::default(); 4];
                    let motion = calculate_motion(
                        slice,
                        curr_mb_addr,
                        PMbType::P_Skip,
                        &default_partitions,
                        &default_sub_mbs,
                    );
                    let block = Macroblock::P(PMb {
                        mb_type: PMbType::P_Skip,
                        motion,
                        ..Default::default()
                    });
                    slice.append_mb(block);
                }
            }
            if slice.get_macroblock_count() >= pic_size_in_mbs {
                break;
            }
            if mb_skip_run > 0 && !more_rbsp_data(input) {
                break;
            }
        }

        let next_mb_addr = slice.get_next_mb_addr() as usize;
        trace!("=============== Parsing macroblock: {next_mb_addr} ===============");
        let block = parse_macroblock(input, slice)?;
        slice.append_mb(block);
        if slice.get_macroblock_count() >= pic_size_in_mbs {
            break;
        }
        if !more_rbsp_data(input) {
            break;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prepare_bits(bit_str: &str) -> u16 {
        let value = u16::from_str_radix(bit_str, 2).unwrap();
        value << (u16::BITS - bit_str.len() as u32)
    }

    fn prepare_bit_vec(bit_str: &str) -> Vec<u8> {
        let mut result = Vec::<u8>::new();
        for byte in bit_str.split(' ') {
            result.push(u8::from_str_radix(byte, 2).unwrap());
        }

        result
    }

    #[test]
    pub fn test_parse_residual() {
        // Examples from the book:
        // The H.264 advanced video compression standard / Iain E. Richardson.
        let data = prepare_bit_vec("00001000 11100101 11101101");
        let mut output = [0i32; 16];

        assert_eq!(parse_residual_block(&mut BitReader::new(&data), &mut output, 0).unwrap(), 5);
        assert_eq!(output, [0, 3, 0, 1, -1, -1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0]);

        let data = prepare_bit_vec("00000001 10100010 01000010 11100110 0");
        output.fill(0);
        assert_eq!(parse_residual_block(&mut BitReader::new(&data), &mut output, 0).unwrap(), 5);
        assert_eq!(output, [-2, 4, 3, -3, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        let data = prepare_bit_vec("00011100 01110010");
        output.fill(0);
        assert_eq!(parse_residual_block(&mut BitReader::new(&data), &mut output, 0).unwrap(), 3);
        assert_eq!(output, [0, 0, 0, 1, 0, 1, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0]);

        // Example from the article:
        // VHDL Implementation of an Efficient Context Adaptive
        // Variable Length Coding Algorithm of H.264 Video Codec
        let data = prepare_bit_vec("01101000 10001111 11001001 11011110 0");
        output.fill(0);
        assert_eq!(parse_residual_block(&mut BitReader::new(&data), &mut output, 5).unwrap(), 8);
        assert_eq!(output, [1, 1, -2, -4, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0]);

        // Examples from the article:
        // IMPLEMENTATION OF CONTEXT ADAPTIVE VARIABLE LENGTH CODING AND DECODING
        // ALGORITHM FOR H.264 VIDEO CODEC USING MATLAB
        let data = prepare_bit_vec("01000100 01001000 01011100 11000000");
        output.fill(0);
        assert_eq!(parse_residual_block(&mut BitReader::new(&data), &mut output, 4).unwrap(), 5);
        assert_eq!(output, [-2, 4, 3, -3, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    pub fn test_parse_residual_refcase1() {
        /*
            @254F: coeff_token (total_coeff =  5 / t1s =  3) :     : (5)  : [00110]
            @2554: trailing_ones_sign                        :  -1 : (1)  : [1]
            @2555: trailing_ones_sign                        :  -1 : (1)  : [1]
            @2556: trailing_ones_sign                        :   1 : (1)  : [0]
            @2557: coeff_level                               :  -1 : (2)  : [01]
            @2559: coeff_level                               :  -2 : (3)  : [011]
            @255C: total_zeros                               :  10 : (4)  : [0001]
            @2560: run_before                                :  10 : (7)  : [0000001]
        */
        let data = prepare_bit_vec("00110110 01011000 10000001");
        let mut output = [0i32; 16];
        assert_eq!(parse_residual_block(&mut BitReader::new(&data), &mut output, 2).unwrap(), 5);
        // Output might be not 100% accurate :\
        assert_eq!(output, [-2, -1, 1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0]);
    }

    #[test]
    pub fn test_parse_residual_refcase2() {
        /*
            @68  : coeff_token (total_coeff =  4 / t1s =  1) :     : (9)  : [000000110]
            @71  : trailing_ones_sign                        :  -1 : (1)  : [1]
            @72  : coeff_level                               :  -5 : (10) : [0000000001]
            @7C  : coeff_level                               : -19 : (12) : [000000000101]
            @88  : coeff_level                               :   6 : (5)  : [01010]
            @8D  : total_zeros                               :   3 : (4)  : [0100]
            @91  : run_before                                :   0 : (2)  : [11]
            @93  : run_before                                :   3 : (2)  : [00]
        */
        let data = prepare_bit_vec("00000011 01000000 00010000 00000101 01010010 01100000");
        let mut output = [0i32; 16];
        assert_eq!(parse_residual_block(&mut BitReader::new(&data), &mut output, 0).unwrap(), 4);
        // Output might be not 100% accurate :\
        assert_eq!(output, [6, -19, 0, 0, 0, -6, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    pub fn test_lookup_run_before() {
        for i in 3..7 {
            assert_eq!(lookup_run_before(prepare_bits("11"), i), (0, 2));
        }

        assert_eq!(lookup_run_before(prepare_bits("010"), 5), (3, 3));
        assert_eq!(lookup_run_before(prepare_bits("0000001"), 7), (10, 7));
        assert_eq!(lookup_run_before(prepare_bits("0000000001"), 7), (13, 10));
        assert_eq!(lookup_run_before(prepare_bits("00000000001"), 7), (14, 11));
        assert_eq!(lookup_run_before(prepare_bits("110"), 7), (1, 3));
        assert_eq!(lookup_run_before(prepare_bits("10"), 5), (1, 2));
        assert_eq!(lookup_run_before(prepare_bits("101"), 6), (5, 3));
        assert_eq!(lookup_run_before(prepare_bits("00"), 2), (2, 2));
    }

    #[test]
    pub fn brute_force_test_lookup_total_zeros() {
        for vcl_idx in 1..16 {
            let mut recognizer_patterns = [false; 16];
            for bits in 0..=u16::MAX {
                let result = lookup_total_zeros(bits, vcl_idx);
                if result.1 == 0 {
                    continue;
                }
                assert!(result.1 <= 9);
                assert!(result.0 < 17 - vcl_idx);
                recognizer_patterns[result.0 as usize] = true;
            }
            for i in 0..(17 - vcl_idx) {
                assert!(recognizer_patterns[i as usize], "vcl_idx:{:} i:{:}", vcl_idx, i);
            }
        }
    }

    #[test]
    pub fn test_lookup_total_zeros() {
        assert_eq!(lookup_total_zeros(prepare_bits("00000010"), 1), (12, 8));
        assert_eq!(lookup_total_zeros(prepare_bits("110"), 3), (2, 3));
        assert_eq!(lookup_total_zeros(prepare_bits("000000"), 7), (9, 6));
        assert_eq!(lookup_total_zeros(prepare_bits("000001"), 6), (0, 6));

        assert_eq!(lookup_total_zeros(prepare_bits("000001"), 8), (0, 6));
        assert_eq!(lookup_total_zeros(prepare_bits("1"), 15), (1, 1));
        assert_eq!(lookup_total_zeros(prepare_bits("00000"), 10), (1, 5));
        assert_eq!(lookup_total_zeros(prepare_bits("0000"), 12), (0, 4));
        assert_eq!(lookup_total_zeros(prepare_bits("00001"), 9), (7, 5));
    }

    #[test]
    pub fn test_lookup_total_zeros_chroma() {
        assert_eq!(lookup_total_zeros_chroma(prepare_bits("1"), 1), (0, 1));
        assert_eq!(lookup_total_zeros_chroma(prepare_bits("1"), 2), (0, 1));
        assert_eq!(lookup_total_zeros_chroma(prepare_bits("1"), 3), (0, 1));

        assert_eq!(lookup_total_zeros_chroma(prepare_bits("01"), 1), (1, 2));
        assert_eq!(lookup_total_zeros_chroma(prepare_bits("01"), 2), (1, 2));
        assert_eq!(lookup_total_zeros_chroma(prepare_bits("0"), 3), (1, 1));

        assert_eq!(lookup_total_zeros_chroma(prepare_bits("001"), 1), (2, 3));
        assert_eq!(lookup_total_zeros_chroma(prepare_bits("00"), 2), (2, 2));

        assert_eq!(lookup_total_zeros_chroma(prepare_bits("000"), 1), (3, 3));
    }

    #[test]
    pub fn test_lookup_coeff_token() {
        assert_eq!(
            lookup_coeff_token(prepare_bits("0000100"), 0),
            CoeffToken { total_coeffs: 5, trailing_ones: 3, pattern_len: 7 }
        );

        assert_eq!(
            lookup_coeff_token(prepare_bits("0000000110"), 0),
            CoeffToken { total_coeffs: 5, trailing_ones: 1, pattern_len: 10 }
        );

        assert_eq!(
            lookup_coeff_token(prepare_bits("00011"), 0),
            CoeffToken { total_coeffs: 3, trailing_ones: 3, pattern_len: 5 }
        );
    }

    #[test]
    pub fn brute_force_test_lookup_coeff_token() {
        for nc in [0, 2, 4, 8, -1, -2, -3] {
            let mut recognizer_patterns = [[false; 4]; 17];
            for bits in 0..=u16::MAX {
                let result = lookup_coeff_token(bits, nc);
                if !result.is_valid() {
                    continue;
                }
                if result.pattern_len == 1 {
                    assert!(bits & 0x8000 != 0);
                }
                recognizer_patterns[result.total_coeffs as usize][result.trailing_ones as usize] =
                    true;
                assert!(result.trailing_ones <= 3);
                match nc {
                    -1 => {
                        assert!(result.pattern_len <= 8);
                        assert!(result.total_coeffs <= 4);
                    }
                    -2 => {
                        assert!(result.pattern_len <= 13);
                        assert!(result.total_coeffs <= 8);
                    }
                    8 => {
                        assert_eq!(result.pattern_len, 6);
                        assert!(result.total_coeffs <= 16);
                    }
                    _ => {
                        assert!(result.pattern_len <= 16);
                        assert!(result.total_coeffs <= 16);
                    }
                };
            }

            match nc {
                -1 => {
                    for total in 0..16 {
                        for ones in 0..4 {
                            let value = total <= 4 && ones <= total;
                            assert_eq!(recognizer_patterns[total][ones], value);
                        }
                    }
                }
                -2 => {
                    for total in 0..16 {
                        for ones in 0..4 {
                            let value = total <= 8 && ones <= total;
                            assert_eq!(recognizer_patterns[total][ones], value);
                        }
                    }
                }
                -3 => {
                    for total in 0..16 {
                        for ones in 0..4 {
                            assert!(!recognizer_patterns[total][ones]);
                        }
                    }
                }
                _ => {
                    for total in 0..16 {
                        for ones in 0..4 {
                            let value = ones <= total;
                            assert_eq!(recognizer_patterns[total][ones], value);
                        }
                    }
                }
            };
        }
    }
}
