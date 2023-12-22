use std::num::NonZeroU32;
use num_traits::cast::FromPrimitive;

pub type MbAddr = u32;
pub const INVALID_MB: MbAddr = u32::MAX;

pub enum NeighborNames {
    A = 1, // left
    B = 2, // above
    C = 3, // above-right
    D = 4, // above-left
}

#[derive(Copy, Clone, Debug, Default)]
pub struct MbNeighbors {
    a: Option<NonZeroU32>, // left
    b: Option<NonZeroU32>, // above
    c: Option<NonZeroU32>, // above-right
    d: Option<NonZeroU32>, // above-left
}

impl MbNeighbors {
    pub fn get(&self, mb_name: NeighborNames) -> Option<MbAddr> {
        match mb_name {
            NeighborNames::A => self.a,
            NeighborNames::B => self.b,
            NeighborNames::C => self.c,
            NeighborNames::D => self.d,
        }
        .map(|addr| addr.get() - 1)
    }
}

// Section 6.4.8 Derivation process of the availability for macroblock addresses
// Section 6.4.9 Derivation process for neighboring macroblock addresses and their availability
pub fn get_neighbor_mbs(width_in_mbs: u32, first_addr: MbAddr, addr: MbAddr) -> MbNeighbors {
    pub fn wrap(addr: MbAddr) -> Option<NonZeroU32> {
        // 1 added to all block addresses values to avoid zeros
        NonZeroU32::new(addr + 1)
    }

    let w_rem = addr % width_in_mbs;
    // Left
    let a = if w_rem == 0 || addr <= first_addr { None } else { wrap(addr - 1) };

    // Above
    let b = if addr < first_addr + width_in_mbs { None } else { wrap(addr - width_in_mbs) };

    // Above-right
    let c = if addr + 1 < first_addr + width_in_mbs || w_rem + 1 == width_in_mbs {
        None
    } else {
        wrap(addr - width_in_mbs + 1)
    };

    // Above-left
    let d = if addr - 1 < first_addr + width_in_mbs || w_rem == 0 {
        None
    } else {
        wrap(addr - width_in_mbs - 1)
    };
    MbNeighbors { a, b, c, d }
}

// Table 7-11 – Macroblock types for I slices
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, FromPrimitive)]
pub enum IMbType {
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

impl TryFrom<u32> for IMbType {
    type Error = &'static str;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value).ok_or("Unknown I-mb type.")
    }
}

// Table 7-13 – Macroblock type values 0 to 4 for P and SP slices
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, FromPrimitive)]
pub enum PMbType {
    #[default]
    P_L0_16x16 = 0,
    P_L0_L0_16x8 = 1,
    P_L0_L0_8x16 = 2,
    P_8x8 = 3,
    P_8x8ref0 = 4,
    P_Skip,
}

impl TryFrom<u32> for PMbType {
    type Error = &'static str;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value).ok_or("Unknown P-mb type.")
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
    pub const fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Residual {
    pub dc_level16x16: [i32; 16],
    pub ac_level16x16: [[i32; 15]; 16],
    pub luma_level4x4: [[i32; 16]; 16],

    pub chroma_cb_dc_level: [i32; 4],
    pub chroma_cr_dc_level: [i32; 4],

    pub chroma_cb_ac_level: [[i32; 15]; 4],
    pub chroma_cr_ac_level: [[i32; 15]; 4],

    pub coded_block_pattern: CodedBlockPattern,
}

// Special case of I macroblock - raw pixels (IMbType::I_PCM)
#[derive(Clone, Debug, Default)]
pub struct PcmMb {
    // hardcoded YUV420, 8bit
    pub pcm_sample_luma: Vec<u8>,
    pub pcm_sample_chroma_cb: Vec<u8>,
    pub pcm_sample_chroma_cr: Vec<u8>,
}

#[derive(Clone, Debug, Default)]
pub struct IMb {
    pub mb_type: IMbType,
    pub transform_size_8x8_flag: bool,
    pub rem_intra4x4_pred_mode: [Option<Intra_4x4_SamplePredictionMode>; 16],
    pub intra_chroma_pred_mode: Intra_Chroma_Pred_Mode,
    pub coded_block_pattern: CodedBlockPattern,
    pub mb_qp_delta: i32,
}

// Macroblock of type P
#[derive(Clone, Debug, Default)]
pub struct P {
    pub mb_type: PMbType,
}

#[derive(Clone, Debug)]
pub enum Macroblock {
    I(IMb),
    PCM(PcmMb),
    P(P),
}

#[allow(non_snake_case)]
impl IMb {
    #[inline]
    pub fn MbPartPredMode(&self, partition: usize) -> MbPredictionMode {
        match self.mb_type {
            IMbType::I_NxN => {
                if self.transform_size_8x8_flag {
                    MbPredictionMode::Intra_8x8
                } else {
                    MbPredictionMode::Intra_4x4
                }
            }
            IMbType::I_PCM => MbPredictionMode::None,
            _ => MbPredictionMode::Intra_16x16,
        }
    }
}

#[allow(non_snake_case)]
impl Macroblock {
    #[inline]
    pub fn MbPartPredMode(&self, partition: usize) -> MbPredictionMode {
        match self {
            Macroblock::I(block) => block.MbPartPredMode(partition),
            _ => MbPredictionMode::None,
        }
    }
}



#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    pub fn test_get_neighbor_mbs() {
        let nbs = get_neighbor_mbs(16, 0, 20);
        assert_eq!(nbs.get(NeighborNames::A), Some(19));
        assert_eq!(nbs.get(NeighborNames::B), Some(4));
        assert_eq!(nbs.get(NeighborNames::C), Some(5));
        assert_eq!(nbs.get(NeighborNames::D), Some(3));

        let nbs = get_neighbor_mbs(8, 0, 15);
        assert_eq!(nbs.get(NeighborNames::A), Some(14));
        assert_eq!(nbs.get(NeighborNames::B), Some(7));
        assert_eq!(nbs.get(NeighborNames::C), None);
        assert_eq!(nbs.get(NeighborNames::D), Some(6));

        let nbs = get_neighbor_mbs(8, 0, 32);
        assert_eq!(nbs.get(NeighborNames::A), None);
        assert_eq!(nbs.get(NeighborNames::B), Some(24));
        assert_eq!(nbs.get(NeighborNames::C), Some(25));
        assert_eq!(nbs.get(NeighborNames::D), None);

        let nbs = get_neighbor_mbs(8, 25, 33);
        assert_eq!(nbs.get(NeighborNames::A), Some(32));
        assert_eq!(nbs.get(NeighborNames::B), Some(25));
        assert_eq!(nbs.get(NeighborNames::C), Some(26));
        assert_eq!(nbs.get(NeighborNames::D), None);

        let nbs = get_neighbor_mbs(10, 1, 1);
        assert!(nbs.get(NeighborNames::A).is_none());
        assert!(nbs.get(NeighborNames::B).is_none());
        assert!(nbs.get(NeighborNames::C).is_none());
        assert!(nbs.get(NeighborNames::D).is_none());
    }
}
