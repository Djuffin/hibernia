use log::trace;
use num_traits::cast::FromPrimitive;
use std::fmt::Display;
use std::num::NonZeroU32;

use super::residual::Residual;
use super::tables::{MB_HEIGHT, MB_WIDTH};
use super::{ColorPlane, Point};

pub type MbAddr = u32;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum MbNeighborName {
    #[default]
    /// Left
    A = 1,
    /// Above
    B = 2,
    /// Above-right
    C = 3,
    /// Above-left
    D = 4,
}

// Section 6.4.8 Derivation process of the availability for macroblock addresses
// Section 6.4.9 Derivation process for neighboring macroblock addresses and their availability
pub fn get_neighbor_mbs(
    width_in_mbs: u32,
    first_addr: MbAddr,
    addr: MbAddr,
    neib: MbNeighborName,
) -> Option<MbAddr> {
    let w_rem = addr % width_in_mbs;
    match neib {
        MbNeighborName::A => {
            // Left
            if w_rem == 0 || addr <= first_addr {
                None
            } else {
                Some(addr - 1)
            }
        }
        MbNeighborName::B => {
            // Above
            if addr < first_addr + width_in_mbs {
                None
            } else {
                Some(addr - width_in_mbs)
            }
        }
        MbNeighborName::C => {
            // Above-right
            if addr + 1 < first_addr + width_in_mbs || w_rem + 1 == width_in_mbs {
                None
            } else {
                Some(addr - width_in_mbs + 1)
            }
        }
        MbNeighborName::D => {
            // Above-left
            if addr < first_addr + width_in_mbs + 1 || w_rem == 0 {
                None
            } else {
                Some(addr - width_in_mbs - 1)
            }
        }
    }
}

/*
    4x4 luma block indexes:

    +--+--+--+--+
    |00|01|04|05|
    +--+--+--+--+
    |02|03|06|07|
    +--+--+--+--+
    |08|09|12|13|
    +--+--+--+--+
    |10|11|14|15|
    +--+--+--+--+
*/

// Section 6.4.3 Inverse 4x4 luma block scanning process
pub const fn get_4x4luma_block_location(idx: u8) -> Point {
    const LOCATIONS: [Point; 16] = [
        Point { x: 0, y: 0 },
        Point { x: 4, y: 0 },
        Point { x: 0, y: 4 },
        Point { x: 4, y: 4 },
        Point { x: 8, y: 0 },
        Point { x: 12, y: 0 },
        Point { x: 8, y: 4 },
        Point { x: 12, y: 4 },
        Point { x: 0, y: 8 },
        Point { x: 4, y: 8 },
        Point { x: 0, y: 12 },
        Point { x: 4, y: 12 },
        Point { x: 8, y: 8 },
        Point { x: 12, y: 8 },
        Point { x: 8, y: 12 },
        Point { x: 12, y: 12 },
    ];
    LOCATIONS[idx as usize]
}

// Section 6.4.5 Inverse 8x8 luma block scanning process.
// Four 8x8 blocks in raster order within a macroblock.
pub const fn get_8x8luma_block_location(idx: u8) -> Point {
    const LOCATIONS: [Point; 4] =
        [Point { x: 0, y: 0 }, Point { x: 8, y: 0 }, Point { x: 0, y: 8 }, Point { x: 8, y: 8 }];
    LOCATIONS[idx as usize]
}

// Section 6.4.13.3 Derivation process for 8x8 luma block indices
fn get_8x8luma_block_index(p: Point) -> u8 {
    let idx = 2 * (p.y / 8) + (p.x / 8);
    idx as u8
}

// Section 6.4.11.2 Derivation process for neighboring 8x8 luma blocks
pub fn get_8x8luma_block_neighbor(idx: u8, n: MbNeighborName) -> (u8, Option<MbNeighborName>) {
    let p = get_8x8luma_block_location(idx);
    let (x, y) = match n {
        MbNeighborName::A => (p.x as i8 - 1, p.y as i8),
        MbNeighborName::B => (p.x as i8, p.y as i8 - 1),
        MbNeighborName::C => (p.x as i8 + 8, p.y as i8),
        MbNeighborName::D => (p.x as i8 - 1, p.y as i8 - 1),
    };
    let (w, h) = (MB_WIDTH as i8, MB_HEIGHT as i8);
    let (xn, yn) = ((x + w) % w, (y + h) % h);
    let p = Point { x: x as u32, y: y as u32 };
    let np = Point { x: xn as u32, y: yn as u32 };
    let idx = get_8x8luma_block_index(np);
    if p == np {
        (idx, None)
    } else {
        (idx, Some(n))
    }
}

