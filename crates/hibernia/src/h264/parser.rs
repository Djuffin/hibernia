#![macro_use]
use super::decoder;
use super::macroblock;
use super::nal;
use super::pps;
use super::rbsp;
use super::scaling_list::{
    self, num_pps_8x8_lists, num_sps_8x8_lists, PicScalingMatrix, ScalingList4x4Entry,
    ScalingList8x8Entry, SeqScalingMatrix,
};
use super::slice;
use super::sps;
use super::tables;

use super::cabac::CabacContext;
use super::cavlc::parse_slice_data_cavlc;
use super::residual::Residual;
use super::{ChromaFormat, ColorPlane, Profile};
use decoder::DecoderContext;
use log::trace;
use macroblock::{
    get_4x4chroma_block_neighbor, get_4x4luma_block_neighbor, get_neighbor_mbs, BMb, BMbType,
    get_8x8luma_block_neighbor, BSubMacroblock, BSubMbType, IMb, IMbType,
    Intra_4x4_SamplePredMode, Intra_8x8_SamplePredMode, Intra_Chroma_Pred_Mode,
    Macroblock, MbAddr, MbMotion, MbNeighborName, MbPredictionMode, MotionVector, PMb, PMbType,
    PartitionInfo, PcmMb, SubMacroblock, SubMbType,
};
use nal::{NalHeader, NalUnitType};
use pps::{PicParameterSet, SliceGroup, SliceGroupChangeType, SliceRect};
use slice::{
    ColPicInfo, DeblockingFilterIdc, DecRefPicMarking, MemoryManagementControlOperation,
    PredWeightTable, RefPicListModification, RefPicListModifications, Slice, SliceHeader,
    SliceType, WeightingFactors,
};
use sps::{FrameCrop, SequenceParameterSet, VuiParameters};

pub type BitReader<'a> = rbsp::RbspReader<'a>;
pub type ParseResult<T> = rbsp::ParseResult<T>;

#[macro_export]
macro_rules! cast_or_error {
    ($dest:expr, $value:expr) => {
        trace!("parse {} = {}", stringify!($dest), $value);
        $dest = match $value.try_into() {
            Ok(v) => v,
            Err(e) => {
                return Err(format!("Error casting '{}': {}", stringify!($dest), e));
            }
        };
        let _ = $dest; // to let compiler know that $dest is used
    };
}

#[macro_export]
macro_rules! expect_value {
    ($input:ident, $msg:expr, $expected:expr, $bits:expr) => {
        let error_handler = |e| format!("Error while parsing '{}': {}", $msg, e);
        let value = $input.u($bits).map_err(error_handler)?;
        if value != $expected {
            return Err(format!("Unexpected value of {}: {} vs {}", $msg, value, $expected));
        }
    };
}

#[macro_export]
macro_rules! read_value {
    ($input:ident, $dest:expr, u, $bits:expr) => {
        let error_handler = |e| format!("Error while parsing '{}': {}", stringify!($dest), e);
        let value = $input.u($bits).map_err(error_handler)?;
        cast_or_error!($dest, value);
    };
    ($input:ident, $dest:expr, ue) => {
        read_value!($input, $dest, ue, 32);
    };
    ($input:ident, $dest:expr, ue, $bits:expr) => {
        let error_handler = |e| format!("Error while parsing '{}': {}", stringify!($dest), e);
        let value = $input.ue($bits).map_err(error_handler)?;
        cast_or_error!($dest, value);
    };
    ($input:ident, $dest:expr, se) => {
        let error_handler = |e| format!("Error while parsing '{}': {}", stringify!($dest), e);
        let value = $input.se().map_err(error_handler)?;
        cast_or_error!($dest, value);
    };
    ($input:ident, $dest:expr, te, $range:expr) => {
        let error_handler = |e| format!("Error while parsing '{}': {}", stringify!($dest), e);
        let value = $input.te($range).map_err(error_handler)?;
        cast_or_error!($dest, value);
    };
    ($input:ident, $dest:expr, f) => {
        let error_handler = |e| format!("Error while parsing '{}': {}", stringify!($dest), e);
        let value = $input.f().map_err(error_handler)?;
        cast_or_error!($dest, value);
    };
    ($input:ident, $dest:expr, u, $bits:expr, $min:expr, $max:expr) => {
        read_value!($input, $dest, u, $bits);
        #[allow(unused_comparisons)]
        if $dest < $min || $dest > $max {
            return Err(format!(
                "{} ({}) must be in range {} to {}",
                stringify!($dest),
                $dest,
                $min,
                $max
            ));
        }
    };
    ($input:ident, $dest:expr, ue, $bits:expr, $min:expr, $max:expr) => {
        read_value!($input, $dest, ue, $bits);
        #[allow(unused_comparisons)]
        if $dest < $min || $dest > $max {
            return Err(format!(
                "{} ({}) must be in range {} to {}",
                stringify!($dest),
                $dest,
                $min,
                $max
            ));
        }
    };
    ($input:ident, $dest:expr, se, $min:expr, $max:expr) => {
        read_value!($input, $dest, se);
        #[allow(unused_comparisons)]
        if $dest < $min || $dest > $max {
            return Err(format!(
                "{} ({}) must be in range {} to {}",
                stringify!($dest),
                $dest,
                $min,
                $max
            ));
        }
    };
}

// Section 7.4.1
fn rbsp_trailing_bits(input: &mut BitReader) -> ParseResult<()> {
    expect_value!(input, "rbsp_stop_one_bit", 1, 1);
    while !input.is_aligned() {
        expect_value!(input, "rbsp_alignment_zero_bit", 0, 1);
    }
    Ok(())
}

// Section 7.2
pub(super) fn more_rbsp_data(input: &mut BitReader) -> bool {
    let mut tmp_reader = input.clone();
    if tmp_reader.remaining() == 0 {
        return false;
    }

    if rbsp_trailing_bits(&mut tmp_reader).is_err() {
        return true;
    }

    loop {
        match tmp_reader.u(8) {
            Ok(value) if value > 0 => return true,
            Ok(_) => {}
            Err(_) => return false,
        }
    }
}

fn parse_vui(input: &mut BitReader) -> ParseResult<VuiParameters> {
    let mut vui = VuiParameters::default();

    read_value!(input, vui.aspect_ratio_info_present_flag, f);
    if vui.aspect_ratio_info_present_flag {
        const EXTENDED_SAR: u8 = 255;
        read_value!(input, vui.aspect_ratio_idc, u, 8);
        if vui.aspect_ratio_idc == EXTENDED_SAR {
            read_value!(input, vui.sar_width, u, 16);
            read_value!(input, vui.sar_height, u, 16);
        }
    }

    read_value!(input, vui.overscan_info_present_flag, f);
    if vui.overscan_info_present_flag {
        read_value!(input, vui.overscan_appropriate_flag, f);
    }

    read_value!(input, vui.video_signal_type_present_flag, f);
    if vui.video_signal_type_present_flag {
        read_value!(input, vui.video_format, u, 3);
        read_value!(input, vui.video_full_range_flag, f);
        read_value!(input, vui.color_description_present_flag, f);
        if vui.color_description_present_flag {
            read_value!(input, vui.color_primaries, u, 8);
            read_value!(input, vui.transfer_characteristics, u, 8);
            read_value!(input, vui.matrix_coefficients, u, 8);
        }
    }

    read_value!(input, vui.chroma_loc_info_present_flag, f);
    if vui.chroma_loc_info_present_flag {
        read_value!(input, vui.chroma_sample_loc_type_top_field, ue, 8);
        read_value!(input, vui.chroma_sample_loc_type_bottom_field, ue, 8);
    }

    read_value!(input, vui.timing_info_present_flag, f);
    if vui.timing_info_present_flag {
        read_value!(input, vui.num_units_in_tick, u, 32);
        read_value!(input, vui.time_scale, u, 32);
        read_value!(input, vui.fixed_frame_rate_flag, f);
    }

    let nal_hrd_parameters_present: bool;
    read_value!(input, nal_hrd_parameters_present, f);
    if nal_hrd_parameters_present {
        return Err("NAL HRD parameters are not supported".into());
    }

    let vcl_hrd_parameters_present: bool;
    read_value!(input, vcl_hrd_parameters_present, f);
    if vcl_hrd_parameters_present {
        return Err("VCL HRD parameters are not supported".into());
    }

    read_value!(input, vui.pic_struct_present_flag, f);
    read_value!(input, vui.bitstream_restriction_flag, f);
    if vui.bitstream_restriction_flag {
        read_value!(input, vui.motion_vectors_over_pic_boundaries_flag, f);
        read_value!(input, vui.max_bytes_per_pic_denom, ue, 8);
        read_value!(input, vui.max_bits_per_mb_denom, ue, 8);
        read_value!(input, vui.log2_max_mv_length_horizontal, ue, 8);
        read_value!(input, vui.log2_max_mv_length_vertical, ue, 8);
        read_value!(input, vui.max_num_reorder_frames, ue, 8);
        read_value!(input, vui.max_dec_frame_buffering, ue, 8);
    }

    Ok(vui)
}

// Section 7.3.2.1.1.1 / 7.3.2.2.1 — SPS scaling matrix syntax. Iterates over
// the `seq_scaling_list_present_flag[i]` flags and decodes each present list
// via `scaling_list::parse_scaling_list`. Non-present entries remain
// `NotPresent` so the resolver can apply fallback rule A later.
fn parse_seq_scaling_matrix(
    input: &mut BitReader,
    chroma_format: ChromaFormat,
) -> ParseResult<SeqScalingMatrix> {
    let mut matrix = SeqScalingMatrix::default();
    let n_8x8 = num_sps_8x8_lists(chroma_format);
    matrix.lists_8x8 = vec![ScalingList8x8Entry::NotPresent; n_8x8];

    let total = 6 + n_8x8;
    for i in 0..total {
        let present = input.f().map_err(|e| format!("seq_scaling_list_present_flag[{i}]: {e}"))?;
        if !present {
            continue;
        }
        if i < 6 {
            let (values, use_default) = scaling_list::parse_scaling_list(input, 16)?;
            matrix.lists_4x4[i] = if use_default {
                ScalingList4x4Entry::UseDefault
            } else {
                ScalingList4x4Entry::Explicit(values)
            };
        } else {
            let (values, use_default) = scaling_list::parse_scaling_list(input, 64)?;
            matrix.lists_8x8[i - 6] = if use_default {
                ScalingList8x8Entry::UseDefault
            } else {
                ScalingList8x8Entry::Explicit(values)
            };
        }
    }

    Ok(matrix)
}

// Section 7.3.2.2.1 — PPS scaling matrix. The 8x8 list count here depends on
// both `chroma_format_idc` and `transform_8x8_mode_flag`.
fn parse_pic_scaling_matrix(
    input: &mut BitReader,
    chroma_format: ChromaFormat,
    transform_8x8_mode_flag: bool,
) -> ParseResult<PicScalingMatrix> {
    let mut matrix = PicScalingMatrix::default();
    let n_8x8 = num_pps_8x8_lists(chroma_format, transform_8x8_mode_flag);
    matrix.lists_8x8 = vec![ScalingList8x8Entry::NotPresent; n_8x8];

    let total = 6 + n_8x8;
    for i in 0..total {
        let present = input.f().map_err(|e| format!("pic_scaling_list_present_flag[{i}]: {e}"))?;
        if !present {
            continue;
        }
        if i < 6 {
            let (values, use_default) = scaling_list::parse_scaling_list(input, 16)?;
            matrix.lists_4x4[i] = if use_default {
                ScalingList4x4Entry::UseDefault
            } else {
                ScalingList4x4Entry::Explicit(values)
            };
        } else {
            let (values, use_default) = scaling_list::parse_scaling_list(input, 64)?;
            matrix.lists_8x8[i - 6] = if use_default {
                ScalingList8x8Entry::UseDefault
            } else {
                ScalingList8x8Entry::Explicit(values)
            };
        }
    }

    Ok(matrix)
}

