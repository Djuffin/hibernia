#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct SliceType(pub u8);
impl From<u8> for SliceType {
    fn from(value: u8) -> Self {
        SliceType(value)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ColourPlane {
    Y,
    Cb,
    Cr,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct SliceHeader {
    pub first_mb_in_slice: u32,
    pub slice_type: SliceType,
    pub pic_parameter_set_id: u8,
    pub colour_plane: Option<ColourPlane>,
    pub frame_num: u16,
    pub bottom_field_flag: Option<bool>,
    pub idr_pic_id: Option<u32>,
    //pub pic_order_cnt_lsb: Option<PicOrderCountLsb>,
    pub redundant_pic_cnt: Option<u32>,
    pub direct_spatial_mv_pred_flag: Option<bool>,
    //pub num_ref_idx_active: Option<NumRefIdxActive>,
    //pub ref_pic_list_modification: Option<RefPicListModifications>, // may become an enum rather than Option in future (for ref_pic_list_mvc_modification)
    //pub pred_weight_table: Option<PredWeightTable>,
    //pub dec_ref_pic_marking: Option<DecRefPicMarking>,
    pub cabac_init_idc: Option<u32>,
    pub slice_qp_delta: i32,
    pub sp_for_switch_flag: Option<bool>,
    pub slice_qs: Option<u32>,
    pub disable_deblocking_filter_idc: u8,
}