// Section 6.4.7 Inverse 4x4 chroma block scanning process
pub const fn get_4x4chroma_block_location(idx: u8) -> Point {
    const LOCATIONS: [Point; 4] =
        [Point { x: 0, y: 0 }, Point { x: 4, y: 0 }, Point { x: 0, y: 4 }, Point { x: 4, y: 4 }];
    LOCATIONS[idx as usize]
}

// Section 6.4.13.1 Derivation process for 4x4 luma block indices
pub fn get_4x4luma_block_index(p: Point) -> u8 {
    let idx = 8 * (p.y / 8) + 4 * (p.x / 8) + 2 * ((p.y % 8) / 4) + ((p.x % 8) / 4);
    idx as u8
}

// Section 6.4.13.2 Derivation process for 4x4 chroma block indices
fn get_4x4chroma_block_index(p: Point) -> u8 {
    let idx = 2 * (p.y / 4) + (p.x / 4);
    idx as u8
}

// Section 6.4.11.4 Derivation process for neighboring 4x4 luma blocks
pub fn get_4x4luma_block_neighbor(idx: u8, n: MbNeighborName) -> (u8, Option<MbNeighborName>) {
    let p = get_4x4luma_block_location(idx);
    let (x, y) = match n {
        MbNeighborName::A => (p.x as i8 - 1, p.y as i8),
        MbNeighborName::B => (p.x as i8, p.y as i8 - 1),
        MbNeighborName::C => (p.x as i8 + 1, p.y as i8),
        MbNeighborName::D => (p.x as i8 - 1, p.y as i8 - 1),
    };
    let (w, h) = (MB_WIDTH as i8, MB_HEIGHT as i8);
    let (xn, yn) = ((x + w) % w, (y + h) % h);
    let p = Point { x: x as u32, y: y as u32 };
    let np = Point { x: xn as u32, y: yn as u32 };
    let idx = get_4x4luma_block_index(np);
    if p == np {
        (idx, None)
    } else {
        (idx, Some(n))
    }
}

// Section 6.4.11.5 Derivation process for neighboring 4x4 chroma blocks
pub fn get_4x4chroma_block_neighbor(idx: u8, n: MbNeighborName) -> (u8, Option<MbNeighborName>) {
    let p = get_4x4chroma_block_location(idx);
    let (x, y) = match n {
        MbNeighborName::A => (p.x as i8 - 1, p.y as i8),
        MbNeighborName::B => (p.x as i8, p.y as i8 - 1),
        MbNeighborName::C => (p.x as i8 + 1, p.y as i8),
        MbNeighborName::D => (p.x as i8 - 1, p.y as i8 - 1),
    };
    let (w, h) = (8, 8);
    let (xn, yn) = ((x + w) % w, (y + h) % h);
    let p = Point { x: x as u32, y: y as u32 };
    let np = Point { x: xn as u32, y: yn as u32 };
    let idx = get_4x4chroma_block_index(np);
    if p == np {
        (idx, None)
    } else {
        (idx, Some(n))
    }
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
    type Error = String;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value).ok_or_else(|| format!("Unknown I-mb type: {value}"))
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
    type Error = String;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value).ok_or_else(|| format!("Unknown P-mb type: {value}"))
    }
}

#[allow(non_snake_case)]
impl PMbType {
    pub const fn NumMbPart(&self) -> usize {
        match self {
            PMbType::P_Skip | PMbType::P_L0_16x16 => 1,
            PMbType::P_L0_L0_16x8 | PMbType::P_L0_L0_8x16 => 2,
            PMbType::P_8x8ref0 | PMbType::P_8x8 => 4,
        }
    }
}

// Table 7-14 – Macroblock type values 0 to 22 for B slices
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, FromPrimitive)]
pub enum BMbType {
    #[default]
    B_Direct_16x16 = 0,
    B_L0_16x16 = 1,
    B_L1_16x16 = 2,
    B_Bi_16x16 = 3,
    B_L0_L0_16x8 = 4,
    B_L0_L0_8x16 = 5,
    B_L1_L1_16x8 = 6,
    B_L1_L1_8x16 = 7,
    B_L0_L1_16x8 = 8,
    B_L0_L1_8x16 = 9,
    B_L1_L0_16x8 = 10,
    B_L1_L0_8x16 = 11,
    B_L0_Bi_16x8 = 12,
    B_L0_Bi_8x16 = 13,
    B_L1_Bi_16x8 = 14,
    B_L1_Bi_8x16 = 15,
    B_Bi_L0_16x8 = 16,
    B_Bi_L0_8x16 = 17,
    B_Bi_L1_16x8 = 18,
    B_Bi_L1_8x16 = 19,
    B_Bi_Bi_16x8 = 20,
    B_Bi_Bi_8x16 = 21,
    B_8x8 = 22,
    B_Skip = 23, // Sentinel, not from bitstream mb_type
}

