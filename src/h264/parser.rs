use super::pps;
use super::slice;
use super::sps;

use super::DecoderContext;
use super::Profile;
use pps::{PicParameterSet, SliceGroup, SliceGroupChangeType, SliceRect};
use slice::{ColourPlane, IMacroblockType, Macroblock, Slice, SliceHeader, SliceType};
use sps::{ProfileIdc, SequenceParameterSet, VuiParameters};

use nom::{
    bits::complete::*,
    error::{
        context, convert_error, ContextError, Error, ErrorKind, ParseError, VerboseError,
        VerboseErrorKind,
    },
    multi::count,
    multi::many0_count,
    Err, IResult, Parser,
};
type BitInput<'a> = (&'a [u8], usize);
type ParseResult<'a, T> = IResult<BitInput<'a>, T, VerboseError<BitInput<'a>>>;

fn make_error<'a>(input: BitInput<'a>, message: &'static str) -> Err<VerboseError<BitInput<'a>>> {
    let error = VerboseError::from_error_kind(input, ErrorKind::Verify);
    Err::Error(ContextError::<BitInput<'a>>::add_context(input, message, error))
}

fn u<'a>(n: usize) -> impl Parser<BitInput<'a>, u32, VerboseError<BitInput<'a>>> {
    take(n)
}

fn ue<'a>(n: usize) -> impl Parser<BitInput<'a>, u32, VerboseError<BitInput<'a>>> {
    move |i| {
        // Parsing process for Exp-Golomb codes. Section 9.1
        let (i, zero_bits) = many0_count(tag(0, 1u8))(i)?;
        let (i, _) = tag(1, 1u8).parse(i)?;
        let (i, x) = u(zero_bits).parse(i)?;
        let result = (1u32 << zero_bits) - 1 + x;
        if zero_bits >= n || result as u64 >= 1u64 << n {
            return Err(make_error(i, "Value is too large to fit the variable"));
        }
        Ok((i, result))
    }
}

fn se<'a>() -> impl Parser<BitInput<'a>, i32, VerboseError<BitInput<'a>>> {
    move |i| {
        // Mapping process for signed Exp-Golomb codes Section 9.1.1
        let (i, value) = ue(32).parse(i)?;
        let result =
            if value & 1 != 0 { ((value >> 1) + 1) as i32 } else { -((value >> 1) as i32) };
        Ok((i, result))
    }
}

macro_rules! read_value {
    ($input:ident, $dest:expr, u, $bits:expr) => {
        let context_str = stringify!($dest);
        let (i, value) = context(context_str, u($bits)).parse($input)?;
        $input = i;
        println!("u({}) {} = {}", $bits, context_str, value);
        $dest = value.try_into().unwrap();
    };
    ($input:ident, $dest:expr, ue) => {
        let context_str = stringify!($dest);
        let (i, value) = context(context_str, ue(32)).parse($input)?;
        $input = i;
        println!("ue {} = {}", context_str, value);
        $dest = value.try_into().unwrap();
    };
    ($input:ident, $dest:expr, ue, $bits:expr) => {
        let context_str = stringify!($dest);
        let (i, value) = context(context_str, ue($bits)).parse($input)?;
        $input = i;
        println!("ue({}) {} = {}", $bits, context_str, value);
        $dest = value.try_into().unwrap();
    };
    ($input:ident, $dest:expr, se) => {
        let context_str = stringify!($dest);
        let (i, value) = context(context_str, se()).parse($input)?;
        $input = i;
        println!("se {} = {}", context_str, value);
        $dest = value.try_into().unwrap();
    };
    ($input:ident, $dest:expr, bool) => {
        let context_str = stringify!($dest);
        let (i, value) = context(context_str, bool).parse($input)?;
        $input = i;
        println!("flag {} = {}", context_str, value);
        $dest = value.try_into().unwrap();
    };
}

fn align_till_next_byte(input: BitInput, bit_value: bool) -> ParseResult<()> {
    let bit_position = input.1;
    let input = if bit_position % 8 != 0 {
        let alignment_bit_cnt = 8 - (bit_position % 8);
        let mask = if bit_value { 0xff } else { 0 };
        let (input, _) = context("alignment_bits", tag(mask, alignment_bit_cnt))(input)?;
        input
    } else {
        input
    };

    Ok((input, ()))
}