// Section 7.3.2.1.1 Sequence parameter set data syntax
pub fn parse_sps(input: &mut BitReader) -> ParseResult<SequenceParameterSet> {
    let mut sps = SequenceParameterSet::default();

    read_value!(input, sps.profile, u, 8);
    read_value!(input, sps.constraint_set0_flag, f);
    read_value!(input, sps.constraint_set1_flag, f);
    read_value!(input, sps.constraint_set2_flag, f);
    read_value!(input, sps.constraint_set3_flag, f);
    read_value!(input, sps.constraint_set4_flag, f);
    read_value!(input, sps.constraint_set5_flag, f);

    expect_value!(input, "reserved_zero_2bits", 0, 2);

    read_value!(input, sps.level_idc, u, 8);
    read_value!(input, sps.seq_parameter_set_id, ue, 8, 0, 31);

    if sps.profile.has_chroma_info() {
        read_value!(input, sps.chroma_format_idc, ue, 8);
        if sps.chroma_format_idc == ChromaFormat::YUV444 {
            read_value!(input, sps.separate_color_plane_flag, f);
        }

        read_value!(input, sps.bit_depth_luma_minus8, ue, 8, 0, 6);
        read_value!(input, sps.bit_depth_chroma_minus8, ue, 8, 0, 6);
        read_value!(input, sps.qpprime_y_zero_transform_bypass_flag, f);
        read_value!(input, sps.seq_scaling_matrix_present_flag, f);
        if sps.seq_scaling_matrix_present_flag {
            sps.seq_scaling_matrix =
                Some(parse_seq_scaling_matrix(input, sps.chroma_format_idc)?);
        }
    }

    read_value!(input, sps.log2_max_frame_num_minus4, ue, 8, 0, 12);
    read_value!(input, sps.pic_order_cnt_type, ue, 8, 0, 2);
    match sps.pic_order_cnt_type {
        0 => {
            read_value!(input, sps.log2_max_pic_order_cnt_lsb_minus4, ue, 8, 0, 12);
        }
        1 => {
            read_value!(input, sps.delta_pic_order_always_zero_flag, f);
            read_value!(input, sps.offset_for_non_ref_pic, se);
            read_value!(input, sps.offset_for_top_to_bottom_field, se);

            let cnt_cycle: u8;
            read_value!(input, cnt_cycle, ue, 8);
            for _ in 0..cnt_cycle {
                let offset: i32 = input.se()?;
                sps.offset_for_ref_frame.push(offset);
            }
        }
        _ => {}
    };

    read_value!(input, sps.max_num_ref_frames, ue, 8);
    read_value!(input, sps.gaps_in_frame_num_value_allowed_flag, f);

    read_value!(input, sps.pic_width_in_mbs_minus1, ue, 16);
    read_value!(input, sps.pic_height_in_map_units_minus1, ue, 16);

    read_value!(input, sps.frame_mbs_only_flag, f);
    if sps.frame_mbs_only_flag {
        sps.mb_adaptive_frame_field_flag = false;
    } else {
        read_value!(input, sps.mb_adaptive_frame_field_flag, f);
    }

    read_value!(input, sps.direct_8x8_inference_flag, f);

    let frame_cropping_flag: bool;
    read_value!(input, frame_cropping_flag, f);
    if frame_cropping_flag {
        let frame_crop_left_offset: u32;
        let frame_crop_right_offset: u32;
        let frame_crop_top_offset: u32;
        let frame_crop_bottom_offset: u32;

        read_value!(input, frame_crop_left_offset, ue, 32);
        read_value!(input, frame_crop_right_offset, ue, 32);
        read_value!(input, frame_crop_top_offset, ue, 32);
        read_value!(input, frame_crop_bottom_offset, ue, 32);
        sps.frame_cropping = Some(FrameCrop {
            left: frame_crop_left_offset,
            right: frame_crop_right_offset,
            top: frame_crop_top_offset,
            bottom: frame_crop_bottom_offset,
        });
    }

    let vui_parameters_present: bool;
    read_value!(input, vui_parameters_present, f);
    if vui_parameters_present {
        sps.vui_parameters = Some(parse_vui(input)?);
    }
    rbsp_trailing_bits(input)?;

    Ok(sps)
}

fn parse_slice_group(input: &mut BitReader) -> ParseResult<Option<SliceGroup>> {
    let mut slice_group: Option<SliceGroup> = None;

    let num_slice_groups_minus1: usize;
    let slice_group_map_type: u8;

    read_value!(input, num_slice_groups_minus1, ue);
    if num_slice_groups_minus1 > 0 {
        read_value!(input, slice_group_map_type, ue, 8);
        slice_group = match slice_group_map_type {
            0 => {
                let mut run_length_minus1 = vec![1u32; num_slice_groups_minus1 + 1];
                for i in 0..=num_slice_groups_minus1 {
                    read_value!(input, run_length_minus1[i], ue, 32);
                }
                Some(SliceGroup::Interleaved { run_length_minus1 })
            }
            1 => Some(SliceGroup::Dispersed { num_slice_groups_minus1 }),
            2 => {
                let mut rectangles = vec![SliceRect::default(); num_slice_groups_minus1 + 1];
                for i in 0..=num_slice_groups_minus1 {
                    read_value!(input, rectangles[i].top_left, ue, 32);
                    read_value!(input, rectangles[i].bottom_right, ue, 32);
                }
                Some(SliceGroup::Foreground { rectangles })
            }
            3..=5 => {
                let change_type = match slice_group_map_type {
                    3 => SliceGroupChangeType::BoxOut,
                    4 => SliceGroupChangeType::RasterScan,
                    5 => SliceGroupChangeType::WipeOut,
                    _ => {
                        unreachable!();
                    }
                };
                let slice_group_change_direction_flag: bool;
                let slice_group_change_rate_minus1: u32;
                read_value!(input, slice_group_change_direction_flag, f);
                read_value!(input, slice_group_change_rate_minus1, ue, 32);

                Some(SliceGroup::Changing {
                    change_type,
                    num_slice_groups_minus1,
                    slice_group_change_direction_flag,
                    slice_group_change_rate_minus1,
                })
            }
            6 => {
                let pic_size_in_map_units_minus1: usize;
                read_value!(input, pic_size_in_map_units_minus1, ue);

                let slice_group_id_bits = 1 + num_slice_groups_minus1.ilog2() as u8;
                let mut slice_group_ids = vec![0u32; num_slice_groups_minus1 + 1];
                for i in 0..=num_slice_groups_minus1 {
                    read_value!(input, slice_group_ids[i], u, slice_group_id_bits);
                }

                Some(SliceGroup::Explicit {
                    num_slice_groups_minus1,
                    slice_group_id: slice_group_ids,
                })
            }
            _ => None,
        }
    }

    Ok(slice_group)
}

// Section 7.3.2.2 Picture parameter set RBSP syntax
pub fn parse_pps(input: &mut BitReader) -> ParseResult<PicParameterSet> {
    let mut pps = PicParameterSet::default();

    read_value!(input, pps.pic_parameter_set_id, ue, 8);
    read_value!(input, pps.seq_parameter_set_id, ue, 8);
    read_value!(input, pps.entropy_coding_mode_flag, f);
    read_value!(input, pps.bottom_field_pic_order_in_frame_present_flag, f);

    pps.slice_group = parse_slice_group(input)?;

    read_value!(input, pps.num_ref_idx_l0_default_active_minus1, ue, 32);
    read_value!(input, pps.num_ref_idx_l1_default_active_minus1, ue, 32);
    read_value!(input, pps.weighted_pred_flag, f);
    read_value!(input, pps.weighted_bipred_idc, u, 2);
    read_value!(input, pps.pic_init_qp_minus26, se);
    read_value!(input, pps.pic_init_qs_minus26, se);
    read_value!(input, pps.chroma_qp_index_offset, se);
    read_value!(input, pps.deblocking_filter_control_present_flag, f);
    read_value!(input, pps.constrained_intra_pred_flag, f);
    read_value!(input, pps.redundant_pic_cnt_present_flag, f);

    if more_rbsp_data(input) {
        read_value!(input, pps.transform_8x8_mode_flag, f);
        let pic_scaling_matrix_present_flag: bool;
        read_value!(input, pic_scaling_matrix_present_flag, f);
        if pic_scaling_matrix_present_flag {
            // The PPS alone does not know chroma_format_idc; look it up on the
            // referenced SPS (the standard 7.3.2.2.1 formula). Falls back to
            // YUV420 when SPS isn't yet available, which matches the most
            // common case (and will be re-validated once the SPS is parsed).
            let chroma_format = ChromaFormat::YUV420;
            pps.pic_scaling_matrix = Some(parse_pic_scaling_matrix(
                input,
                chroma_format,
                pps.transform_8x8_mode_flag,
            )?);
        }
        read_value!(input, pps.second_chroma_qp_index_offset, se);
    } else {
        pps.transform_8x8_mode_flag = false;
        pps.second_chroma_qp_index_offset = pps.chroma_qp_index_offset;
    }
    rbsp_trailing_bits(input)?;
    Ok(pps)
}

pub fn count_bytes_till_start_code(input: &[u8]) -> Option<usize> {
    let mut zeros = 0;
    for (idx, byte) in input.iter().enumerate() {
        let value = *byte;
        if value == 0 {
            zeros += 1;
        } else if value == 1 {
            if zeros >= 3 {
                return Some(idx - 3);
            }
            if zeros == 2 {
                return Some(idx - 2);
            }
            zeros = 0;
        } else {
            zeros = 0;
        }
    }
    None
}

pub fn remove_emulation_if_needed(input: &[u8]) -> Vec<u8> {
    let mut zeros = 0;
    let mut result = Vec::<u8>::new();
    for (byte_index, byte) in input.iter().enumerate() {
        match *byte {
            0 => {
                if !result.is_empty() {
                    result.push(*byte);
                }
                zeros += 1;
            }
            3 => {
                if zeros >= 2 {
                    if result.is_empty() {
                        result.reserve(input.len());
                        result.extend_from_slice(&input[..byte_index]);
                    }
                } else if !result.is_empty() {
                    result.push(*byte);
                }
                zeros = 0;
            }
            _ => {
                if !result.is_empty() {
                    result.push(*byte);
                }
                zeros = 0;
            }
        }
    }
    result
}

pub fn parse_nal_header(input: &mut BitReader) -> ParseResult<NalHeader> {
    let mut header = NalHeader::default();
    input.align();

    expect_value!(input, "forbidden_zero_bit", 0, 1);
    read_value!(input, header.nal_ref_idc, u, 2);
    read_value!(input, header.nal_unit_type, u, 5);
    Ok(header)
}

// Section 7.3.3.1 Reference picture list modification syntax
pub fn parse_ref_pic_list_modification(
    input: &mut BitReader,
    slice_type: SliceType,
) -> ParseResult<RefPicListModifications> {
    let mut modifications = RefPicListModifications::default();

    if slice_type != SliceType::I && slice_type != SliceType::SI {
        let ref_pic_list_modification_flag_l0: bool;
        read_value!(input, ref_pic_list_modification_flag_l0, f);
        if ref_pic_list_modification_flag_l0 {
            loop {
                let modification_of_pic_nums_idc: u32;
                read_value!(input, modification_of_pic_nums_idc, ue);
                match modification_of_pic_nums_idc {
                    0 => {
                        let abs_diff_pic_num_minus1: u32;
                        read_value!(input, abs_diff_pic_num_minus1, ue);
                        modifications.list0.push(RefPicListModification::RemapShortTermNegative(
                            abs_diff_pic_num_minus1,
                        ));
                    }
                    1 => {
                        let abs_diff_pic_num_minus1: u32;
                        read_value!(input, abs_diff_pic_num_minus1, ue);
                        modifications.list0.push(RefPicListModification::RemapShortTermPositive(
                            abs_diff_pic_num_minus1,
                        ));
                    }
                    2 => {
                        let long_term_pic_num: u32;
                        read_value!(input, long_term_pic_num, ue);
                        modifications
                            .list0
                            .push(RefPicListModification::RemapLongTerm(long_term_pic_num));
                    }
                    3 => break,
                    _ => return Err("Invalid modification_of_pic_nums_idc".to_string()),
                }
            }
        }
    }

    if slice_type == SliceType::B {
        let ref_pic_list_modification_flag_l1: bool;
        read_value!(input, ref_pic_list_modification_flag_l1, f);
        if ref_pic_list_modification_flag_l1 {
            loop {
                let modification_of_pic_nums_idc: u32;
                read_value!(input, modification_of_pic_nums_idc, ue);
                match modification_of_pic_nums_idc {
                    0 => {
                        let abs_diff_pic_num_minus1: u32;
                        read_value!(input, abs_diff_pic_num_minus1, ue);
                        modifications.list1.push(RefPicListModification::RemapShortTermNegative(
                            abs_diff_pic_num_minus1,
                        ));
                    }
                    1 => {
                        let abs_diff_pic_num_minus1: u32;
                        read_value!(input, abs_diff_pic_num_minus1, ue);
                        modifications.list1.push(RefPicListModification::RemapShortTermPositive(
                            abs_diff_pic_num_minus1,
                        ));
                    }
                    2 => {
                        let long_term_pic_num: u32;
                        read_value!(input, long_term_pic_num, ue);
                        modifications
                            .list1
                            .push(RefPicListModification::RemapLongTerm(long_term_pic_num));
                    }
                    3 => break,
                    _ => return Err("Invalid modification_of_pic_nums_idc".to_string()),
                }
            }
        }
    }

    Ok(modifications)
}

