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

impl SliceType {
    pub fn is_intra(&self) -> bool {
        matches!(self, SliceType::I | SliceType::SI)
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

impl DecRefPicMarking {
    pub fn write(&self, writer: &mut super::rbsp_writer::RbspWriter, idr_pic_flag: bool) -> super::rbsp_writer::WriteResult {
        if idr_pic_flag {
            writer.f(self.no_output_of_prior_pics_flag.unwrap_or(false))?;
            writer.f(self.long_term_reference_flag.unwrap_or(false))?;
        } else {
            let adaptive = self.adaptive_ref_pic_marking_mode_flag.unwrap_or(false);
            writer.f(adaptive)?;
            if adaptive {
                for op in &self.memory_management_operations {
                    match op {
                        MemoryManagementControlOperation::MarkShortTermUnused { difference_of_pic_nums_minus1 } => {
                            writer.ue(1)?;
                            writer.ue(*difference_of_pic_nums_minus1)?;
                        }
                        MemoryManagementControlOperation::MarkLongTermUnused { long_term_pic_num } => {
                            writer.ue(2)?;
                            writer.ue(*long_term_pic_num)?;
                        }
                        MemoryManagementControlOperation::MarkShortTermAsLongTerm { difference_of_pic_nums_minus1, long_term_frame_idx } => {
                            writer.ue(3)?;
                            writer.ue(*difference_of_pic_nums_minus1)?;
                            writer.ue(*long_term_frame_idx)?;
                        }
                        MemoryManagementControlOperation::SetMaxLongTermFrameIdx { max_long_term_frame_idx_plus1 } => {
                            writer.ue(4)?;
                            writer.ue(*max_long_term_frame_idx_plus1)?;
                        }
                        MemoryManagementControlOperation::MarkAllUnused => {
                            writer.ue(5)?;
                        }
                        MemoryManagementControlOperation::MarkCurrentAsLongTerm { long_term_frame_idx } => {
                            writer.ue(6)?;
                            writer.ue(*long_term_frame_idx)?;
                        }
                    }
                }
                writer.ue(0)?;
            }
        }
        Ok(())
    }
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

impl RefPicListModifications {
    pub fn write(&self, writer: &mut super::rbsp_writer::RbspWriter, slice_type: SliceType) -> super::rbsp_writer::WriteResult {
        if slice_type != SliceType::I && slice_type != SliceType::SI {
            writer.f(!self.list0.is_empty())?;
            for modification in &self.list0 {
                match modification {
                    RefPicListModification::RemapShortTermNegative(abs_diff_pic_num_minus1) => {
                        writer.ue(0)?;
                        writer.ue(*abs_diff_pic_num_minus1)?;
                    }
                    RefPicListModification::RemapShortTermPositive(abs_diff_pic_num_minus1) => {
                        writer.ue(1)?;
                        writer.ue(*abs_diff_pic_num_minus1)?;
                    }
                    RefPicListModification::RemapLongTerm(long_term_pic_num) => {
                        writer.ue(2)?;
                        writer.ue(*long_term_pic_num)?;
                    }
                }
            }
            if !self.list0.is_empty() {
                writer.ue(3)?;
            }
        }

        if slice_type == SliceType::B {
            writer.f(!self.list1.is_empty())?;
            for modification in &self.list1 {
                match modification {
                    RefPicListModification::RemapShortTermNegative(abs_diff_pic_num_minus1) => {
                        writer.ue(0)?;
                        writer.ue(*abs_diff_pic_num_minus1)?;
                    }
                    RefPicListModification::RemapShortTermPositive(abs_diff_pic_num_minus1) => {
                        writer.ue(1)?;
                        writer.ue(*abs_diff_pic_num_minus1)?;
                    }
                    RefPicListModification::RemapLongTerm(long_term_pic_num) => {
                        writer.ue(2)?;
                        writer.ue(*long_term_pic_num)?;
                    }
                }
            }
            if !self.list1.is_empty() {
                writer.ue(3)?;
            }
        }

        Ok(())
    }
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

impl PredWeightTable {
    pub fn write(&self, writer: &mut super::rbsp_writer::RbspWriter, slice_header: &SliceHeader, sps: &SequenceParameterSet, pps: &PicParameterSet) -> super::rbsp_writer::WriteResult {
        writer.ue(self.luma_log2_weight_denom)?;
        if sps.ChromaArrayType() != super::ChromaFormat::Monochrome {
            writer.ue(self.chroma_log2_weight_denom)?;
        }

        for i in 0..=slice_header.num_ref_idx_l0_active_minus1 as usize {
            let factors = &self.list0[i];
            let luma_default = 1 << self.luma_log2_weight_denom;
            let luma_modified = factors.luma_weight != luma_default || factors.luma_offset != 0;
            writer.f(luma_modified)?;
            if luma_modified {
                writer.se(factors.luma_weight)?;
                writer.se(factors.luma_offset)?;
            }

            if sps.ChromaArrayType() != super::ChromaFormat::Monochrome {
                let chroma_default = 1 << self.chroma_log2_weight_denom;
                let chroma_modified = factors.chroma_weights[0] != chroma_default || factors.chroma_offsets[0] != 0 || factors.chroma_weights[1] != chroma_default || factors.chroma_offsets[1] != 0;
                writer.f(chroma_modified)?;
                if chroma_modified {
                    for j in 0..2 {
                        writer.se(factors.chroma_weights[j])?;
                        writer.se(factors.chroma_offsets[j])?;
                    }
                }
            }
        }

        if slice_header.slice_type == SliceType::B {
            for i in 0..=slice_header.num_ref_idx_l1_active_minus1 as usize {
                let factors = &self.list1[i];
                let luma_default = 1 << self.luma_log2_weight_denom;
                let luma_modified = factors.luma_weight != luma_default || factors.luma_offset != 0;
                writer.f(luma_modified)?;
                if luma_modified {
                    writer.se(factors.luma_weight)?;
                    writer.se(factors.luma_offset)?;
                }

                if sps.ChromaArrayType() != super::ChromaFormat::Monochrome {
                    let chroma_default = 1 << self.chroma_log2_weight_denom;
                    let chroma_modified = factors.chroma_weights[0] != chroma_default || factors.chroma_offsets[0] != 0 || factors.chroma_weights[1] != chroma_default || factors.chroma_offsets[1] != 0;
                    writer.f(chroma_modified)?;
                    if chroma_modified {
                        for j in 0..2 {
                            writer.se(factors.chroma_weights[j])?;
                            writer.se(factors.chroma_offsets[j])?;
                        }
                    }
                }
            }
        }

        Ok(())
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
    pub delta_pic_order_cnt: [i32; 2],
    pub redundant_pic_cnt: Option<u32>,

    pub direct_spatial_mv_pred_flag: Option<bool>,
    pub num_ref_idx_l0_active_minus1: u32,
    pub num_ref_idx_l1_active_minus1: u32,

    // may become an enum rather than Option in future (for ref_pic_list_mvc_modification)
    pub ref_pic_list_modification: RefPicListModifications,
    pub pred_weight_table: Option<PredWeightTable>,
    pub dec_ref_pic_marking: Option<DecRefPicMarking>,
    pub cabac_init_idc: u32,
    pub slice_qp_delta: i32,
    pub sp_for_switch_flag: Option<bool>,
    pub slice_qs_delta: Option<i32>,
    pub deblocking_filter_idc: DeblockingFilterIdc,
    pub slice_alpha_c0_offset_div2: i32,
    pub slice_beta_offset_div2: i32,
    pub slice_group_change_cycle: Option<u32>,
}

impl SliceHeader {
    pub fn write_slice_header(&self, sps: &SequenceParameterSet, pps: &PicParameterSet, idr_pic_flag: bool, writer: &mut super::rbsp_writer::RbspWriter) -> super::rbsp_writer::WriteResult {
        writer.ue(self.first_mb_in_slice)?;
        writer.ue(self.slice_type as u32)?;
        writer.ue(self.pic_parameter_set_id as u32)?;
        
        if sps.separate_color_plane_flag {
            let plane_id = match self.color_plane {
                Some(super::ColorPlane::Y) => 0,
                Some(super::ColorPlane::Cb) => 1,
                Some(super::ColorPlane::Cr) => 2,
                None => 0, // Fallback
            };
            writer.u(2, plane_id)?;
        }
        
        writer.u(sps.bits_in_frame_num(), self.frame_num as u32)?;
        
        if !sps.frame_mbs_only_flag {
            writer.f(self.field_pic_flag)?;
            if self.field_pic_flag {
                if let Some(b) = self.bottom_field_flag {
                    writer.f(b)?;
                }
            }
        }
        
        if idr_pic_flag {
            writer.ue(self.idr_pic_id.unwrap_or(0))?;
        }
        
        if sps.pic_order_cnt_type == 0 {
            writer.u(sps.bits_in_max_pic_order_cnt(), self.pic_order_cnt_lsb.unwrap_or(0))?;
            if pps.bottom_field_pic_order_in_frame_present_flag && !self.field_pic_flag {
                writer.se(self.delta_pic_order_cnt_bottom.unwrap_or(0))?;
            }
        } else if sps.pic_order_cnt_type == 1 && !sps.delta_pic_order_always_zero_flag {
            writer.se(self.delta_pic_order_cnt[0])?;
            if pps.bottom_field_pic_order_in_frame_present_flag && !self.field_pic_flag {
                writer.se(self.delta_pic_order_cnt[1])?;
            }
        }
        
        if pps.redundant_pic_cnt_present_flag {
            writer.ue(self.redundant_pic_cnt.unwrap_or(0))?;
        }
        
        if matches!(self.slice_type, SliceType::P | SliceType::SP | SliceType::B) {
            let num_ref_idx_override = self.num_ref_idx_l0_active_minus1 != pps.num_ref_idx_l0_default_active_minus1 || (self.slice_type == SliceType::B && self.num_ref_idx_l1_active_minus1 != pps.num_ref_idx_l1_default_active_minus1);
            writer.f(num_ref_idx_override)?;
            if num_ref_idx_override {
                writer.ue(self.num_ref_idx_l0_active_minus1)?;
                if self.slice_type == SliceType::B {
                    writer.ue(self.num_ref_idx_l1_active_minus1)?;
                }
            }
        }
        
        self.ref_pic_list_modification.write(writer, self.slice_type)?;
        
        if (pps.weighted_pred_flag && matches!(self.slice_type, SliceType::P | SliceType::SP))
            || (pps.weighted_bipred_idc == 1 && self.slice_type == SliceType::B) {
            if let Some(table) = &self.pred_weight_table {
                table.write(writer, self, sps, pps)?;
            }
        }
        
        if let Some(marking) = &self.dec_ref_pic_marking {
            marking.write(writer, idr_pic_flag)?;
        }
        
        writer.se(self.slice_qp_delta)?;
        
        if pps.deblocking_filter_control_present_flag {
            writer.ue(self.deblocking_filter_idc as u32)?;
            if self.deblocking_filter_idc != DeblockingFilterIdc::Off {
                writer.se(self.slice_alpha_c0_offset_div2)?;
                writer.se(self.slice_beta_offset_div2)?;
            }
        }
        
        if pps.slice_group.as_ref().map_or(false, |sg| matches!(sg, super::pps::SliceGroup::Changing { .. })) {
            if let Some(super::pps::SliceGroup::Changing { slice_group_change_rate_minus1, .. }) = pps.slice_group.as_ref() {
                let pic_size_in_map_units = (sps.pic_width_in_mbs_minus1 as u32 + 1) * (sps.pic_height_in_map_units_minus1 as u32 + 1);
                let slice_group_change_rate = slice_group_change_rate_minus1 + 1;
                let bits = ((pic_size_in_map_units as f64 / slice_group_change_rate as f64) + 1.0).log2().ceil() as u8;
                writer.u(bits, self.slice_group_change_cycle.unwrap_or(0))?;
            }
        }
        
        Ok(())
    }
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

    pub fn slice_qp_y(&self) -> i32 {
        26 + self.pps.pic_init_qp_minus26 + self.header.slice_qp_delta
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
