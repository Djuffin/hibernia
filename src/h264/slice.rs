use std::collections::HashMap;
use std::{default, fmt, result};

use num_traits::FromPrimitive;

use super::macroblock::{get_neighbor_mbs, Macroblock, MbAddr, MbNeighborName};
use super::pps::PicParameterSet;
use super::sps::SequenceParameterSet;
use super::{tables, ColorPlane, Point};
use crate::h264::dpb::ReferenceDisposition;

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
    type Error = String;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value)
            .ok_or_else(|| format!("Unknown deblocking filter profile: {value}"))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MemoryManagementControlOperation {
    MarkShortTermUnused { difference_of_pic_nums_minus1: u32 },
    MarkLongTermUnused { long_term_pic_num: u32 },
    MarkShortTermAsLongTerm { difference_of_pic_nums_minus1: u32, long_term_frame_idx: u32 },
    SetMaxLongTermFrameIdx { max_long_term_frame_idx_plus1: u32 },
    MarkAllUnused,
    MarkCurrentAsLongTerm { long_term_frame_idx: u32 },
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct DecRefPicMarking {
    pub no_output_of_prior_pics_flag: Option<bool>,
    pub long_term_reference_flag: Option<bool>,
    pub adaptive_ref_pic_marking_mode_flag: Option<bool>,
    pub memory_management_operations: Vec<MemoryManagementControlOperation>,
}

// Table 7-7 – modification_of_pic_nums_idc operations for modification of reference picture lists
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RefPicListModification {
    RemapShortTermNegative(u32), // abs_diff_pic_num_minus1
    RemapShortTermPositive(u32), // abs_diff_pic_num_minus1
    RemapLongTerm(u32),          // long_term_pic_num
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct RefPicListModifications {
    pub list0: Vec<RefPicListModification>,
    pub list1: Vec<RefPicListModification>,
}

/// Holds the weighting factors for a single reference picture.
/// These values are used to apply a weighted prediction, modifying
/// the sample values from the reference picture before they are used
/// for inter-prediction.
/// Corresponds to the set of luma_weight_lX, luma_offset_lX,
/// chroma_weight_lX, and chroma_offset_lX syntax elements.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WeightingFactors {
    /// The weight to be applied to the luma component of the reference picture.
    pub luma_weight: i32,
    /// The offset to be applied to the luma component after weighting.
    pub luma_offset: i32,
    /// An array holding the weights for Cb and Cr components, respectively.
    pub chroma_weights: [i32; 2],
    /// An array holding the offsets for Cb and Cr components after weighting.
    pub chroma_offsets: [i32; 2],
}

/// Represents the `pred_weight_table()` syntax structure.
/// It contains all information required for weighted prediction for a P, SP, or B slice.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PredWeightTable {
    /// The base-2 logarithm of the denominator for luma weighting.
    /// The final weight is `luma_weight / (1 << luma_log2_weight_denom)`.
    pub luma_log2_weight_denom: u32,
    /// The base-2 logarithm of the denominator for chroma weighting.
    pub chroma_log2_weight_denom: u32,

    /// A list of weighting factors for each reference in reference picture list 0.
    /// The vector is indexed by `ref_idx_l0`.
    pub list0: Vec<WeightingFactors>,

    /// A list of weighting factors for each reference in reference picture list 1.
    /// This is only present for B-slices. The vector is indexed by `ref_idx_l1`.
    pub list1: Vec<WeightingFactors>,
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
    pub delta_pic_order_cnt: [i32; 2],
    pub redundant_pic_cnt: Option<u32>,

    //pub direct_spatial_mv_pred_flag: Option<bool>,
    pub num_ref_idx_l0_active_minus1: u32,
    pub num_ref_idx_l1_active_minus1: u32,

    // may become an enum rather than Option in future (for ref_pic_list_mvc_modification)
    pub ref_pic_list_modification: RefPicListModifications,
    pub pred_weight_table: Option<PredWeightTable>,
    pub dec_ref_pic_marking: Option<DecRefPicMarking>,
    //pub cabac_init_idc: Option<u32>,
    pub slice_qp_delta: i32,
    pub sp_for_switch_flag: Option<bool>,
    pub slice_qs: Option<u32>,
    pub deblocking_filter_idc: DeblockingFilterIdc,
    pub slice_alpha_c0_offset_div2: i32,
    pub slice_beta_offset_div2: i32,
}

#[derive(Debug, Default, Clone)]
pub struct PocState {
    // POC Type 0 state
    pub prev_pic_order_cnt_msb: i32,
    pub prev_pic_order_cnt_lsb: i32,

    // POC Type 2 state
    pub prev_frame_num: i32,
    pub prev_frame_num_offset: i32,
}

#[derive(Clone)]
pub struct Slice {
    pub sps: SequenceParameterSet,
    pub pps: PicParameterSet,
    pub header: SliceHeader,