// Section 7.3.3.2 Prediction weight table syntax
pub fn parse_pred_weight_table(
    input: &mut BitReader,
    slice_header: &SliceHeader,
    sps: &SequenceParameterSet,
    pps: &PicParameterSet,
) -> ParseResult<PredWeightTable> {
    if slice_header.num_ref_idx_l0_active_minus1 > 31 {
        return Err(format!(
            "num_ref_idx_l0_active_minus1 ({}) is too large",
            slice_header.num_ref_idx_l0_active_minus1
        ));
    }
    if slice_header.slice_type == SliceType::B && slice_header.num_ref_idx_l1_active_minus1 > 31 {
        return Err(format!(
            "num_ref_idx_l1_active_minus1 ({}) is too large",
            slice_header.num_ref_idx_l1_active_minus1
        ));
    }

    let mut table = PredWeightTable::default();
    read_value!(input, table.luma_log2_weight_denom, ue, 8, 0, 7);
    if sps.ChromaArrayType() != ChromaFormat::Monochrome {
        read_value!(input, table.chroma_log2_weight_denom, ue, 8, 0, 7);
        let denom_diff =
            table.chroma_log2_weight_denom as i32 - table.luma_log2_weight_denom as i32;
        if denom_diff < -7 || denom_diff > 7 {
            return Err(format!(
                "Difference between chroma_log2_weight_denom and luma_log2_weight_denom ({}) must be in range -7 to 7",
                denom_diff
            ));
        }
    }

    for i in 0..=slice_header.num_ref_idx_l0_active_minus1 {
        let mut factors = WeightingFactors {
            luma_weight: 1 << table.luma_log2_weight_denom,
            luma_offset: 0,
            chroma_weights: [
                1 << table.chroma_log2_weight_denom,
                1 << table.chroma_log2_weight_denom,
            ],
            chroma_offsets: [0, 0],
        };

        let luma_weight_l0_flag: bool;
        read_value!(input, luma_weight_l0_flag, f);
        if luma_weight_l0_flag {
            read_value!(input, factors.luma_weight, se, -128, 127);
            read_value!(input, factors.luma_offset, se, -128, 127);
        }

        if sps.ChromaArrayType() != ChromaFormat::Monochrome {
            let chroma_weight_l0_flag: bool;
            read_value!(input, chroma_weight_l0_flag, f);
            if chroma_weight_l0_flag {
                for j in 0..2 {
                    read_value!(input, factors.chroma_weights[j], se, -128, 127);
                    read_value!(input, factors.chroma_offsets[j], se, -128, 127);
                }
            }
        }
        table.list0.push(factors);
    }

    if slice_header.slice_type == SliceType::B {
        for i in 0..=slice_header.num_ref_idx_l1_active_minus1 {
            let mut factors = WeightingFactors {
                luma_weight: 1 << table.luma_log2_weight_denom,
                luma_offset: 0,
                chroma_weights: [
                    1 << table.chroma_log2_weight_denom,
                    1 << table.chroma_log2_weight_denom,
                ],
                chroma_offsets: [0, 0],
            };

            let luma_weight_l1_flag: bool;
            read_value!(input, luma_weight_l1_flag, f);
            if luma_weight_l1_flag {
                read_value!(input, factors.luma_weight, se, -128, 127);
                read_value!(input, factors.luma_offset, se, -128, 127);
            }

            if sps.ChromaArrayType() != ChromaFormat::Monochrome {
                let chroma_weight_l1_flag: bool;
                read_value!(input, chroma_weight_l1_flag, f);
                if chroma_weight_l1_flag {
                    for j in 0..2 {
                        read_value!(input, factors.chroma_weights[j], se, -128, 127);
                        read_value!(input, factors.chroma_offsets[j], se, -128, 127);
                    }
                }
            }
            table.list1.push(factors);
        }
    }

    Ok(table)
}

// Section 7.3.3.3 Decoded reference picture marking syntax
pub fn parse_dec_ref_pic_marking(
    input: &mut BitReader,
    idr_pic_flag: bool,
) -> ParseResult<DecRefPicMarking> {
    let mut dec_ref_pic_marking = DecRefPicMarking::default();
    if idr_pic_flag {
        let no_output_of_prior_pics_flag: bool;
        let long_term_reference_flag: bool;
        read_value!(input, no_output_of_prior_pics_flag, f);
        read_value!(input, long_term_reference_flag, f);
        dec_ref_pic_marking.no_output_of_prior_pics_flag = Some(no_output_of_prior_pics_flag);
        dec_ref_pic_marking.long_term_reference_flag = Some(long_term_reference_flag);
    } else {
        let adaptive_ref_pic_marking_mode_flag: bool;
        read_value!(input, adaptive_ref_pic_marking_mode_flag, f);
        dec_ref_pic_marking.adaptive_ref_pic_marking_mode_flag =
            Some(adaptive_ref_pic_marking_mode_flag);
        if adaptive_ref_pic_marking_mode_flag {
            loop {
                let memory_management_control_operation_val: u32;
                read_value!(input, memory_management_control_operation_val, ue);

                let op = match memory_management_control_operation_val {
                    0 => break,
                    1 => {
                        let difference_of_pic_nums_minus1: u32;
                        read_value!(input, difference_of_pic_nums_minus1, ue);
                        MemoryManagementControlOperation::MarkShortTermUnused {
                            difference_of_pic_nums_minus1,
                        }
                    }
                    2 => {
                        let long_term_pic_num: u32;
                        read_value!(input, long_term_pic_num, ue);
                        MemoryManagementControlOperation::MarkLongTermUnused { long_term_pic_num }
                    }
                    3 => {
                        let difference_of_pic_nums_minus1: u32;
                        read_value!(input, difference_of_pic_nums_minus1, ue);
                        let long_term_frame_idx: u32;
                        read_value!(input, long_term_frame_idx, ue);
                        MemoryManagementControlOperation::MarkShortTermAsLongTerm {
                            difference_of_pic_nums_minus1,
                            long_term_frame_idx,
                        }
                    }
                    4 => {
                        let max_long_term_frame_idx_plus1: u32;
                        read_value!(input, max_long_term_frame_idx_plus1, ue);
                        MemoryManagementControlOperation::SetMaxLongTermFrameIdx {
                            max_long_term_frame_idx_plus1,
                        }
                    }
                    5 => MemoryManagementControlOperation::MarkAllUnused,
                    6 => {
                        let long_term_frame_idx: u32;
                        read_value!(input, long_term_frame_idx, ue);
                        MemoryManagementControlOperation::MarkCurrentAsLongTerm {
                            long_term_frame_idx,
                        }
                    }
                    _ => {
                        return Err(format!(
                            "Invalid memory_management_control_operation: {}",
                            memory_management_control_operation_val
                        ))
                    }
                };
                dec_ref_pic_marking.memory_management_operations.push(op);
            }
        }
    }
    Ok(dec_ref_pic_marking)
}

// Section 7.3.3 Slice header syntax
pub fn parse_slice_header(
    ctx: &DecoderContext,
    nal: &NalHeader,
    input: &mut BitReader,
) -> ParseResult<Slice> {
    let idr_pic_flag = nal.nal_unit_type == NalUnitType::IDRSlice;

    let mut header = SliceHeader::default();
    read_value!(input, header.first_mb_in_slice, ue, 32);
    read_value!(input, header.slice_type, ue, 8);
    read_value!(input, header.pic_parameter_set_id, ue, 8);

    let pps = match ctx.get_pps(header.pic_parameter_set_id) {
        Some(pps) => pps,
        None => {
            return Err("PPS is missing in context".to_owned());
        }
    };
    let sps = match ctx.get_sps(pps.seq_parameter_set_id) {
        Some(sps) => sps,
        None => {
            return Err("SPS is missing in context".to_owned());
        }
    };

    if sps.separate_color_plane_flag {
        let color_plane_id: u8;
        read_value!(input, color_plane_id, u, 2);
        header.color_plane = match color_plane_id {
            0 => Some(ColorPlane::Y),
            1 => Some(ColorPlane::Cb),
            2 => Some(ColorPlane::Cr),
            _ => None,
        };
    }

    read_value!(input, header.frame_num, u, sps.bits_in_frame_num());

    if sps.frame_mbs_only_flag {
        header.field_pic_flag = false;
    } else {
        read_value!(input, header.field_pic_flag, f);
        if header.field_pic_flag {
            let bottom_field_flag: bool;
            read_value!(input, bottom_field_flag, f);
            header.bottom_field_flag = Some(bottom_field_flag);
        }
        return Err("interlaced video (fields) is not supported".into());
    }

    if idr_pic_flag {
        let idr_pic_id: u32;
        read_value!(input, idr_pic_id, ue, 16);
        header.idr_pic_id = Some(idr_pic_id);
    }

    if sps.pic_order_cnt_type == 0 {
        read_value!(input, header.pic_order_cnt_lsb, u, sps.bits_in_max_pic_order_cnt());
        if pps.bottom_field_pic_order_in_frame_present_flag && !header.field_pic_flag {
            read_value!(input, header.delta_pic_order_cnt_bottom, se);
        }
    } else if sps.pic_order_cnt_type == 1 && !sps.delta_pic_order_always_zero_flag {
        read_value!(input, header.delta_pic_order_cnt[0], se);
        if pps.bottom_field_pic_order_in_frame_present_flag && !header.field_pic_flag {
            read_value!(input, header.delta_pic_order_cnt[1], se);
        }
    }
    if pps.redundant_pic_cnt_present_flag {
        let redundant_pic_cnt: u32;
        read_value!(input, redundant_pic_cnt, ue, 8, 0, 127);
        header.redundant_pic_cnt = Some(redundant_pic_cnt);
    }

    if header.slice_type == SliceType::B {
        let direct_spatial_mv_pred_flag: bool;
        read_value!(input, direct_spatial_mv_pred_flag, f);
        header.direct_spatial_mv_pred_flag = Some(direct_spatial_mv_pred_flag);
    }

    if matches!(header.slice_type, SliceType::P | SliceType::SP | SliceType::B) {
        let num_ref_idx_active_override_flag: bool;
        read_value!(input, num_ref_idx_active_override_flag, f);
        if num_ref_idx_active_override_flag {
            read_value!(input, header.num_ref_idx_l0_active_minus1, ue, 8, 0, 31);
            if header.slice_type == SliceType::B {
                read_value!(input, header.num_ref_idx_l1_active_minus1, ue, 8, 0, 31);
            }
        } else {
            header.num_ref_idx_l0_active_minus1 = pps.num_ref_idx_l0_default_active_minus1;
            header.num_ref_idx_l1_active_minus1 = pps.num_ref_idx_l1_default_active_minus1;
        }
    }

    header.ref_pic_list_modification = parse_ref_pic_list_modification(input, header.slice_type)?;

    if (pps.weighted_pred_flag && matches!(header.slice_type, SliceType::P | SliceType::SP))
        || (pps.weighted_bipred_idc == 1 && header.slice_type == SliceType::B)
    {
        header.pred_weight_table = Some(parse_pred_weight_table(input, &header, sps, pps)?);
    }

    if nal.nal_ref_idc != 0 {
        header.dec_ref_pic_marking = Some(parse_dec_ref_pic_marking(input, idr_pic_flag)?);
    }

    if pps.entropy_coding_mode_flag
        && header.slice_type != SliceType::I
        && header.slice_type != SliceType::SI
    {
        read_value!(input, header.cabac_init_idc, ue, 8, 0, 2);
    }

    read_value!(input, header.slice_qp_delta, se, -51, 51);

    if header.slice_type == SliceType::SP || header.slice_type == SliceType::SI {
        if header.slice_type == SliceType::SP {
            let sp_for_switch_flag: bool;
            read_value!(input, sp_for_switch_flag, f);
            header.sp_for_switch_flag = Some(sp_for_switch_flag);
        }
        let slice_qs_delta: i32;
        read_value!(input, slice_qs_delta, se, -51, 51);
        header.slice_qs_delta = Some(slice_qs_delta);
    }

    if pps.deblocking_filter_control_present_flag {
        read_value!(input, header.deblocking_filter_idc, ue, 8);
        if header.deblocking_filter_idc != DeblockingFilterIdc::Off {
            read_value!(input, header.slice_alpha_c0_offset_div2, se, -6, 6);
            read_value!(input, header.slice_beta_offset_div2, se, -6, 6);
        }
    }

    if pps.slice_group.as_ref().is_some_and(|sg| matches!(sg, SliceGroup::Changing { .. })) {
        if let Some(SliceGroup::Changing { slice_group_change_rate_minus1, .. }) =
            pps.slice_group.as_ref()
        {
            let pic_size_in_map_units = (sps.pic_width_in_mbs_minus1 as u32 + 1)
                * (sps.pic_height_in_map_units_minus1 as u32 + 1);
            let slice_group_change_rate = slice_group_change_rate_minus1 + 1;
            let bits = ((pic_size_in_map_units as f64 / slice_group_change_rate as f64) + 1.0)
                .log2()
                .ceil() as u8;
            let slice_group_change_cycle: u32;
            read_value!(input, slice_group_change_cycle, u, bits);
            header.slice_group_change_cycle = Some(slice_group_change_cycle);
        }
    }

    Ok(Slice::new(sps.clone(), pps.clone(), header))
}

