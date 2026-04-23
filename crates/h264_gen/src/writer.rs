use crate::rbsp_writer::{RbspWriter, WriteResult};
use hibernia::h264::pps::{PicParameterSet, SliceGroup, SliceGroupChangeType};
use hibernia::h264::slice::{
    DeblockingFilterIdc, DecRefPicMarking, MemoryManagementControlOperation, PredWeightTable,
    RefPicListModification, RefPicListModifications, SliceHeader, SliceType,
};
use hibernia::h264::sps::{SequenceParameterSet, VuiParameters};
use hibernia::h264::{ChromaFormat, ColorPlane};

pub fn write_sps(sps: &SequenceParameterSet, writer: &mut RbspWriter) -> WriteResult {
    writer.u(8, sps.profile as u32)?;
    writer.f(sps.constraint_set0_flag)?;
    writer.f(sps.constraint_set1_flag)?;
    writer.f(sps.constraint_set2_flag)?;
    writer.f(sps.constraint_set3_flag)?;
    writer.f(sps.constraint_set4_flag)?;
    writer.f(sps.constraint_set5_flag)?;
    writer.u(2, 0)?; // reserved_zero_2bits
    writer.u(8, sps.level_idc as u32)?;
    writer.ue(sps.seq_parameter_set_id as u32)?;

    if sps.profile.has_chroma_info() {
        writer.ue(sps.chroma_format_idc as u32)?;
        if sps.chroma_format_idc == ChromaFormat::YUV444 {
            writer.f(sps.separate_color_plane_flag)?;
        }

        writer.ue(sps.bit_depth_luma_minus8 as u32)?;
        writer.ue(sps.bit_depth_chroma_minus8 as u32)?;
        writer.f(sps.qpprime_y_zero_transform_bypass_flag)?;
        writer.f(sps.seq_scaling_matrix_present_flag)?;
        if sps.seq_scaling_matrix_present_flag {
            todo!("scaling matrix writer");
        }
    }

    writer.ue(sps.log2_max_frame_num_minus4 as u32)?;
    writer.ue(sps.pic_order_cnt_type as u32)?;
    match sps.pic_order_cnt_type {
        0 => {
            writer.ue(sps.log2_max_pic_order_cnt_lsb_minus4 as u32)?;
        }
        1 => {
            writer.f(sps.delta_pic_order_always_zero_flag)?;
            writer.se(sps.offset_for_non_ref_pic)?;
            writer.se(sps.offset_for_top_to_bottom_field)?;

            writer.ue(sps.offset_for_ref_frame.len() as u32)?;
            for offset in &sps.offset_for_ref_frame {
                writer.se(*offset)?;
            }
        }
        _ => {}
    };

    writer.ue(sps.max_num_ref_frames as u32)?;
    writer.f(sps.gaps_in_frame_num_value_allowed_flag)?;

    writer.ue(sps.pic_width_in_mbs_minus1 as u32)?;
    writer.ue(sps.pic_height_in_map_units_minus1 as u32)?;

    writer.f(sps.frame_mbs_only_flag)?;
    if !sps.frame_mbs_only_flag {
        writer.f(sps.mb_adaptive_frame_field_flag)?;
    }

    writer.f(sps.direct_8x8_inference_flag)?;

    if let Some(crop) = &sps.frame_cropping {
        writer.f(true)?;
        writer.ue(crop.left)?;
        writer.ue(crop.right)?;
        writer.ue(crop.top)?;
        writer.ue(crop.bottom)?;
    } else {
        writer.f(false)?;
    }

    if let Some(vui) = &sps.vui_parameters {
        writer.f(true)?;
        write_vui_parameters(vui, writer)?;
    } else {
        writer.f(false)?;
    }
    writer.rbsp_trailing_bits()?;

    Ok(())
}

