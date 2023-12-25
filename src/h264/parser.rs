#![macro_use]

use crate::h264::macroblock::get_neighbor_mbs;
use crate::h264::sps::FrameCrop;

use super::decoder;
use super::macroblock;
use super::macroblock::MbNeighbors;
use super::nal;
use super::pps;
use super::rbsp;
use super::slice;
use super::sps;
use super::tables;

use super::cavlc::parse_residual_block;
use super::{ChromaFormat, ColorPlane, Profile};
use decoder::DecoderContext;
use log::trace;
use macroblock::{IMb, IMbType, Macroblock, MbPredictionMode, NeighborNames, PcmMb, Residual};
use nal::{NalHeader, NalUnitType};
use pps::{PicParameterSet, SliceGroup, SliceGroupChangeType, SliceRect};
use slice::{Slice, SliceHeader, SliceType};
use sps::{SequenceParameterSet, VuiParameters};

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
        read_value!(input, vui.colour_description_present_flag, f);
        if vui.colour_description_present_flag {
            read_value!(input, vui.colour_primaries, u, 8);
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
            read_value!(input, sps.separate_colour_plane_flag, f);
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

    if sps.separate_colour_plane_flag {
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

    if nal.nal_ref_idc != 0 {
        if idr_pic_flag {
            let no_output_of_prior_pics_flag: bool;
            let long_term_reference_flag: bool;
            read_value!(input, no_output_of_prior_pics_flag, f);
            read_value!(input, long_term_reference_flag, f);
        } else {
            todo!("non IDR slice");
        }
    }

    read_value!(input, header.slice_qp_delta, se);
    if pps.deblocking_filter_control_present_flag {
        read_value!(input, header.disable_deblocking_filter_idc, ue, 8);
        if header.disable_deblocking_filter_idc != 1 {
            let slice_alpha_c0_offset_div2: i32;
            let slice_beta_offset_div2: i32;
            read_value!(input, slice_alpha_c0_offset_div2, se);
            read_value!(input, slice_beta_offset_div2, se);
        }
    }

    Ok(Slice::new(sps.clone(), pps.clone(), header))
}

fn calculate_nc(
    slice: &Slice,
    neighbor_mbs: &MbNeighbors,
    blk_idx: u8,
    residual: &Residual,
    pred_mode: MbPredictionMode,
    plane: ColorPlane,
) -> i32 {
    let get_block_neighbor = if plane.is_luma() {
        macroblock::get_4x4luma_block_neighbor
    } else {
        macroblock::get_4x4chroma_block_neighbor
    };

    let mut total_nc = 0;
    let mut nc_counted = 0;
    for neighbor in [NeighborNames::A, NeighborNames::B] {
        let (block_neighbor_idx, mb_neighbor) = get_block_neighbor(blk_idx, neighbor);
        if let Some(mb_neighbor) = mb_neighbor {
            if let Some(addr) = neighbor_mbs.get(mb_neighbor) {
                if let Some(mb) = slice.get_mb(addr) {
                    let nc = mb.get_nc(block_neighbor_idx, plane) as i32;
                    total_nc += nc;
                    nc_counted += 1;
                }
            }
        } else {
            let nc = residual.get_nc(block_neighbor_idx, plane, pred_mode) as i32;
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
    block: &Macroblock,
    neighbor_mbs: &MbNeighbors,
    residual: &mut Residual,
) -> ParseResult<()> {
    trace!("parse_residual_luma");
    let pred_mode = block.MbPartPredMode(0);
    if pred_mode == MbPredictionMode::Intra_16x16 {
        trace!(" luma DC");
        let levels = residual.get_dc_levels_for(ColorPlane::Y, pred_mode);
        parse_residual_block(input, levels, /* nC = */ 0, levels.len())?;
    }
    let coded_block_pattern = block.get_coded_block_pattern();
    for i8x8 in 0..4 {
        if coded_block_pattern.luma() & (1 << i8x8) != 0 {
            for i4x4 in 0..4 {
                let blk_idx = i8x8 * 4 + i4x4;
                trace!(" luma BK {}", blk_idx);
                let nc =
                    calculate_nc(slice, neighbor_mbs, blk_idx, residual, pred_mode, ColorPlane::Y);
                let (levels_ref, total_coeff_ref) =
                    residual.get_ac_levels_for(blk_idx, ColorPlane::Y, pred_mode);
                *total_coeff_ref = parse_residual_block(input, levels_ref, nc, levels_ref.len())?;
            }
        }
    }
    Ok(())
}

pub fn parse_residual(
    input: &mut BitReader,
    slice: &Slice,
    block: &Macroblock,
    residual: &mut Residual,
) -> ParseResult<()> {
    trace!("parse_residual");
    let coded_block_pattern = block.get_coded_block_pattern();
    let neighbor_mbs = get_neighbor_mbs(
        slice.sps.pic_width_in_mbs() as u32,
        slice.header.first_mb_in_slice,
        slice.current_mb_address,
    );
    parse_residual_luma(input, slice, block, &neighbor_mbs, residual)?;
    let pred_mode = block.MbPartPredMode(0);
    if slice.sps.ChromaArrayType().is_chrome_subsampled() {
        if coded_block_pattern.chroma() & 3 != 0 {
            for plane in [ColorPlane::Cb, ColorPlane::Cr] {
                let levels = residual.get_dc_levels_for(plane, pred_mode);
                trace!(" chroma {:?} DC", plane);
                let nc = -1; // Section 9.2.1, If ChromaArrayType is 1, nC = −1,
                parse_residual_block(input, levels, nc, levels.len())?;
            }
        }

        for plane in [ColorPlane::Cb, ColorPlane::Cr] {
            if coded_block_pattern.chroma() & 2 != 0 {
                for blk_idx in 0..4 {
                    let nc =
                        calculate_nc(slice, &neighbor_mbs, blk_idx, residual, pred_mode, plane);
                    let (levels_ref, total_coeff_ref) =
                        residual.get_ac_levels_for(blk_idx, plane, pred_mode);

                    trace!(" chroma {:?} BK {}", plane, blk_idx);
                    *total_coeff_ref =
                        parse_residual_block(input, levels_ref, nc, levels_ref.len())?;
                }
            }
        }
    } else {
        todo!("i444");
    }
    Ok(())
}

pub fn parse_macroblock(input: &mut BitReader, slice: &Slice) -> ParseResult<Macroblock> {
    let mb_type: IMbType;
    read_value!(input, mb_type, ue);

    if mb_type == IMbType::I_PCM {
        let mut block = PcmMb::default();
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
        let mut block = IMb { mb_type, ..IMb::default() };
        if slice.pps.transform_8x8_mode_flag && block.mb_type == IMbType::I_NxN {
            read_value!(input, block.transform_size_8x8_flag, f);
        }
        match block.MbPartPredMode(0) {
            MbPredictionMode::Intra_4x4 => {
                for mode in block.rem_intra4x4_pred_mode.iter_mut() {
                    let prev_intra4x4_pred_mode_flag: bool;
                    read_value!(input, prev_intra4x4_pred_mode_flag, f);
                    if !prev_intra4x4_pred_mode_flag {
                        let rem_intra4x4_pred_mode: macroblock::Intra_4x4_SamplePredictionMode;
                        read_value!(input, rem_intra4x4_pred_mode, u, 3);
                        *mode = Some(rem_intra4x4_pred_mode);
                    }
                }
            }
            MbPredictionMode::Intra_16x16 => {}
            _ => todo!("implement Intra_8x8"),
        };
        if slice.sps.ChromaArrayType().is_chrome_subsampled() {
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
        if !block.coded_block_pattern.is_zero()
            || block.MbPartPredMode(0) == MbPredictionMode::Intra_16x16
        {
            read_value!(input, block.mb_qp_delta, se);
        }

        let mut residual = Box::<Residual>::default();
        let mut result = Macroblock::I(block);
        parse_residual(input, slice, &result, &mut residual)?;
        result.set_residual(Some(residual));
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
    assert!(slice.sps.frame_mbs_only_flag, "interlaced video is not implemented yet");
    assert!(slice.pps.slice_group.is_none(), "slice groups not implemented yet");
    assert!(!slice.pps.constrained_intra_pred_flag);
    assert_eq!(slice.sps.ChromaArrayType().get_chroma_shift(), super::Size { width: 1, height: 1 });

    if slice.pps.entropy_coding_mode_flag {
        input.align();
    }

    if slice.header.slice_type != SliceType::I && slice.header.slice_type != SliceType::SI {
        todo!("non I-slices");
    }

    slice.current_mb_address = slice.header.first_mb_in_slice;
    let mut more_data = true;
    let pic_size_in_mbs = slice.sps.pic_size_in_mbs() as u32;
    while more_data {
        trace!("=============== Parsing macroblock: {} ===============", slice.current_mb_address);
        let block = parse_macroblock(input, slice)?;
        slice.put_mb(slice.current_mb_address, block);
        if slice.current_mb_address < pic_size_in_mbs {
            slice.current_mb_address += 1;
            more_data = more_rbsp_data(input);
        } else {
            more_data = false;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn reader(bytes: &[u8]) -> BitReader {
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
            separate_colour_plane_flag: false,
            log2_max_frame_num_minus4: 11,
            log2_max_pic_order_cnt_lsb_minus4: 12,
            max_num_ref_frames: 1,
            pic_width_in_mbs_minus1: 3,
            pic_height_in_map_units_minus1: 3,
            frame_mbs_only_flag: true,
            vui_parameters: Some(VuiParameters {
                video_signal_type_present_flag: true,
                video_format: 5,
                colour_description_present_flag: true,
                colour_primaries: 6,
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
        assert_eq!(header.disable_deblocking_filter_idc, 0);

        parse_slice_data(&mut input, &mut slice).expect("blocks parsing failed");
        assert_eq!(slice.get_block_count(), 16);
        if let Some(Macroblock::I(block)) = slice.get_mb(0) {
            assert_eq!(block.mb_type, IMbType::I_NxN);
            assert_eq!(block.coded_block_pattern, macroblock::CodedBlockPattern(1));
            assert_eq!(block.mb_qp_delta, 0);
            assert_eq!(block.residual.is_some(), true);
        } else {
            assert!(false, "Should be I-block")
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

        assert_eq!(vui.colour_primaries, 6);
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