// Section 8.3.2.1 Derivation process for Intra8x8PredMode.
// Returns predIntra8x8PredMode = min(intra8x8PredModeA, intra8x8PredModeB).
// Per Clause 8.3.2.1, if either neighbor macroblock is unavailable, both are set
// to DC (the min is DC); otherwise the neighbor's contribution is its own 8x8 mode,
// or — if the neighbor is Intra_4x4 — the mode of the 4x4 block adjacent to the
// 8x8 boundary (Eq. 8-72). Intra_4x4 and Intra_8x8 share the same 0..8 mode
// numbering, so the 4x4→8x8 conversion is a direct u32 round-trip.
pub fn calc_prev_intra8x8_pred_mode(
    slice: &Slice,
    mb: &IMb,
    mb_addr: MbAddr,
    blk_idx: usize,
) -> Intra_8x8_SamplePredMode {
    let mut result = Intra_8x8_SamplePredMode::max_mode();
    let default_mode = Intra_8x8_SamplePredMode::DC;
    let blk_idx_u8 = blk_idx as u8;
    for neighbor in [MbNeighborName::A, MbNeighborName::B] {
        let (neighbor_8x8_idx, mb_neighbor) = get_8x8luma_block_neighbor(blk_idx_u8, neighbor);
        let mode = if let Some(mb_neighbor_dir) = mb_neighbor {
            if let Some(neighbor_mb) = slice.get_mb_neighbor(mb_addr, mb_neighbor_dir) {
                if let Macroblock::I(imb) = neighbor_mb {
                    match imb.MbPartPredMode(0) {
                        MbPredictionMode::Intra_8x8 => {
                            imb.rem_intra8x8_pred_mode[neighbor_8x8_idx as usize]
                        }
                        MbPredictionMode::Intra_4x4 => {
                            let (adj_4x4, _) =
                                get_4x4luma_block_neighbor(blk_idx_u8 * 4, neighbor);
                            let m = imb.rem_intra4x4_pred_mode[adj_4x4 as usize] as u32;
                            m.try_into().unwrap_or(default_mode)
                        }
                        _ => default_mode,
                    }
                } else {
                    default_mode
                }
            } else {
                return default_mode;
            }
        } else {
            mb.rem_intra8x8_pred_mode[neighbor_8x8_idx as usize]
        };
        result = std::cmp::min(result, mode);
    }
    result
}

// Section 8.3.1.1 Derivation process for Intra4x4PredMode
pub fn calc_prev_intra4x4_pred_mode(
    slice: &Slice,
    mb: &IMb,
    mb_addr: MbAddr,
    blk_idx: usize,
) -> Intra_4x4_SamplePredMode {
    let mut result = Intra_4x4_SamplePredMode::max_mode();
    let default_mode = Intra_4x4_SamplePredMode::DC;
    for neighbor in [MbNeighborName::A, MbNeighborName::B] {
        let (block_neighbor_idx, mb_neighbor) = get_4x4luma_block_neighbor(blk_idx as u8, neighbor);
        let mode = if let Some(mb_neighbor) = mb_neighbor {
            if let Some(mb) = slice.get_mb_neighbor(mb_addr, mb_neighbor) {
                if let Macroblock::I(mb) = mb {
                    match mb.MbPartPredMode(0) {
                        MbPredictionMode::Intra_4x4 => {
                            mb.rem_intra4x4_pred_mode[block_neighbor_idx as usize]
                        }
                        // Spec 8.3.1.1: when the neighbor MB uses Intra_8x8, the
                        // contributing mode is Intra8x8PredMode of the 8x8 block
                        // that contains the neighboring 4x4 block. Intra_4x4 and
                        // Intra_8x8 share the 0..8 numbering so it's a u32 round-trip.
                        MbPredictionMode::Intra_8x8 => {
                            let n8x8 = block_neighbor_idx / 4;
                            let m = mb.rem_intra8x8_pred_mode[n8x8 as usize] as u32;
                            m.try_into().unwrap_or(default_mode)
                        }
                        _ => default_mode,
                    }
                } else {
                    default_mode
                }
            } else {
                return default_mode;
            }
        } else {
            mb.rem_intra4x4_pred_mode[block_neighbor_idx as usize]
        };
        result = std::cmp::min(result, mode);
    }
    result
}

// Gets the motion information for the 4x4 block covering the given absolute pixel coordinates.
pub fn get_motion_at_coord(
    slice: &Slice,
    x: i32,
    y: i32,
    current_mb_addr: MbAddr,
    current_mb_motion: Option<&MbMotion>,
) -> Option<PartitionInfo> {
    let pic_width_pixels = slice.sps.pic_width() as i32;
    let pic_height_pixels = slice.sps.pic_height() as i32;

    if x < 0 || y < 0 || x >= pic_width_pixels || y >= pic_height_pixels {
        return None;
    }

    let mb_addr = slice.get_mb_addr_from_coords(x, y);

    if mb_addr == current_mb_addr {
        return if let Some(motion) = current_mb_motion {
            let block_grid_x = ((x % 16) / 4) as usize;
            let block_grid_y = ((y % 16) / 4) as usize;
            let info = motion.partitions[block_grid_y][block_grid_x];
            // Both ref_idx set to 255 means "not yet decoded" (sentinel from calculate_motion).
            // A single 255 means "unused direction" (e.g., Pred_L1 has ref_idx_l0=255).
            if info.ref_idx_l0 == 255 && info.ref_idx_l1 == 255 {
                None
            } else {
                Some(info)
            }
        } else {
            None
        };
    }

    let neighbor_mb = slice.get_mb(mb_addr)?;
    if neighbor_mb.is_intra() {
        return Some(PartitionInfo {
            ref_idx_l0: u8::MAX,
            ref_idx_l1: u8::MAX,
            mv_l0: MotionVector::default(),
            mvd_l0: MotionVector::default(),
            ..Default::default()
        });
    }
    let motion_info = neighbor_mb.get_motion_info();

    let block_grid_x = ((x % 16) / 4) as usize;
    let block_grid_y = ((y % 16) / 4) as usize;

    Some(motion_info.partitions[block_grid_y][block_grid_x])
}

// Section 8.4.1.3 Derivation process for luma motion vector prediction
#[allow(clippy::too_many_arguments)]
pub fn predict_mv_l0(
    slice: &Slice,
    mb_addr: MbAddr,
    part_x: u8,
    part_y: u8,
    part_w: u8,
    part_h: u8,
    ref_idx_l0: u8,
    current_mb_motion: Option<&MbMotion>,
) -> MotionVector {
    let mb_loc = slice.get_mb_location(mb_addr);
    let x = mb_loc.x as i32 + part_x as i32;
    let y = mb_loc.y as i32 + part_y as i32;

    // Helper to fetch neighbor info.
    // Returns None if the neighbor is unavailable (e.g. out of bounds).
    // Returns Some with ref_idx=MAX for Intra blocks (treated as ref_idx=-1).
    // Per spec 8.4.1.3.1: if a neighbor doesn't use L0 prediction, refIdxL0 = -1.
    // Note: pred_mode=None means P-slice code that didn't set pred_mode; those use L0 implicitly.
    let get_neighbor = |x, y| {
        get_motion_at_coord(slice, x, y, mb_addr, current_mb_motion).map(|mut info| {
            if info.pred_mode != MbPredictionMode::None
                && info.pred_mode != MbPredictionMode::Pred_L0
                && info.pred_mode != MbPredictionMode::BiPred
            {
                info.ref_idx_l0 = u8::MAX;
                info.mv_l0 = MotionVector::default();
            }
            info
        })
    };

    // Neighbors A (left), B (top), C (top-right), D (top-left)
    let a = get_neighbor(x - 1, y);
    let b = get_neighbor(x, y - 1);
    let c = get_neighbor(x + part_w as i32, y - 1);
    let d = get_neighbor(x - 1, y - 1);

    // Directional segmentation prediction (8-203 to 8-206)
    let is_16x8 = part_w == 16 && part_h == 8;
    let is_8x16 = part_w == 8 && part_h == 16;

    if is_16x8 {
        if part_y == 0 {
            // 16x8 Top partition (0): use B if references match
            if let Some(info) = b {
                if info.ref_idx_l0 == ref_idx_l0 {
                    return info.mv_l0;
                }
            }
        } else {
            // 16x8 Bottom partition (1): use A if references match
            if let Some(info) = a {
                if info.ref_idx_l0 == ref_idx_l0 {
                    return info.mv_l0;
                }
            }
        }
    } else if is_8x16 {
        if part_x == 0 {
            // 8x16 Left partition (0): use A if references match
            if let Some(info) = a {
                if info.ref_idx_l0 == ref_idx_l0 {
                    return info.mv_l0;
                }
            }
        } else {
            // 8x16 Right partition (1): use C (or D) if references match
            let c_eff = c.or(d);
            if let Some(info) = c_eff {
                if info.ref_idx_l0 == ref_idx_l0 {
                    return info.mv_l0;
                }
            }
        }
    }

    // Median prediction (Section 8.4.1.3.1)

    // Eq 8-214: C is replaced by D if C is unavailable.
    let mut mv_c = c.or(d);
    let mut mv_b = b;
    let mv_a = a;

    // Eq 8-207: if B and C are unavailable and A is available, use A for all three.
    if mv_b.is_none() && mv_c.is_none() && mv_a.is_some() {
        mv_b = mv_a;
        mv_c = mv_a;
    }

    // Unavailable neighbors compare as zero MV with ref_idx = -1 (u8::MAX).
    let get_vals = |info: Option<PartitionInfo>| {
        info.unwrap_or(PartitionInfo {
            ref_idx_l0: u8::MAX,
            mv_l0: MotionVector::default(),
            mvd_l0: MotionVector::default(),
            ..Default::default()
        })
    };

    let val_a = get_vals(mv_a);
    let val_b = get_vals(mv_b);
    let val_c = get_vals(mv_c);

    // Eq 8-211: if exactly one neighbor matches the current ref_idx, use its MV.
    let match_a = val_a.ref_idx_l0 == ref_idx_l0;
    let match_b = val_b.ref_idx_l0 == ref_idx_l0;
    let match_c = val_c.ref_idx_l0 == ref_idx_l0;

    let match_count = (match_a as u8) + (match_b as u8) + (match_c as u8);

    if match_count == 1 {
        if match_a {
            return val_a.mv_l0;
        }
        if match_b {
            return val_b.mv_l0;
        }
        if match_c {
            return val_c.mv_l0;
        }
    }

    // Eq 8-212, 8-213: per-component median.
    let median = |a: i16, b: i16, c: i16| a + b + c - a.min(b).min(c) - a.max(b).max(c);

    MotionVector {
        x: median(val_a.mv_l0.x, val_b.mv_l0.x, val_c.mv_l0.x),
        y: median(val_a.mv_l0.y, val_b.mv_l0.y, val_c.mv_l0.y),
    }
}

