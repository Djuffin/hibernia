pub mod cabac;
pub mod cavlc;
pub mod deblocking;
pub mod decoder;
pub mod dpb;
pub mod inter_pred;
pub mod intra_pred;
pub mod macroblock;
pub mod nal;
pub mod nal_parser;
pub mod parser;
pub mod poc;
pub mod pps;
pub mod rbsp;
pub mod residual;
pub mod scaling_list;
pub mod slice;
pub mod sps;
pub mod tables;

#[cfg(test)]
pub mod e2e_tests;

use num_traits::cast::FromPrimitive;

#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Default,
    FromPrimitive,
    schemars::JsonSchema,
)]
pub enum Profile {
    #[default]
    Baseline = 66,
    Main = 77,
    High = 100,
    High422 = 122,
    High10 = 110,
    High444 = 244,
    Extended = 88,
    ScalableBase = 83,
    ScalableHigh = 86,
    MultiviewHigh = 118,
    StereoHigh = 128,
    MFCDepthHigh = 135,
    MultiviewDepthHigh = 138,
    EnhancedMultiviewDepthHigh = 139,
}

impl Profile {
    /// Whether the SPS for this profile contains the chroma/bit-depth/scaling
    /// extension block (chroma_format_idc, bit_depth_luma_minus8, ...).
    pub fn has_chroma_info(&self) -> bool {
        matches!(
            self,
            Profile::High
                | Profile::High10
                | Profile::High422
                | Profile::High444
                | Profile::ScalableBase
                | Profile::ScalableHigh
                | Profile::MultiviewHigh
                | Profile::StereoHigh
                | Profile::MFCDepthHigh
                | Profile::MultiviewDepthHigh
                | Profile::EnhancedMultiviewDepthHigh
        )
    }
}

impl TryFrom<u32> for Profile {
    type Error = String;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value).ok_or_else(|| format!("Unknown profile: {value}"))
    }
}

#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Default,
    FromPrimitive,
    schemars::JsonSchema,
)]
pub enum ChromaFormat {
    Monochrome = 0,
    #[default]
    YUV420 = 1,
    YUV422 = 2,
    YUV444 = 3,
}

impl ChromaFormat {
    #[inline]
    pub fn is_chroma_subsampled(&self) -> bool {
        matches!(self, ChromaFormat::YUV420 | ChromaFormat::YUV422)
    }

    #[inline]
    pub fn get_chroma_shift(&self) -> Size {
        match self {
            ChromaFormat::YUV420 => Size { width: 1, height: 1 },
            ChromaFormat::YUV422 => Size { width: 1, height: 0 },
            _ => Size { width: 0, height: 0 },
        }
    }
}

#[derive(
    serde::Serialize, serde::Deserialize, Debug, Copy, Clone, Eq, PartialEq, schemars::JsonSchema,
)]
pub enum ColorPlane {
    Y = 0,
    Cb = 1,
    Cr = 2,
}

impl ColorPlane {
    pub fn is_luma(&self) -> bool {
        *self == ColorPlane::Y
    }
}

impl TryFrom<u32> for ChromaFormat {
    type Error = String;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value).ok_or_else(|| format!("Unknown chroma format: {value}"))
    }
}

#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Default,
    schemars::JsonSchema,
)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Default,
    schemars::JsonSchema,
)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Default,
    schemars::JsonSchema,
)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}