pub fn write_vui_parameters(vui: &VuiParameters, writer: &mut RbspWriter) -> WriteResult {
    writer.f(vui.aspect_ratio_info_present_flag)?;
    if vui.aspect_ratio_info_present_flag {
        writer.u(8, vui.aspect_ratio_idc as u32)?;
        if vui.aspect_ratio_idc == 255 {
            writer.u(16, vui.sar_width as u32)?;
            writer.u(16, vui.sar_height as u32)?;
        }
    }

    writer.f(vui.overscan_info_present_flag)?;
    if vui.overscan_info_present_flag {
        writer.f(vui.overscan_appropriate_flag)?;
    }

    writer.f(vui.video_signal_type_present_flag)?;
    if vui.video_signal_type_present_flag {
        writer.u(3, vui.video_format as u32)?;
        writer.f(vui.video_full_range_flag)?;
        writer.f(vui.color_description_present_flag)?;
        if vui.color_description_present_flag {
            writer.u(8, vui.color_primaries as u32)?;
            writer.u(8, vui.transfer_characteristics as u32)?;
            writer.u(8, vui.matrix_coefficients as u32)?;
        }
    }

    writer.f(vui.chroma_loc_info_present_flag)?;
    if vui.chroma_loc_info_present_flag {
        writer.ue(vui.chroma_sample_loc_type_top_field as u32)?;
        writer.ue(vui.chroma_sample_loc_type_bottom_field as u32)?;
    }

    writer.f(vui.timing_info_present_flag)?;
    if vui.timing_info_present_flag {
        writer.u(32, vui.num_units_in_tick)?;
        writer.u(32, vui.time_scale)?;
        writer.f(vui.fixed_frame_rate_flag)?;
    }

    writer.f(vui.nal_hrd_parameters.is_some())?;
    if vui.nal_hrd_parameters.is_some() {
        todo!("NAL HRD write");
    }

    writer.f(vui.vcl_hrd_parameters.is_some())?;
    if vui.vcl_hrd_parameters.is_some() {
        todo!("VCL HRD write");
    }

    writer.f(vui.pic_struct_present_flag)?;
    writer.f(vui.bitstream_restriction_flag)?;
    if vui.bitstream_restriction_flag {
        writer.f(vui.motion_vectors_over_pic_boundaries_flag)?;
        writer.ue(vui.max_bytes_per_pic_denom as u32)?;
        writer.ue(vui.max_bits_per_mb_denom as u32)?;
        writer.ue(vui.log2_max_mv_length_horizontal as u32)?;
        writer.ue(vui.log2_max_mv_length_vertical as u32)?;
        writer.ue(vui.max_num_reorder_frames as u32)?;
        writer.ue(vui.max_dec_frame_buffering as u32)?;
    }

    Ok(())
}

pub fn write_pps(pps: &PicParameterSet, writer: &mut RbspWriter) -> WriteResult {
    writer.ue(pps.pic_parameter_set_id as u32)?;
    writer.ue(pps.seq_parameter_set_id as u32)?;
    writer.f(pps.entropy_coding_mode_flag)?;
    writer.f(pps.bottom_field_pic_order_in_frame_present_flag)?;

    if let Some(slice_group) = &pps.slice_group {
        match slice_group {
            SliceGroup::Interleaved { run_length_minus1 } => {
                writer.ue((run_length_minus1.len() - 1) as u32)?;
                writer.ue(0)?;
                for &len in run_length_minus1 {
                    writer.ue(len)?;
                }
            }
            SliceGroup::Dispersed { num_slice_groups_minus1 } => {
                writer.ue(*num_slice_groups_minus1 as u32)?;
                writer.ue(1)?;
            }
            SliceGroup::Foreground { rectangles } => {
                writer.ue((rectangles.len() - 1) as u32)?;
                writer.ue(2)?;
                for rect in rectangles {
                    writer.ue(rect.top_left)?;
                    writer.ue(rect.bottom_right)?;
                }
            }
            SliceGroup::Changing {
                change_type,
                num_slice_groups_minus1,
                slice_group_change_direction_flag,
                slice_group_change_rate_minus1,
            } => {
                writer.ue(*num_slice_groups_minus1 as u32)?;
                match change_type {
                    SliceGroupChangeType::BoxOut => writer.ue(3)?,
                    SliceGroupChangeType::RasterScan => writer.ue(4)?,
                    SliceGroupChangeType::WipeOut => writer.ue(5)?,
                }
                writer.f(*slice_group_change_direction_flag)?;
                writer.ue(*slice_group_change_rate_minus1)?;
            }
            SliceGroup::Explicit { num_slice_groups_minus1, slice_group_id } => {
                writer.ue(*num_slice_groups_minus1 as u32)?;
                writer.ue(6)?;
                writer.ue((slice_group_id.len() - 1) as u32)?;
                let bits = 1 + num_slice_groups_minus1.ilog2() as u8;
                for id in slice_group_id {
                    writer.u(bits, *id)?;
                }
            }
        }
    } else {
        writer.ue(0)?; // num_slice_groups_minus1 = 0
    }

    writer.ue(pps.num_ref_idx_l0_default_active_minus1)?;
    writer.ue(pps.num_ref_idx_l1_default_active_minus1)?;
    writer.f(pps.weighted_pred_flag)?;
    writer.u(2, pps.weighted_bipred_idc as u32)?;
    writer.se(pps.pic_init_qp_minus26)?;
    writer.se(pps.pic_init_qs_minus26)?;
    writer.se(pps.chroma_qp_index_offset)?;
    writer.f(pps.deblocking_filter_control_present_flag)?;
    writer.f(pps.constrained_intra_pred_flag)?;
    writer.f(pps.redundant_pic_cnt_present_flag)?;

    if pps.transform_8x8_mode_flag
        || pps.second_chroma_qp_index_offset != pps.chroma_qp_index_offset
    {
        writer.f(pps.transform_8x8_mode_flag)?;
        writer.f(false)?; // pic_scaling_matrix_present_flag
        writer.se(pps.second_chroma_qp_index_offset)?;
    }

    writer.rbsp_trailing_bits()?;
    Ok(())
}