impl TryFrom<u32> for BMbType {
    type Error = String;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value).ok_or_else(|| format!("Unknown B-mb type: {value}"))
    }
}

#[allow(non_snake_case)]
impl BMbType {
    pub const fn NumMbPart(&self) -> usize {
        match self {
            BMbType::B_Direct_16x16 | BMbType::B_Skip => 0, // Direct: sub-block level
            BMbType::B_L0_16x16 | BMbType::B_L1_16x16 | BMbType::B_Bi_16x16 => 1,
            BMbType::B_L0_L0_16x8
            | BMbType::B_L0_L0_8x16
            | BMbType::B_L1_L1_16x8
            | BMbType::B_L1_L1_8x16
            | BMbType::B_L0_L1_16x8
            | BMbType::B_L0_L1_8x16
            | BMbType::B_L1_L0_16x8
            | BMbType::B_L1_L0_8x16
            | BMbType::B_L0_Bi_16x8
            | BMbType::B_L0_Bi_8x16
            | BMbType::B_L1_Bi_16x8
            | BMbType::B_L1_Bi_8x16
            | BMbType::B_Bi_L0_16x8
            | BMbType::B_Bi_L0_8x16
            | BMbType::B_Bi_L1_16x8
            | BMbType::B_Bi_L1_8x16
            | BMbType::B_Bi_Bi_16x8
            | BMbType::B_Bi_Bi_8x16 => 2,
            BMbType::B_8x8 => 4,
        }
    }

    /// Table 7-14: MbPartPredMode for partition `mbPartIdx`
    pub const fn MbPartPredMode(&self, mb_part_idx: usize) -> MbPredictionMode {
        // For two-partition types, the name encodes [part0]_[part1]_[size]
        match self {
            BMbType::B_Direct_16x16 | BMbType::B_Skip => MbPredictionMode::Direct,
            BMbType::B_L0_16x16 => MbPredictionMode::Pred_L0,
            BMbType::B_L1_16x16 => MbPredictionMode::Pred_L1,
            BMbType::B_Bi_16x16 => MbPredictionMode::BiPred,
            BMbType::B_L0_L0_16x8 | BMbType::B_L0_L0_8x16 => MbPredictionMode::Pred_L0,
            BMbType::B_L1_L1_16x8 | BMbType::B_L1_L1_8x16 => MbPredictionMode::Pred_L1,
            BMbType::B_L0_L1_16x8 | BMbType::B_L0_L1_8x16 => {
                if mb_part_idx == 0 {
                    MbPredictionMode::Pred_L0
                } else {
                    MbPredictionMode::Pred_L1
                }
            }
            BMbType::B_L1_L0_16x8 | BMbType::B_L1_L0_8x16 => {
                if mb_part_idx == 0 {
                    MbPredictionMode::Pred_L1
                } else {
                    MbPredictionMode::Pred_L0
                }
            }
            BMbType::B_L0_Bi_16x8 | BMbType::B_L0_Bi_8x16 => {
                if mb_part_idx == 0 {
                    MbPredictionMode::Pred_L0
                } else {
                    MbPredictionMode::BiPred
                }
            }
            BMbType::B_L1_Bi_16x8 | BMbType::B_L1_Bi_8x16 => {
                if mb_part_idx == 0 {
                    MbPredictionMode::Pred_L1
                } else {
                    MbPredictionMode::BiPred
                }
            }
            BMbType::B_Bi_L0_16x8 | BMbType::B_Bi_L0_8x16 => {
                if mb_part_idx == 0 {
                    MbPredictionMode::BiPred
                } else {
                    MbPredictionMode::Pred_L0
                }
            }
            BMbType::B_Bi_L1_16x8 | BMbType::B_Bi_L1_8x16 => {
                if mb_part_idx == 0 {
                    MbPredictionMode::BiPred
                } else {
                    MbPredictionMode::Pred_L1
                }
            }
            BMbType::B_Bi_Bi_16x8 | BMbType::B_Bi_Bi_8x16 => MbPredictionMode::BiPred,
            BMbType::B_8x8 => MbPredictionMode::None, // Delegated to sub-mb types
        }
    }