// Section 7.4.1
fn rbsp_trailing_bits(input: BitInput) -> ParseResult<()> {
    // 1-bit at the end
    let (input, _) = context("rbsp_trailing_bits_sentinel", tag(1, 1u8))(input)?;
    let bit_position = input.1;

    return align_till_next_byte(input, false);
}

// Section 7.2
fn more_rbsp_data(input: BitInput) -> bool {
    let (data, index) = input;
    if data.is_empty() {
        return false;
    }

    if rbsp_trailing_bits(input).is_err() {
        return true;
    }

    for i in 1..data.len() {
        if data[i] != 0 {
            return true;
        }
    }
    false
}

fn parse_vui(i: BitInput) -> ParseResult<VuiParameters> {
    let mut vui = VuiParameters::default();
    let mut input = i;

    read_value!(input, vui.aspect_ratio_info_present_flag, bool);
    if vui.aspect_ratio_info_present_flag {
        const EXTENDED_SAR: u8 = 255;
        read_value!(input, vui.aspect_ratio_idc, u, 8);
        if vui.aspect_ratio_idc == EXTENDED_SAR {
            read_value!(input, vui.sar_width, u, 16);
            read_value!(input, vui.sar_height, u, 16);
        }
    }

    read_value!(input, vui.overscan_info_present_flag, bool);
    if vui.overscan_info_present_flag {
        read_value!(input, vui.overscan_appropriate_flag, bool);
    }

    read_value!(input, vui.video_signal_type_present_flag, bool);
    if vui.video_signal_type_present_flag {
        read_value!(input, vui.video_format, u, 3);
        read_value!(input, vui.video_full_range_flag, bool);
        read_value!(input, vui.colour_description_present_flag, bool);
        if vui.colour_description_present_flag {
            read_value!(input, vui.colour_primaries, u, 8);
            read_value!(input, vui.transfer_characteristics, u, 8);
            read_value!(input, vui.matrix_coefficients, u, 8);
        }
    }

    read_value!(input, vui.chroma_loc_info_present_flag, bool);
    if vui.chroma_loc_info_present_flag {
        read_value!(input, vui.chroma_sample_loc_type_top_field, ue, 8);
        read_value!(input, vui.chroma_sample_loc_type_bottom_field, ue, 8);
    }

    read_value!(input, vui.timing_info_present_flag, bool);
    if vui.timing_info_present_flag {
        read_value!(input, vui.num_units_in_tick, u, 32);
        read_value!(input, vui.time_scale, u, 32);
        read_value!(input, vui.fixed_frame_rate_flag, bool);
    }

    let mut nal_hrd_parameters_present = false;
    read_value!(input, nal_hrd_parameters_present, bool);
    if nal_hrd_parameters_present {
        todo!("NAL HDR");
    }

    let mut vcl_hrd_parameters_present = false;
    read_value!(input, vcl_hrd_parameters_present, bool);
    if vcl_hrd_parameters_present {
        todo!("VCL HDR");
    }

    read_value!(input, vui.pic_struct_present_flag, bool);
    read_value!(input, vui.bitstream_restriction_flag, bool);
    if vui.bitstream_restriction_flag {
        read_value!(input, vui.motion_vectors_over_pic_boundaries_flag, bool);
        read_value!(input, vui.max_bytes_per_pic_denom, ue, 8);
        read_value!(input, vui.max_bits_per_mb_denom, ue, 8);
        read_value!(input, vui.log2_max_mv_length_horizontal, ue, 8);
        read_value!(input, vui.log2_max_mv_length_vertical, ue, 8);
        read_value!(input, vui.max_num_reorder_frames, ue, 8);
        read_value!(input, vui.max_dec_frame_buffering, ue, 8);
    }

    Ok((input, vui))
}

