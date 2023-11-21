use super::sps;

use nom::{
    bits::complete::*,
    multi::many0_count,
    multi::count,
    error::make_error,
    error::ErrorKind,
    Err,
    error::Error,
    IResult
};
type BitInput<'a> = (&'a [u8], usize);

fn u(n: usize, i: BitInput) -> IResult<BitInput, u32> {
    take(n)(i)
}

fn ue(n: usize, i: BitInput) -> IResult<BitInput, u32> {
    // Parsing process for Exp-Golomb codes. Section 9.1
    let (i, zero_bits) = many0_count(tag(0, 1u8))(i)?;
    let (i, _) = tag(1, 1u8)(i)?;
    let (i, x) = u(zero_bits, i)?;
    let result = (1u32 << zero_bits) - 1 + x;
    if (zero_bits >= n || result as u64 >= 1u64 << n) {
        return Err(Err::Error(Error::new(i, ErrorKind::Verify)));
    }
    Ok((i, result))
}

fn se(i: BitInput) -> IResult<BitInput, i32> {
    // Mapping process for signed Exp-Golomb codes Section 9.1.1
    let (i, value) = ue(32, i)?;
    let result: i32;
    if (value & 1 != 0) {
        result = ((value >> 1) + 1) as i32;
    } else {
        result = -((value >> 1) as i32);
    }
    Ok((i, result))
}

macro_rules! read_value {
    ($input:ident, $dest:expr, u, 3) => {
        let ($input, value) = u(3, $input)?;
        $dest = value as u8;
    };
    ($input:ident, $dest:expr, u, 8) => {
        let ($input, value) = u(8, $input)?;
        $dest = value as u8;
    };
    ($input:ident, $dest:expr, u, 16) => {
        let ($input, value) = u(16, $input)?;
        $dest = value as u16;
    };
    ($input:ident, $dest:expr, u, 32) => {
        let ($input, value) = u(32, $input)?;
        $dest = value;
    };
    ($input:ident, $dest:expr, ue, 8) => {
        let ($input, value) = ue(8, $input)?;
        $dest = value as u8;
    };
    ($input:ident, $dest:expr, ue, 16) => {
        let ($input, value) = ue(16, $input)?;
        $dest = value as u16;
    };
    ($input:ident, $dest:expr, ue, 32) => {
        let ($input, value) = ue(32, $input)?;
        $dest = value as u32;
    };
    ($input:ident, $dest:expr, se) => {
        let ($input, value) = se($input)?;
        $dest = value;
    };
    ($input:ident, $dest:expr, bool) => {
        let ($input, value) = bool($input)?;
        $dest = value;
    };
}

macro_rules! if_flag {
    ($input:ident, $condition:expr, $branch:block) => {
        let $input = if ($condition) {
            $branch;
            $input
        } else {
            $input
        };
    };
}