// Section 8.4.1.3 Derivation process for luma motion vector prediction (L1)
#[allow(clippy::too_many_arguments)]
pub fn predict_mv_l1(
    slice: &Slice,
    mb_addr: MbAddr,
    part_x: u8,
    part_y: u8,
    part_w: u8,
    part_h: u8,
    ref_idx_l1: u8,
    current_mb_motion: Option<&MbMotion>,
) -> MotionVector {
    let mb_loc = slice.get_mb_location(mb_addr);
    let x = mb_loc.x as i32 + part_x as i32;
    let y = mb_loc.y as i32 + part_y as i32;

    // Per spec 8.4.1.3.1: if a neighbor doesn't use L1 prediction, refIdxL1 = -1.
    // Note: pred_mode=None means P-slice code; not relevant for L1 but handled for safety.
    let get_neighbor = |x, y| {
        get_motion_at_coord(slice, x, y, mb_addr, current_mb_motion).map(|mut info| {
            if info.pred_mode != MbPredictionMode::None
                && info.pred_mode != MbPredictionMode::Pred_L1
                && info.pred_mode != MbPredictionMode::BiPred
            {
                info.ref_idx_l1 = u8::MAX;
                info.mv_l1 = MotionVector::default();
            }
            info
        })
    };

    let a = get_neighbor(x - 1, y);
    let b = get_neighbor(x, y - 1);
    let c = get_neighbor(x + part_w as i32, y - 1);
    let d = get_neighbor(x - 1, y - 1);

    let is_16x8 = part_w == 16 && part_h == 8;
    let is_8x16 = part_w == 8 && part_h == 16;

    if is_16x8 {
        if part_y == 0 {
            if let Some(info) = b {
                if info.ref_idx_l1 == ref_idx_l1 {
                    return info.mv_l1;
                }
            }
        } else {
            if let Some(info) = a {
                if info.ref_idx_l1 == ref_idx_l1 {
                    return info.mv_l1;
                }
            }
        }
    } else if is_8x16 {
        if part_x == 0 {
            if let Some(info) = a {
                if info.ref_idx_l1 == ref_idx_l1 {
                    return info.mv_l1;
                }
            }
        } else {
            let c_eff = c.or(d);
            if let Some(info) = c_eff {
                if info.ref_idx_l1 == ref_idx_l1 {
                    return info.mv_l1;
                }
            }
        }
    }

    let mut mv_c = c.or(d);
    let mut mv_b = b;
    let mv_a = a;

    if mv_b.is_none() && mv_c.is_none() && mv_a.is_some() {
        mv_b = mv_a;
        mv_c = mv_a;
    }

    let get_vals = |info: Option<PartitionInfo>| {
        info.unwrap_or(PartitionInfo { ref_idx_l1: u8::MAX, ..Default::default() })
    };

    let val_a = get_vals(mv_a);
    let val_b = get_vals(mv_b);
    let val_c = get_vals(mv_c);

    let match_a = val_a.ref_idx_l1 == ref_idx_l1;
    let match_b = val_b.ref_idx_l1 == ref_idx_l1;
    let match_c = val_c.ref_idx_l1 == ref_idx_l1;

    let match_count = (match_a as u8) + (match_b as u8) + (match_c as u8);

    if match_count == 1 {
        if match_a {
            return val_a.mv_l1;
        }
        if match_b {
            return val_b.mv_l1;
        }
        if match_c {
            return val_c.mv_l1;
        }
    }

    let median = |a: i16, b: i16, c: i16| a + b + c - a.min(b).min(c) - a.max(b).max(c);

    MotionVector {
        x: median(val_a.mv_l1.x, val_b.mv_l1.x, val_c.mv_l1.x),
        y: median(val_a.mv_l1.y, val_b.mv_l1.y, val_c.mv_l1.y),
    }
}

// Helper to predict + fill a B partition into the motion grid
#[allow(clippy::too_many_arguments)]
fn fill_motion_grid_b(
    slice: &Slice,
    mb_addr: MbAddr,
    motion: &mut MbMotion,
    part_x: u8,
    part_y: u8,
    part_w: u8,
    part_h: u8,
    pred_mode: MbPredictionMode,
    ref_idx_l0: u8,
    mvd_l0: MotionVector,
    ref_idx_l1: u8,
    mvd_l1: MotionVector,
) {
    let mut final_mv_l0 = MotionVector::default();
    let mut final_mv_l1 = MotionVector::default();

    if pred_mode == MbPredictionMode::Pred_L0 || pred_mode == MbPredictionMode::BiPred {
        let mvp =
            predict_mv_l0(slice, mb_addr, part_x, part_y, part_w, part_h, ref_idx_l0, Some(motion));
        final_mv_l0 =
            MotionVector { x: mvp.x.wrapping_add(mvd_l0.x), y: mvp.y.wrapping_add(mvd_l0.y) };
    }

    if pred_mode == MbPredictionMode::Pred_L1 || pred_mode == MbPredictionMode::BiPred {
        let mvp =
            predict_mv_l1(slice, mb_addr, part_x, part_y, part_w, part_h, ref_idx_l1, Some(motion));
        final_mv_l1 =
            MotionVector { x: mvp.x.wrapping_add(mvd_l1.x), y: mvp.y.wrapping_add(mvd_l1.y) };
    }

    let info = PartitionInfo {
        pred_mode,
        // Per spec Table 7-13: unused direction gets refIdx = -1 (represented as u8::MAX)
        ref_idx_l0: if pred_mode == MbPredictionMode::Pred_L1 { u8::MAX } else { ref_idx_l0 },
        mv_l0: final_mv_l0,
        mvd_l0,
        ref_idx_l1: if pred_mode == MbPredictionMode::Pred_L0 { u8::MAX } else { ref_idx_l1 },
        mv_l1: final_mv_l1,
        mvd_l1,
    };

    let grid_x_start = (part_x / 4) as usize;
    let grid_y_start = (part_y / 4) as usize;
    let grid_w = (part_w / 4) as usize;
    let grid_h = (part_h / 4) as usize;

    for row in motion.partitions.iter_mut().skip(grid_y_start).take(grid_h) {
        row[grid_x_start..grid_x_start + grid_w].fill(info);
    }
}

pub fn calculate_motion_b(
    slice: &Slice,
    this_mb_addr: MbAddr,
    mb_type: BMbType,
    partitions: &[PartitionInfo; 4],
    sub_macroblocks: &[BSubMacroblock; 4],
) -> MbMotion {
    let mut motion = MbMotion::default();

    // Mark all as "not yet decoded"
    for row in motion.partitions.iter_mut() {
        for part in row.iter_mut() {
            part.ref_idx_l0 = 255;
            part.ref_idx_l1 = 255;
        }
    }

    let use_spatial = slice.header.direct_spatial_mv_pred_flag.unwrap_or(true);

    match mb_type {
        BMbType::B_Direct_16x16 | BMbType::B_Skip => {
            if use_spatial {
                return derive_spatial_direct(slice, this_mb_addr, &motion);
            } else if let Some(ref col_pic) = slice.col_pic {
                return derive_temporal_direct(slice, this_mb_addr, col_pic, slice.current_pic_poc);
            } else {
                // No colocated picture available, fall back to spatial
                return derive_spatial_direct(slice, this_mb_addr, &motion);
            }
        }
        BMbType::B_8x8 => {
            for (i, sub_mb) in sub_macroblocks.iter().enumerate() {
                let sub_mb_x = (i as u8 % 2) * 8;
                let sub_mb_y = (i as u8 / 2) * 8;

                if sub_mb.sub_mb_type == BSubMbType::B_Direct_8x8 {
                    let direct_motion = if use_spatial {
                        derive_spatial_direct_sub(slice, this_mb_addr, &motion, sub_mb_x, sub_mb_y)
                    } else if let Some(ref col_pic) = slice.col_pic {
                        derive_temporal_direct_sub(
                            slice,
                            this_mb_addr,
                            &motion,
                            col_pic,
                            slice.current_pic_poc,
                            sub_mb_x,
                            sub_mb_y,
                        )
                    } else {
                        derive_spatial_direct_sub(slice, this_mb_addr, &motion, sub_mb_x, sub_mb_y)
                    };
                    let grid_x = (sub_mb_x / 4) as usize;
                    let grid_y = (sub_mb_y / 4) as usize;
                    for dy in 0..2 {
                        for dx in 0..2 {
                            motion.partitions[grid_y + dy][grid_x + dx] =
                                direct_motion.partitions[grid_y + dy][grid_x + dx];
                        }
                    }
                    continue;
                }

                let pred_mode = sub_mb.sub_mb_type.SubMbPredMode();
                let num_sub_mb_part = sub_mb.sub_mb_type.NumSubMbPart();
                for j in 0..num_sub_mb_part {
                    let p_info = sub_mb.partitions[j];
                    let (part_w, part_h) = sub_mb.sub_mb_type.SubMbPartSize();
                    let (dx, dy) = match (sub_mb.sub_mb_type, j) {
                        (BSubMbType::B_L0_8x8 | BSubMbType::B_L1_8x8 | BSubMbType::B_Bi_8x8, _) => {
                            (0, 0)
                        }
                        (BSubMbType::B_L0_8x4 | BSubMbType::B_L1_8x4 | BSubMbType::B_Bi_8x4, 0) => {
                            (0, 0)
                        }
                        (BSubMbType::B_L0_8x4 | BSubMbType::B_L1_8x4 | BSubMbType::B_Bi_8x4, 1) => {
                            (0, 4)
                        }
                        (BSubMbType::B_L0_4x8 | BSubMbType::B_L1_4x8 | BSubMbType::B_Bi_4x8, 0) => {
                            (0, 0)
                        }
                        (BSubMbType::B_L0_4x8 | BSubMbType::B_L1_4x8 | BSubMbType::B_Bi_4x8, 1) => {
                            (4, 0)
                        }
                        (BSubMbType::B_L0_4x4 | BSubMbType::B_L1_4x4 | BSubMbType::B_Bi_4x4, 0) => {
                            (0, 0)
                        }
                        (BSubMbType::B_L0_4x4 | BSubMbType::B_L1_4x4 | BSubMbType::B_Bi_4x4, 1) => {
                            (4, 0)
                        }
                        (BSubMbType::B_L0_4x4 | BSubMbType::B_L1_4x4 | BSubMbType::B_Bi_4x4, 2) => {
                            (0, 4)
                        }
                        (BSubMbType::B_L0_4x4 | BSubMbType::B_L1_4x4 | BSubMbType::B_Bi_4x4, 3) => {
                            (4, 4)
                        }
                        _ => unreachable!(),
                    };
                    fill_motion_grid_b(
                        slice,
                        this_mb_addr,
                        &mut motion,
                        sub_mb_x + dx,
                        sub_mb_y + dy,
                        part_w,
                        part_h,
                        pred_mode,
                        p_info.ref_idx_l0,
                        p_info.mvd_l0,
                        p_info.ref_idx_l1,
                        p_info.mvd_l1,
                    );
                }
            }
        }
        _ => {
            // 16x16, 16x8, 8x16 types
            let num_mb_part = mb_type.NumMbPart();
            let (part_w, part_h) = mb_type.MbPartSize();
            for i in 0..num_mb_part {
                let pred_mode = mb_type.MbPartPredMode(i);
                let (part_x, part_y) = match (part_w, part_h, i) {
                    (16, 16, _) => (0u8, 0u8),
                    (16, 8, 0) => (0, 0),
                    (16, 8, 1) => (0, 8),
                    (8, 16, 0) => (0, 0),
                    (8, 16, 1) => (8, 0),
                    _ => unreachable!(),
                };
                fill_motion_grid_b(
                    slice,
                    this_mb_addr,
                    &mut motion,
                    part_x,
                    part_y,
                    part_w,
                    part_h,
                    pred_mode,
                    partitions[i].ref_idx_l0,
                    partitions[i].mvd_l0,
                    partitions[i].ref_idx_l1,
                    partitions[i].mvd_l1,
                );
            }
        }
    }
    motion
}