// 7.3.2.1.1 Sequence parameter set data syntax
pub fn parse_sps(i: BitInput) -> ParseResult<SequenceParameterSet> {
    let mut sps = SequenceParameterSet::default();
    let mut input = i;

    read_value!(input, sps.profile, u, 8);
    read_value!(input, sps.constraint_set0_flag, bool);
    read_value!(input, sps.constraint_set1_flag, bool);
    read_value!(input, sps.constraint_set2_flag, bool);
    read_value!(input, sps.constraint_set3_flag, bool);
    read_value!(input, sps.constraint_set4_flag, bool);
    read_value!(input, sps.constraint_set5_flag, bool);

    let (i, _) = context("reserved_zero_2bits", tag(0b00, 2u8))(input)?;
    input = i;

    read_value!(input, sps.level_idc, u, 8);
    read_value!(input, sps.seq_parameter_set_id, ue, 8);

    if sps.profile.has_chroma_info() {
        read_value!(input, sps.chroma_format_idc, ue, 8);

        if sps.chroma_format_idc == 3 {
            read_value!(input, sps.separate_colour_plane_flag, bool);
        }

        read_value!(input, sps.bit_depth_luma_minus8, ue, 8);
        read_value!(input, sps.bit_depth_chroma_minus8, ue, 8);
        read_value!(input, sps.qpprime_y_zero_transform_bypass_flag, bool);
        read_value!(input, sps.seq_scaling_matrix_present_flag, bool);
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
            let (input, offsets) = count(se(), cnt_cycle as usize)(input)?;
            sps.offset_for_ref_frame = offsets;
        }
        _ => {}
    };

    read_value!(input, sps.max_num_ref_frames, ue, 8);
    read_value!(input, sps.gaps_in_frame_num_value_allowed_flag, bool);

    read_value!(input, sps.pic_width_in_mbs_minus1, ue, 16);
    read_value!(input, sps.pic_height_in_map_units_minus1, ue, 16);

    read_value!(input, sps.frame_mbs_only_flag, bool);
    if sps.frame_mbs_only_flag {
        sps.mb_adaptive_frame_field_flag = false;
    } else {
        read_value!(input, sps.mb_adaptive_frame_field_flag, bool);
    }

    read_value!(input, sps.direct_8x8_inference_flag, bool);
    read_value!(input, sps.frame_cropping_flag, bool);
    if sps.frame_cropping_flag {
        read_value!(input, sps.frame_crop_left_offset, ue, 32);
        read_value!(input, sps.frame_crop_right_offset, ue, 32);
        read_value!(input, sps.frame_crop_top_offset, ue, 32);
        read_value!(input, sps.frame_crop_bottom_offset, ue, 32);
    }

    let mut vui_parameters_present = false;
    read_value!(input, vui_parameters_present, bool);
    if vui_parameters_present {
        let (i, vui) = parse_vui(input)?;
        sps.vui_parameters = Some(vui);
        input = i;
    }
    rbsp_trailing_bits(input)?;

    Ok((input, sps))
}

fn parse_slice_group(i: BitInput) -> ParseResult<Option<SliceGroup>> {
    let mut slice_group: Option<SliceGroup> = None;
    let mut input = i;

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
                read_value!(input, slice_group_change_direction_flag, bool);
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

                let slice_group_id_bits = 1 + num_slice_groups_minus1.ilog2() as usize;
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

    Ok((input, slice_group))
}