fn parse_vui(input: BitInput) -> IResult<BitInput, sps::VuiParameters> {
    let mut result = sps::VuiParameters::default();

    read_value!(input, result.aspect_ratio_info_present_flag, bool);
    if_flag! {input, result.aspect_ratio_info_present_flag, {
        const EXTENDED_SAR:u8 = 255;
        read_value!(input, result.aspect_ratio_idc, u, 8);
        if_flag! {input, result.aspect_ratio_idc == EXTENDED_SAR, {
            read_value!(input, result.sar_width, u, 16);
            read_value!(input, result.sar_height, u, 16);
        }}
    }}

    if_flag! {input, result.overscan_info_present_flag, {
        read_value!(input, result.overscan_appropriate_flag, bool);
    }}

    read_value!(input, result.video_signal_type_present_flag, bool);
    if_flag! {input, result.video_signal_type_present_flag, {
        read_value!(input, result.video_format, u, 3);
        read_value!(input, result.video_full_range_flag, bool);
        read_value!(input, result.colour_description_present_flag, bool);
        if_flag! {input, result.colour_description_present_flag, {
            read_value!(input, result.colour_primaries, u, 8);
            read_value!(input, result.transfer_characteristics, u, 8);
            read_value!(input, result.matrix_coefficients, u, 8);
        }}
    }}

    read_value!(input, result.chroma_loc_info_present_flag, bool);
    if_flag! {input, result.chroma_loc_info_present_flag, {
        read_value!(input, result.chroma_sample_loc_type_top_field, ue, 8);
        read_value!(input, result.chroma_sample_loc_type_bottom_field, ue, 8);
    }}

    read_value!(input, result.timing_info_present_flag, bool);
    if_flag! {input, result.timing_info_present_flag, {
        read_value!(input, result.num_units_in_tick, u, 32);
        read_value!(input, result.time_scale, u, 32);
        read_value!(input, result.fixed_frame_rate_flag, bool);
    }}


    let (input, nal_hrd_parameters_present) = bool(input)?;
    if (nal_hrd_parameters_present) {
        unimplemented!();
    }
    let (input, vcl_hrd_parameters_present) = bool(input)?;
    if (vcl_hrd_parameters_present) {
        unimplemented!();
    }
    //read_value!(input, result.nal_hrd_parameters, Option<HdrParameters>);
    //read_value!(input, result.vcl_hrd_parameters, Option<HdrParameters>);
    //read_value!(input, result.low_delay_hrd_flag, bool);

    read_value!(input, result.pic_struct_present_flag, bool);
    read_value!(input, result.bitstream_restriction_flag, bool);
    if_flag! {input, result.bitstream_restriction_flag, {
        read_value!(input, result.motion_vectors_over_pic_boundaries_flag, bool);
        read_value!(input, result.max_bytes_per_pic_denom, ue, 8);
        read_value!(input, result.max_bits_per_mb_denom, ue, 8);
        read_value!(input, result.log2_max_mv_length_horizontal, ue, 8);
        read_value!(input, result.log2_max_mv_length_vertical, ue, 8);
        read_value!(input, result.max_num_reorder_frames, ue, 8);
        read_value!(input, result.max_dec_frame_buffering, ue, 8);
    }}

    Ok((input, result))
}


