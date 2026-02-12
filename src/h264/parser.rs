#![macro_use]
use super::decoder;
use super::macroblock;
use super::nal;
use super::pps;
use super::rbsp;
use super::slice;
use super::sps;
use super::tables;

use super::cavlc::parse_residual_block;
use super::residual::Residual;
use super::{ChromaFormat, ColorPlane, Profile};
use decoder::DecoderContext;
use log::trace;
use macroblock::{
    get_4x4chroma_block_neighbor, get_4x4luma_block_neighbor, get_neighbor_mbs, IMb, IMbType,
    Intra_4x4_SamplePredMode, Intra_Chroma_Pred_Mode, Macroblock, MbAddr, MbMotion, MbNeighborName,
    MbPredictionMode, MotionVector, PMb, PMbType, PartitionInfo, PcmMb, SubMacroblock, SubMbType,
};
use nal::{NalHeader, NalUnitType};
use pps::{PicParameterSet, SliceGroup, SliceGroupChangeType, SliceRect};
use slice::{
    DeblockingFilterIdc, DecRefPicMarking, MemoryManagementControlOperation, PredWeightTable,
    RefPicListModification, RefPicListModifications, Slice, SliceHeader, SliceType,
    WeightingFactors,
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
}

// Section 7.4.1
fn rbsp_trailing_bits(input: &mut BitReader) -> ParseResult<()> {
    // 1-bit at the end
    expect_value!(input, "rbsp_trailing_bits", 1, 1);
    input.align();
    Ok(())
}

