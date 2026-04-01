#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SliceRect {
    pub top_left: u32,
    pub bottom_right: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SliceGroupChangeType {
    #[default]
    BoxOut,
    RasterScan,
    WipeOut,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SliceGroup {
    Interleaved {
        run_length_minus1: Vec<u32>,
    },
    Dispersed {
        num_slice_groups_minus1: usize,
    },
    Foreground {
        rectangles: Vec<SliceRect>,
    },
    Changing {
        change_type: SliceGroupChangeType,
        num_slice_groups_minus1: usize,
        slice_group_change_direction_flag: bool,
        slice_group_change_rate_minus1: u32,
    },
    Explicit {
        num_slice_groups_minus1: usize,
        slice_group_id: Vec<u32>,
    },
}

use super::ColorPlane;

// Section 7.4.2.2 Picture parameter set RBSP semantics
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PicParameterSet {
    pub pic_parameter_set_id: u8,
    pub seq_parameter_set_id: u8,

    // This flag selects the entropy decoding method to be applied for the syntax elements.
    // false: Exp-Golomb coded, see section 9.1 or CAVLC, see section 9.2.
    // true: CABAC coded, see section 9.3. (Only available in profiles Main and higher)
    pub entropy_coding_mode_flag: bool,
    pub bottom_field_pic_order_in_frame_present_flag: bool,
    pub slice_group: Option<SliceGroup>,
    pub num_ref_idx_l0_default_active_minus1: u32,
    pub num_ref_idx_l1_default_active_minus1: u32,
    pub weighted_pred_flag: bool,
    pub weighted_bipred_idc: u8,
    pub pic_init_qp_minus26: i32,
    pub pic_init_qs_minus26: i32,
    pub chroma_qp_index_offset: i32,
    pub deblocking_filter_control_present_flag: bool,

    // False specifies that intra prediction allows usage of residual data and decoded samples
    // of neighboring macroblocks coded using Inter macroblock prediction modes for the
    // prediction of macroblocks coded using Intra macroblock prediction modes.
    // True specifies constrained intra prediction, in which case prediction of macroblocks coded
    // using Intra macroblock prediction modes only uses residual data and decoded samples
    // from I or SI macroblock types.
    pub constrained_intra_pred_flag: bool,
    pub redundant_pic_cnt_present_flag: bool,

    // True specifies that the 8x8 transform decoding process may be in use.
    // False if not present.
    pub transform_8x8_mode_flag: bool,
    //pub pic_scaling_matrix: Option<PicScalingMatrix>,
    pub second_chroma_qp_index_offset: i32,
}

impl PicParameterSet {
    pub fn write_pps(&self, writer: &mut super::rbsp_writer::RbspWriter) -> super::rbsp_writer::WriteResult {
        writer.ue(self.pic_parameter_set_id as u32)?;
        writer.ue(self.seq_parameter_set_id as u32)?;
        writer.f(self.entropy_coding_mode_flag)?;
        writer.f(self.bottom_field_pic_order_in_frame_present_flag)?;

        if let Some(slice_group) = &self.slice_group {
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
                SliceGroup::Changing { change_type, num_slice_groups_minus1, slice_group_change_direction_flag, slice_group_change_rate_minus1 } => {
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

        writer.ue(self.num_ref_idx_l0_default_active_minus1)?;
        writer.ue(self.num_ref_idx_l1_default_active_minus1)?;
        writer.f(self.weighted_pred_flag)?;
        writer.u(2, self.weighted_bipred_idc as u32)?;
        writer.se(self.pic_init_qp_minus26)?;
        writer.se(self.pic_init_qs_minus26)?;
        writer.se(self.chroma_qp_index_offset)?;
        writer.f(self.deblocking_filter_control_present_flag)?;
        writer.f(self.constrained_intra_pred_flag)?;
        writer.f(self.redundant_pic_cnt_present_flag)?;

        if self.transform_8x8_mode_flag || self.second_chroma_qp_index_offset != self.chroma_qp_index_offset {
            writer.f(self.transform_8x8_mode_flag)?;
            writer.f(false)?; // pic_scaling_matrix_present_flag
            writer.se(self.second_chroma_qp_index_offset)?;
        }

        writer.rbsp_trailing_bits()?;
        Ok(())
    }

    pub fn get_chroma_qp_index_offset(&self, plane: ColorPlane) -> i32 {
        match plane {
            ColorPlane::Cb => self.chroma_qp_index_offset,
            ColorPlane::Cr => self.second_chroma_qp_index_offset,
            _ => 0,
        }
    }
}
