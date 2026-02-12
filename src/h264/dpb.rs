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

// Annex C: Decoded Picture Buffer (DPB)
#[derive(Debug)]
pub struct DecodedPictureBuffer {
    pub pictures: Vec<DpbPicture>,
    pub max_size: usize, // max_dec_frame_buffering
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

    /// Stores a picture in the DPB.
    /// Manages the "bumping" process if the DPB is full.
    pub fn store_picture(&mut self, dpb_picture: DpbPicture) -> Vec<Picture> {
        let mut output_pictures = Vec::new();
        if self.is_full() {
            output_pictures = self.get_pictures_for_output();
        }
        self.pictures.push(dpb_picture);
        output_pictures
    }

    /// Section C.4.5.3: Implements the "bumping" process to output frames when the DPB is full.
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

    /// Section 8.2.5 Decoded reference picture marking process.
    ///
    /// Returns true if a memory_management_control_operation equal to 5 (MMCO 5) was processed,
    /// indicating that the POC state should be reset for the next picture.
    pub fn mark_references(
        &mut self,
        header: &SliceHeader,
        disposition: ReferenceDisposition,
        sps: &SequenceParameterSet,
    ) -> bool {
        let mut has_mmco5 = false;
        match disposition {
            ReferenceDisposition::Idr => self.mark_idr_references(header),
            ReferenceDisposition::NonIdrReference => {
                let adaptive_ref_pic_marking_mode_flag = header
                    .dec_ref_pic_marking
                    .as_ref()
                    .and_then(|m| m.adaptive_ref_pic_marking_mode_flag)
                    .unwrap_or(false);

                if adaptive_ref_pic_marking_mode_flag {
                    has_mmco5 = self.mark_adaptive_references(header, sps);
                } else {
                    self.mark_sliding_window_references(sps);
                }

                // Section 8.2.5.1 Step 3:
                // When the current picture is not an IDR picture and it was not marked as
                // "used for long-term reference" by MMCO 6, it is marked as "used for short-term reference".
                if let Some(last) = self.pictures.last_mut() {
                    if !last.marking.is_long_term() {
                        last.marking = DpbMarking::UsedForShortTermReference;
                    }
                }
            }
            ReferenceDisposition::NonReference => {}
        }
        has_mmco5
    }

    fn mark_idr_references(&mut self, header: &SliceHeader) {
        // Section 8.2.5.1
        for pic in self.pictures.iter_mut() {
            pic.marking = DpbMarking::UnusedForReference;
        }

        if let Some(dec_ref_pic_marking) = &header.dec_ref_pic_marking {
            if dec_ref_pic_marking.no_output_of_prior_pics_flag.unwrap_or(false) {
                self.pictures.clear();
            }

            if let Some(last) = self.pictures.last_mut() {
                if dec_ref_pic_marking.long_term_reference_flag.unwrap_or(false) {
                    last.marking = DpbMarking::UsedForLongTermReference(0);
                    // max_long_term_frame_idx = 0; (implicit)
                } else {
                    last.marking = DpbMarking::UsedForShortTermReference;
                    // max_long_term_frame_idx = -1; (implicit)
                }
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
    ) -> bool {
        // Section 8.2.5.4 Adaptive memory control decoded reference picture marking process
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
                    self.mark_short_term_unused(pic_num_x, curr_frame_num, sps);
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

                    if let Some(idx) = self.find_short_term_index(pic_num_x, curr_frame_num, sps) {
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
                    if let Some(last) = self.pictures.last_mut() {
                        last.picture.frame_num = 0;
                        last.picture.pic_order_cnt = 0;
                    }
                }
                MemoryManagementControlOperation::MarkCurrentAsLongTerm { long_term_frame_idx } => {
                    self.mark_long_term_unused(*long_term_frame_idx);
                    if let Some(last) = self.pictures.last_mut() {
                        last.marking = DpbMarking::UsedForLongTermReference(*long_term_frame_idx);
                    }
                }
            }
        }
        has_mmco5
    }

    fn mark_sliding_window_references(&mut self, sps: &SequenceParameterSet) {
        // Section 8.2.5.3 Sliding window decoded reference picture marking process
        let num_ref = self.pictures.iter().filter(|p| !p.marking.is_unused()).count();

        if num_ref > sps.max_num_ref_frames as usize {
            // We remove the first found short-term reference that is NOT the current picture.
            // The current picture is always the last one in our Vec.
            // Since we want the "oldest", and we push to the back, searching from the front is correct.

            let index_to_remove = self.pictures.iter().enumerate().position(|(i, pic)| {
                // Don't remove the current picture (last one)
                i != self.pictures.len() - 1 && pic.marking.is_short_term()
            });

            if let Some(i) = index_to_remove {
                self.pictures[i].marking = DpbMarking::UnusedForReference;
            }
        }
    }

    fn find_short_term_index(
        &self,
        pic_num: i32,
        curr_frame_num: i32,
        sps: &SequenceParameterSet,
    ) -> Option<usize> {
        let max_frame_num = 1 << (sps.log2_max_frame_num_minus4 + 4);
        self.pictures.iter().position(|pic| {
            if pic.marking.is_short_term() {
                let pic_frame_num = pic.picture.frame_num as i32;
                let pn = if pic_frame_num > curr_frame_num {
                    pic_frame_num - max_frame_num
                } else {
                    pic_frame_num
                };
                pn == pic_num
            } else {
                false
            }
        })
    }

    fn mark_short_term_unused(
        &mut self,
        pic_num: i32,
        curr_frame_num: i32,
        sps: &SequenceParameterSet,
    ) {
        if let Some(idx) = self.find_short_term_index(pic_num, curr_frame_num, sps) {
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

        // Add some reference pictures
        dpb.store_picture(create_dummy_dpb_picture(1, 2, DpbMarking::UsedForShortTermReference));
        dpb.store_picture(create_dummy_dpb_picture(2, 4, DpbMarking::UsedForShortTermReference));

        // Current picture with MMCO 5
        let current = create_dummy_dpb_picture(3, 6, DpbMarking::UsedForShortTermReference);
        dpb.store_picture(current);

        let mut header = SliceHeader::default();
        header.frame_num = 3;
        let mut marking = DecRefPicMarking::default();
        marking.adaptive_ref_pic_marking_mode_flag = Some(true);
        marking.memory_management_operations.push(MemoryManagementControlOperation::MarkAllUnused);
        header.dec_ref_pic_marking = Some(marking);

        let sps = SequenceParameterSet::default();
        let has_mmco5 = dpb.mark_references(&header, ReferenceDisposition::NonIdrReference, &sps);

        assert!(has_mmco5);
        // All previous should be unused
        assert!(dpb.pictures[0].marking.is_unused());
        assert!(dpb.pictures[1].marking.is_unused());

        // Current (last) should be short-term reference (Section 8.2.5.1 Step 3)
        let last = dpb.pictures.last().unwrap();
        assert!(last.marking.is_short_term());
        // And reset to 0
        assert_eq!(last.picture.frame_num, 0);
        assert_eq!(last.picture.pic_order_cnt, 0);
    }
}
