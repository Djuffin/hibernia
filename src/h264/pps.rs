#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum SliceGroup {
    Interleaved {
        run_length_minus1: Vec<u32>,
    },
    Dispersed {
        num_slice_groups_minus1: u32,
    },
    ForegroundAndLeftover {
        rectangles: Vec<SliceRect>,
    },
    Changing {
        change_type: SliceGroupChangeType,
        num_slice_groups_minus1: u32,
        slice_group_change_direction_flag: bool,
        slice_group_change_rate_minus1: u32,
    },
    ExplicitAssignment {
        num_slice_groups_minus1: u32,
        slice_group_id: Vec<u32>,
    },
}


#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PicParameterSet {
    pub pic_parameter_set_id: u8,
    pub seq_parameter_set_id: u8,
    pub entropy_coding_mode_flag: bool,
    pub bottom_field_pic_order_in_frame_present_flag: bool,
    pub slice_groups: Option<SliceGroup>,
    pub num_ref_idx_l0_default_active_minus1: u32,
    pub num_ref_idx_l1_default_active_minus1: u32,
    pub weighted_pred_flag: bool,
    pub weighted_bipred_idc: u8,
    pub pic_init_qp_minus26: i32,
    pub pic_init_qs_minus26: i32,
    pub chroma_qp_index_offset: i32,
    pub deblocking_filter_control_present_flag: bool,
    pub constrained_intra_pred_flag: bool,
    pub redundant_pic_cnt_present_flag: bool,
    //pub extension: Option<PicParameterSetExtra>,
}