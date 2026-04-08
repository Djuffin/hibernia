use super::decoder::{Picture, VideoFrame};
use super::slice::{MemoryManagementControlOperation, SliceHeader};
use super::sps::SequenceParameterSet;

// Section 8.2.5: Decoded reference picture marking process
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DpbMarking {
    UnusedForReference,
    UsedForShortTermReference,
    UsedForLongTermReference(u32), // long_term_frame_idx
}

impl DpbMarking {
    pub fn is_unused(&self) -> bool {
        matches!(self, DpbMarking::UnusedForReference)
    }

    pub fn is_short_term(&self) -> bool {
        matches!(self, DpbMarking::UsedForShortTermReference)
    }

    pub fn is_long_term(&self) -> bool {
        matches!(self, DpbMarking::UsedForLongTermReference(_))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReferenceDisposition {
    Idr,
    NonIdrReference,
    NonReference,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DpbPictureStructure {
    Frame,
    TopField,
    BottomField,
}

#[derive(Clone, Debug)]
pub struct DpbPicture {
    pub picture: Picture,
    pub marking: DpbMarking,
    pub is_idr: bool,
    pub structure: DpbPictureStructure,
    pub needed_for_output: bool,
}

/// Compute MaxDpbFrames per spec A.3.1 / Table A-1.
/// Returns the maximum number of frames the DPB can hold based on level limits.
pub fn max_dpb_frames(sps: &SequenceParameterSet) -> usize {
    // Table A-1: MaxDPB in bytes for each level_idc
    let max_dpb_bytes: u64 = match sps.level_idc {
        10 => 152_064,
        11 => {
            if sps.constraint_set3_flag {
                // Level 1b
                152_064
            } else {
                345_600
            }
        }
        12 => 912_384,
        13 => 912_384,
        20 => 912_384,
        21 => 1_824_768,
        22 => 3_110_400,
        30 => 3_110_400,
        31 => 6_912_000,
        32 => 7_864_320,
        40 => 12_582_912,
        41 => 12_582_912,
        42 => 13_369_344,
        50 => 42_393_600,
        51 => 70_778_880,
        52 => 70_778_880,
        // Level 1b encoded as 9
        9 => 152_064,
        _ => 70_778_880, // fallback to max
    };

    // MaxDpbFrames = Min( MaxDPB / ( PicWidthInMbs * FrameHeightInMbs * 384 ), 16 )
    // For frame coding, FrameHeightInMbs = PicHeightInMapUnits
    let frame_size = sps.pic_width_in_mbs() as u64 * sps.pic_height_in_mbs() as u64 * 384;
    std::cmp::min((max_dpb_bytes / frame_size) as usize, 16)
}

// Annex C: Decoded Picture Buffer (DPB)
#[derive(Debug)]
pub struct DecodedPictureBuffer {
    pub pictures: Vec<DpbPicture>,
    pub max_size: usize, // max_dec_frame_buffering
}

impl Default for DecodedPictureBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl DecodedPictureBuffer {
    pub fn new() -> Self {
        DecodedPictureBuffer { pictures: Vec::with_capacity(16), max_size: 16 }
    }

    pub fn set_max_size(&mut self, max_size: usize) {
        self.max_size = max_size;
    }

    pub fn is_full(&self) -> bool {
        self.pictures.len() >= self.max_size
    }

    /// Remove pictures that are both unused for reference and not needed for output.
    /// These are "dead" pictures that were already output via bumping and later
    /// marked as unused by the sliding window or adaptive reference marking.
    pub fn remove_dead_pictures(&mut self) {
        self.pictures.retain(|p| !p.marking.is_unused() || p.needed_for_output);
    }

    /// Stores a picture in the DPB.
    /// Manages the "bumping" process (Section C.4.5.3) if the DPB is full.
    pub fn store_picture(&mut self, dpb_picture: DpbPicture) -> Vec<Picture> {
        let mut output_pictures = Vec::new();
        while self.is_full() {
            match self.bump_one() {
                Some(pic) => output_pictures.push(pic),
                None => break,
            }
        }
        self.pictures.push(dpb_picture);
        output_pictures
    }

    /// Section C.4.5.3: One step of the "bumping" process.
    /// Outputs the picture with the smallest PicOrderCnt among all pictures
    /// marked as "needed for output". If the picture is unused for reference,
    /// it is removed from the DPB (freeing space). Otherwise, only its frame
    /// is cloned and it is marked as "not needed for output".
    fn bump_one(&mut self) -> Option<Picture> {
        let idx = self
            .pictures
            .iter()
            .enumerate()
            .filter(|(_, p)| p.needed_for_output)
            .min_by_key(|(_, p)| p.picture.pic_order_cnt)
            .map(|(i, _)| i)?;

        if self.pictures[idx].marking.is_unused() {
            // Non-reference: remove from DPB (frees space)
            Some(self.pictures.remove(idx).picture)
        } else {
            // Reference: clone frame, mark not-needed-for-output (doesn't free space)
            let pic = Picture {
                frame: self.pictures[idx].picture.frame.clone(),
                frame_num: self.pictures[idx].picture.frame_num,
                pic_order_cnt: self.pictures[idx].picture.pic_order_cnt,
                motion_field: None,
            };
            self.pictures[idx].needed_for_output = false;
            Some(pic)
        }
    }

    /// Outputs all non-reference pictures that are needed for output, sorted by POC.
    /// Used by flush_on_idr after marking all pictures as unused.
    pub fn get_pictures_for_output(&mut self) -> Vec<Picture> {
        let mut output = Vec::new();
        let mut i = 0;
        while i < self.pictures.len() {
            if self.pictures[i].marking.is_unused() && self.pictures[i].needed_for_output {
                let dpb_pic = self.pictures.remove(i);
                output.push(dpb_pic.picture);
            } else {
                i += 1;
            }
        }

        // Sort for correct display order
        output.sort_by_key(|p| p.pic_order_cnt);

        output
    }

    pub fn flush_on_idr(&mut self) -> Vec<Picture> {
        for pic in self.pictures.iter_mut() {
            pic.marking = DpbMarking::UnusedForReference;
        }
        let output = self.get_pictures_for_output();
        self.pictures.clear();
        output
    }

    pub fn find_short_term_ref(&self, pic_num: u16) -> Option<&DpbPicture> {
        self.pictures.iter().find(|p| p.marking.is_short_term() && p.picture.frame_num == pic_num)
    }

    pub fn find_long_term_ref(&self, long_term_pic_num: u32) -> Option<&DpbPicture> {
        self.pictures.iter().find(|p| match p.marking {
            DpbMarking::UsedForLongTermReference(idx) => idx == long_term_pic_num,
            _ => false,
        })
    }

    /// Section C.2.3 / 8.2.5: Mark prior references and set current picture's marking.
    ///
    /// Called BEFORE the current picture is stored in the DPB (spec C.2.3 before C.2.4).
    /// Operations on prior pictures modify the DPB directly; operations that affect the
    /// current picture (IDR marking, MMCO 5/6, short-term default) are applied to `current_pic`.
    ///
    /// Returns true if MMCO 5 was processed, indicating POC state should be reset.
    pub fn mark_prior_references(
        &mut self,
        header: &SliceHeader,
        disposition: ReferenceDisposition,
        sps: &SequenceParameterSet,
        current_pic: &mut DpbPicture,
    ) -> bool {
        let mut has_mmco5 = false;
        match disposition {
            ReferenceDisposition::Idr => {
                self.mark_idr_prior_references(header, current_pic);
            }
            ReferenceDisposition::NonIdrReference => {
                let adaptive_ref_pic_marking_mode_flag = header
                    .dec_ref_pic_marking
                    .as_ref()
                    .and_then(|m| m.adaptive_ref_pic_marking_mode_flag)
                    .unwrap_or(false);

                if adaptive_ref_pic_marking_mode_flag {
                    has_mmco5 = self.mark_adaptive_references(header, sps, current_pic);
                } else {
                    self.mark_sliding_window_references(sps);
                }

                // Section 8.2.5.1 Step 3:
                // When the current picture is not an IDR picture and it was not marked as
                // "used for long-term reference" by MMCO 6, it is marked as "used for short-term reference".
                if !current_pic.marking.is_long_term() {
                    current_pic.marking = DpbMarking::UsedForShortTermReference;
                }
            }
            ReferenceDisposition::NonReference => {}
        }
        has_mmco5
    }

    /// Section 8.2.5.1 for IDR pictures.
    /// Mark all DPB pictures as unused, optionally flush, and set current_pic marking.
    fn mark_idr_prior_references(
        &mut self,
        header: &SliceHeader,
        current_pic: &mut DpbPicture,
    ) {
        // Mark all pictures currently in the DPB as unused for reference.
        for pic in self.pictures.iter_mut() {
            pic.marking = DpbMarking::UnusedForReference;
        }

        if let Some(dec_ref_pic_marking) = &header.dec_ref_pic_marking {
            if dec_ref_pic_marking.no_output_of_prior_pics_flag.unwrap_or(false) {
                // Remove all prior pictures without output.
                self.pictures.clear();
            }

            if dec_ref_pic_marking.long_term_reference_flag.unwrap_or(false) {
                current_pic.marking = DpbMarking::UsedForLongTermReference(0);
                // max_long_term_frame_idx = 0; (implicit)
            } else {
                current_pic.marking = DpbMarking::UsedForShortTermReference;
                // max_long_term_frame_idx = -1; (implicit)
            }
        }
    }

    /// Section 8.2.5.4 Adaptive memory control decoded reference picture marking process.
    ///
    /// Returns true if MMCO 5 was encountered.
    fn mark_adaptive_references(
        &mut self,
        header: &SliceHeader,
        sps: &SequenceParameterSet,
        current_pic: &mut DpbPicture,
    ) -> bool {
        let ops = match &header.dec_ref_pic_marking {
            Some(m) => &m.memory_management_operations,
            None => return false,
        };

        let curr_frame_num = header.frame_num as i32;
        let mut has_mmco5 = false;

        for op in ops {
            match op {
                MemoryManagementControlOperation::MarkShortTermUnused {
                    difference_of_pic_nums_minus1,
                } => {
                    let pic_num_x = calculate_pic_num_x(
                        curr_frame_num,
                        *difference_of_pic_nums_minus1,
                        sps.log2_max_frame_num_minus4,
                    );
                    self.mark_short_term_unused(pic_num_x);
                }
                MemoryManagementControlOperation::MarkLongTermUnused { long_term_pic_num } => {
                    self.mark_long_term_unused(*long_term_pic_num);
                }
                MemoryManagementControlOperation::MarkShortTermAsLongTerm {
                    difference_of_pic_nums_minus1,
                    long_term_frame_idx,
                } => {
                    let pic_num_x = calculate_pic_num_x(
                        curr_frame_num,
                        *difference_of_pic_nums_minus1,
                        sps.log2_max_frame_num_minus4,
                    );

                    self.mark_long_term_unused(*long_term_frame_idx);

                    if let Some(idx) = self.find_short_term_index(pic_num_x) {
                        self.pictures[idx].marking =
                            DpbMarking::UsedForLongTermReference(*long_term_frame_idx);
                    }
                }
                MemoryManagementControlOperation::SetMaxLongTermFrameIdx {
                    max_long_term_frame_idx_plus1,
                } => {
                    let max_long_term_frame_idx = *max_long_term_frame_idx_plus1 as i32 - 1;
                    for pic in self.pictures.iter_mut() {
                        if let DpbMarking::UsedForLongTermReference(idx) = pic.marking {
                            if idx as i32 > max_long_term_frame_idx {
                                pic.marking = DpbMarking::UnusedForReference;
                            }
                        }
                    }
                }
                MemoryManagementControlOperation::MarkAllUnused => {
                    has_mmco5 = true;
                    for pic in self.pictures.iter_mut() {
                        pic.marking = DpbMarking::UnusedForReference;
                    }
                    current_pic.picture.frame_num = 0;
                    current_pic.picture.pic_order_cnt = 0;
                }
                MemoryManagementControlOperation::MarkCurrentAsLongTerm { long_term_frame_idx } => {
                    self.mark_long_term_unused(*long_term_frame_idx);
                    current_pic.marking =
                        DpbMarking::UsedForLongTermReference(*long_term_frame_idx);
                }
            }
        }
        has_mmco5
    }

    /// Section 8.2.5.3 Sliding window decoded reference picture marking process.
    /// Called when the current picture is NOT yet in the DPB.
    /// The spec triggers eviction when numShortTerm + numLongTerm == Max(max_num_ref_frames, 1),
    /// counting only prior references (current picture not yet stored/marked).
    fn mark_sliding_window_references(&mut self, sps: &SequenceParameterSet) {
        let num_ref = self.pictures.iter().filter(|p| !p.marking.is_unused()).count();
        let max_ref = std::cmp::max(sps.max_num_ref_frames as usize, 1);

        if num_ref >= max_ref {
            // Find the oldest short-term reference picture.
            let index_to_remove = self
                .pictures
                .iter()
                .position(|pic| pic.marking.is_short_term());

            if let Some(i) = index_to_remove {
                self.pictures[i].marking = DpbMarking::UnusedForReference;
            }
        }
    }

    fn find_short_term_index(&self, frame_num: i32) -> Option<usize> {
        self.pictures.iter().position(|pic| {
            pic.marking.is_short_term() && pic.picture.frame_num as i32 == frame_num
        })
    }

    fn mark_short_term_unused(&mut self, frame_num: i32) {
        if let Some(idx) = self.find_short_term_index(frame_num) {
            self.pictures[idx].marking = DpbMarking::UnusedForReference;
        }
    }

    fn mark_long_term_unused(&mut self, long_term_pic_num: u32) {
        for pic in self.pictures.iter_mut() {
            if let DpbMarking::UsedForLongTermReference(idx) = pic.marking {
                if idx == long_term_pic_num {
                    pic.marking = DpbMarking::UnusedForReference;
                }
            }
        }
    }

    pub fn flush(&mut self) -> Vec<DpbPicture> {
        let mut output = Vec::new();
        let mut i = 0;
        while i < self.pictures.len() {
            if self.pictures[i].needed_for_output {
                let mut pic = self.pictures.remove(i);
                pic.needed_for_output = false;
                output.push(pic);
            } else {
                i += 1;
            }
        }
        output.sort_by_key(|p| p.picture.pic_order_cnt);
        output
    }
}

fn calculate_pic_num_x(
    curr_frame_num: i32,
    difference_of_pic_nums_minus1: u32,
    log2_max_frame_num_minus4: u8,
) -> i32 {
    let pic_num_x = curr_frame_num - (difference_of_pic_nums_minus1 as i32 + 1);
    if pic_num_x < 0 {
        pic_num_x + (1 << (log2_max_frame_num_minus4 + 4))
    } else {
        pic_num_x
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::h264::slice::DecRefPicMarking;
    use v_frame::pixel::ChromaSampling;

    fn create_dummy_picture(frame_num: u16, pic_order_cnt: i32) -> Picture {
        Picture {
            frame: VideoFrame::new_with_padding(16, 16, ChromaSampling::Cs420, 0),
            frame_num,
            pic_order_cnt,
            motion_field: None,
        }
    }

    fn create_dummy_dpb_picture(
        frame_num: u16,
        pic_order_cnt: i32,
        marking: DpbMarking,
    ) -> DpbPicture {
        DpbPicture {
            picture: create_dummy_picture(frame_num, pic_order_cnt),
            marking,
            is_idr: false,
            structure: DpbPictureStructure::Frame,
            needed_for_output: true,
        }
    }

    #[test]
    fn test_dpb_store_and_full() {
        let mut dpb = DecodedPictureBuffer::new();
        dpb.set_max_size(2);
        assert!(!dpb.is_full());

        let pic1 = create_dummy_dpb_picture(1, 2, DpbMarking::UsedForShortTermReference);
        dpb.store_picture(pic1);
        assert!(!dpb.is_full());
        assert_eq!(dpb.pictures.len(), 1);

        let pic2 = create_dummy_dpb_picture(2, 4, DpbMarking::UsedForShortTermReference);
        dpb.store_picture(pic2);
        assert!(dpb.is_full());
        assert_eq!(dpb.pictures.len(), 2);
    }

    #[test]
    fn test_dpb_bumping_process() {
        let mut dpb = DecodedPictureBuffer::new();
        dpb.set_max_size(2);

        let pic1 = create_dummy_dpb_picture(1, 2, DpbMarking::UnusedForReference);
        let pic2 = create_dummy_dpb_picture(2, 4, DpbMarking::UsedForShortTermReference);
        dpb.store_picture(pic1);
        dpb.store_picture(pic2);

        assert!(dpb.is_full());

        // Storing a new picture should trigger the bumping process.
        // pic1 should be output because it's marked as UnusedForReference.
        let pic3 = create_dummy_dpb_picture(3, 6, DpbMarking::UsedForShortTermReference);
        let output_pics = dpb.store_picture(pic3);

        assert_eq!(output_pics.len(), 1);
        assert_eq!(output_pics[0].frame_num, 1);
        assert_eq!(dpb.pictures.len(), 2); // pic2 and pic3 remain
        assert_eq!(dpb.pictures[0].picture.frame_num, 2);
        assert_eq!(dpb.pictures[1].picture.frame_num, 3);
    }

    #[test]
    fn test_dpb_output_order() {
        let mut dpb = DecodedPictureBuffer::new();
        dpb.set_max_size(3);

        let pic1 = create_dummy_dpb_picture(3, 6, DpbMarking::UnusedForReference);
        let pic2 = create_dummy_dpb_picture(1, 2, DpbMarking::UnusedForReference);
        let pic3 = create_dummy_dpb_picture(2, 4, DpbMarking::UnusedForReference);

        dpb.store_picture(pic1);
        dpb.store_picture(pic2);
        dpb.store_picture(pic3);

        let output_pics = dpb.get_pictures_for_output();
        assert_eq!(output_pics.len(), 3);
        assert_eq!(output_pics[0].pic_order_cnt, 2);
        assert_eq!(output_pics[1].pic_order_cnt, 4);
        assert_eq!(output_pics[2].pic_order_cnt, 6);
    }

    #[test]
    fn test_dpb_flush_on_idr() {
        let mut dpb = DecodedPictureBuffer::new();
        dpb.set_max_size(3);

        let pic1 = create_dummy_dpb_picture(1, 2, DpbMarking::UsedForShortTermReference);
        let pic2 = create_dummy_dpb_picture(2, 4, DpbMarking::UsedForLongTermReference(0));
        // Lower POC, but needed for output
        let pic3 = create_dummy_dpb_picture(3, 0, DpbMarking::UnusedForReference);

        dpb.store_picture(pic1);
        dpb.store_picture(pic2);
        dpb.store_picture(pic3);

        let output_pics = dpb.flush_on_idr();

        assert_eq!(output_pics.len(), 3);
        assert_eq!(output_pics[0].pic_order_cnt, 0); // Check if sorted by POC
        assert_eq!(output_pics[1].pic_order_cnt, 2);
        assert_eq!(output_pics[2].pic_order_cnt, 4);
        assert!(dpb.pictures.is_empty());
    }

    #[test]
    fn test_find_reference_pictures() {
        let mut dpb = DecodedPictureBuffer::new();
        dpb.set_max_size(3);

        let pic1 = create_dummy_dpb_picture(1, 2, DpbMarking::UsedForShortTermReference);
        let pic2 = create_dummy_dpb_picture(2, 4, DpbMarking::UsedForLongTermReference(0));
        let pic3 = create_dummy_dpb_picture(3, 6, DpbMarking::UnusedForReference);

        dpb.store_picture(pic1);
        dpb.store_picture(pic2);
        dpb.store_picture(pic3);

        // Find short-term reference
        let st_ref = dpb.find_short_term_ref(1);
        assert!(st_ref.is_some());
        assert_eq!(st_ref.unwrap().picture.frame_num, 1);

        // Find long-term reference
        let lt_ref = dpb.find_long_term_ref(0);
        assert!(lt_ref.is_some());
        assert_eq!(lt_ref.unwrap().picture.frame_num, 2);

        // Find non-existent references
        assert!(dpb.find_short_term_ref(99).is_none());
        assert!(dpb.find_long_term_ref(99).is_none());
    }

    #[test]
    fn test_mmco5_marking() {
        let mut dpb = DecodedPictureBuffer::new();
        dpb.set_max_size(4);

        // Add some reference pictures (prior pictures, already in DPB)
        dpb.store_picture(create_dummy_dpb_picture(1, 2, DpbMarking::UsedForShortTermReference));
        dpb.store_picture(create_dummy_dpb_picture(2, 4, DpbMarking::UsedForShortTermReference));

        // Current picture (NOT stored in DPB yet — per spec C.2.3 before C.2.4)
        let mut current = create_dummy_dpb_picture(3, 6, DpbMarking::UsedForShortTermReference);

        let mut header = SliceHeader::default();
        header.frame_num = 3;
        let mut marking = DecRefPicMarking::default();
        marking.adaptive_ref_pic_marking_mode_flag = Some(true);
        marking.memory_management_operations.push(MemoryManagementControlOperation::MarkAllUnused);
        header.dec_ref_pic_marking = Some(marking);

        let sps = SequenceParameterSet::default();
        let has_mmco5 = dpb.mark_prior_references(
            &header,
            ReferenceDisposition::NonIdrReference,
            &sps,
            &mut current,
        );

        assert!(has_mmco5);
        // All prior pictures should be unused
        assert!(dpb.pictures[0].marking.is_unused());
        assert!(dpb.pictures[1].marking.is_unused());

        // Current should be short-term reference (Section 8.2.5.1 Step 3)
        assert!(current.marking.is_short_term());
        // And reset to 0
        assert_eq!(current.picture.frame_num, 0);
        assert_eq!(current.picture.pic_order_cnt, 0);
    }
}
