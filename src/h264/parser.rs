use super::macroblock;
use super::nal;
use super::pps;
use super::slice;
use super::sps;
use super::tables;

use super::{ChromaFormat, DecoderContext, Profile};
use log::trace;
use macroblock::{IMacroblockType, IMb, Macroblock, MbPredictionMode};
use nal::{NalHeader, NalUnitType};
use pps::{PicParameterSet, SliceGroup, SliceGroupChangeType, SliceRect};
use slice::{ColourPlane, Slice, SliceHeader, SliceType};
use sps::{SequenceParameterSet, VuiParameters};

pub use bitreader::BitReader;
type ParseResult<T> = Result<T, String>;

fn f(input: &mut BitReader) -> ParseResult<bool> {
    input.read_bool().map_err(|e| "f(): end of stream".to_owned())
}

fn u(input: &mut BitReader, n: u8) -> ParseResult<u32> {
    input.read_u32(n).map_err(|e| "u(): end of stream".to_owned())
}

fn ue(input: &mut BitReader, n: u8) -> ParseResult<u32> {
    // Parsing process for Exp-Golomb codes. Section 9.1
    if n > 32 {
        return Err(format!("ue(): too many ({}) bits requested", n));
    }

    let mut zero_bits = 0u8;
    while !input.read_bool().map_err(|e| "ue() end of stream".to_owned())? {
        zero_bits += 1;
        if (zero_bits > n) {
            return Err(format!("ue(): too many ({}) leading zeros", zero_bits));
        }
    }

    let x = input.read_u64(zero_bits).map_err(|e| "ue(): end of stream".to_owned())?;
    let result = (1u64 << zero_bits) - 1 + x;
    if result >= 1u64 << n {
        return Err(format!("ue(): value ({}) is too large to fit the variable", result));
    }
    Ok(result as u32)
}

fn se(input: &mut BitReader) -> ParseResult<i32> {
    // Mapping process for signed Exp-Golomb codes Section 9.1.1
    let value = ue(input, 32)?;
    let result = if value & 1 != 0 { ((value >> 1) + 1) as i32 } else { -((value >> 1) as i32) };
    Ok(result)
}

macro_rules! cast_or_error {
    ($dest:expr, $value:expr) => {
        trace!("set {} = {}", stringify!($dest), $value);
        $dest = match $value.try_into() {
            Ok(v) => v,
            Err(e) => {
                return Err(format!("Error casting '{}': {}", stringify!($dest), e));
            }
        };
    };
}

macro_rules! expect_value {
    ($input:ident, $msg:expr, $expected:expr, $bits:expr) => {
        let error_handler = |e| format!("Not enough data for {}", $msg);
        let value = $input.read_u32($bits).map_err(error_handler)?;
        if value != $expected {
            return Err(format!("Unexpected value of {}: {} vs {}", $msg, value, $expected));
        }
    };
}

macro_rules! read_value {
    ($input:ident, $dest:expr, u, $bits:expr) => {
        let error_handler = |e| format!("Error while parsing '{}': {}", stringify!($dest), e);
        let value = u($input, $bits).map_err(error_handler)?;
        cast_or_error!($dest, value);
    };
    ($input:ident, $dest:expr, ue) => {
        read_value!($input, $dest, ue, 32);
    };
    ($input:ident, $dest:expr, ue, $bits:expr) => {
        let error_handler = |e| format!("Error while parsing '{}': {}", stringify!($dest), e);
        let value = ue($input, $bits).map_err(error_handler)?;
        cast_or_error!($dest, value);
    };
    ($input:ident, $dest:expr, se) => {
        let error_handler = |e| format!("Error while parsing '{}': {}", stringify!($dest), e);
        let value = se($input).map_err(error_handler)?;
        cast_or_error!($dest, value);
    };
    ($input:ident, $dest:expr, f) => {
        let error_handler = |e| format!("Error while parsing '{}': {}", stringify!($dest), e);
        let value = f($input).map_err(error_handler)?;
        cast_or_error!($dest, value);
    };
}

// Section 7.4.1
fn rbsp_trailing_bits(input: &mut BitReader) -> ParseResult<()> {
    // 1-bit at the end
    expect_value!(input, "rbsp_trailing_bits", 1, 1);
    input.align(1).map_err(|e| "can't align in rbsp_trailing_bits")?;
    Ok(())
}

