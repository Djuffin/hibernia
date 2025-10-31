use std::collections::HashMap;
use std::{default, fmt, result};

use num_traits::FromPrimitive;

use super::macroblock::{get_neighbor_mbs, Macroblock, MbAddr, MbNeighborName};
use super::pps::PicParameterSet;
use super::sps::SequenceParameterSet;
use super::{tables, ColorPlane, Point};

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, FromPrimitive)]
pub enum DeblockingFilterIdc {
    #[default]
    On = 0,
    Off = 1,
    OnExceptSliceBounds = 2,
}

impl TryFrom<u32> for DeblockingFilterIdc {
    type Error = &'static str;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value).ok_or("Unknown deblocking filter profile idc.")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
pub enum MemoryManagementControlOperation {
    End = 0,
    MarkShortTermUnused = 1,
    MarkLongTermUnused = 2,
    MarkShortTermAsLongTerm = 3,
    SetMaxLongTermFrameIdx = 4,
    MarkAllUnused = 5,
    MarkCurrentAsLongTerm = 6,
}

impl TryFrom<u32> for MemoryManagementControlOperation {
    type Error = &'static str;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value).ok_or("Unknown memory_management_control_operation.")
    }
}

// Section 7.4.3 Slice header semantics
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct SliceHeader {
    pub first_mb_in_slice: MbAddr,
    pub slice_type: SliceType,
    pub pic_parameter_set_id: u8,
    pub color_plane: Option<ColorPlane>,
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
    pub deblocking_filter_idc: DeblockingFilterIdc,
}

#[derive(Clone)]
pub struct Slice {
    pub sps: SequenceParameterSet,
    pub pps: PicParameterSet,
    pub header: SliceHeader,

    macroblocks: Vec<Macroblock>,
}

impl fmt::Debug for Slice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Slice").field("header", &self.header).finish()
    }
}

#[allow(non_snake_case)]
impl Slice {
    pub fn new(sps: SequenceParameterSet, pps: PicParameterSet, header: SliceHeader) -> Slice {
        let mb_count = sps.pic_size_in_mbs();
        let macroblocks = Vec::with_capacity(mb_count);
        Slice { sps, pps, header, macroblocks }
    }

    pub fn MbaffFrameFlag(&self) -> bool {
        self.sps.mb_adaptive_frame_field_flag && !self.header.field_pic_flag
    }

    pub fn get_mb(&self, mb_addr: MbAddr) -> Option<&Macroblock> {
        let index = mb_addr - self.header.first_mb_in_slice;
        self.macroblocks.get(index as usize)
    }

    pub fn get_mb_neighbor(
        &self,
        mb_addr: MbAddr,
        neighbor: MbNeighborName,
    ) -> Option<&Macroblock> {
        get_neighbor_mbs(
            self.sps.pic_width_in_mbs() as u32,
            self.header.first_mb_in_slice,
            mb_addr,
            neighbor,
        )
        .and_then(|x| self.get_mb(x))
    }

    pub fn has_mb_neighbor(&self, mb_addr: MbAddr, neighbor: MbNeighborName) -> bool {
        get_neighbor_mbs(
            self.sps.pic_width_in_mbs() as u32,
            self.header.first_mb_in_slice,
            mb_addr,
            neighbor,
        )
        .is_some()
    }

    pub fn append_mb(&mut self, block: Macroblock) -> MbAddr {
        let result = self.get_next_mb_addr();
        self.macroblocks.push(block);
        result
    }

    pub fn get_macroblock_count(&self) -> usize {
        self.macroblocks.len()
    }

    pub fn get_next_mb_addr(&self) -> MbAddr {
        self.macroblocks.len() as MbAddr + self.header.first_mb_in_slice
    }

    pub fn get_mb_location(&self, addr: MbAddr) -> Point {
        let width_in_mbs = self.sps.pic_width_in_mbs();
        let x = addr % (width_in_mbs as u32) * (tables::MB_WIDTH as u32);
        let y = addr / (width_in_mbs as u32) * (tables::MB_HEIGHT as u32);
        Point { x, y }
    }
}

#[cfg(test)]
mod tests {
    use crate::h264::{macroblock::PcmMb, sps::VuiParameters, ChromaFormat, Profile};

    pub use super::*;

    pub fn prepare_slice() -> Slice {
        let sps = SequenceParameterSet {
            profile: Profile::Baseline,
            level_idc: 20,
            seq_parameter_set_id: 0,
            chroma_format_idc: ChromaFormat::YUV420,
            separate_color_plane_flag: false,
            log2_max_frame_num_minus4: 11,
            log2_max_pic_order_cnt_lsb_minus4: 12,
            max_num_ref_frames: 1,
            pic_width_in_mbs_minus1: 3,
            pic_height_in_map_units_minus1: 3,
            frame_mbs_only_flag: true,
            vui_parameters: None,
            ..SequenceParameterSet::default()
        };

        let pps = PicParameterSet {
            pic_parameter_set_id: 0,
            seq_parameter_set_id: 0,
            entropy_coding_mode_flag: false,
            deblocking_filter_control_present_flag: true,
            ..PicParameterSet::default()
        };

        let header = SliceHeader { first_mb_in_slice: 100, ..SliceHeader::default() };

        Slice::new(sps, pps, header)
    }

    fn prepare_mb() -> Macroblock {
        Macroblock::PCM(PcmMb::default())
    }

    #[test]
    pub fn test_slice_addressing() {
        let mut slice = prepare_slice();
        assert_eq!(slice.get_macroblock_count(), 0);
        assert_eq!(slice.get_next_mb_addr(), 100);
        let neighbors =
            [MbNeighborName::A, MbNeighborName::B, MbNeighborName::C, MbNeighborName::D];
        for nb in neighbors {
            assert!(slice.get_mb_neighbor(slice.get_next_mb_addr(), nb).is_none());
        }

        assert_eq!(slice.append_mb(prepare_mb()), 100);
        assert!(slice.get_mb_neighbor(slice.get_next_mb_addr(), MbNeighborName::A).is_some());
        assert!(slice.get_mb_neighbor(slice.get_next_mb_addr(), MbNeighborName::B).is_none());

        for _ in 0..slice.sps.pic_width_in_mbs() {
            slice.append_mb(prepare_mb());
        }

        for nb in neighbors {
            assert!(slice.get_mb_neighbor(slice.get_next_mb_addr(), nb).is_some());
        }
        assert_eq!(slice.get_macroblock_count(), slice.sps.pic_width_in_mbs() + 1);
    }
}