    /// Table 7-14: Partition size (width, height)
    pub const fn MbPartSize(&self) -> (u8, u8) {
        match self {
            BMbType::B_Direct_16x16 | BMbType::B_Skip => (16, 16), // subdivided at sub-block level
            BMbType::B_L0_16x16 | BMbType::B_L1_16x16 | BMbType::B_Bi_16x16 => (16, 16),
            BMbType::B_L0_L0_16x8
            | BMbType::B_L1_L1_16x8
            | BMbType::B_L0_L1_16x8
            | BMbType::B_L1_L0_16x8
            | BMbType::B_L0_Bi_16x8
            | BMbType::B_L1_Bi_16x8
            | BMbType::B_Bi_L0_16x8
            | BMbType::B_Bi_L1_16x8
            | BMbType::B_Bi_Bi_16x8 => (16, 8),
            BMbType::B_L0_L0_8x16
            | BMbType::B_L1_L1_8x16
            | BMbType::B_L0_L1_8x16
            | BMbType::B_L1_L0_8x16
            | BMbType::B_L0_Bi_8x16
            | BMbType::B_L1_Bi_8x16
            | BMbType::B_Bi_L0_8x16
            | BMbType::B_Bi_L1_8x16
            | BMbType::B_Bi_Bi_8x16 => (8, 16),
            BMbType::B_8x8 => (8, 8),
        }
    }
}

// Table 7-18 – Sub-macroblock types in B macroblocks
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, FromPrimitive)]
pub enum BSubMbType {
    #[default]
    B_Direct_8x8 = 0,
    B_L0_8x8 = 1,
    B_L1_8x8 = 2,
    B_Bi_8x8 = 3,
    B_L0_8x4 = 4,
    B_L0_4x8 = 5,
    B_L1_8x4 = 6,
    B_L1_4x8 = 7,
    B_Bi_8x4 = 8,
    B_Bi_4x8 = 9,
    B_L0_4x4 = 10,
    B_L1_4x4 = 11,
    B_Bi_4x4 = 12,
}

impl TryFrom<u32> for BSubMbType {
    type Error = String;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value).ok_or_else(|| format!("Unknown B sub-mb type: {value}"))
    }
}

#[allow(non_snake_case)]
impl BSubMbType {
    pub const fn NumSubMbPart(&self) -> usize {
        match self {
            BSubMbType::B_Direct_8x8
            | BSubMbType::B_L0_8x8
            | BSubMbType::B_L1_8x8
            | BSubMbType::B_Bi_8x8 => 1,
            BSubMbType::B_L0_8x4
            | BSubMbType::B_L0_4x8
            | BSubMbType::B_L1_8x4
            | BSubMbType::B_L1_4x8
            | BSubMbType::B_Bi_8x4
            | BSubMbType::B_Bi_4x8 => 2,
            BSubMbType::B_L0_4x4 | BSubMbType::B_L1_4x4 | BSubMbType::B_Bi_4x4 => 4,
        }
    }

    pub const fn SubMbPredMode(&self) -> MbPredictionMode {
        match self {
            BSubMbType::B_Direct_8x8 => MbPredictionMode::Direct,
            BSubMbType::B_L0_8x8
            | BSubMbType::B_L0_8x4
            | BSubMbType::B_L0_4x8
            | BSubMbType::B_L0_4x4 => MbPredictionMode::Pred_L0,
            BSubMbType::B_L1_8x8
            | BSubMbType::B_L1_8x4
            | BSubMbType::B_L1_4x8
            | BSubMbType::B_L1_4x4 => MbPredictionMode::Pred_L1,
            BSubMbType::B_Bi_8x8
            | BSubMbType::B_Bi_8x4
            | BSubMbType::B_Bi_4x8
            | BSubMbType::B_Bi_4x4 => MbPredictionMode::BiPred,
        }
    }

    pub const fn SubMbPartSize(&self) -> (u8, u8) {
        match self {
            BSubMbType::B_Direct_8x8
            | BSubMbType::B_L0_8x8
            | BSubMbType::B_L1_8x8
            | BSubMbType::B_Bi_8x8 => (8, 8),
            BSubMbType::B_L0_8x4 | BSubMbType::B_L1_8x4 | BSubMbType::B_Bi_8x4 => (8, 4),
            BSubMbType::B_L0_4x8 | BSubMbType::B_L1_4x8 | BSubMbType::B_Bi_4x8 => (4, 8),
            BSubMbType::B_L0_4x4 | BSubMbType::B_L1_4x4 | BSubMbType::B_Bi_4x4 => (4, 4),
        }
    }
}