fn parse_sps(input: BitInput) -> IResult<BitInput, sps::SequenceParameterSet> {
    let mut result = sps::SequenceParameterSet::default();

    let (input, profile_idc) = u(8, input)?;
    result.profile_idc = sps::ProfileIdc(profile_idc as u8);

    read_value!(input, result.constraint_set0_flag, bool);
    read_value!(input, result.constraint_set1_flag, bool);
    read_value!(input, result.constraint_set2_flag, bool);
    read_value!(input, result.constraint_set3_flag, bool);
    read_value!(input, result.constraint_set4_flag, bool);
    read_value!(input, result.constraint_set5_flag, bool);

    let (input, _) = tag(0x00, 2u8)(input)?; // two reserver zero bits

    read_value!(input, result.level_idc, u, 8);
    read_value!(input, result.seq_parameter_set_id, ue, 8);

    if_flag! {input, result.profile_idc.has_chroma_info(), {
        read_value!(input, result.chroma_format_idc, ue, 8);

        if_flag! {input, result.chroma_format_idc == 3, {
            read_value!(input, result.separate_colour_plane_flag, bool);
        }}

        read_value!(input, result.bit_depth_luma_minus8, ue, 8);
        read_value!(input, result.bit_depth_chroma_minus8, ue, 8);
        read_value!(input, result.qpprime_y_zero_transform_bypass_flag, bool);
        read_value!(input, result.seq_scaling_matrix_present_flag, bool);
        if (result.seq_scaling_matrix_present_flag) {
             return Err(Err::Error(Error::new(input, ErrorKind::Not)));
        }
    }}

    read_value!(input, result.log2_max_frame_num_minus4, ue, 8);
    read_value!(input, result.pic_order_cnt_type, ue, 8);
    let input = match result.pic_order_cnt_type {
        0 => {
            read_value!(input, result.log2_max_pic_order_cnt_lsb_minus4, ue, 8);
            input

        },
        1 => {
            read_value!(input, result.offset_for_non_ref_pic, se);
            read_value!(input, result.offset_for_top_to_bottom_field, se);

            let (input, cnt_cycle) = ue(8, input)?;
            let (input, offsets) = count(se, cnt_cycle as usize)(input)?;
            result.offset_for_ref_frame = offsets;

            input
        },
        _ => input
    };

    read_value!(input, result.max_num_ref_frames, ue, 8);
    read_value!(input, result.gaps_in_frame_num_value_allowed_flag, bool);

    read_value!(input, result.pic_width_in_mbs_minus1, ue, 16);
    read_value!(input, result.pic_height_in_map_units_minus1, ue, 16);

    read_value!(input, result.frame_mbs_only_flag, bool);

    if_flag! {input, !result.frame_mbs_only_flag, {
        read_value!(input, result.mb_adaptive_frame_field_flag, bool);
    }}

    read_value!(input, result.direct_8x8_inference_flag, bool);
    read_value!(input, result.frame_cropping_flag, bool);
    if_flag! {input, !result.frame_cropping_flag, {
        read_value!(input, result.frame_crop_left_offset, ue, 32);
        read_value!(input, result.frame_crop_right_offset, ue, 32);
        read_value!(input, result.frame_crop_top_offset, ue, 32);
        read_value!(input, result.frame_crop_bottom_offset, ue, 32);
    }}

    let (input, vui_parameters_present) = bool(input)?;
    if_flag! {input, vui_parameters_present, {
        let (input, vui) = parse_vui(input)?;
        result.vui_parameters = Some(vui);
    }}

    Ok((input, result))
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
pub fn test_ue() {
 assert_eq!(0, ue(8, (&[0b10000000], 0)).unwrap().1);
 assert_eq!(1, ue(8, (&[0b01000000], 0)).unwrap().1);
 assert_eq!(2, ue(8, (&[0b01100000], 0)).unwrap().1);
 assert_eq!(3, ue(8, (&[0b00100000], 0)).unwrap().1);
 assert_eq!(4, ue(8, (&[0b00101000], 0)).unwrap().1);
 assert_eq!(5, ue(8, (&[0b00110000], 0)).unwrap().1);
 assert_eq!(6, ue(8, (&[0b00111000], 0)).unwrap().1);
 assert_eq!(7, ue(8, (&[0b00010000], 0)).unwrap().1);
 assert_eq!(8, ue(8, (&[0b00010010], 0)).unwrap().1);
 assert_eq!(9, ue(8, (&[0b00010100], 0)).unwrap().1);
}

#[test]
pub fn test_se() {
 assert_eq!(0, se((&[0b10000000], 0)).unwrap().1);
 assert_eq!(1, se((&[0b01000000], 0)).unwrap().1);
 assert_eq!(-1, se((&[0b01100000], 0)).unwrap().1);
 assert_eq!(2, se((&[0b00100000], 0)).unwrap().1);
 assert_eq!(-2, se((&[0b00101000], 0)).unwrap().1);
 assert_eq!(3, se((&[0b00110000], 0)).unwrap().1);
 assert_eq!(-3, se((&[0b00111000], 0)).unwrap().1);
 assert_eq!(4, se((&[0b00010000], 0)).unwrap().1);
 assert_eq!(-4, se((&[0b00010010], 0)).unwrap().1);
 assert_eq!(5, se((&[0b00010100], 0)).unwrap().1);
}

fn parse_sps_test(data: &[u8]) -> sps::SequenceParameterSet {
    parse_sps((data, 0)).expect("SPS parsing failed").1
}

#[test]
pub fn test_sps1() {
    let data = [
        0x64, 0x00, 0x0c, 0xac, 0x3b, 0x50, 0xb0,
        0x4b, 0x42, 0x00, 0x00, 0x03, 0x00, 0x02, 0x00,
        0x00, 0x03, 0x00, 0x3d, 0x08,
    ];
    let sps = parse_sps_test(&data);
    assert_eq!(sps.profile_idc, sps::ProfileIdc(100), "profile");
    assert_eq!(sps.constraint_set0_flag, false);
    assert_eq!(sps.constraint_set1_flag, false);
    assert_eq!(sps.constraint_set2_flag, false);
    assert_eq!(sps.constraint_set3_flag, false);
    assert_eq!(sps.constraint_set4_flag, false);
    assert_eq!(sps.constraint_set5_flag, false);
    assert_eq!(sps.level_idc, 12, "level");
    assert_eq!(sps.seq_parameter_set_id, 0, "sps id");
}

}