// Section 7.3.2.2 Picture parameter set RBSP syntax
pub fn parse_pps(i: BitInput) -> ParseResult<PicParameterSet> {
    let mut pps = PicParameterSet::default();
    let mut input = i;

    read_value!(input, pps.pic_parameter_set_id, ue, 8);
    read_value!(input, pps.seq_parameter_set_id, ue, 8);
    read_value!(input, pps.entropy_coding_mode_flag, bool);
    read_value!(input, pps.bottom_field_pic_order_in_frame_present_flag, bool);

    let (i, group) = parse_slice_group(input)?;
    input = i;
    pps.slice_group = group;

    read_value!(input, pps.num_ref_idx_l0_default_active_minus1, ue, 32);
    read_value!(input, pps.num_ref_idx_l1_default_active_minus1, ue, 32);
    read_value!(input, pps.weighted_pred_flag, bool);
    read_value!(input, pps.weighted_bipred_idc, u, 2);
    read_value!(input, pps.pic_init_qp_minus26, se);
    read_value!(input, pps.pic_init_qs_minus26, se);
    read_value!(input, pps.chroma_qp_index_offset, se);
    read_value!(input, pps.deblocking_filter_control_present_flag, bool);
    read_value!(input, pps.constrained_intra_pred_flag, bool);
    read_value!(input, pps.redundant_pic_cnt_present_flag, bool);

    if more_rbsp_data(input) {
        read_value!(input, pps.transform_8x8_mode_flag, bool);
        let mut pic_scaling_matrix_present_flag = false;
        read_value!(input, pic_scaling_matrix_present_flag, bool);
        if pic_scaling_matrix_present_flag {
            todo!("scaling matrix");
        }
        read_value!(input, pps.second_chroma_qp_index_offset, se);
    } else {
        pps.transform_8x8_mode_flag = false;
        pps.second_chroma_qp_index_offset = pps.chroma_qp_index_offset;
    }
    rbsp_trailing_bits(input)?;
    Ok((input, pps))
}