pub fn write_ref_pic_list_modifications(
    mods: &RefPicListModifications,
    writer: &mut RbspWriter,
    slice_type: SliceType,
) -> WriteResult {
    if slice_type != SliceType::I && slice_type != SliceType::SI {
        writer.f(!mods.list0.is_empty())?;
        for modification in &mods.list0 {
            match modification {
                RefPicListModification::RemapShortTermNegative(abs_diff_pic_num_minus1) => {
                    writer.ue(0)?;
                    writer.ue(*abs_diff_pic_num_minus1)?;
                }
                RefPicListModification::RemapShortTermPositive(abs_diff_pic_num_minus1) => {
                    writer.ue(1)?;
                    writer.ue(*abs_diff_pic_num_minus1)?;
                }
                RefPicListModification::RemapLongTerm(long_term_pic_num) => {
                    writer.ue(2)?;
                    writer.ue(*long_term_pic_num)?;
                }
            }
        }
        if !mods.list0.is_empty() {
            writer.ue(3)?;
        }
    }

    if slice_type == SliceType::B {
        writer.f(!mods.list1.is_empty())?;
        for modification in &mods.list1 {
            match modification {
                RefPicListModification::RemapShortTermNegative(abs_diff_pic_num_minus1) => {
                    writer.ue(0)?;
                    writer.ue(*abs_diff_pic_num_minus1)?;
                }
                RefPicListModification::RemapShortTermPositive(abs_diff_pic_num_minus1) => {
                    writer.ue(1)?;
                    writer.ue(*abs_diff_pic_num_minus1)?;
                }
                RefPicListModification::RemapLongTerm(long_term_pic_num) => {
                    writer.ue(2)?;
                    writer.ue(*long_term_pic_num)?;
                }
            }
        }
        if !mods.list1.is_empty() {
            writer.ue(3)?;
        }
    }

    Ok(())
}