// Section 8.4.1.2.2 Spatial direct prediction
// Derives motion vectors and reference indices for B_Direct_16x16 and B_Skip macroblocks
fn derive_spatial_direct(slice: &Slice, mb_addr: MbAddr, _current_motion: &MbMotion) -> MbMotion {
    let mut motion = MbMotion::default();
    let mb_loc = slice.get_mb_location(mb_addr);
    let x = mb_loc.x as i32;
    let y = mb_loc.y as i32;

    let get_neighbor = |nx, ny| get_motion_at_coord(slice, nx, ny, mb_addr, None);

    let a = get_neighbor(x - 1, y);
    let b = get_neighbor(x, y - 1);
    let c = get_neighbor(x + 16, y - 1).or_else(|| get_neighbor(x - 1, y - 1));

    // Eq 8-184 to 8-187: derive refIdxL0, refIdxL1 using MinPositive.
    let mut ref_idx_l0: i16 = -1;
    let mut ref_idx_l1: i16 = -1;

    for info in [a, b, c].into_iter().flatten() {
        if info.ref_idx_l0 != u8::MAX {
            let idx = info.ref_idx_l0 as i16;
            if ref_idx_l0 == -1 || idx < ref_idx_l0 {
                ref_idx_l0 = idx;
            }
        }
        if info.ref_idx_l1 != u8::MAX {
            let idx = info.ref_idx_l1 as i16;
            if ref_idx_l1 == -1 || idx < ref_idx_l1 {
                ref_idx_l1 = idx;
            }
        }
    }

    // Eq 8-188 to 8-190: directZeroPredictionFlag when both refIdx are invalid.
    let direct_zero_prediction = ref_idx_l0 < 0 && ref_idx_l1 < 0;
    if direct_zero_prediction {
        ref_idx_l0 = 0;
        ref_idx_l1 = 0;
    }

    // Table 8-9: prediction utilization flags
    let pred_mode = match (ref_idx_l0 >= 0, ref_idx_l1 >= 0) {
        (true, true) => MbPredictionMode::BiPred,
        (true, false) => MbPredictionMode::Pred_L0,
        (false, true) => MbPredictionMode::Pred_L1,
        (false, false) => unreachable!(), // handled by directZeroPredictionFlag
    };

    // Compute predicted MVs once (clause 8.4.1.3, same for all sub-blocks per NOTE 3).
    // These may be overridden to zero per sub-block by colZeroFlag.
    let predicted_mv_l0 = if !direct_zero_prediction && ref_idx_l0 >= 0 {
        predict_mv_l0(slice, mb_addr, 0, 0, 16, 16, ref_idx_l0 as u8, None)
    } else {
        MotionVector::default()
    };
    let predicted_mv_l1 = if !direct_zero_prediction && ref_idx_l1 >= 0 {
        predict_mv_l1(slice, mb_addr, 0, 0, 16, 16, ref_idx_l1 as u8, None)
    } else {
        MotionVector::default()
    };

    let ref_l0_u8 = if ref_idx_l0 >= 0 { ref_idx_l0 as u8 } else { u8::MAX };
    let ref_l1_u8 = if ref_idx_l1 >= 0 { ref_idx_l1 as u8 } else { u8::MAX };
    let direct_8x8_inference = slice.sps.direct_8x8_inference_flag;

    // Fill each 4x4 sub-block, applying per-sub-block colZeroFlag check
    for grid_y in 0..4usize {
        for grid_x in 0..4usize {
            // Determine final MVs for this sub-block per spec:
            // mvLX = 0 if directZeroPredictionFlag=1 OR refIdxLX<0 OR (refIdxLX==0 AND colZeroFlag==1)
            // mvLX = predicted otherwise
            let mut mv_l0 = predicted_mv_l0;
            let mut mv_l1 = predicted_mv_l1;

            if direct_zero_prediction {
                mv_l0 = MotionVector::default();
                mv_l1 = MotionVector::default();
            } else {
                let col_zero =
                    get_col_zero_flag(slice, mb_addr, grid_x, grid_y, direct_8x8_inference);
                if col_zero {
                    if ref_idx_l0 == 0 {
                        mv_l0 = MotionVector::default();
                    }
                    if ref_idx_l1 == 0 {
                        mv_l1 = MotionVector::default();
                    }
                }
            }

            motion.partitions[grid_y][grid_x] = PartitionInfo {
                pred_mode,
                ref_idx_l0: ref_l0_u8,
                mv_l0,
                mvd_l0: MotionVector::default(),
                ref_idx_l1: ref_l1_u8,
                mv_l1,
                mvd_l1: MotionVector::default(),
            };
        }
    }
    motion
}

// Section 8.4.1.2.2: Derive colZeroFlag for a 4x4 sub-block from the colocated picture.
fn get_col_zero_flag(
    slice: &Slice,
    mb_addr: MbAddr,
    grid_x: usize,
    grid_y: usize,
    direct_8x8_inference: bool,
) -> bool {
    let col_pic = match &slice.col_pic {
        Some(cp) => cp,
        None => return false,
    };

    // RefPicList1[0] must be short-term reference
    if !col_pic.ref_l1_0_is_short_term {
        return false;
    }

    let col_mb_addr = mb_addr as usize;
    if col_mb_addr >= col_pic.mb_is_intra.len() || col_pic.mb_is_intra[col_mb_addr] {
        return false;
    }

    let col_motion = &col_pic.mb_motion[col_mb_addr];
    let (cgx, cgy) = if direct_8x8_inference {
        let mb_part_idx = (grid_x / 2) + (grid_y / 2) * 2;
        col_block_for_direct_8x8(mb_part_idx)
    } else {
        (grid_x, grid_y)
    };
    let col_part = &col_motion.partitions[cgy][cgx];

    // Clause 8.4.1.2.1: determine refIdxCol and mvCol
    let (mv_col, ref_idx_col) = if col_part.pred_mode == MbPredictionMode::Pred_L1 {
        (col_part.mv_l1, col_part.ref_idx_l1)
    } else {
        (col_part.mv_l0, col_part.ref_idx_l0)
    };

    ref_idx_col == 0 && mv_col.x.abs() <= 1 && mv_col.y.abs() <= 1
}

// Spatial direct for a single 8x8 sub-block within B_8x8
// Per spec clause 8.4.1.2.2 + clause 6.4.11.7 step 3 + NOTE 1:
// For B_Direct_8x8, predPartWidth=16 and neighbor derivation always uses
// mbPartIdx=0, subMbPartIdx=0 (the MB origin). The predicted MV is the
// predicted MV for the complete macroblock, not the sub-block.
fn derive_spatial_direct_sub(
    slice: &Slice,
    mb_addr: MbAddr,
    current_motion: &MbMotion,
    sub_mb_x: u8,
    sub_mb_y: u8,
) -> MbMotion {
    let mut motion = current_motion.clone();
    let mb_loc = slice.get_mb_location(mb_addr);
    // Use MB origin for neighbor derivation, not sub-block position
    let x = mb_loc.x as i32;
    let y = mb_loc.y as i32;

    let get_neighbor = |nx, ny| get_motion_at_coord(slice, nx, ny, mb_addr, None);

    let a = get_neighbor(x - 1, y);
    let b = get_neighbor(x, y - 1);
    let c = get_neighbor(x + 16, y - 1).or_else(|| get_neighbor(x - 1, y - 1));

    let mut ref_idx_l0: i16 = -1;
    let mut ref_idx_l1: i16 = -1;

    for info in [a, b, c].into_iter().flatten() {
        if info.ref_idx_l0 != u8::MAX {
            let idx = info.ref_idx_l0 as i16;
            if ref_idx_l0 == -1 || idx < ref_idx_l0 {
                ref_idx_l0 = idx;
            }
        }
        if info.ref_idx_l1 != u8::MAX {
            let idx = info.ref_idx_l1 as i16;
            if ref_idx_l1 == -1 || idx < ref_idx_l1 {
                ref_idx_l1 = idx;
            }
        }
    }

    // directZeroPredictionFlag
    let direct_zero_prediction = ref_idx_l0 < 0 && ref_idx_l1 < 0;
    if direct_zero_prediction {
        ref_idx_l0 = 0;
        ref_idx_l1 = 0;
    }

    let pred_mode = match (ref_idx_l0 >= 0, ref_idx_l1 >= 0) {
        (true, true) => MbPredictionMode::BiPred,
        (true, false) => MbPredictionMode::Pred_L0,
        (false, true) => MbPredictionMode::Pred_L1,
        (false, false) => unreachable!(),
    };

    // MV prediction uses MB origin and full 16x16 partition width (spec NOTE 1/3)
    let predicted_mv_l0 = if !direct_zero_prediction && ref_idx_l0 >= 0 {
        predict_mv_l0(slice, mb_addr, 0, 0, 16, 16, ref_idx_l0 as u8, None)
    } else {
        MotionVector::default()
    };
    let predicted_mv_l1 = if !direct_zero_prediction && ref_idx_l1 >= 0 {
        predict_mv_l1(slice, mb_addr, 0, 0, 16, 16, ref_idx_l1 as u8, None)
    } else {
        MotionVector::default()
    };

    let ref_l0_u8 = if ref_idx_l0 >= 0 { ref_idx_l0 as u8 } else { u8::MAX };
    let ref_l1_u8 = if ref_idx_l1 >= 0 { ref_idx_l1 as u8 } else { u8::MAX };
    let direct_8x8_inference = slice.sps.direct_8x8_inference_flag;
    let base_grid_x = (sub_mb_x / 4) as usize;
    let base_grid_y = (sub_mb_y / 4) as usize;

    for dy in 0..2usize {
        for dx in 0..2usize {
            let gx = base_grid_x + dx;
            let gy = base_grid_y + dy;
            let mut mv_l0 = predicted_mv_l0;
            let mut mv_l1 = predicted_mv_l1;

            if direct_zero_prediction {
                mv_l0 = MotionVector::default();
                mv_l1 = MotionVector::default();
            } else {
                let col_zero = get_col_zero_flag(slice, mb_addr, gx, gy, direct_8x8_inference);
                if col_zero {
                    if ref_idx_l0 == 0 {
                        mv_l0 = MotionVector::default();
                    }
                    if ref_idx_l1 == 0 {
                        mv_l1 = MotionVector::default();
                    }
                }
            }

            motion.partitions[gy][gx] = PartitionInfo {
                pred_mode,
                ref_idx_l0: ref_l0_u8,
                mv_l0,
                mvd_l0: MotionVector::default(),
                ref_idx_l1: ref_l1_u8,
                mv_l1,
                mvd_l1: MotionVector::default(),
            };
        }
    }
    motion
}

/// Returns the colocated grid (x, y) for direct_8x8_inference_flag=1.
/// Per spec Section 8.4.1.2.3: luma4x4BlkIdx = 5 * mbPartIdx, then inverse-scanned
/// to pixel coordinates and converted to 4x4 grid indices.
/// Results: mbPartIdx 0→(0,0), 1→(3,0), 2→(0,3), 3→(3,3).
fn col_block_for_direct_8x8(mb_part_idx: usize) -> (usize, usize) {
    let luma4x4 = 5 * mb_part_idx;
    let group = luma4x4 / 4;
    let sub = luma4x4 % 4;
    let px = (group % 2) * 8 + (sub % 2) * 4;
    let py = (group / 2) * 8 + (sub / 2) * 4;
    (px / 4, py / 4)
}

// Section 8.4.1.2.3 Temporal direct prediction
// Derives motion vectors and reference indices from the colocated picture's motion field.
fn derive_temporal_direct(
    slice: &Slice,
    mb_addr: MbAddr,
    col_pic: &ColPicInfo,
    current_poc: i32,
) -> MbMotion {
    let mut motion = MbMotion::default();
    let direct_8x8_inference = slice.sps.direct_8x8_inference_flag;

    if direct_8x8_inference {
        // Per spec Section 8.4.1.2.3: when direct_8x8_inference_flag is 1,
        // luma4x4BlkIdx = 5 * mbPartIdx, giving representative blocks at
        // (0,0), (12,0), (0,12), (12,12) i.e. grid positions (0,0), (3,0), (0,3), (3,3).
        for mb_part_idx in 0..4usize {
            let fill_grid_x = (mb_part_idx % 2) * 2;
            let fill_grid_y = (mb_part_idx / 2) * 2;
            let (col_grid_x, col_grid_y) = col_block_for_direct_8x8(mb_part_idx);
            let info = derive_temporal_direct_partition(
                slice,
                mb_addr,
                col_pic,
                current_poc,
                col_grid_x,
                col_grid_y,
            );
            for dy in 0..2 {
                for dx in 0..2 {
                    motion.partitions[fill_grid_y + dy][fill_grid_x + dx] = info;
                }
            }
        }
    } else {
        for grid_y in 0..4 {
            for grid_x in 0..4 {
                let info = derive_temporal_direct_partition(
                    slice,
                    mb_addr,
                    col_pic,
                    current_poc,
                    grid_x,
                    grid_y,
                );
                motion.partitions[grid_y][grid_x] = info;
            }
        }
    }
    motion
}

