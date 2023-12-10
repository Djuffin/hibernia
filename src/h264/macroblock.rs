use super::pps::{PicParameterSet, SliceGroup, SliceGroupChangeType, SliceRect};
use super::sps::{SequenceParameterSet, VuiParameters};
use num_traits::cast::FromPrimitive;

// Table 7-11 – Macroblock types for I slices
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, FromPrimitive)]
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

impl TryFrom<u32> for IMacroblockType {
    type Error = &'static str;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value).ok_or("Unknown prediction mode.")
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum MbPredictionMode {
    #[default]
    None,

    Intra_4x4,
    Intra_8x8,
    Intra_16x16,

    Pred_L0,
    Pred_L1,
}
// Section 8.3.1.2 Intra_4x4 sample prediction
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, FromPrimitive)]
pub enum Intra_4x4_SamplePredictionMode {
    Vertical = 0,
    Horizontal = 1,
    #[default]
    DC = 2,
    Diagonal_Down_Left = 3,
    Diagonal_Down_Right = 4,
    Vertical_Right = 5,
    Horizontal_Down = 6,
    Vertical_Left = 7,
    Horizontal_Up = 8,
}

impl TryFrom<u32> for Intra_4x4_SamplePredictionMode {
    type Error = &'static str;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value).ok_or("Unknown sample prediction mode.")
    }
}
// Table 7-16
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, FromPrimitive)]
pub enum Intra_Chroma_Pred_Mode {
    #[default]
    DC = 0,
    Horizontal = 1,
    Vertical = 2,
    Plane = 3,
}

impl TryFrom<u32> for Intra_Chroma_Pred_Mode {
    type Error = &'static str;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value).ok_or("Unknown chroma prediction mode.")
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct CodedBlockPattern(pub u8);

impl CodedBlockPattern {
    pub const fn new(luma: u8, chroma: u8) -> CodedBlockPattern {
        CodedBlockPattern((luma & 0b1111) | (chroma << 4))
    }

    #[inline]
    pub const fn luma(&self) -> u8 {
        self.0 & 0b1111
    }

    #[inline]
    pub const fn chroma(&self) -> u8 {
        self.0 >> 4
    }

    #[inline]
    pub const fn non_zero(&self) -> bool {
        self.0 != 0
    }
}

// Macroblock of type I
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct IMb {
    pub mb_type: IMacroblockType,
    pub transform_size_8x8_flag: bool,
    pub rem_intra4x4_pred_mode: [Option<Intra_4x4_SamplePredictionMode>; 16],
    pub intra_chroma_pred_mode: Intra_Chroma_Pred_Mode,
    pub coded_block_pattern: CodedBlockPattern,
    pub mb_qp_delta: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct P {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Macroblock {
    I(IMb),
    P(P),
}

#[allow(non_snake_case)]
impl IMb {
    #[inline]
    pub fn MbPartPredMode(&self, partition: usize) -> MbPredictionMode {
        match self.mb_type {
            IMacroblockType::I_NxN => {
                if (self.transform_size_8x8_flag) {
                    MbPredictionMode::Intra_8x8
                } else {
                    MbPredictionMode::Intra_4x4
                }
            }
            IMacroblockType::I_PCM => MbPredictionMode::None,
            _ => MbPredictionMode::Intra_16x16,
        }
    }
}