// Section 7.3.3 Slice header syntax
pub fn parse_slice_header<'a>(ctx: &DecoderContext, i: BitInput<'a>) -> ParseResult<'a, Slice> {
    let idr_pic_flag = true;
    let nal_ref_idc = 3;
    let nal_type = 5;

    let mut header = SliceHeader::default();
    let mut input = i;
    read_value!(input, header.first_mb_in_slice, ue, 32);
    read_value!(input, header.slice_type, ue, 8);
    read_value!(input, header.pic_parameter_set_id, ue, 8);

    let pps = match ctx.get_pps(header.pic_parameter_set_id) {
        Some(pps) => pps,
        None => {
            return Err(make_error(input, "PPS is missing in context"));
        }
    };
    let sps = match ctx.get_sps(pps.seq_parameter_set_id) {
        Some(sps) => sps,
        None => {
            return Err(make_error(input, "SPS is missing in context"));
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
        read_value!(input, header.field_pic_flag, bool);
        if header.field_pic_flag {
            let mut bottom_field_flag = false;
            read_value!(input, bottom_field_flag, bool);
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

    if nal_ref_idc != 0 {
        if (idr_pic_flag) {
            let mut no_output_of_prior_pics_flag = false;
            let mut long_term_reference_flag = false;
            read_value!(input, no_output_of_prior_pics_flag, bool);
            read_value!(input, long_term_reference_flag, bool);
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

    Ok((input, Slice { sps: sps.clone(), pps: pps.clone(), header }))
}

pub fn parse_macroblock<'a>(i: BitInput<'a>, slice: &Slice) -> ParseResult<'a, Macroblock> {
    let mut input = i;
    let mut block: Macroblock = Macroblock::default();
    read_value!(input, block.mb_type, ue);

    if block.mb_type == IMacroblockType::I_PCM {
        todo!("PCM macroblock");
    } else {
        if slice.pps.transform_8x8_mode_flag && block.mb_type == IMacroblockType::I_NxN {
            read_value!(input, block.transform_size_8x8_flag, bool);
        }
        let prediction_mode = block.MbPartPredMode(0);
    }

    Ok((input, block))
}

// Section 7.3.4 Slice data syntax
pub fn parse_slice_data<'a>(i: BitInput<'a>, slice: &Slice) -> ParseResult<'a, Vec<Macroblock>> {
    let mut input = i;
    let mut blocks = Vec::<Macroblock>::new();

    // Baseline profile features
    assert!(!slice.pps.entropy_coding_mode_flag, "entropy coding is not implemented yet");
    assert!(!slice.pps.transform_8x8_mode_flag, "8x8 transform decoding is not implemented yet");
    assert!(slice.sps.frame_mbs_only_flag, "interlaced video is not implemented yet");

    if slice.pps.entropy_coding_mode_flag {
        // cabac_alignment_one_bit
        let (i, _) = align_till_next_byte(input, true)?;
        input = i;
    }

    if slice.header.slice_type != SliceType::I && slice.header.slice_type != SliceType::SI {
        todo!("non I-slices");
    }

    let mut curr_mb_addr = slice.header.first_mb_in_slice * (1 + slice.MbaffFrameFlag() as u32);
    let mut more_data = true;
    let mut prev_mb_skipped = false;
    while more_data {
        let (i, block) = parse_macroblock(input, slice)?;
        input = i;
        blocks.push(block);
        more_data = false;
    }

    Ok((input, blocks))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_ue() {
        assert_eq!(0, ue(8).parse((&[0b10000000], 0)).unwrap().1);
        assert_eq!(1, ue(8).parse((&[0b01000000], 0)).unwrap().1);
        assert_eq!(2, ue(8).parse((&[0b01100000], 0)).unwrap().1);
        assert_eq!(3, ue(8).parse((&[0b00100000], 0)).unwrap().1);
        assert_eq!(4, ue(8).parse((&[0b00101000], 0)).unwrap().1);
        assert_eq!(5, ue(8).parse((&[0b00110000], 0)).unwrap().1);
        assert_eq!(6, ue(8).parse((&[0b00111000], 0)).unwrap().1);
        assert_eq!(7, ue(8).parse((&[0b00010000], 0)).unwrap().1);
        assert_eq!(8, ue(8).parse((&[0b00010010], 0)).unwrap().1);
        assert_eq!(9, ue(8).parse((&[0b00010100], 0)).unwrap().1);
    }

    #[test]
    pub fn test_se() {
        assert_eq!(0, se().parse((&[0b10000000], 0)).unwrap().1);
        assert_eq!(1, se().parse((&[0b01000000], 0)).unwrap().1);
        assert_eq!(-1, se().parse((&[0b01100000], 0)).unwrap().1);
        assert_eq!(2, se().parse((&[0b00100000], 0)).unwrap().1);
        assert_eq!(-2, se().parse((&[0b00101000], 0)).unwrap().1);
        assert_eq!(3, se().parse((&[0b00110000], 0)).unwrap().1);
        assert_eq!(-3, se().parse((&[0b00111000], 0)).unwrap().1);
        assert_eq!(4, se().parse((&[0b00010000], 0)).unwrap().1);
        assert_eq!(-4, se().parse((&[0b00010010], 0)).unwrap().1);
        assert_eq!(5, se().parse((&[0b00010100], 0)).unwrap().1);
    }

    fn parse_sps_test(data: &[u8]) -> SequenceParameterSet {
        let sps = parse_sps((data, 0)).expect("SPS parsing failed").1;
        sps
    }

    fn parse_pps_test(data: &[u8]) -> PicParameterSet {
        let pps = parse_pps((data, 0)).expect("PPS parsing failed").1;
        pps
    }

    #[test]
    pub fn test_slice() {
        let sps = SequenceParameterSet {
            profile: Profile::Baseline,
            constraint_set0_flag: true,
            constraint_set1_flag: true,
            level_idc: 20,
            seq_parameter_set_id: 0,
            chroma_format_idc: 0,
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
            0xB8, 0x00, 0x04, 0x00, 0x00, 0x09, 0xFF, 0xFF, 0xF8, 0x7A, 0x28, 0x00, 0x08, 0x24,
            0x79, 0x31, 0x72, 0x72, 0x75, 0x8B, 0xAE, 0xBA, 0xEB, 0xAE, 0xBA, 0xEB, 0xAE, 0xBA,
            0xF0,
        ];
        let (input, slice) =
            parse_slice_header(&ctx, (&slice_data, 0)).expect("header parsing failed");
        let header = &slice.header;
        assert_eq!(header.slice_type, SliceType::I);
        assert_eq!(header.frame_num, 0);
        assert_eq!(header.pic_parameter_set_id, 0);
        assert_eq!(header.idr_pic_id, Some(1));
        assert_eq!(header.pic_order_cnt_lsb, Some(0));
        assert_eq!(header.slice_qp_delta, -4);
        assert_eq!(header.disable_deblocking_filter_idc, 0);

        let (input, blocks) = parse_slice_data(input, &slice).expect("blocks parsing failed");
        assert_eq!(blocks.len(), 1);
        println!("{:?}", blocks[0].mb_type);
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
}