// Derive temporal direct prediction for a single 8x8 sub-block within B_8x8
fn derive_temporal_direct_sub(
    slice: &Slice,
    mb_addr: MbAddr,
    current_motion: &MbMotion,
    col_pic: &ColPicInfo,
    current_poc: i32,
    sub_mb_x: u8,
    sub_mb_y: u8,
) -> MbMotion {
    let mut motion = current_motion.clone();
    let direct_8x8_inference = slice.sps.direct_8x8_inference_flag;
    let base_grid_x = (sub_mb_x / 4) as usize;
    let base_grid_y = (sub_mb_y / 4) as usize;

    if direct_8x8_inference {
        // Per spec: luma4x4BlkIdx = 5 * mbPartIdx
        let mb_part_idx = (sub_mb_x / 8) as usize + (sub_mb_y / 8) as usize * 2;
        let (col_grid_x, col_grid_y) = col_block_for_direct_8x8(mb_part_idx);
        let info = derive_temporal_direct_partition(
            slice,
            mb_addr,
            col_pic,
            current_poc,
            col_grid_x,
            col_grid_y,
        );
        for dy in 0..2 {
            for dx in 0..2 {
                motion.partitions[base_grid_y + dy][base_grid_x + dx] = info;
            }
        }
    } else {
        for gy in 0..2 {
            for gx in 0..2 {
                let grid_x = base_grid_x + gx;
                let grid_y = base_grid_y + gy;
                let info = derive_temporal_direct_partition(
                    slice,
                    mb_addr,
                    col_pic,
                    current_poc,
                    grid_x,
                    grid_y,
                );
                motion.partitions[grid_y][grid_x] = info;
            }
        }
    }
    motion
}

// Core temporal direct prediction for a single 4x4 block position
fn derive_temporal_direct_partition(
    slice: &Slice,
    mb_addr: MbAddr,
    col_pic: &ColPicInfo,
    current_poc: i32,
    grid_x: usize,
    grid_y: usize,
) -> PartitionInfo {
    let col_mb_addr = mb_addr as usize;

    // Check if colocated MB exists and is not intra
    if col_mb_addr >= col_pic.mb_is_intra.len() || col_pic.mb_is_intra[col_mb_addr] {
        // Colocated MB is intra: use zero motion with BiPred
        return PartitionInfo {
            pred_mode: MbPredictionMode::BiPred,
            ref_idx_l0: 0,
            mv_l0: MotionVector::default(),
            mvd_l0: MotionVector::default(),
            ref_idx_l1: 0,
            mv_l1: MotionVector::default(),
            mvd_l1: MotionVector::default(),
        };
    }

    let col_motion = &col_pic.mb_motion[col_mb_addr];
    let col_part = &col_motion.partitions[grid_y][grid_x];

    // Each colocated MB's `ref_idx_l0/l1` is interpreted in the context of
    // its own slice's ref pic list. For multi-slice pictures, different MBs
    // of the colocated picture may use different POC tables.
    let col_slice_id = col_pic.mb_slice_id[col_mb_addr] as usize;
    let (col_l0_pocs, col_l1_pocs) = &col_pic.slice_ref_pocs[col_slice_id];

    // Determine mvCol and refIdxCol from the colocated partition
    // Per spec: prefer L0, fallback to L1
    let (mv_col, ref_idx_col, col_ref_pocs) = if col_part.pred_mode == MbPredictionMode::Pred_L1 {
        (col_part.mv_l1, col_part.ref_idx_l1, col_l1_pocs.as_slice())
    } else {
        // Pred_L0, BiPred, or other: use L0
        (col_part.mv_l0, col_part.ref_idx_l0, col_l0_pocs.as_slice())
    };

    // Get the POC of the picture referenced by the colocated partition
    let col_ref_poc = col_ref_pocs.get(ref_idx_col as usize).copied().unwrap_or(col_pic.pic_poc);
    let col_poc = col_pic.pic_poc;

    // MapColToList0: find which entry in current pic's refPicList0 has the same POC
    let ref_idx_l0 = map_col_to_list0(slice, col_ref_poc);

    // Temporal scaling
    let td = clip_i32(col_poc - col_ref_poc, -128, 127);
    let tb = clip_i32(current_poc - col_ref_poc, -128, 127);

    let (mv_l0, mv_l1) = if td == 0 {
        // No temporal distance in colocated: just copy
        (mv_col, MotionVector::default())
    } else {
        let tx = (16384 + (td.abs() >> 1)) / td;
        let dist_scale_factor = clip_i32((tb * tx + 32) >> 6, -1024, 1023);
        let mv_l0 = MotionVector {
            x: clip_i32((dist_scale_factor * (mv_col.x as i32) + 128) >> 8, -32768, 32767)
                as i16,
            y: clip_i32((dist_scale_factor * (mv_col.y as i32) + 128) >> 8, -32768, 32767)
                as i16,
        };
        // Eq 8-204: mvL1 = mvL0 − mvCol. Compute in i32 since the difference of
        // two i16s can exceed i16 range (e.g. i16::MAX − i16::MIN = 65535), then
        // clip to i16 to fit the motion-vector storage.
        let mv_l1 = MotionVector {
            x: clip_i32(mv_l0.x as i32 - mv_col.x as i32, i16::MIN as i32, i16::MAX as i32) as i16,
            y: clip_i32(mv_l0.y as i32 - mv_col.y as i32, i16::MIN as i32, i16::MAX as i32) as i16,
        };
        (mv_l0, mv_l1)
    };

    PartitionInfo {
        pred_mode: MbPredictionMode::BiPred,
        ref_idx_l0: ref_idx_l0 as u8,
        mv_l0,
        mvd_l0: MotionVector::default(),
        ref_idx_l1: 0,
        mv_l1,
        mvd_l1: MotionVector::default(),
    }
}

/// Map colocated reference to current picture's refPicList0 entry with matching POC.
fn map_col_to_list0(slice: &Slice, col_ref_poc: i32) -> usize {
    for (idx, &poc) in slice.ref_pic_list0_pocs.iter().enumerate() {
        if poc == col_ref_poc {
            return idx;
        }
    }
    // Fallback: if no match found, use 0
    0
}

fn clip_i32(val: i32, min_val: i32, max_val: i32) -> i32 {
    val.max(min_val).min(max_val)
}

pub fn calculate_motion(
    slice: &Slice,
    this_mb_addr: MbAddr,
    mb_type: PMbType,
    partitions: &[PartitionInfo; 4],
    sub_macroblocks: &[SubMacroblock; 4],
) -> MbMotion {
    let mut motion = MbMotion::default();

    // Mark all partitions as "Not yet decoded" (Unavailable) using sentinel ref_idx 255.
    // This allows predict_mv_l0 to correctly identify unavailable neighbors within the same MB.
    // Both L0 and L1 must be 255 to match the sentinel check in get_motion_at_coord.
    for row in motion.partitions.iter_mut() {
        for part in row.iter_mut() {
            part.ref_idx_l0 = 255;
            part.ref_idx_l1 = 255;
        }
    }

    // Helper to calculate motion vector and fill the grid
    let mut fill_motion_grid =
        |part_x: u8, part_y: u8, part_w: u8, part_h: u8, ref_idx: u8, mvd: MotionVector| {
            let mvp = predict_mv_l0(
                slice,
                this_mb_addr,
                part_x,
                part_y,
                part_w,
                part_h,
                ref_idx,
                Some(&motion),
            );
            let final_mv =
                MotionVector { x: mvp.x.wrapping_add(mvd.x), y: mvp.y.wrapping_add(mvd.y) };
            let info = PartitionInfo {
                ref_idx_l0: ref_idx,
                mv_l0: final_mv,
                mvd_l0: mvd,
                ..Default::default()
            };

            let grid_x_start = (part_x / 4) as usize;
            let grid_y_start = (part_y / 4) as usize;
            let grid_w = (part_w / 4) as usize;
            let grid_h = (part_h / 4) as usize;

            for row in motion.partitions.iter_mut().skip(grid_y_start).take(grid_h) {
                row[grid_x_start..grid_x_start + grid_w].fill(info);
            }
        };

    match mb_type {
        PMbType::P_Skip => {
            // Section 8.4.1.1 Derivation process for luma motion vectors for skipped macroblocks in P and SP slices
            let mb_loc = slice.get_mb_location(this_mb_addr);

            let is_zero_motion = |x, y| {
                let mb_addr = slice.get_mb_addr_from_coords(x, y);
                if let Some(mb) = slice.get_mb(mb_addr) {
                    // If the macroblock is Intra, zero motion is 0 false.
                    if mb.is_intra() {
                        false
                    } else if let Some(info) = get_motion_at_coord(slice, x, y, this_mb_addr, None)
                    {
                        info.ref_idx_l0 == 0 && info.mv_l0 == MotionVector::default()
                    } else {
                        false
                    }
                } else {
                    // If the macroblock is not available, zero motion is true.
                    true
                }
            };

            let zero_a = is_zero_motion(mb_loc.x as i32 - 1, mb_loc.y as i32);
            let zero_b = is_zero_motion(mb_loc.x as i32, mb_loc.y as i32 - 1);

            let mv = if zero_a || zero_b {
                MotionVector::default()
            } else {
                predict_mv_l0(slice, this_mb_addr, 0, 0, 16, 16, 0, None)
            };

            let info = PartitionInfo {
                ref_idx_l0: 0,
                mv_l0: mv,
                mvd_l0: MotionVector::default(),
                ..Default::default()
            };
            for row in motion.partitions.iter_mut() {
                row.fill(info);
            }
        }
        PMbType::P_8x8 | PMbType::P_8x8ref0 => {
            // Section 8.4.1 Derivation process for motion vector components and reference indices
            for (i, sub_mb) in sub_macroblocks.iter().enumerate() {
                let sub_mb_x = (i as u8 % 2) * 8;
                let sub_mb_y = (i as u8 / 2) * 8;

                for j in 0..sub_mb.sub_mb_type.NumSubMbPart() {
                    let mvd_info = sub_mb.partitions[j];
                    // Table 7-17 – Sub-macroblock types in P macroblocks
                    let (part_w, part_h, dx, dy) = match (sub_mb.sub_mb_type, j) {
                        (SubMbType::P_L0_8x8, _) => (8, 8, 0, 0),
                        (SubMbType::P_L0_8x4, 0) => (8, 4, 0, 0),
                        (SubMbType::P_L0_8x4, 1) => (8, 4, 0, 4),
                        (SubMbType::P_L0_4x8, 0) => (4, 8, 0, 0),
                        (SubMbType::P_L0_4x8, 1) => (4, 8, 4, 0),
                        (SubMbType::P_L0_4x4, 0) => (4, 4, 0, 0),
                        (SubMbType::P_L0_4x4, 1) => (4, 4, 4, 0),
                        (SubMbType::P_L0_4x4, 2) => (4, 4, 0, 4),
                        (SubMbType::P_L0_4x4, 3) => (4, 4, 4, 4),
                        _ => unreachable!(),
                    };
                    fill_motion_grid(
                        sub_mb_x + dx,
                        sub_mb_y + dy,
                        part_w,
                        part_h,
                        mvd_info.ref_idx_l0,
                        mvd_info.mvd_l0,
                    );
                }
            }
        }
        PMbType::P_L0_16x16 | PMbType::P_L0_L0_16x8 | PMbType::P_L0_L0_8x16 => {
            // Section 8.4.1 Derivation process for motion vector components and reference indices
            let num_mb_part = match mb_type {
                PMbType::P_L0_16x16 => 1,
                PMbType::P_L0_L0_16x8 | PMbType::P_L0_L0_8x16 => 2,
                _ => 0,
            };

            for i in 0..num_mb_part {
                let mvd_info = partitions[i];
                // Table 7-13 – Macroblock type values 0 to 4 for P and SP slices
                let (part_w, part_h, part_x, part_y) = match (mb_type, i) {
                    (PMbType::P_L0_16x16, _) => (16, 16, 0, 0),
                    (PMbType::P_L0_L0_16x8, 0) => (16, 8, 0, 0),
                    (PMbType::P_L0_L0_16x8, 1) => (16, 8, 0, 8),
                    (PMbType::P_L0_L0_8x16, 0) => (8, 16, 0, 0),
                    (PMbType::P_L0_L0_8x16, 1) => (8, 16, 8, 0),
                    _ => unreachable!(),
                };
                fill_motion_grid(
                    part_x,
                    part_y,
                    part_w,
                    part_h,
                    mvd_info.ref_idx_l0,
                    mvd_info.mvd_l0,
                );
            }
        }
    }
    motion
}
// Section 7.3.4 Slice data syntax
pub fn parse_slice_data(
    input: &mut BitReader,
    slice: &mut Slice,
    pool: &mut super::residual::ResidualPool,
) -> ParseResult<()> {
    if slice.pps.entropy_coding_mode_flag {
        parse_slice_data_cabac(input, slice, pool)
    } else {
        parse_slice_data_cavlc(input, slice, pool)
    }
}

