use super::pps::{PicParameterSet, SliceGroup, SliceGroupChangeType, SliceRect};
use super::sps::{SequenceParameterSet, VuiParameters};
use super::Profile;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum SliceType {
    #[default]
    P = 0,
    B = 1,
    I = 2,
    SP = 3,
    SI = 4,
}

impl TryFrom<u32> for SliceType {
    type Error = &'static str;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 | 5 => Ok(SliceType::P),
            1 | 6 => Ok(SliceType::B),
            2 | 7 => Ok(SliceType::I),
            3 | 8 => Ok(SliceType::SP),
            4 | 9 => Ok(SliceType::SI),
            _ => Err("Unknown slice type."),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ColourPlane {
    Y,
    Cb,
    Cr,
}

// Section 7.4.3 Slice header semantics
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct SliceHeader {
    pub first_mb_in_slice: u32,
    pub slice_type: SliceType,
    pub pic_parameter_set_id: u8,
    pub colour_plane: Option<ColourPlane>,
    pub frame_num: u16,

    // True specifies that the slice is a slice of a coded field.
    // False specifies that the slice is a slice of a coded frame.
    pub field_pic_flag: bool,

    pub bottom_field_flag: Option<bool>,
    pub idr_pic_id: Option<u32>,
    pub pic_order_cnt_lsb: Option<u32>,
    pub delta_pic_order_cnt_bottom: Option<i32>,
    pub redundant_pic_cnt: Option<u32>,

    //pub direct_spatial_mv_pred_flag: Option<bool>,
    //pub num_ref_idx_active: Option<NumRefIdxActive>,
    // may become an enum rather than Option in future (for ref_pic_list_mvc_modification)
    //pub ref_pic_list_modification: Option<RefPicListModifications>,
    //pub pred_weight_table: Option<PredWeightTable>,
    //pub dec_ref_pic_marking: Option<DecRefPicMarking>,
    //pub cabac_init_idc: Option<u32>,
    pub slice_qp_delta: i32,
    pub sp_for_switch_flag: Option<bool>,
    pub slice_qs: Option<u32>,
    pub disable_deblocking_filter_idc: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Slice {
    pub sps: SequenceParameterSet,
    pub pps: PicParameterSet,
    pub header: SliceHeader,
}

#[allow(non_snake_case)]
impl Slice {
    pub fn MbaffFrameFlag(&self) -> bool {
        self.sps.mb_adaptive_frame_field_flag && !self.header.field_pic_flag
    }
}