// Holds data for a B_8x8 sub-macroblock
#[derive(Copy, Clone, Debug, Default)]
pub struct BSubMacroblock {
    pub sub_mb_type: BSubMbType,
    pub partitions: [PartitionInfo; 4],
}

// Macroblock of type B
#[derive(Clone, Debug, Default)]
pub struct BMb {
    pub mb_type: BMbType,
    pub motion: MbMotion,
    pub coded_block_pattern: CodedBlockPattern,
    pub mb_qp_delta: i32,
    pub qp: u8,
    pub transform_size_8x8_flag: bool,
    pub residual: Option<Box<Residual>>,
    pub cbf_info: CbfInfo,
    /// Sub-macroblock types for B_8x8 mode. Used for B_Direct_8x8 detection
    /// in CABAC context derivation.
    pub sub_mb_types: [BSubMbType; 4],
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
    BiPred,
    Direct,
}

impl MbPredictionMode {
    pub const fn is_intra(&self) -> bool {
        matches!(
            self,
            MbPredictionMode::Intra_16x16
                | MbPredictionMode::Intra_4x4
                | MbPredictionMode::Intra_8x8
        )
    }

    pub const fn is_inter(&self) -> bool {
        !self.is_intra()
    }
}

// Section 8.3.1.2 Intra_4x4 sample prediction
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, FromPrimitive, PartialOrd, Ord)]
pub enum Intra_4x4_SamplePredMode {
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

impl Display for Intra_4x4_SamplePredMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({})", self, *self as u32)
    }
}

impl Intra_4x4_SamplePredMode {
    pub fn max_mode() -> Intra_4x4_SamplePredMode {
        Intra_4x4_SamplePredMode::Horizontal_Up
    }
}

impl TryFrom<u32> for Intra_4x4_SamplePredMode {
    type Error = String;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value)
            .ok_or_else(|| format!("Unknown 4x4 sample prediction mode: {value}"))
    }
}

// Section 8.3.2 Intra_8x8 sample prediction — same mode numbering as Intra_4x4.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, FromPrimitive, PartialOrd, Ord)]
pub enum Intra_8x8_SamplePredMode {
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

impl Intra_8x8_SamplePredMode {
    pub fn max_mode() -> Intra_8x8_SamplePredMode {
        Intra_8x8_SamplePredMode::Horizontal_Up
    }
}

impl TryFrom<u32> for Intra_8x8_SamplePredMode {
    type Error = String;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value)
            .ok_or_else(|| format!("Unknown 8x8 sample prediction mode: {value}"))
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, FromPrimitive)]
pub enum Intra_16x16_SamplePredMode {
    #[default]
    Intra_16x16_Vertical = 0,
    Intra_16x16_Horizontal = 1,
    Intra_16x16_DC = 2,
    Intra_16x16_Plane = 3,
}

impl TryFrom<u32> for Intra_16x16_SamplePredMode {
    type Error = String;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value)
            .ok_or_else(|| format!("Unknown 16x16 sample prediction mode: {value}"))
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

impl Display for Intra_Chroma_Pred_Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({})", self, *self as u32)
    }
}

impl TryFrom<u32> for Intra_Chroma_Pred_Mode {
    type Error = String;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value)
            .ok_or_else(|| format!("Unknown chrome prediction mode: {value}"))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct CodedBlockPattern(pub u8);

