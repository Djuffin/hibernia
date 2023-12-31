use std::collections::HashMap;
use std::{fmt, result};

use super::macroblock::{get_neighbor_mbs, Macroblock, MbAddr, MbNeighborNames, MbNeighbors};
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
    pub disable_deblocking_filter_idc: u8,
}

#[derive(Clone)]
pub struct Slice {
    pub sps: SequenceParameterSet,
    pub pps: PicParameterSet,
    pub header: SliceHeader,

    macroblocks: Vec<Macroblock>,
    next_mb_neighbors: MbNeighbors,
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
        Slice { sps, pps, header, macroblocks, next_mb_neighbors: MbNeighbors::default() }
    }

    pub fn MbaffFrameFlag(&self) -> bool {
        self.sps.mb_adaptive_frame_field_flag && !self.header.field_pic_flag
    }

    pub fn get_mb(&self, mb_addr: MbAddr) -> Option<&Macroblock> {
        let index = mb_addr - self.header.first_mb_in_slice;
        self.macroblocks.get(index as usize)
    }

    pub fn get_neighbor_mb(&self, neighbor: MbNeighborNames) -> Option<&Macroblock> {
        let mb_addr = self.next_mb_neighbors.get(neighbor);
        mb_addr.and_then(|x| self.get_mb(x))
    }

    pub fn append_mb(&mut self, block: Macroblock) -> MbAddr {
        let result = self.get_next_mb_addr();
        self.next_mb_neighbors = get_neighbor_mbs(
            self.sps.pic_width_in_mbs() as u32,
            self.header.first_mb_in_slice,
            result,
        );
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
