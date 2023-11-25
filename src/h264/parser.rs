use super::pps;
use super::sps;

use super::pps::{PicParameterSet, SliceGroup, SliceGroupChangeType, SliceRect};
use super::sps::{ProfileIdc, SequenceParameterSet, VuiParameters};

use nom::{
    bits::complete::*,
    error::{context, convert_error, Error, ErrorKind, ParseError, VerboseError, VerboseErrorKind},
    multi::count,
    multi::many0_count,
    Err, IResult, Parser,
};
type BitInput<'a> = (&'a [u8], usize);
type ParseResult<'a, T> = IResult<BitInput<'a>, T, VerboseError<BitInput<'a>>>;

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
        if (zero_bits >= n || result as u64 >= 1u64 << n) {
            return Err(Err::Error(VerboseError::from_error_kind(
                i,
                ErrorKind::Verify,
            )));
        }
        Ok((i, result))
    }
}

fn se<'a>() -> impl Parser<BitInput<'a>, i32, VerboseError<BitInput<'a>>> {
    move |i| {
        // Mapping process for signed Exp-Golomb codes Section 9.1.1
        let (i, value) = ue(32).parse(i)?;
        let result: i32;
        if (value & 1 != 0) {
            result = ((value >> 1) + 1) as i32;
        } else {
            result = -((value >> 1) as i32);
        }
        Ok((i, result))
    }
}

macro_rules! read_value {
    ($input:ident, $dest:expr, u, 2) => {
        let context_str = stringify!($dest);
        let offset = $input.1;
        let byte_value = $input.0[0];
        let (i, value) = context(context_str, u(2)).parse($input)?;
        $input = i;
        println!(
            "u2 {:?}[{:?}]: {:?} = {:?}",
            byte_value, offset, context_str, value
        );
        $dest = value as u8;
    };
    ($input:ident, $dest:expr, u, 3) => {
        let context_str = stringify!($dest);
        let offset = $input.1;
        let byte_value = $input.0[0];
        let (i, value) = context(context_str, u(3)).parse($input)?;
        $input = i;
        println!(
            "u3 {:?}[{:?}]: {:?} = {:?}",
            byte_value, offset, context_str, value
        );
        $dest = value as u8;
    };
    ($input:ident, $dest:expr, u, 8) => {
        let context_str = stringify!($dest);
        let offset = $input.1;
        let byte_value = $input.0[0];
        let (i, value) = context(context_str, u(8)).parse($input)?;
        $input = i;
        println!(
            "u8 {:?}[{:?}]: {:?} = {:?}",
            byte_value, offset, context_str, value
        );
        $dest = value as u8;
    };
    ($input:ident, $dest:expr, u, 16) => {
        let context_str = stringify!($dest);
        let offset = $input.1;
        let byte_value = $input.0[0];
        let (i, value) = context(context_str, u(16)).parse($input)?;
        $input = i;
        println!(
            "u16 {:?}[{:?}]: {:?} = {:?}",
            byte_value, offset, context_str, value
        );
        $dest = value as u16;
    };
    ($input:ident, $dest:expr, u, 32) => {
        let context_str = stringify!($dest);
        let offset = $input.1;
        let byte_value = $input.0[0];
        let (i, value) = context(context_str, u(32)).parse($input)?;
        $input = i;
        println!(
            "u32 {:?}[{:?}]: {:?} = {:?}",
            byte_value, offset, context_str, value
        );
        $dest = value;
    };
    ($input:ident, $dest:expr, ue) => {
        let context_str = stringify!($dest);
        let offset = $input.1;
        let byte_value = $input.0[0];
        let (i, value) = context(context_str, ue(32)).parse($input)?;
        $input = i;
        println!(
            "u {:?}[{:?}]: {:?} = {:?}",
            byte_value, offset, context_str, value
        );
        $dest = value as usize;
    };
    ($input:ident, $dest:expr, ue, 8) => {
        let context_str = stringify!($dest);
        let offset = $input.1;
        let byte_value = $input.0[0];
        let (i, value) = context(context_str, ue(8)).parse($input)?;
        $input = i;
        println!(
            "ue8 {:?}[{:?}]: {:?} = {:?}",
            byte_value, offset, context_str, value
        );
        $dest = value as u8;
    };
    ($input:ident, $dest:expr, ue, 16) => {
        let context_str = stringify!($dest);
        let offset = $input.1;
        let byte_value = $input.0[0];
        let (i, value) = context(context_str, ue(16)).parse($input)?;
        $input = i;
        println!(
            "ue16 {:?}[{:?}]: {:?} = {:?}",
            byte_value, offset, context_str, value
        );
        $dest = value as u16;
    };
    ($input:ident, $dest:expr, ue, 32) => {
        let context_str = stringify!($dest);
        let offset = $input.1;
        let byte_value = $input.0[0];
        let (i, value) = context(context_str, ue(32)).parse($input)?;
        $input = i;
        println!(
            "ue32 {:?}[{:?}]: {:?} = {:?}",
            byte_value, offset, context_str, value
        );
        $dest = value as u32;
    };
    ($input:ident, $dest:expr, se) => {
        let context_str = stringify!($dest);
        let offset = $input.1;
        let byte_value = $input.0[0];
        let (i, value) = context(context_str, se()).parse($input)?;
        $input = i;
        println!(
            "se {:?}[{:?}]: {:?} = {:?}",
            byte_value, offset, context_str, value
        );
        $dest = value;
    };
    ($input:ident, $dest:expr, bool) => {
        let context_str = stringify!($dest);
        let offset = $input.1;
        let byte_value = $input.0[0];
        let (i, value) = context(context_str, bool).parse($input)?;
        $input = i;
        println!(
            "flag {:?}[{:?}]: {:?} = {:?}",
            byte_value, offset, context_str, value
        );
        $dest = value;
    };
}