impl CodedBlockPattern {
    pub const fn new(chroma: u8, luma: u8) -> CodedBlockPattern {
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

// Special case of I macroblock - raw pixels (IMbType::I_PCM)
#[derive(Clone, Debug, Default)]
pub struct PcmMb {
    // hardcoded YUV420, 8bit
    pub pcm_sample_luma: Vec<u8>,
    pub pcm_sample_chroma_cb: Vec<u8>,
    pub pcm_sample_chroma_cr: Vec<u8>,
    pub qp: u8,
}

/// Helper structure to track Coded Block Flags (CBF) for CABAC context modeling.
/// It stores whether specific blocks (DC or AC) within the macroblock contain non-zero coefficients.
/// This information is required when deriving context indices for neighboring blocks.
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct CbfInfo {
    pub luma_dc: bool,
    pub luma_ac: u16, // 16 bits
    pub cb_dc: bool,
    pub cb_ac: u8, // 4 bits
    pub cr_dc: bool,
    pub cr_ac: u8, // 4 bits
}

// Macroblock of type I
#[derive(Clone, Debug, Default)]
pub struct IMb {
    pub mb_type: IMbType,
    pub transform_size_8x8_flag: bool,
    pub rem_intra4x4_pred_mode: [Intra_4x4_SamplePredMode; 16],
    pub rem_intra8x8_pred_mode: [Intra_8x8_SamplePredMode; 4],
    pub intra_chroma_pred_mode: Intra_Chroma_Pred_Mode,
    pub coded_block_pattern: CodedBlockPattern,
    pub mb_qp_delta: i32,
    pub qp: u8,
    pub residual: Option<Box<Residual>>,
    pub cbf_info: CbfInfo,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct MotionVector {
    pub x: i16,
    pub y: i16,
}

// Holds prediction data for one partition
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct PartitionInfo {
    /// Which prediction lists are active for this partition (Pred_L0, Pred_L1, BiPred, Direct).
    pub pred_mode: MbPredictionMode,
    pub ref_idx_l0: u8,
    /// Motion Vector (MV) = MVP + MVD
    /// This is the final vector used for prediction.
    pub mv_l0: MotionVector,
    /// Motion Vector Difference (MVD)
    /// This is the delta value parsed from the bitstream.
    pub mvd_l0: MotionVector,
    pub ref_idx_l1: u8,
    pub mv_l1: MotionVector,
    pub mvd_l1: MotionVector,
}

// Holds the final motion information for a single decoded macroblock,
// resolved to a 4x4 grid.
#[derive(Clone, Debug, Default)]
pub struct MbMotion {
    pub partitions: [[PartitionInfo; 4]; 4],
}

// Table 7-17 - Sub-macroblock types in P macroblock
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, FromPrimitive)]
pub enum SubMbType {
    #[default]
    P_L0_8x8 = 0,
    P_L0_8x4 = 1,
    P_L0_4x8 = 2,
    P_L0_4x4 = 3,
}

impl TryFrom<u32> for SubMbType {
    type Error = String;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value).ok_or_else(|| format!("Unknown sub-mb type: {value}"))
    }
}

#[allow(non_camel_case_types)]
impl SubMbType {
    // Table 7-17 – Sub-macroblock types in P macroblocks
    pub fn NumSubMbPart(&self) -> usize {
        match self {
            SubMbType::P_L0_8x8 => 1,
            SubMbType::P_L0_8x4 | SubMbType::P_L0_4x8 => 2,
            SubMbType::P_L0_4x4 => 4,
        }
    }
}

// Holds data for a P_8x8 sub-macroblock
#[derive(Copy, Clone, Debug, Default)]
pub struct SubMacroblock {
    pub sub_mb_type: SubMbType,
    // A sub-macroblock can have up to 4 partitions (for 4x4)
    pub partitions: [PartitionInfo; 4],
}

// Macroblock of type P
#[derive(Clone, Debug, Default)]
pub struct PMb {
    pub mb_type: PMbType,
    pub motion: MbMotion,
    pub coded_block_pattern: CodedBlockPattern,
    pub mb_qp_delta: i32,
    pub qp: u8,
    pub transform_size_8x8_flag: bool,
    pub residual: Option<Box<Residual>>,
    pub cbf_info: CbfInfo,
}

#[derive(Clone, Debug)]
pub enum Macroblock {
    I(IMb),
    PCM(PcmMb),
    P(PMb),
    B(BMb),
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
impl PMb {
    #[inline]
    pub const fn MbPartPredMode(&self, partition: usize) -> MbPredictionMode {
        match self.mb_type {
            PMbType::P_Skip
            | PMbType::P_L0_16x16
            | PMbType::P_L0_L0_16x8
            | PMbType::P_L0_L0_8x16 => MbPredictionMode::Pred_L0,
            PMbType::P_8x8ref0 | PMbType::P_8x8 => MbPredictionMode::None,
        }
    }

    #[inline]
    pub const fn NumMbPart(&self) -> usize {
        match self.mb_type {
            PMbType::P_Skip | PMbType::P_L0_16x16 => 1,
            PMbType::P_L0_L0_16x8 | PMbType::P_L0_L0_8x16 => 2,
            PMbType::P_8x8ref0 | PMbType::P_8x8 => 4,
        }
    }
}

#[allow(non_snake_case)]
impl BMb {
    #[inline]
    pub const fn MbPartPredMode(&self, partition: usize) -> MbPredictionMode {
        self.mb_type.MbPartPredMode(partition)
    }

    #[inline]
    pub const fn NumMbPart(&self) -> usize {
        self.mb_type.NumMbPart()
    }
}

#[allow(non_snake_case)]
impl Macroblock {
    #[inline]
    pub fn MbPartPredMode(&self, partition: usize) -> MbPredictionMode {
        match self {
            Macroblock::I(mb) => mb.MbPartPredMode(partition),
            Macroblock::P(mb) => mb.MbPartPredMode(partition),
            Macroblock::B(mb) => mb.MbPartPredMode(partition),
            Macroblock::PCM(_) => MbPredictionMode::None,
        }
    }