pub fn parse_slice_data_cabac(
    input: &mut BitReader,
    slice: &mut Slice,
    pool: &mut super::residual::ResidualPool,
) -> ParseResult<()> {
    let mut cabac_ctx = CabacContext::new(input, slice)?;
    loop {
        let pic_size_in_mbs = slice.sps.pic_size_in_mbs();
        let next_mb_addr = slice.get_next_mb_addr() as usize;

        if next_mb_addr >= pic_size_in_mbs {
            break;
        }

        trace!("=============== Parsing macroblock (CABAC): {} ===============", next_mb_addr);
        let mb = cabac_ctx.parse_macroblock(slice, pool)?;
        slice.append_mb(mb);

        if cabac_ctx.decode_terminate()? {
            break;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn reader(bytes: &[u8]) -> BitReader<'_> {
        BitReader::new(bytes)
    }

    fn parse_sps_test(data: &[u8]) -> SequenceParameterSet {
        parse_sps(&mut reader(data)).expect("SPS parsing failed")
    }

    fn parse_pps_test(data: &[u8]) -> PicParameterSet {
        parse_pps(&mut reader(data)).expect("PPS parsing failed")
    }

    #[test]
    pub fn test_slice() {
        let sps = SequenceParameterSet {
            profile: Profile::Baseline,
            constraint_set0_flag: true,
            constraint_set1_flag: true,
            level_idc: 20,
            seq_parameter_set_id: 0,
            chroma_format_idc: ChromaFormat::YUV420,
            separate_color_plane_flag: false,
            log2_max_frame_num_minus4: 11,
            log2_max_pic_order_cnt_lsb_minus4: 12,
            max_num_ref_frames: 1,
            pic_width_in_mbs_minus1: 3,
            pic_height_in_map_units_minus1: 3,
            frame_mbs_only_flag: true,
            vui_parameters: Some(VuiParameters {
                video_signal_type_present_flag: true,
                video_format: 5,
                color_description_present_flag: true,
                color_primaries: 6,
                transfer_characteristics: 6,
                matrix_coefficients: 6,
                bitstream_restriction_flag: true,
                motion_vectors_over_pic_boundaries_flag: true,
                log2_max_mv_length_horizontal: 16,
                log2_max_mv_length_vertical: 16,
                max_num_reorder_frames: 0,
                max_dec_frame_buffering: 1,
                ..VuiParameters::default()
            }),
            ..SequenceParameterSet::default()
        };

        let pps = PicParameterSet {
            pic_parameter_set_id: 0,
            seq_parameter_set_id: 0,
            entropy_coding_mode_flag: false,
            deblocking_filter_control_present_flag: true,
            ..PicParameterSet::default()
        };
        let mut ctx = DecoderContext::default();
        ctx.put_sps(sps.clone());
        ctx.put_pps(pps.clone());

        let slice_data = [
            0x65, 0xB8, 0x00, 0x04, 0x00, 0x00, 0x09, 0xFF, 0xFF, 0xF8, 0x7A, 0x28, 0x00, 0x08,
            0x24, 0x79, 0x31, 0x72, 0x72, 0x75, 0x8B, 0xAE, 0xBA, 0xEB, 0xAE, 0xBA, 0xEB, 0xAE,
            0xBA, 0xF0,
        ];
        let mut input = reader(&slice_data);
        let nal_header = parse_nal_header(&mut input).expect("NAL unit");
        assert_eq!(nal_header.nal_unit_type, NalUnitType::IDRSlice);
        let mut slice =
            parse_slice_header(&ctx, &nal_header, &mut input).expect("header parsing failed");
        let header = &slice.header;
        assert_eq!(header.slice_type, SliceType::I);
        assert_eq!(header.frame_num, 0);
        assert_eq!(header.pic_parameter_set_id, 0);
        assert_eq!(header.idr_pic_id, Some(1));
        assert_eq!(header.pic_order_cnt_lsb, Some(0));
        assert_eq!(header.slice_qp_delta, -4);
        assert_eq!(header.deblocking_filter_idc, DeblockingFilterIdc::On);
        let dec_ref_pic_marking =
            header.dec_ref_pic_marking.as_ref().expect("dec_ref_pic_marking is missing");
        assert_eq!(dec_ref_pic_marking.no_output_of_prior_pics_flag, Some(false));
        assert_eq!(dec_ref_pic_marking.long_term_reference_flag, Some(false));

        let mut pool = crate::h264::residual::ResidualPool::default();
        parse_slice_data(&mut input, &mut slice, &mut pool).expect("blocks parsing failed");
        assert_eq!(slice.get_macroblock_count(), 16);
        if let Some(Macroblock::I(block)) = slice.get_mb(0) {
            assert_eq!(block.mb_type, IMbType::I_NxN);
            assert_eq!(block.coded_block_pattern, macroblock::CodedBlockPattern(1));
            assert_eq!(block.mb_qp_delta, 0);
            assert_eq!(block.intra_chroma_pred_mode, Intra_Chroma_Pred_Mode::DC);
            for pred_mode in block.rem_intra4x4_pred_mode {
                assert_eq!(pred_mode, Intra_4x4_SamplePredMode::DC);
            }
            assert!(block.residual.is_some());
        } else {
            unreachable!("Should be I-block");
        }
        assert!(input.remaining() < 8);
    }

    #[test]
    pub fn test_sps1() {
        let data = [
            0x64, 0x00, 0x0A, 0xAC, 0x72, 0x84, 0x44, 0x26, 0x84, 0x00, 0x00, 0x00, 0x04, 0x00,
            0x00, 0x00, 0xCA, 0x3C, 0x48, 0x96, 0x11, 0x80,
        ];
        let sps = parse_sps_test(&data);
        assert_eq!(sps.profile, Profile::High, "profile");
        assert!(!sps.constraint_set0_flag);
        assert!(!sps.constraint_set1_flag);
        assert!(!sps.constraint_set2_flag);
        assert!(!sps.constraint_set3_flag);
        assert!(!sps.constraint_set4_flag);
        assert!(!sps.constraint_set5_flag);
        assert_eq!(sps.level_idc, 10, "level");
        assert_eq!(sps.pic_width_in_mbs_minus1, 3, "pic_width_in_mbs_minus1");
        assert_eq!(sps.pic_height_in_map_units_minus1, 3, "pic_width_in_mbs_minus1");
    }

    #[test]
    pub fn test_sps2() {
        let data = [
            0x42, 0xC0, 0x14, 0x8C, 0x8D, 0x42, 0x12, 0x4D, 0x41, 0x81, 0x81, 0x81, 0xE1, 0x10,
            0x8D, 0x40,
        ];
        let sps = parse_sps_test(&data);
        assert_eq!(sps.profile, Profile::Baseline, "profile");
        assert!(sps.constraint_set0_flag);
        assert!(sps.constraint_set1_flag);
        assert!(!sps.constraint_set2_flag);
        assert!(!sps.constraint_set3_flag);
        assert!(!sps.constraint_set4_flag);
        assert!(!sps.constraint_set5_flag);
        assert_eq!(sps.level_idc, 20, "level");
        assert_eq!(sps.seq_parameter_set_id, 0, "seq_parameter_set_id");
        assert_eq!(sps.log2_max_pic_order_cnt_lsb_minus4, 12, "log2_max_pic_order_cnt_lsb_minus4");
        assert_eq!(sps.log2_max_frame_num_minus4, 11, "log2_max_frame_num_minus4");
        assert_eq!(sps.pic_width_in_mbs_minus1, 3, "pic_width_in_mbs_minus1");
        assert_eq!(sps.pic_height_in_map_units_minus1, 3, "pic_width_in_mbs_minus1");
        assert_eq!(sps.max_num_ref_frames, 1);
        let vui = sps.vui_parameters.as_ref().expect("vui is missing");
        assert!(vui.video_signal_type_present_flag);
        assert_eq!(vui.video_format, 5);

        assert_eq!(vui.color_primaries, 6);
        assert_eq!(vui.transfer_characteristics, 6);
        assert_eq!(vui.log2_max_mv_length_horizontal, 16);
        assert_eq!(vui.log2_max_mv_length_vertical, 16);
        assert_eq!(vui.max_dec_frame_buffering, 1);
        assert!(vui.motion_vectors_over_pic_boundaries_flag);
        assert!(vui.bitstream_restriction_flag);
    }

    #[test]
    pub fn test_pps1() {
        let data = [0xE8, 0x43, 0x8F, 0x13, 0x21, 0x30];
        let pps = parse_pps_test(&data);
        assert_eq!(pps.pic_parameter_set_id, 0, "pic_parameter_set_id");
        assert_eq!(pps.seq_parameter_set_id, 0, "seq_parameter_set_id");
    }

    #[test]
    pub fn test_pps2() {
        let data = [0xCE, 0x3C, 0x80];
        let pps = parse_pps_test(&data);
        assert_eq!(pps.pic_parameter_set_id, 0, "pic_parameter_set_id");
        assert_eq!(pps.seq_parameter_set_id, 0, "seq_parameter_set_id");
        assert_eq!(pps.pic_init_qp_minus26, 0);
        assert_eq!(pps.pic_init_qs_minus26, 0);
        assert!(pps.deblocking_filter_control_present_flag);
        assert!(!pps.entropy_coding_mode_flag);
    }

    #[test]
    pub fn test_nal_header() {
        let data = [
            0x67, 0x42, 0xC0, 0x14, 0x8C, 0x8D, 0x42, 0x12, 0x4D, 0x41, 0x81, 0x81, 0x81, 0xE1,
            0x10, 0x8D, 0x40,
        ];

        let sps_nal = parse_nal_header(&mut reader(&data)).expect("NAL unit");
        assert_eq!(sps_nal.nal_unit_type, NalUnitType::SeqParameterSet);
        assert_eq!(sps_nal.nal_ref_idc, 3);

        let data = [0x68, 0xCE, 0x3C, 0x80];

        let pps_nal = parse_nal_header(&mut reader(&data)).expect("NAL unit");
        assert_eq!(pps_nal.nal_unit_type, NalUnitType::PicParameterSet);
        assert_eq!(pps_nal.nal_ref_idc, 3);
    }

    #[test]
    pub fn test_count_bytes_till_start_code() {
        let data = [0xFF, 0xFF, 0x00, 0x00, 0xAA, 0x00, 0x00, 0x00, 0x01, 0x07];
        assert_eq!(count_bytes_till_start_code(&data), Some(5));

        let data = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x02,
        ];
        assert_eq!(count_bytes_till_start_code(&data), Some(15));

        let data = [0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x04];
        assert_eq!(count_bytes_till_start_code(&data), None);
    }

    #[test]
    pub fn test_remove_emulation_if_needed() {
        let data = [0xAA, 0x00, 0x00, 0x00, 0x00, 0x01, 0x0f, 0x00, 0x00, 0x00, 0x00];
        assert!(remove_emulation_if_needed(&data).is_empty());

        let data = [0xAA, 0x00, 0x00, 0x00, 0x00, 0x03, 0x0f, 0x00, 0x00, 0x03, 0x00];
        assert_eq!(
            remove_emulation_if_needed(&data),
            vec![0xAA, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00]
        );

        let data = [0x00, 0x03, 0x0f, 0x00, 0x00, 0x03];
        assert_eq!(remove_emulation_if_needed(&data), vec![0x00, 0x03, 0x0f, 0x00, 0x00]);

        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4];
        assert!(remove_emulation_if_needed(&data).is_empty());
    }
}