pub fn write_pred_weight_table(
    table: &PredWeightTable,
    writer: &mut RbspWriter,
    slice_header: &SliceHeader,
    sps: &SequenceParameterSet,
) -> WriteResult {
    writer.ue(table.luma_log2_weight_denom)?;
    if sps.ChromaArrayType() != ChromaFormat::Monochrome {
        writer.ue(table.chroma_log2_weight_denom)?;
    }

    for i in 0..=slice_header.num_ref_idx_l0_active_minus1 as usize {
        let factors = &table.list0[i];
        let luma_default = 1 << table.luma_log2_weight_denom;
        let luma_modified = factors.luma_weight != luma_default || factors.luma_offset != 0;
        writer.f(luma_modified)?;
        if luma_modified {
            writer.se(factors.luma_weight)?;
            writer.se(factors.luma_offset)?;
        }

        if sps.ChromaArrayType() != ChromaFormat::Monochrome {
            let chroma_default = 1 << table.chroma_log2_weight_denom;
            let chroma_modified = factors.chroma_weights[0] != chroma_default
                || factors.chroma_offsets[0] != 0
                || factors.chroma_weights[1] != chroma_default
                || factors.chroma_offsets[1] != 0;
            writer.f(chroma_modified)?;
            if chroma_modified {
                for j in 0..2 {
                    writer.se(factors.chroma_weights[j])?;
                    writer.se(factors.chroma_offsets[j])?;
                }
            }
        }
    }

    if slice_header.slice_type == SliceType::B {
        for i in 0..=slice_header.num_ref_idx_l1_active_minus1 as usize {
            let factors = &table.list1[i];
            let luma_default = 1 << table.luma_log2_weight_denom;
            let luma_modified = factors.luma_weight != luma_default || factors.luma_offset != 0;
            writer.f(luma_modified)?;
            if luma_modified {
                writer.se(factors.luma_weight)?;
                writer.se(factors.luma_offset)?;
            }

            if sps.ChromaArrayType() != ChromaFormat::Monochrome {
                let chroma_default = 1 << table.chroma_log2_weight_denom;
                let chroma_modified = factors.chroma_weights[0] != chroma_default
                    || factors.chroma_offsets[0] != 0
                    || factors.chroma_weights[1] != chroma_default
                    || factors.chroma_offsets[1] != 0;
                writer.f(chroma_modified)?;
                if chroma_modified {
                    for j in 0..2 {
                        writer.se(factors.chroma_weights[j])?;
                        writer.se(factors.chroma_offsets[j])?;
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn write_dec_ref_pic_marking(
    marking: &DecRefPicMarking,
    writer: &mut RbspWriter,
    idr_pic_flag: bool,
) -> WriteResult {
    if idr_pic_flag {
        writer.f(marking.no_output_of_prior_pics_flag.unwrap_or(false))?;
        writer.f(marking.long_term_reference_flag.unwrap_or(false))?;
    } else {
        let adaptive = marking.adaptive_ref_pic_marking_mode_flag.unwrap_or(false);
        writer.f(adaptive)?;
        if adaptive {
            for op in &marking.memory_management_operations {
                match op {
                    MemoryManagementControlOperation::MarkShortTermUnused {
                        difference_of_pic_nums_minus1,
                    } => {
                        writer.ue(1)?;
                        writer.ue(*difference_of_pic_nums_minus1)?;
                    }
                    MemoryManagementControlOperation::MarkLongTermUnused { long_term_pic_num } => {
                        writer.ue(2)?;
                        writer.ue(*long_term_pic_num)?;
                    }
                    MemoryManagementControlOperation::MarkShortTermAsLongTerm {
                        difference_of_pic_nums_minus1,
                        long_term_frame_idx,
                    } => {
                        writer.ue(3)?;
                        writer.ue(*difference_of_pic_nums_minus1)?;
                        writer.ue(*long_term_frame_idx)?;
                    }
                    MemoryManagementControlOperation::SetMaxLongTermFrameIdx {
                        max_long_term_frame_idx_plus1,
                    } => {
                        writer.ue(4)?;
                        writer.ue(*max_long_term_frame_idx_plus1)?;
                    }
                    MemoryManagementControlOperation::MarkAllUnused => {
                        writer.ue(5)?;
                    }
                    MemoryManagementControlOperation::MarkCurrentAsLongTerm {
                        long_term_frame_idx,
                    } => {
                        writer.ue(6)?;
                        writer.ue(*long_term_frame_idx)?;
                    }
                }
            }
            writer.ue(0)?;
        }
    }
    Ok(())
}

pub fn write_slice_header(
    header: &SliceHeader,
    sps: &SequenceParameterSet,
    pps: &PicParameterSet,
    idr_pic_flag: bool,
    writer: &mut RbspWriter,
) -> WriteResult {
    writer.ue(header.first_mb_in_slice)?;
    writer.ue(header.slice_type as u32)?;
    writer.ue(header.pic_parameter_set_id as u32)?;

    if sps.separate_color_plane_flag {
        let plane_id = match header.color_plane {
            Some(ColorPlane::Y) => 0,
            Some(ColorPlane::Cb) => 1,
            Some(ColorPlane::Cr) => 2,
            None => 0, // Fallback
        };
        writer.u(2, plane_id)?;
    }

    writer.u(sps.bits_in_frame_num(), header.frame_num as u32)?;

    if !sps.frame_mbs_only_flag {
        writer.f(header.field_pic_flag)?;
        if header.field_pic_flag {
            if let Some(b) = header.bottom_field_flag {
                writer.f(b)?;
            }
        }
    }

    if idr_pic_flag {
        writer.ue(header.idr_pic_id.unwrap_or(0))?;
    }

    if sps.pic_order_cnt_type == 0 {
        writer.u(sps.bits_in_max_pic_order_cnt(), header.pic_order_cnt_lsb.unwrap_or(0))?;
        if pps.bottom_field_pic_order_in_frame_present_flag && !header.field_pic_flag {
            writer.se(header.delta_pic_order_cnt_bottom.unwrap_or(0))?;
        }
    } else if sps.pic_order_cnt_type == 1 && !sps.delta_pic_order_always_zero_flag {
        writer.se(header.delta_pic_order_cnt[0])?;
        if pps.bottom_field_pic_order_in_frame_present_flag && !header.field_pic_flag {
            writer.se(header.delta_pic_order_cnt[1])?;
        }
    }

    if pps.redundant_pic_cnt_present_flag {
        writer.ue(header.redundant_pic_cnt.unwrap_or(0))?;
    }

    if matches!(header.slice_type, SliceType::P | SliceType::SP | SliceType::B) {
        let num_ref_idx_override = header.num_ref_idx_l0_active_minus1
            != pps.num_ref_idx_l0_default_active_minus1
            || (header.slice_type == SliceType::B
                && header.num_ref_idx_l1_active_minus1 != pps.num_ref_idx_l1_default_active_minus1);
        writer.f(num_ref_idx_override)?;
        if num_ref_idx_override {
            writer.ue(header.num_ref_idx_l0_active_minus1)?;
            if header.slice_type == SliceType::B {
                writer.ue(header.num_ref_idx_l1_active_minus1)?;
            }
        }
    }

    write_ref_pic_list_modifications(&header.ref_pic_list_modification, writer, header.slice_type)?;

    if (pps.weighted_pred_flag && matches!(header.slice_type, SliceType::P | SliceType::SP))
        || (pps.weighted_bipred_idc == 1 && header.slice_type == SliceType::B)
    {
        if let Some(table) = &header.pred_weight_table {
            write_pred_weight_table(table, writer, header, sps)?;
        }
    }

    if let Some(marking) = &header.dec_ref_pic_marking {
        write_dec_ref_pic_marking(marking, writer, idr_pic_flag)?;
    }

    writer.se(header.slice_qp_delta)?;

    if pps.deblocking_filter_control_present_flag {
        writer.ue(header.deblocking_filter_idc as u32)?;
        if header.deblocking_filter_idc != DeblockingFilterIdc::Off {
            writer.se(header.slice_alpha_c0_offset_div2)?;
            writer.se(header.slice_beta_offset_div2)?;
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
            writer.u(bits, header.slice_group_change_cycle.unwrap_or(0))?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use hibernia::h264::decoder::DecoderContext;
    use hibernia::h264::parser::{parse_pps, parse_slice_header, parse_sps};
    use hibernia::h264::pps::PicParameterSet;
    use hibernia::h264::rbsp::RbspReader;
    use hibernia::h264::slice::SliceHeader;
    use hibernia::h264::slice::SliceType;
    use hibernia::h264::sps::{SequenceParameterSet, VuiParameters};
    use hibernia::h264::{nal::NalHeader, nal::NalUnitType, ChromaFormat, Profile};

    #[test]
    fn test_write_read_sps() {
        let mut sps = SequenceParameterSet::default();
        sps.profile = Profile::High;
        sps.level_idc = 42;
        sps.seq_parameter_set_id = 1;
        sps.chroma_format_idc = ChromaFormat::YUV420;
        sps.log2_max_frame_num_minus4 = 4;
        sps.pic_order_cnt_type = 0;
        sps.log2_max_pic_order_cnt_lsb_minus4 = 5;
        sps.max_num_ref_frames = 2;
        sps.pic_width_in_mbs_minus1 = 119;
        sps.pic_height_in_map_units_minus1 = 67;
        sps.frame_mbs_only_flag = true;
        sps.direct_8x8_inference_flag = true;

        let mut writer = RbspWriter::new();
        write_sps(&sps, &mut writer).unwrap();

        let data = writer.into_inner();
        let mut reader = RbspReader::new(&data);

        let parsed_sps = parse_sps(&mut reader).unwrap();
        assert_eq!(sps, parsed_sps);
    }

    #[test]
    fn test_write_read_pps() {
        let mut pps = PicParameterSet::default();
        pps.pic_parameter_set_id = 1;
        pps.seq_parameter_set_id = 1;
        pps.entropy_coding_mode_flag = true;
        pps.num_ref_idx_l0_default_active_minus1 = 0;
        pps.num_ref_idx_l1_default_active_minus1 = 0;
        pps.pic_init_qp_minus26 = 0;
        pps.pic_init_qs_minus26 = 0;
        pps.chroma_qp_index_offset = 2;
        pps.deblocking_filter_control_present_flag = true;
        pps.constrained_intra_pred_flag = false;
        pps.redundant_pic_cnt_present_flag = false;

        let mut writer = RbspWriter::new();
        write_pps(&pps, &mut writer).unwrap();

        let data = writer.into_inner();
        let mut reader = RbspReader::new(&data);

        let parsed_pps = parse_pps(&mut reader).unwrap();
        assert_eq!(pps, parsed_pps);
    }

    #[test]
    fn test_write_read_slice_header() {
        let mut sps = SequenceParameterSet::default();
        sps.log2_max_frame_num_minus4 = 4;
        sps.pic_order_cnt_type = 0;
        sps.log2_max_pic_order_cnt_lsb_minus4 = 5;
        sps.frame_mbs_only_flag = true;

        let mut pps = PicParameterSet::default();
        pps.deblocking_filter_control_present_flag = true;

        let mut header = SliceHeader::default();
        header.first_mb_in_slice = 0;
        header.slice_type = SliceType::I;
        header.pic_parameter_set_id = 0;
        header.frame_num = 5;
        header.idr_pic_id = Some(1);
        header.pic_order_cnt_lsb = Some(10);
        header.dec_ref_pic_marking = Some(DecRefPicMarking {
            no_output_of_prior_pics_flag: Some(false),
            long_term_reference_flag: Some(false),
            adaptive_ref_pic_marking_mode_flag: None,
            memory_management_operations: vec![],
        });
        header.slice_qp_delta = -2;
        header.deblocking_filter_idc = DeblockingFilterIdc::On;

        let mut writer = RbspWriter::new();
        write_slice_header(&header, &sps, &pps, true, &mut writer).unwrap();
        writer.align().unwrap();
        let data = writer.into_inner();

        let mut reader = RbspReader::new(&data);
        let mut ctx = DecoderContext::default();
        ctx.put_sps(sps.clone());
        ctx.put_pps(pps.clone());

        let nal_header = NalHeader { nal_ref_idc: 3, nal_unit_type: NalUnitType::IDRSlice };

        let slice = parse_slice_header(&ctx, &nal_header, &mut reader).unwrap();
        assert_eq!(header, slice.header);
    }

    #[test]
    fn test_write_read_vui_parameters() {
        let mut sps = SequenceParameterSet::default();
        sps.profile = Profile::High;
        sps.level_idc = 42;
        sps.seq_parameter_set_id = 1;
        sps.chroma_format_idc = ChromaFormat::YUV420;
        sps.log2_max_frame_num_minus4 = 4;
        sps.pic_order_cnt_type = 0;
        sps.log2_max_pic_order_cnt_lsb_minus4 = 5;
        sps.max_num_ref_frames = 2;
        sps.pic_width_in_mbs_minus1 = 119;
        sps.pic_height_in_map_units_minus1 = 67;
        sps.frame_mbs_only_flag = true;
        sps.direct_8x8_inference_flag = true;

        let mut vui = VuiParameters::default();
        vui.aspect_ratio_info_present_flag = true;
        vui.aspect_ratio_idc = 255;
        vui.sar_width = 16;
        vui.sar_height = 9;

        vui.overscan_info_present_flag = true;
        vui.overscan_appropriate_flag = true;

        vui.video_signal_type_present_flag = true;
        vui.video_format = 5;
        vui.video_full_range_flag = true;
        vui.color_description_present_flag = true;
        vui.color_primaries = 1;
        vui.transfer_characteristics = 2;
        vui.matrix_coefficients = 3;

        vui.chroma_loc_info_present_flag = true;
        vui.chroma_sample_loc_type_top_field = 1;
        vui.chroma_sample_loc_type_bottom_field = 2;

        vui.timing_info_present_flag = true;
        vui.num_units_in_tick = 1000;
        vui.time_scale = 60000;
        vui.fixed_frame_rate_flag = true;

        vui.pic_struct_present_flag = true;

        vui.bitstream_restriction_flag = true;
        vui.motion_vectors_over_pic_boundaries_flag = true;
        vui.max_bytes_per_pic_denom = 0;
        vui.max_bits_per_mb_denom = 0;
        vui.log2_max_mv_length_horizontal = 10;
        vui.log2_max_mv_length_vertical = 10;
        vui.max_num_reorder_frames = 2;
        vui.max_dec_frame_buffering = 3;

        sps.vui_parameters = Some(vui);

        let mut writer = RbspWriter::new();
        write_sps(&sps, &mut writer).unwrap();

        let data = writer.into_inner();
        let mut reader = RbspReader::new(&data);

        let parsed_sps = parse_sps(&mut reader).unwrap();
        assert_eq!(sps, parsed_sps);
    }
}