// Section 7.2
fn more_rbsp_data(input: &mut BitReader) -> bool {
    if input.remaining() == 0 {
        return false;
    }

    let mut tmp_reader = input.relative_reader();
    if rbsp_trailing_bits(&mut tmp_reader).is_err() {
        return true;
    }

    loop {
        match tmp_reader.read_u8(8) {
            Ok(value) if value > 0 => return true,
            Ok(value) => {}
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

    let mut nal_hrd_parameters_present = false;
    read_value!(input, nal_hrd_parameters_present, f);
    if nal_hrd_parameters_present {
        todo!("NAL HDR");
    }

    let mut vcl_hrd_parameters_present = false;
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

            let mut cnt_cycle = 0;
            read_value!(input, cnt_cycle, ue, 8);
            for _ in 0..cnt_cycle {
                let offset: i32 = se(input)?;
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
    read_value!(input, sps.frame_cropping_flag, f);
    if sps.frame_cropping_flag {
        read_value!(input, sps.frame_crop_left_offset, ue, 32);
        read_value!(input, sps.frame_crop_right_offset, ue, 32);
        read_value!(input, sps.frame_crop_top_offset, ue, 32);
        read_value!(input, sps.frame_crop_bottom_offset, ue, 32);
    }

    let mut vui_parameters_present = false;
    read_value!(input, vui_parameters_present, f);
    if vui_parameters_present {
        sps.vui_parameters = Some(parse_vui(input)?);
    }
    rbsp_trailing_bits(input)?;

    Ok(sps)
}

fn parse_slice_group(input: &mut BitReader) -> ParseResult<Option<SliceGroup>> {
    let mut slice_group: Option<SliceGroup> = None;

    let mut num_slice_groups_minus1: usize = 0;
    let mut slice_group_map_type: u8 = 0;

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
                let mut slice_group_change_direction_flag: bool = false;
                let mut slice_group_change_rate_minus1: u32 = 0;
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
                let mut pic_size_in_map_units_minus1: usize = 0;
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
        let mut pic_scaling_matrix_present_flag = false;
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
    let mut reader = BitReader::new(input);
    if reader.align(1) != Ok(()) {
        return None;
    }

    while let Ok(bits) = reader.peek_u32(32) {
        if bits == 1 || (bits >> 8) == 1 {
            return Some((reader.position() / 8) as usize);
        }

        let bytes_to_skip = match bits.leading_zeros() {
            9..=16 => 2,
            17..=24 => 3,
            25..=31 => 4,
            _ => 1,
        };
        reader.skip(bytes_to_skip * 8);
    }

    None
}

pub fn parse_nal_header(input: &mut BitReader) -> ParseResult<NalHeader> {
    let mut forbidden_zero_bit = true;
    let mut header = NalHeader::default();
    input.align(1).map_err(|e| "can't align for NAL header")?;

    // Skip zeros and the start code prefix.
    loop {
        // Short start code: 0x00_00_00_01
        if input.peek_u32(24) == Ok(1) {
            input.read_u32(24).map_err(|e| "broken start code")?;
            break;
        }

        expect_value!(input, "NAL start code zero_byte", 0, 8);
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
        let mut colour_plane_id: u8 = 0;
        read_value!(input, colour_plane_id, u, 2);
        header.colour_plane = match colour_plane_id {
            0 => Some(ColourPlane::Y),
            1 => Some(ColourPlane::Cb),
            2 => Some(ColourPlane::Cr),
            _ => None,
        };
    }

    read_value!(input, header.frame_num, u, sps.bits_in_frame_num());

    if sps.frame_mbs_only_flag {
        header.field_pic_flag = false;
    } else {
        read_value!(input, header.field_pic_flag, f);
        if header.field_pic_flag {
            let mut bottom_field_flag = false;
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
        if (pps.bottom_field_pic_order_in_frame_present_flag && !header.field_pic_flag) {
            read_value!(input, header.delta_pic_order_cnt_bottom, se);
        }
    } else {
        todo!();
    }
    if pps.redundant_pic_cnt_present_flag {
        read_value!(input, header.redundant_pic_cnt, ue);
    }

    if nal.nal_ref_idc != 0 {
        if (idr_pic_flag) {
            let mut no_output_of_prior_pics_flag = false;
            let mut long_term_reference_flag = false;
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
            let mut slice_alpha_c0_offset_div2: i32 = 0;
            let mut slice_beta_offset_div2: i32 = 0;
            read_value!(input, slice_alpha_c0_offset_div2, se);
            read_value!(input, slice_beta_offset_div2, se);
        }
    }

    Ok(Slice { sps: sps.clone(), pps: pps.clone(), header })
}

pub fn parse_macroblock(input: &mut BitReader, slice: &Slice) -> ParseResult<Macroblock> {
    let mut block = IMb::default();
    read_value!(input, block.mb_type, ue);

    if block.mb_type == IMacroblockType::I_PCM {
        todo!("PCM macroblock");
    } else {
        if slice.pps.transform_8x8_mode_flag && block.mb_type == IMacroblockType::I_NxN {
            read_value!(input, block.transform_size_8x8_flag, f);
        }
        match block.MbPartPredMode(0) {
            MbPredictionMode::Intra_4x4 => {
                for mode in block.rem_intra4x4_pred_mode.iter_mut() {
                    let mut prev_intra4x4_pred_mode_flag = false;
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
            let mut coded_block_pattern_num: u8;
            read_value!(input, coded_block_pattern_num, ue, 8);
            block.coded_block_pattern =
                tables::code_num_to_intra_coded_block_pattern(coded_block_pattern_num)
                    .ok_or("Invalid coded_block_pattern")?;
        }
        if block.coded_block_pattern.non_zero()
            || block.MbPartPredMode(0) == MbPredictionMode::Intra_16x16
        {
            read_value!(input, block.mb_qp_delta, se);
        }
        return Ok(Macroblock::I(block));
    }

    Err("Unknown macroblock".to_owned())
}

// Section 7.3.4 Slice data syntax
pub fn parse_slice_data(input: &mut BitReader, slice: &Slice) -> ParseResult<Vec<Macroblock>> {
    let mut blocks = Vec::<Macroblock>::new();

    // Baseline profile features
    if (slice.sps.profile != Profile::Baseline) {
        todo!("profiles above baseline");
    }
    assert!(!slice.pps.entropy_coding_mode_flag, "entropy coding is not implemented yet");
    assert!(!slice.pps.transform_8x8_mode_flag, "8x8 transform decoding is not implemented yet");
    assert!(slice.sps.frame_mbs_only_flag, "interlaced video is not implemented yet");

    if slice.pps.entropy_coding_mode_flag {
        input.align(1);
    }

    if slice.header.slice_type != SliceType::I && slice.header.slice_type != SliceType::SI {
        todo!("non I-slices");
    }

    let mut curr_mb_addr = slice.header.first_mb_in_slice * (1 + slice.MbaffFrameFlag() as u32);
    let mut more_data = true;
    let mut prev_mb_skipped = false;
    while more_data {
        let block = parse_macroblock(input, slice)?;
        blocks.push(block);
        more_data = false;
    }

    Ok(blocks)
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn reader(bytes: &[u8]) -> BitReader {
        BitReader::new(bytes)
    }

    #[test]
    pub fn test_ue() {
        assert_eq!(0, ue(&mut reader(&[0b10000000]), 8).unwrap());
        assert_eq!(1, ue(&mut reader(&[0b01000000]), 8).unwrap());
        assert_eq!(2, ue(&mut reader(&[0b01100000]), 8).unwrap());
        assert_eq!(3, ue(&mut reader(&[0b00100000]), 8).unwrap());
        assert_eq!(4, ue(&mut reader(&[0b00101000]), 8).unwrap());
        assert_eq!(5, ue(&mut reader(&[0b00110000]), 8).unwrap());
        assert_eq!(6, ue(&mut reader(&[0b00111000]), 8).unwrap());
        assert_eq!(7, ue(&mut reader(&[0b00010000]), 8).unwrap());
        assert_eq!(8, ue(&mut reader(&[0b00010010]), 8).unwrap());
        assert_eq!(9, ue(&mut reader(&[0b00010100]), 8).unwrap());
        assert_eq!(255, ue(&mut reader(&[0b00000000, 0b10000000, 0]), 8).unwrap());
        assert_eq!(
            u32::MAX,
            ue(
                &mut reader(&[
                    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b10000000, 0b00000000,
                    0b00000000, 0b00000000, 0
                ]),
                32
            )
            .unwrap()
        );
    }

    #[test]
    pub fn test_se() {
        assert_eq!(0, se(&mut reader(&[0b10000000])).unwrap());
        assert_eq!(1, se(&mut reader(&[0b01000000])).unwrap());
        assert_eq!(-1, se(&mut reader(&[0b01100000])).unwrap());
        assert_eq!(2, se(&mut reader(&[0b00100000])).unwrap());
        assert_eq!(-2, se(&mut reader(&[0b00101000])).unwrap());
        assert_eq!(3, se(&mut reader(&[0b00110000])).unwrap());
        assert_eq!(-3, se(&mut reader(&[0b00111000])).unwrap());
        assert_eq!(4, se(&mut reader(&[0b00010000])).unwrap());
        assert_eq!(-4, se(&mut reader(&[0b00010010])).unwrap());
        assert_eq!(5, se(&mut reader(&[0b00010100])).unwrap());
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
        let slice =
            parse_slice_header(&ctx, &nal_header, &mut input).expect("header parsing failed");
        let header = &slice.header;
        assert_eq!(header.slice_type, SliceType::I);
        assert_eq!(header.frame_num, 0);
        assert_eq!(header.pic_parameter_set_id, 0);
        assert_eq!(header.idr_pic_id, Some(1));
        assert_eq!(header.pic_order_cnt_lsb, Some(0));
        assert_eq!(header.slice_qp_delta, -4);
        assert_eq!(header.disable_deblocking_filter_idc, 0);

        let blocks = parse_slice_data(&mut input, &slice).expect("blocks parsing failed");
        assert_eq!(blocks.len(), 1);
        println!("{:?}", blocks[0]);
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
}