    pub fn is_intra(&self) -> bool {
        matches!(self, Macroblock::I(_) | Macroblock::PCM(_))
    }

    pub fn is_skipped(&self) -> bool {
        match self {
            Macroblock::P(mb) => mb.mb_type == PMbType::P_Skip,
            Macroblock::B(mb) => mb.mb_type == BMbType::B_Skip,
            _ => false,
        }
    }

    pub fn is_direct(&self) -> bool {
        match self {
            Macroblock::B(mb) => mb.mb_type == BMbType::B_Direct_16x16,
            _ => false,
        }
    }

    // Calculates nC for the block withing the macroblock
    pub fn get_nc(&self, blk_idx: u8, plane: ColorPlane) -> u8 {
        // Section 9.2.1
        match self {
            Macroblock::I(mb) => match &mb.residual {
                Some(r) => r.get_nc(blk_idx, plane),
                None => 0,
            },
            Macroblock::PCM(_) => 16,
            Macroblock::P(mb) => match &mb.residual {
                Some(r) => r.get_nc(blk_idx, plane),
                None => 0,
            },
            Macroblock::B(mb) => match &mb.residual {
                Some(r) => r.get_nc(blk_idx, plane),
                None => 0,
            },
        }
    }

    pub fn get_coded_block_pattern(&self) -> CodedBlockPattern {
        match self {
            Macroblock::I(mb) => mb.coded_block_pattern,
            Macroblock::P(mb) => mb.coded_block_pattern,
            Macroblock::B(mb) => mb.coded_block_pattern,
            Macroblock::PCM(_) => CodedBlockPattern::default(),
        }
    }

    pub fn set_residual(&mut self, r: Option<Box<Residual>>) {
        match self {
            Macroblock::I(mb) => {
                mb.residual = r;
            }
            Macroblock::P(mb) => {
                mb.residual = r;
            }
            Macroblock::B(mb) => {
                mb.residual = r;
            }
            Macroblock::PCM(_) => {}
        }
    }

    /// Returns the final, calculated motion information for this macroblock.
    /// For Intra-coded macroblocks, this will return a default (zeroed)
    /// motion info struct, as per H.264 spec requirements for MVP derivation.
    pub fn get_motion_info(&self) -> MbMotion {
        match self {
            Macroblock::P(mb) => mb.motion.clone(),
            Macroblock::B(mb) => mb.motion.clone(),
            Macroblock::I(_) | Macroblock::PCM(_) => MbMotion::default(),
        }
    }

    pub fn set_qp(&mut self, qp: u8) {
        match self {
            Macroblock::I(m) => m.qp = qp,
            Macroblock::P(m) => m.qp = qp,
            Macroblock::B(m) => m.qp = qp,
            Macroblock::PCM(m) => m.qp = qp,
        }
    }

