use super::pps::{PicParameterSet, SliceGroup, SliceGroupChangeType, SliceRect};
use super::sps::{ProfileIdc, SequenceParameterSet, VuiParameters};

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
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 | 5 => Ok(SliceType::P),
            1 | 6 => Ok(SliceType::B),
            2 | 7 => Ok(SliceType::I),
            3 | 8 => Ok(SliceType::SP),
            4 | 9 => Ok(SliceType::SI),
            _ => Err(()),
        }
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
    //pub ref_pic_list_modification: Option<RefPicListModifications>, // may become an enum rather than Option in future (for ref_pic_list_mvc_modification)
    //pub pred_weight_table: Option<PredWeightTable>,
    //pub dec_ref_pic_marking: Option<DecRefPicMarking>,
    //pub cabac_init_idc: Option<u32>,
    pub slice_qp_delta: i32,
    pub sp_for_switch_flag: Option<bool>,
    pub slice_qs: Option<u32>,
    pub disable_deblocking_filter_idc: u8,
}

// Table 7-11 – Macroblock types for I slices
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum IMacroblockType {
    #[default]
    I_NxN = 0,
    I_16x16_0_0_0 = 1,
    I_16x16_1_0_0 = 2,
    I_16x16_2_0_0 = 3,
    I_16x16_3_0_0 = 4,
    I_16x16_0_1_0 = 5,
    I_16x16_1_1_0 = 6,
    I_16x16_2_1_0 = 7,
    I_16x16_3_1_0 = 8,
    I_16x16_0_2_0 = 9,
    I_16x16_1_2_0 = 10,
    I_16x16_2_2_0 = 11,
    I_16x16_3_2_0 = 12,
    I_16x16_0_0_1 = 13,
    I_16x16_1_0_1 = 14,
    I_16x16_2_0_1 = 15,
    I_16x16_3_0_1 = 16,
    I_16x16_0_1_1 = 17,
    I_16x16_1_1_1 = 18,
    I_16x16_2_1_1 = 19,
    I_16x16_3_1_1 = 20,
    I_16x16_0_2_1 = 21,
    I_16x16_1_2_1 = 22,
    I_16x16_2_2_1 = 23,
    I_16x16_3_2_1 = 24,
    I_PCM = 25,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum PredictionMode {
    #[default]
    None,

    Intra_4x4,
    Intra_8x8,
    Intra_16x16,

    Pred_L0,
    Pred_L1,
}

impl TryFrom<u32> for IMacroblockType {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(IMacroblockType::I_NxN),
            1 => Ok(IMacroblockType::I_16x16_0_0_0),
            2 => Ok(IMacroblockType::I_16x16_1_0_0),
            3 => Ok(IMacroblockType::I_16x16_2_0_0),
            4 => Ok(IMacroblockType::I_16x16_3_0_0),
            5 => Ok(IMacroblockType::I_16x16_0_1_0),
            6 => Ok(IMacroblockType::I_16x16_1_1_0),
            7 => Ok(IMacroblockType::I_16x16_2_1_0),
            8 => Ok(IMacroblockType::I_16x16_3_1_0),
            9 => Ok(IMacroblockType::I_16x16_0_2_0),
            10 => Ok(IMacroblockType::I_16x16_1_2_0),
            11 => Ok(IMacroblockType::I_16x16_2_2_0),
            12 => Ok(IMacroblockType::I_16x16_3_2_0),
            13 => Ok(IMacroblockType::I_16x16_0_0_1),
            14 => Ok(IMacroblockType::I_16x16_1_0_1),
            15 => Ok(IMacroblockType::I_16x16_2_0_1),
            16 => Ok(IMacroblockType::I_16x16_3_0_1),
            17 => Ok(IMacroblockType::I_16x16_0_1_1),
            18 => Ok(IMacroblockType::I_16x16_1_1_1),
            19 => Ok(IMacroblockType::I_16x16_2_1_1),
            20 => Ok(IMacroblockType::I_16x16_3_1_1),
            21 => Ok(IMacroblockType::I_16x16_0_2_1),
            22 => Ok(IMacroblockType::I_16x16_1_2_1),
            23 => Ok(IMacroblockType::I_16x16_2_2_1),
            24 => Ok(IMacroblockType::I_16x16_3_2_1),
            25 => Ok(IMacroblockType::I_PCM),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Macroblock {
    pub mb_type: IMacroblockType,
    pub transform_size_8x8_flag: bool,
}

#[allow(non_snake_case)]
impl Macroblock {
    pub fn MbPartPredMode(&self, partition: usize) -> PredictionMode {
        match self.mb_type {
            IMacroblockType::I_NxN => {
                if (self.transform_size_8x8_flag) {
                    PredictionMode::Intra_4x4
                } else {
                    PredictionMode::Intra_8x8
                }
            }
            IMacroblockType::I_PCM => PredictionMode::None,
            _ => PredictionMode::Intra_16x16,
        }
    }
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