fn rbsp_trailing_bits(input: BitInput) -> ParseResult<()> {
    // 1-bit at the end
    let (input, _) = context("rbsp_trailing_bits_sentinel", tag(1, 1u8))(input)?;
    let bit_position = input.1;

    // alignment with 0-bits till next byte
    let input = if (bit_position % 8 != 0) {
        let zero_bits_count = 8 - (bit_position % 8);
        println!("{:?}", zero_bits_count);
        let (input, _) = context("rbsp_trailing_bits_padding", tag(0, zero_bits_count))(input)?;
        input
    } else {
        input
    };

    Ok((input, ()))
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
    if (nal_hrd_parameters_present) {
        unimplemented!();
    }

    let mut vcl_hrd_parameters_present = false;
    read_value!(input, vcl_hrd_parameters_present, bool);
    if (vcl_hrd_parameters_present) {
        unimplemented!();
    }
    //read_value!(input, vui.nal_hrd_parameters, Option<HdrParameters>);
    //read_value!(input, vui.vcl_hrd_parameters, Option<HdrParameters>);
    //read_value!(input, vui.low_delay_hrd_flag, bool);

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

fn parse_sps(i: BitInput) -> ParseResult<SequenceParameterSet> {
    let mut sps = SequenceParameterSet::default();
    let mut input = i;

    let mut profile_idc = 0u8;
    read_value!(input, profile_idc, u, 8);
    sps.profile_idc = ProfileIdc(profile_idc);

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

    if sps.profile_idc.has_chroma_info() {
        read_value!(input, sps.chroma_format_idc, ue, 8);

        if sps.chroma_format_idc == 3 {
            read_value!(input, sps.separate_colour_plane_flag, bool);
        }

        read_value!(input, sps.bit_depth_luma_minus8, ue, 8);
        read_value!(input, sps.bit_depth_chroma_minus8, ue, 8);
        read_value!(input, sps.qpprime_y_zero_transform_bypass_flag, bool);
        read_value!(input, sps.seq_scaling_matrix_present_flag, bool);
        if (sps.seq_scaling_matrix_present_flag) {
            unimplemented!();
            //return Err(Err::Error(Error::new(input, ErrorKind::Not)));
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
    if !sps.frame_mbs_only_flag {
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
    if (num_slice_groups_minus1 > 0) {
        read_value!(input, slice_group_map_type, ue, 8);
        slice_group = match slice_group_map_type {
            0 => {
                let mut run_length_minus1 = vec![1u32; num_slice_groups_minus1 + 1];
                for i in 0..=num_slice_groups_minus1 {
                    read_value!(input, run_length_minus1[i], ue, 32);
                }
                Some(SliceGroup::Interleaved {
                    run_length_minus1: run_length_minus1,
                })
            }
            1 => Some(SliceGroup::Dispersed {
                num_slice_groups_minus1: num_slice_groups_minus1,
            }),
            2 => {
                let mut rectangles = vec![SliceRect::default(); num_slice_groups_minus1 + 1];
                for i in 0..=num_slice_groups_minus1 {
                    read_value!(input, rectangles[i].top_left, ue, 32);
                    read_value!(input, rectangles[i].bottom_right, ue, 32);
                }
                Some(SliceGroup::Foreground {
                    rectangles: rectangles,
                })
            }
            3 | 4 | 5 => {
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
                    change_type: change_type,
                    num_slice_groups_minus1: num_slice_groups_minus1,
                    slice_group_change_direction_flag: slice_group_change_direction_flag,
                    slice_group_change_rate_minus1: slice_group_change_rate_minus1,
                })
            }
            6 => {
                let mut pic_size_in_map_units_minus1: usize = 0;
                read_value!(input, pic_size_in_map_units_minus1, ue);

                let slice_group_id_bits = 1 + num_slice_groups_minus1.ilog2() as usize;
                let mut slice_group_ids = vec![0u32; num_slice_groups_minus1 + 1];
                for i in 0..=num_slice_groups_minus1 {
                    let (ipt, value) =
                        context("slice_group_ids", u(slice_group_id_bits)).parse(input)?;
                    slice_group_ids[i] = value;
                    input = ipt;
                }

                Some(SliceGroup::Explicit {
                    num_slice_groups_minus1: num_slice_groups_minus1,
                    slice_group_id: slice_group_ids,
                })
            }
            _ => None,
        }
    }

    Ok((input, slice_group))
}

fn parse_pps(i: BitInput) -> ParseResult<PicParameterSet> {
    let mut pps = PicParameterSet::default();
    let mut input = i;

    read_value!(input, pps.pic_parameter_set_id, ue, 8);
    read_value!(input, pps.seq_parameter_set_id, ue, 8);
    read_value!(input, pps.entropy_coding_mode_flag, bool);
    read_value!(
        input,
        pps.bottom_field_pic_order_in_frame_present_flag,
        bool
    );

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

    //rbsp_trailing_bits(input)?;
    Ok((input, pps))
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

    fn parse_pps_test(data: &[u8]) -> pps::PicParameterSet {
        let pps = parse_pps((data, 0)).expect("PPS parsing failed").1;
        pps
    }

    #[test]
    pub fn test_sps1() {
        let data = [
            0x64, 0x00, 0x0A, 0xAC, 0x72, 0x84, 0x44, 0x26, 0x84, 0x00, 0x00, 0x00, 0x04, 0x00,
            0x00, 0x00, 0xCA, 0x3C, 0x48, 0x96, 0x11, 0x80,
        ];
        let sps = parse_sps_test(&data);
        assert_eq!(sps.profile_idc, sps::ProfileIdc(100), "profile");
        assert_eq!(sps.constraint_set0_flag, false);
        assert_eq!(sps.constraint_set1_flag, false);
        assert_eq!(sps.constraint_set2_flag, false);
        assert_eq!(sps.constraint_set3_flag, false);
        assert_eq!(sps.constraint_set4_flag, false);
        assert_eq!(sps.constraint_set5_flag, false);
        assert_eq!(sps.level_idc, 10, "level");
        assert_eq!(sps.pic_width_in_mbs_minus1, 3, "pic_width_in_mbs_minus1");
        assert_eq!(
            sps.pic_height_in_map_units_minus1, 3,
            "pic_width_in_mbs_minus1"
        );
    }

    #[test]
    pub fn test_pps1() {
        let data = [0xE8, 0x43, 0x8F, 0x13, 0x21, 0x30];
        let pps = parse_pps_test(&data);
        assert_eq!(pps.pic_parameter_set_id, 0, "pic_parameter_set_id");
        assert_eq!(pps.seq_parameter_set_id, 0, "seq_parameter_set_id");
    }
}
