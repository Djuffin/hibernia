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

fn flag(i: BitInput) -> IResult<BitInput, bool> {
    bool(i)
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

fn ue_8(i: BitInput) -> IResult<BitInput, u8> {
    let (i, x) = ue(8, i)?;
    Ok((i, x as u8))
}

fn ue_16(i: BitInput) -> IResult<BitInput, u16> {
    let (i, x) = ue(16, i)?;
    Ok((i, x as u16))
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

fn parse_vui(input: BitInput) -> IResult<BitInput, sps::VuiParameters> {
    let mut result = sps::VuiParameters::default();

    let (input, flg) = flag(input)?;
    result.aspect_ratio_info_present_flag = flg;

    let input = if (result.aspect_ratio_info_present_flag) {
        const EXTENDED_SAR:u8 = 255;
        let (input, value) = ue_8(input)?;
        result.aspect_ratio_idc = value as u8;
        if (result.aspect_ratio_idc == EXTENDED_SAR) {
            let (input, width) = ue_16(input)?;
            result.sar_width = width;
            let (input, height) = ue_16(input)?;
            result.sar_height = height;
            input
        } else {
            input
        }
    } else {
        input
    };

    let (input, flg) = flag(input)?;
    result.overscan_info_present_flag = flg;
    let input = if (result.overscan_info_present_flag) {
        let (input, flg) = flag(input)?;
        result.overscan_appropriate_flag = flg;
        input
    } else {
        input
    };


    let (input, flg) = flag(input)?;
    result.video_signal_type_present_flag = flg;

    let input = if (result.video_signal_type_present_flag) {
        unimplemented!();
    } else {
        input
    };

    Ok((input, result))
}

fn parse_sps(input: BitInput) -> IResult<BitInput, sps::SequenceParameterSet> {
    let mut result = sps::SequenceParameterSet::default();

    let (input, profile_idc) = u(8, input)?;
    result.profile_idc = sps::ProfileIdc(profile_idc as u8);

    let (input, flags) = count(bool, 6)(input)?;
    result.constraint_set0_flag = flags[0];
    result.constraint_set1_flag = flags[1];
    result.constraint_set2_flag = flags[2];
    result.constraint_set3_flag = flags[3];
    result.constraint_set4_flag = flags[4];
    result.constraint_set5_flag = flags[4];

    let (input, _) = tag(0x00, 2u8)(input)?; // two reserver zero bits

    let (input, level_idc) = u(8, input)?;
    result.level_idc = level_idc as u8;

    let (input, sps_id) = ue_8(input)?;
    result.seq_parameter_set_id = sps_id as u8;

    let input = if (result.profile_idc.has_chroma_info()) {
        let (input, chroma_format_idc) = ue(8, input)?;
        result.chroma_format_idc = chroma_format_idc as u8;

        let input = if (chroma_format_idc == 3) {
            let (input, flg) = flag(input)?;
            result.separate_colour_plane_flag = flg;
            input
        } else {
            input
        };

        let (input, depths) = count(ue_8, 2)(input)?;
        result.bit_depth_luma_minus8 = depths[0] as u8;
        result.bit_depth_chroma_minus8 = depths[1] as u8;

        let (input, flg) = flag(input)?;
        result.qpprime_y_zero_transform_bypass_flag = flg;

        let (input, flg) = flag(input)?;
        result.seq_scaling_matrix_present_flag = flg;
        if (result.seq_scaling_matrix_present_flag) {
             return Err(Err::Error(Error::new(input, ErrorKind::Not)));
        }
        input
    } else {
        input
    };

    let (input, value) = ue_8(input)?;
    result.log2_max_frame_num_minus4 = value as u8;

    let (input, pic_order_cnt_type) = ue_8(input)?;
    result.pic_order_cnt_type = pic_order_cnt_type as u8;
    let input = match pic_order_cnt_type {
        0 => {
            let (input, value) = ue_8(input)?;
            result.log2_max_pic_order_cnt_lsb_minus4 = value;
            input

        },
        1 => {
            let (input, value) = se(input)?;
            result.offset_for_non_ref_pic = value;

            let (input, value) = se(input)?;
            result.offset_for_top_to_bottom_field = value;

            let (input, cnt_cycle) = ue_8(input)?;
            let (input, offsets) = count(se, cnt_cycle as usize)(input)?;
            result.offset_for_ref_frame = offsets;

            input
        },
        _ => input
    };

    let (input, value) = ue_8(input)?;
    result.max_num_ref_frames = value as u8;

    let (input, flg) = flag(input)?;
    result.gaps_in_frame_num_value_allowed_flag = flg;

    let (input, value) = ue_16(input)?;
    result.pic_width_in_mbs_minus1 = value;

    let (input, value) = ue_16(input)?;
    result.pic_height_in_map_units_minus1 = value;

    let (input, flg) = flag(input)?;
    result.frame_mbs_flags = flg;

    let input = if (result.frame_mbs_flags) {
        input
    } else {
        let (input, flg) = flag(input)?;
        result.mb_adaptive_frame_field_flag = flg;
        input
    };

    let (input, flg) = flag(input)?;
    result.direct_8x8_inference_flag = flg;

    let (input, flg) = flag(input)?;
    result.frame_cropping_flag = flg;

    let input = if (result.frame_cropping_flag) {
        let (input, offsets) = count(ue_8, 4)(input)?;
        result.frame_crop_left_offset = offsets[0];
        result.frame_crop_right_offset = offsets[1];
        result.frame_crop_top_offset = offsets[2];
        result.frame_crop_bottom_offset = offsets[3];
        input
    } else {
        input
    };

    let (input, vui_parameters_present) = flag(input)?;
    if (vui_parameters_present) {
        let (input, vui) = parse_vui(input)?;
        result.vui_parameters = Some(vui);
    }

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