    macroblocks: Vec<Macroblock>,
    pub ref_pic_list0: Vec<usize>,
    pub ref_pic_list1: Vec<usize>,
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
        Slice {
            sps,
            pps,
            header,
            macroblocks,
            ref_pic_list0: Vec::new(),
            ref_pic_list1: Vec::new(),
        }
    }

    pub fn set_ref_pic_lists(&mut self, list0: Vec<usize>, list1: Vec<usize>) {
        self.ref_pic_list0 = list0;
        self.ref_pic_list1 = list1;
    }

    pub fn MbaffFrameFlag(&self) -> bool {
        self.sps.mb_adaptive_frame_field_flag && !self.header.field_pic_flag
    }

    pub fn get_mb(&self, mb_addr: MbAddr) -> Option<&Macroblock> {
        let index = mb_addr - self.header.first_mb_in_slice;
        self.macroblocks.get(index as usize)
    }

    pub fn get_mb_mut(&mut self, mb_addr: MbAddr) -> Option<&mut Macroblock> {
        let index = mb_addr - self.header.first_mb_in_slice;
        self.macroblocks.get_mut(index as usize)
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

    pub fn get_mb_addr_from_coords(&self, x: i32, y: i32) -> MbAddr {
        let mb_x = x / tables::MB_WIDTH as i32;
        let mb_y = y / tables::MB_HEIGHT as i32;
        (mb_y * self.sps.pic_width_in_mbs() as i32 + mb_x) as MbAddr
    }

    pub fn calculate_poc(&self, state: &mut PocState, disposition: ReferenceDisposition) -> i32 {
        match self.sps.pic_order_cnt_type {
            0 => self.calculate_poc_type0(state, disposition),
            1 => self.calculate_poc_type1(state, disposition),
            2 => self.calculate_poc_type2(state, disposition),
            _ => 0,
        }
    }

    fn calculate_poc_type1(&self, state: &mut PocState, disposition: ReferenceDisposition) -> i32 {
        // Section 8.2.1.2 Decoding process for picture order count type 1
        let (prev_frame_num_offset, prev_frame_num) = if disposition == ReferenceDisposition::Idr {
            (0, 0)
        } else {
            (state.prev_frame_num_offset, state.prev_frame_num)
        };

        let frame_num_offset = if self.header.frame_num < prev_frame_num as u16 {
            prev_frame_num_offset + (1 << (self.sps.log2_max_frame_num_minus4 + 4))
        } else {
            prev_frame_num_offset
        };

        let num_ref_frames_in_pic_order_cnt_cycle = self.sps.offset_for_ref_frame.len() as i32;
        let abs_frame_num = if num_ref_frames_in_pic_order_cnt_cycle != 0 {
            frame_num_offset + self.header.frame_num as i32
        } else {
            0
        };

        let abs_frame_num = if self.header.pic_parameter_set_id == 0 && abs_frame_num > 0 { // nal_ref_idc == 0 check is tricky here without NAL header
             // Assuming nal_ref_idc is not 0 for this logic or handled elsewhere
             // Actually, spec says: if( nal_ref_idc == 0 && absFrameNum > 0 ) absFrameNum = absFrameNum - 1
             // But we don't have nal_ref_idc easily here? It is passed in 'disposition'.
             if disposition == ReferenceDisposition::NonReference && abs_frame_num > 0 {
                 abs_frame_num - 1
             } else {
                 abs_frame_num
             }
        } else {
             abs_frame_num
        };

        let expected_pic_order_cnt = if abs_frame_num > 0 {
            let pic_order_cnt_cycle_cnt = (abs_frame_num - 1) / num_ref_frames_in_pic_order_cnt_cycle;
            let frame_num_in_pic_order_cnt_cycle = (abs_frame_num - 1) % num_ref_frames_in_pic_order_cnt_cycle;

            let expected_delta_per_pic_order_cnt_cycle: i32 = self.sps.offset_for_ref_frame.iter().sum();

            let mut expected_poc = pic_order_cnt_cycle_cnt * expected_delta_per_pic_order_cnt_cycle;
            for i in 0..=frame_num_in_pic_order_cnt_cycle {
                expected_poc += self.sps.offset_for_ref_frame[i as usize];
            }
            expected_poc
        } else {
            0
        };

        let expected_pic_order_cnt = if disposition == ReferenceDisposition::NonReference {
            expected_pic_order_cnt + self.sps.offset_for_non_ref_pic
        } else {
            expected_pic_order_cnt
        };

        let top_field_order_cnt = expected_pic_order_cnt + self.header.delta_pic_order_cnt[0];
        // TODO: Handle BottomFieldOrderCnt

        if disposition != ReferenceDisposition::NonReference {
            state.prev_frame_num = self.header.frame_num as i32;
            state.prev_frame_num_offset = frame_num_offset;
        }

        top_field_order_cnt
    }

    fn calculate_poc_type0(&self, state: &mut PocState, disposition: ReferenceDisposition) -> i32 {
        // Section 8.2.1.1 Decoding process for picture order count type 0
        let (prev_msb, prev_lsb) = if disposition == ReferenceDisposition::Idr {
            (0, 0)
        } else {
            (state.prev_pic_order_cnt_msb, state.prev_pic_order_cnt_lsb)
        };

        let max_lsb = 1 << (self.sps.log2_max_pic_order_cnt_lsb_minus4 + 4);
        let lsb = self.header.pic_order_cnt_lsb.unwrap_or(0) as i32;

        // Check for MSB overflow/underflow (wrapping)
        let msb = if lsb < prev_lsb && (prev_lsb - lsb) >= (max_lsb / 2) {
            prev_msb + max_lsb
        } else if lsb > prev_lsb && (lsb - prev_lsb) > (max_lsb / 2) {
            prev_msb - max_lsb
        } else {
            prev_msb
        };

        if disposition != ReferenceDisposition::NonReference {
            state.prev_pic_order_cnt_lsb = lsb;
            state.prev_pic_order_cnt_msb = msb;
        }

        // TopFieldOrderCnt = PicOrderCntMsb + pic_order_cnt_lsb
        let top_field_order_cnt = msb + lsb;

        // TODO: Handle BottomFieldOrderCnt if bottom_field_flag is present (interlaced)
        // For progressive frames, POC is min(TopFieldOrderCnt, BottomFieldOrderCnt),
        // but effectively just TopFieldOrderCnt for now.

        top_field_order_cnt
    }

    fn calculate_poc_type2(&self, state: &mut PocState, disposition: ReferenceDisposition) -> i32 {
        // Section 8.2.1.3 Decoding process for picture order count type 2
        let (frame_num_offset, temp_pic_order_cnt) = if disposition == ReferenceDisposition::Idr {
            (0, 0)
        } else {
            let prev_frame_num = state.prev_frame_num;
            let prev_frame_num_offset = state.prev_frame_num_offset;
            let frame_num = self.header.frame_num as i32;

            let frame_num_offset = if prev_frame_num > frame_num {
                let max_frame_num = 1 << (self.sps.log2_max_frame_num_minus4 + 4);
                prev_frame_num_offset + max_frame_num
            } else {
                prev_frame_num_offset
            };

            let temp_pic_order_cnt = if disposition == ReferenceDisposition::NonReference {
                2 * (frame_num_offset + frame_num) - 1
            } else {
                2 * (frame_num_offset + frame_num)
            };

            (frame_num_offset, temp_pic_order_cnt)
        };

        if disposition != ReferenceDisposition::NonReference {
            state.prev_frame_num_offset = frame_num_offset;
            state.prev_frame_num = self.header.frame_num as i32;
        }

        temp_pic_order_cnt
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

    #[test]
    fn test_poc_type2_non_ref() {
        let mut poc_state = PocState::default();
        let mut slice = prepare_slice();
        // Modify prepare_slice result to have pic_order_cnt_type 2
        slice.sps.pic_order_cnt_type = 2;
        slice.sps.log2_max_frame_num_minus4 = 0; // max_frame_num = 16

        // IDR
        slice.calculate_poc(&mut poc_state, ReferenceDisposition::Idr);

        // Frame 1 (Ref)
        slice.header.frame_num = 1;
        slice.calculate_poc(&mut poc_state, ReferenceDisposition::NonIdrReference);

        // Frame 2 (Non-Ref)
        slice.header.frame_num = 2;
        let poc = slice.calculate_poc(&mut poc_state, ReferenceDisposition::NonReference);
        // tempPOC = 2 * (0 + 2) - 1 = 3
        assert_eq!(poc, 3);

        // State should NOT update for non-ref
        assert_eq!(poc_state.prev_frame_num, 1);
    }

    #[test]
    fn test_poc_type2_wrapping() {
        let mut poc_state = PocState::default();
        let mut slice = prepare_slice();
        slice.sps.pic_order_cnt_type = 2;
        slice.sps.log2_max_frame_num_minus4 = 0; // max_frame_num = 16

        // IDR
        slice.calculate_poc(&mut poc_state, ReferenceDisposition::Idr);

        // Frame 15 (Ref)
        slice.header.frame_num = 15;
        let poc = slice.calculate_poc(&mut poc_state, ReferenceDisposition::NonIdrReference);
        assert_eq!(poc, 30); // 2 * 15
        assert_eq!(poc_state.prev_frame_num, 15);

        // Frame 0 (Ref) - Wrap around
        slice.header.frame_num = 0;
        let poc = slice.calculate_poc(&mut poc_state, ReferenceDisposition::NonIdrReference);
        // prev_frame_num (15) > frame_num (0) => offset += 16 => offset = 16.
        // POC = 2 * (16 + 0) = 32.
        assert_eq!(poc, 32);
        assert_eq!(poc_state.prev_frame_num, 0);
        assert_eq!(poc_state.prev_frame_num_offset, 16);

        // Frame 1 (Ref)
        slice.header.frame_num = 1;
        let poc = slice.calculate_poc(&mut poc_state, ReferenceDisposition::NonIdrReference);
        // prev_frame_num (0) > frame_num (1) is False. Offset stays 16.
        // POC = 2 * (16 + 1) = 34.
        assert_eq!(poc, 34);
    }
}