// Section 7.2
fn more_rbsp_data(input: &mut BitReader) -> bool {
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
        todo!("NAL HDR");
    }

    let vcl_hrd_parameters_present: bool;
    read_value!(input, vcl_hrd_parameters_present, f);
    if vcl_hrd_parameters_present {
        todo!("VCL HDR");
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
    read_value!(input, sps.seq_parameter_set_id, ue, 8);

    if sps.profile.has_chroma_info() {
        read_value!(input, sps.chroma_format_idc, ue, 8);
        if sps.chroma_format_idc == ChromaFormat::YUV444 {
            read_value!(input, sps.separate_color_plane_flag, f);
        }

        read_value!(input, sps.bit_depth_luma_minus8, ue, 8);
        read_value!(input, sps.bit_depth_chroma_minus8, ue, 8);
        read_value!(input, sps.qpprime_y_zero_transform_bypass_flag, f);
        read_value!(input, sps.seq_scaling_matrix_present_flag, f);
        if sps.seq_scaling_matrix_present_flag {
            todo!("scaling matrix");
        }
    }

    read_value!(input, sps.log2_max_frame_num_minus4, ue, 8);
    read_value!(input, sps.pic_order_cnt_type, ue, 8);
    match sps.pic_order_cnt_type {
        0 => {
            read_value!(input, sps.log2_max_pic_order_cnt_lsb_minus4, ue, 8);
        }
        1 => {
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
            todo!("scaling matrix");
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

    let mut zeros = 0;
    while let Ok(value) = input.u(8) {
        if value == 0 {
            zeros += 1;
        } else if value == 1 {
            if zeros < 2 {
                return Err(format!("Not enough zeros ({zeros}) in NAL header."));
            }
            break;
        } else {
            return Err(format!("Unexpected value in NAL header: {value}"));
        }
    }

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
    let mut table = PredWeightTable::default();
    read_value!(input, table.luma_log2_weight_denom, ue, 8);
    if sps.ChromaArrayType() != ChromaFormat::Monochrome {
        read_value!(input, table.chroma_log2_weight_denom, ue, 8);
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
            read_value!(input, factors.luma_weight, se);
            read_value!(input, factors.luma_offset, se);
        }

        if sps.ChromaArrayType() != ChromaFormat::Monochrome {
            let chroma_weight_l0_flag: bool;
            read_value!(input, chroma_weight_l0_flag, f);
            if chroma_weight_l0_flag {
                for j in 0..2 {
                    read_value!(input, factors.chroma_weights[j], se);
                    read_value!(input, factors.chroma_offsets[j], se);
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
                read_value!(input, factors.luma_weight, se);
                read_value!(input, factors.luma_offset, se);
            }

            if sps.ChromaArrayType() != ChromaFormat::Monochrome {
                let chroma_weight_l1_flag: bool;
                read_value!(input, chroma_weight_l1_flag, f);
                if chroma_weight_l1_flag {
                    for j in 0..2 {
                        read_value!(input, factors.chroma_weights[j], se);
                        read_value!(input, factors.chroma_offsets[j], se);
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
        todo!("implement interlaced video. i.e. fields");
    }

    if idr_pic_flag {
        read_value!(input, header.idr_pic_id, ue, 16);
    }

    if sps.pic_order_cnt_type == 0 {
        read_value!(input, header.pic_order_cnt_lsb, u, sps.bits_in_max_pic_order_cnt());
        if pps.bottom_field_pic_order_in_frame_present_flag && !header.field_pic_flag {
            read_value!(input, header.delta_pic_order_cnt_bottom, se);
        }
    } else {
        todo!();
    }
    if pps.redundant_pic_cnt_present_flag {
        read_value!(input, header.redundant_pic_cnt, ue);
    }

    if matches!(header.slice_type, SliceType::P | SliceType::SP | SliceType::B) {
        let num_ref_idx_active_override_flag: bool;
        read_value!(input, num_ref_idx_active_override_flag, f);
        if num_ref_idx_active_override_flag {
            read_value!(input, header.num_ref_idx_l0_active_minus1, ue);
            if header.slice_type == SliceType::B {
                read_value!(input, header.num_ref_idx_l1_active_minus1, ue);
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

    read_value!(input, header.slice_qp_delta, se);
    if pps.deblocking_filter_control_present_flag {
        read_value!(input, header.deblocking_filter_idc, ue, 8);
        if header.deblocking_filter_idc != DeblockingFilterIdc::Off {
            read_value!(input, header.slice_alpha_c0_offset_div2, se);
            read_value!(input, header.slice_beta_offset_div2, se);
        }
    }

    Ok(Slice::new(sps.clone(), pps.clone(), header))
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
        todo!("i444");
    }
    Ok(())
}

// Section 8.3.1.1 Derivation process for Intra4x4PredMode
fn calc_prev_intra4x4_pred_mode(
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
                    if mb.MbPartPredMode(0) == MbPredictionMode::Intra_4x4 {
                        mb.rem_intra4x4_pred_mode[block_neighbor_idx as usize]
                    } else {
                        default_mode
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

// Section 7.3.5 Macroblock layer syntax
pub fn parse_macroblock(input: &mut BitReader, slice: &Slice) -> ParseResult<Macroblock> {
    let mb_type_val: u32;
    read_value!(input, mb_type_val, ue);

    if matches!(slice.header.slice_type, SliceType::I | SliceType::SI) {
        let mb_type = IMbType::try_from(mb_type_val)?;
        parse_i_macroblock(input, slice, mb_type)
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

// Gets the motion information for the 4x4 block covering the given absolute pixel coordinates.
pub fn get_motion_at_coord(
    slice: &Slice,
    x: i32,
    y: i32,
    current_mb_addr: MbAddr,
    current_mb_motion: Option<&MbMotion>,
) -> Option<PartitionInfo> {
    let pic_width_pixels = slice.sps.pic_width() as i32;
    let pic_height_pixels = slice.sps.pic_hight() as i32;

    if x < 0 || y < 0 || x >= pic_width_pixels || y >= pic_height_pixels {
        return None;
    }

    let mb_addr = slice.get_mb_addr_from_coords(x, y);

    if mb_addr == current_mb_addr {
        return if let Some(motion) = current_mb_motion {
            let block_grid_x = ((x % 16) / 4) as usize;
            let block_grid_y = ((y % 16) / 4) as usize;
            let info = motion.partitions[block_grid_y][block_grid_x];
            // 255 is used as a sentinel for "not yet decoded" in calculate_motion
            if info.ref_idx_l0 == 255 {
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
        return Some(PartitionInfo { ref_idx_l0: u8::MAX, mv_l0: MotionVector::default() });
    }
    let motion_info = neighbor_mb.get_motion_info();

    let block_grid_x = ((x % 16) / 4) as usize;
    let block_grid_y = ((y % 16) / 4) as usize;

    Some(motion_info.partitions[block_grid_y][block_grid_x])
}

// Section 8.4.1.2 Derivation process for motion vector prediction
pub fn predict_mv_l0(
    slice: &Slice,
    mb_addr: MbAddr,
    part_x: u8, // in pixels, relative to MB top-left
    part_y: u8,
    part_w: u8,
    part_h: u8,
    ref_idx_l0: u8,
    current_mb_motion: Option<&MbMotion>,
) -> MotionVector {
    let mb_loc = slice.get_mb_location(mb_addr);
    let abs_part_x = mb_loc.x as i32 + part_x as i32;
    let abs_part_y = mb_loc.y as i32 + part_y as i32;

    let get_neighbor_raw = |x, y| get_motion_at_coord(slice, x, y, mb_addr, current_mb_motion);

    // Directional segmentation prediction for 16x8 and 8x16
    if part_w == 16 && part_h == 8 {
        if part_y == 0 {
            // 16x8 Top partition (0)
            // If refIdxL0B == refIdxL0, mvpL0 = mvL0B.
            // Otherwise, mvpL0 = Median( mvL0A, mvL0B, mvL0C )
            if let Some(b) = get_neighbor_raw(abs_part_x, abs_part_y - 1) {
                if b.ref_idx_l0 == ref_idx_l0 {
                    return b.mv_l0;
                }
            }
        } else {
            // 16x8 Bottom partition (1)
            // If refIdxL0A == refIdxL0, mvpL0 = mvL0A
            // Otherwise, mvpL0 = Median( mvL0A, mvL0B, mvL0C )
            if let Some(a) = get_neighbor_raw(abs_part_x - 1, abs_part_y) {
                if a.ref_idx_l0 == ref_idx_l0 {
                    return a.mv_l0;
                }
            }
        }
    }

    if part_w == 8 && part_h == 16 {
        if part_x == 0 {
            // 8x16 Left partition (0)
            // If refIdxL0A == refIdxL0, mvpL0 = mvL0A
            // Otherwise, mvpL0 = Median( mvL0A, mvL0B, mvL0C )
            if let Some(a) = get_neighbor_raw(abs_part_x - 1, abs_part_y) {
                if a.ref_idx_l0 == ref_idx_l0 {
                    return a.mv_l0;
                }
            }
        } else {
            // 8x16 Right partition (1)
            // If refIdxL0C == refIdxL0, mvpL0 = mvL0C
            // Otherwise, mvpL0 = Median( mvL0A, mvL0B, mvL0C )
            let c = get_neighbor_raw(abs_part_x + part_w as i32, abs_part_y - 1)
                .or_else(|| get_neighbor_raw(abs_part_x - 1, abs_part_y - 1)); // D fallback

            if let Some(c) = c {
                if c.ref_idx_l0 == ref_idx_l0 {
                    return c.mv_l0;
                }
            }
        }
    }

    // Get raw neighbor info (None if Intra or Unavailable)
    let raw_a = get_neighbor_raw(abs_part_x - 1, abs_part_y);
    let raw_b = get_neighbor_raw(abs_part_x, abs_part_y - 1);
    let raw_c = get_neighbor_raw(abs_part_x + part_w as i32, abs_part_y - 1)
        .or_else(|| get_neighbor_raw(abs_part_x - 1, abs_part_y - 1));

    // Check if neighbors match the current ref_idx
    let match_a = raw_a.map_or(false, |p| p.ref_idx_l0 == ref_idx_l0);
    let match_b = raw_b.map_or(false, |p| p.ref_idx_l0 == ref_idx_l0);
    let match_c = raw_c.map_or(false, |p| p.ref_idx_l0 == ref_idx_l0);

    let count_matches = match_a as i32 + match_b as i32 + match_c as i32;

    // If exactly one neighbor has the same reference index, use its MV.
    if count_matches == 1 {
        if match_a {
            return raw_a.unwrap().mv_l0;
        }
        if match_b {
            return raw_b.unwrap().mv_l0;
        }
        if match_c {
            return raw_c.unwrap().mv_l0;
        }
    }

    // Otherwise, use Median of MVs.
    // For Median calculation, unavailable/Intra neighbors are treated as (0,0).
    // Neighbors with different ref_idx are used AS IS (they are not zeroed).
    let mv_a = raw_a.map(|p| p.mv_l0).unwrap_or_default();
    let mv_b = raw_b.map(|p| p.mv_l0).unwrap_or_default();
    let mv_c = raw_c.map(|p| p.mv_l0).unwrap_or_default();

    // Median prediction.
    fn median(a: i16, b: i16, c: i16) -> i16 {
        let min_val = a.min(b).min(c);
        let max_val = a.max(b).max(c);
        a + b + c - min_val - max_val
    }

    MotionVector { x: median(mv_a.x, mv_b.x, mv_c.x), y: median(mv_a.y, mv_b.y, mv_c.y) }
}

fn calculate_motion(
    slice: &Slice,
    this_mb_addr: MbAddr,
    mb_type: PMbType,
    partitions: &[PartitionInfo; 4],
    sub_macroblocks: &[SubMacroblock; 4],
) -> MbMotion {
    let mut motion = MbMotion::default();

    // Mark all partitions as "Not yet decoded" (Unavailable) using sentinel ref_idx 255.
    // This allows predict_mv_l0 to correctly identify unavailable neighbors within the same MB.
    for row in motion.partitions.iter_mut() {
        for part in row.iter_mut() {
            part.ref_idx_l0 = 255;
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
            let info = PartitionInfo { ref_idx_l0: ref_idx, mv_l0: final_mv };

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

            let info = PartitionInfo { ref_idx_l0: 0, mv_l0: mv };
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
                    // Table 7-18 – Sub-macroblock types in P macroblocks
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
                        mvd_info.mv_l0,
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
                    mvd_info.mv_l0,
                );
            }
        }
    }
    motion
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
                    read_value!(input, partitions[i].ref_idx_l0, te, slice.header.num_ref_idx_l0_active_minus1);
                }
            }

            for i in 0..num_mb_part {
                let mvd_x: i32;
                let mvd_y: i32;
                read_value!(input, mvd_x, se);
                read_value!(input, mvd_y, se);
                partitions[i].mv_l0 = MotionVector { x: mvd_x as i16, y: mvd_y as i16 };
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
                    read_value!(input, ref_idx_l0[i], te, slice.header.num_ref_idx_l0_active_minus1);
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
                sub_macroblocks[i].partitions[j].mv_l0 = mvd_l0[i][j];
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
            _ => todo!("implement Intra_8x8"),
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

// Section 7.3.4 Slice data syntax
pub fn parse_slice_data(input: &mut BitReader, slice: &mut Slice) -> ParseResult<()> {
    // Baseline profile features
    if slice.sps.profile != Profile::Baseline {
        todo!("profiles above baseline");
    }
    assert!(!slice.pps.entropy_coding_mode_flag, "entropy coding is not implemented yet");
    assert!(!slice.pps.transform_8x8_mode_flag, "8x8 transform decoding is not implemented yet");
    assert!(!slice.sps.seq_scaling_matrix_present_flag, "scaling list is not implemented yet");
    assert!(slice.sps.frame_mbs_only_flag, "interlaced video is not implemented yet");
    assert!(slice.pps.slice_group.is_none(), "slice groups not implemented yet");
    assert!(!slice.pps.constrained_intra_pred_flag);
    assert_eq!(slice.sps.ChromaArrayType().get_chroma_shift(), super::Size { width: 1, height: 1 });

    if slice.pps.entropy_coding_mode_flag {
        input.align();
    }

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
            let default_partitions = [PartitionInfo::default(); 4];
            let default_sub_mbs = [SubMacroblock::default(); 4];
            for i in 0..mb_skip_run {
                let curr_mb_addr = slice.get_next_mb_addr();
                let motion = calculate_motion(
                    slice,
                    curr_mb_addr,
                    PMbType::P_Skip,
                    &default_partitions,
                    &default_sub_mbs,
                );

                let block =
                    Macroblock::P(PMb { mb_type: PMbType::P_Skip, motion, ..Default::default() });
                slice.append_mb(block);
            }
            if mb_skip_run > 0 {
                if !more_rbsp_data(input) {
                    break;
                }
            }
        }

        let next_mb_addr = slice.get_next_mb_addr() as usize;
        trace!("=============== Parsing macroblock: {next_mb_addr} ===============");
        let block = parse_macroblock(input, slice)?;
        slice.append_mb(block);
        if slice.get_macroblock_count() < pic_size_in_mbs {
            if !more_rbsp_data(input) {
                break;
            }
        } else {
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
        ctx.put_sps(sps);
        ctx.put_pps(pps);

        let slice_data = [
            0x00, 0x00, 0x00, 0x01, 0x65, 0xB8, 0x00, 0x04, 0x00, 0x00, 0x09, 0xFF, 0xFF, 0xF8,
            0x7A, 0x28, 0x00, 0x08, 0x24, 0x79, 0x31, 0x72, 0x72, 0x75, 0x8B, 0xAE, 0xBA, 0xEB,
            0xAE, 0xBA, 0xEB, 0xAE, 0xBA, 0xF0,
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

        parse_slice_data(&mut input, &mut slice).expect("blocks parsing failed");
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
        let vui = sps.vui_parameters.expect("vui is missing");
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
            0x00, 0x00, 0x00, 0x01, 0x67, 0x42, 0xC0, 0x14, 0x8C, 0x8D, 0x42, 0x12, 0x4D, 0x41,
            0x81, 0x81, 0x81, 0xE1, 0x10, 0x8D, 0x40,
        ];

        let sps_nal = parse_nal_header(&mut reader(&data)).expect("NAL unit");
        assert_eq!(sps_nal.nal_unit_type, NalUnitType::SeqParameterSet);
        assert_eq!(sps_nal.nal_ref_idc, 3);

        let data =
            [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x68, 0xCE, 0x3C, 0x80];

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