    pub fn get_cbf_info(&self) -> CbfInfo {
        match self {
            Macroblock::I(mb) => mb.cbf_info,
            Macroblock::P(mb) => mb.cbf_info,
            Macroblock::B(mb) => mb.cbf_info,
            Macroblock::PCM(_) => CbfInfo::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    pub fn test_get_4x4luma_block_neighbor() {
        let (a, b) = (MbNeighborName::A, MbNeighborName::B);
        assert_eq!(get_4x4luma_block_neighbor(0, a), (5, Some(a)));
        assert_eq!(get_4x4luma_block_neighbor(2, a), (7, Some(a)));
        assert_eq!(get_4x4luma_block_neighbor(8, a), (13, Some(a)));
        assert_eq!(get_4x4luma_block_neighbor(10, a), (15, Some(a)));

        assert_eq!(get_4x4luma_block_neighbor(1, a), (0, None));
        assert_eq!(get_4x4luma_block_neighbor(3, a), (2, None));
        assert_eq!(get_4x4luma_block_neighbor(9, a), (8, None));
        assert_eq!(get_4x4luma_block_neighbor(11, a), (10, None));

        assert_eq!(get_4x4luma_block_neighbor(4, a), (1, None));
        assert_eq!(get_4x4luma_block_neighbor(6, a), (3, None));
        assert_eq!(get_4x4luma_block_neighbor(12, a), (9, None));
        assert_eq!(get_4x4luma_block_neighbor(14, a), (11, None));

        assert_eq!(get_4x4luma_block_neighbor(5, a), (4, None));
        assert_eq!(get_4x4luma_block_neighbor(7, a), (6, None));
        assert_eq!(get_4x4luma_block_neighbor(13, a), (12, None));
        assert_eq!(get_4x4luma_block_neighbor(15, a), (14, None));

        assert_eq!(get_4x4luma_block_neighbor(0, b), (10, Some(b)));
        assert_eq!(get_4x4luma_block_neighbor(1, b), (11, Some(b)));
        assert_eq!(get_4x4luma_block_neighbor(4, b), (14, Some(b)));
        assert_eq!(get_4x4luma_block_neighbor(5, b), (15, Some(b)));

        assert_eq!(get_4x4luma_block_neighbor(2, b), (0, None));
        assert_eq!(get_4x4luma_block_neighbor(3, b), (1, None));
        assert_eq!(get_4x4luma_block_neighbor(6, b), (4, None));
        assert_eq!(get_4x4luma_block_neighbor(7, b), (5, None));

        assert_eq!(get_4x4luma_block_neighbor(8, b), (2, None));
        assert_eq!(get_4x4luma_block_neighbor(9, b), (3, None));
        assert_eq!(get_4x4luma_block_neighbor(12, b), (6, None));
        assert_eq!(get_4x4luma_block_neighbor(13, b), (7, None));

        assert_eq!(get_4x4luma_block_neighbor(10, b), (8, None));
        assert_eq!(get_4x4luma_block_neighbor(11, b), (9, None));
        assert_eq!(get_4x4luma_block_neighbor(14, b), (12, None));
        assert_eq!(get_4x4luma_block_neighbor(15, b), (13, None));
    }

    #[test]
    pub fn test_get_4x4chroma_block_neighbor() {
        let (a, b) = (MbNeighborName::A, MbNeighborName::B);
        assert_eq!(get_4x4chroma_block_neighbor(0, a), (1, Some(a)));
        assert_eq!(get_4x4chroma_block_neighbor(1, a), (0, None));
        assert_eq!(get_4x4chroma_block_neighbor(2, a), (3, Some(a)));
        assert_eq!(get_4x4chroma_block_neighbor(3, a), (2, None));

        assert_eq!(get_4x4chroma_block_neighbor(0, b), (2, Some(b)));
        assert_eq!(get_4x4chroma_block_neighbor(1, b), (3, Some(b)));
        assert_eq!(get_4x4chroma_block_neighbor(2, b), (0, None));
        assert_eq!(get_4x4chroma_block_neighbor(3, b), (1, None));
    }

    #[test]
    pub fn test_get_neighbor_mbs() {
        assert_eq!(get_neighbor_mbs(16, 0, 20, MbNeighborName::A), Some(19));
        assert_eq!(get_neighbor_mbs(16, 0, 20, MbNeighborName::B), Some(4));
        assert_eq!(get_neighbor_mbs(16, 0, 20, MbNeighborName::C), Some(5));
        assert_eq!(get_neighbor_mbs(16, 0, 20, MbNeighborName::D), Some(3));

        assert_eq!(get_neighbor_mbs(8, 0, 15, MbNeighborName::A), Some(14));
        assert_eq!(get_neighbor_mbs(8, 0, 15, MbNeighborName::B), Some(7));
        assert_eq!(get_neighbor_mbs(8, 0, 15, MbNeighborName::C), None);
        assert_eq!(get_neighbor_mbs(8, 0, 15, MbNeighborName::D), Some(6));

        assert_eq!(get_neighbor_mbs(8, 0, 32, MbNeighborName::A), None);
        assert_eq!(get_neighbor_mbs(8, 0, 32, MbNeighborName::B), Some(24));
        assert_eq!(get_neighbor_mbs(8, 0, 32, MbNeighborName::C), Some(25));
        assert_eq!(get_neighbor_mbs(8, 0, 32, MbNeighborName::D), None);

        assert_eq!(get_neighbor_mbs(8, 25, 33, MbNeighborName::A), Some(32));
        assert_eq!(get_neighbor_mbs(8, 25, 33, MbNeighborName::B), Some(25));
        assert_eq!(get_neighbor_mbs(8, 25, 33, MbNeighborName::C), Some(26));
        assert_eq!(get_neighbor_mbs(8, 25, 33, MbNeighborName::D), None);

        assert!(get_neighbor_mbs(10, 1, 1, MbNeighborName::A).is_none());
        assert!(get_neighbor_mbs(10, 1, 1, MbNeighborName::B).is_none());
        assert!(get_neighbor_mbs(10, 1, 1, MbNeighborName::C).is_none());
        assert!(get_neighbor_mbs(10, 1, 1, MbNeighborName::D).is_none());
    }
}
