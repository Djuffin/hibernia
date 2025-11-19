use std::collections::VecDeque;

use super::decoder::{Picture, VideoFrame};
use super::nal::{NalHeader, NalUnitType};
use super::slice::{MemoryManagementControlOperation, SliceHeader};
use super::sps::SequenceParameterSet;

// Section 8.2.5: Decoded reference picture marking process
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DpbMarking {
    UnusedForReference,
    UsedForShortTermReference,
    UsedForLongTermReference(u32), // long_term_frame_idx
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
    pub pictures: VecDeque<DpbPicture>,
    pub max_size: usize, // max_dec_frame_buffering
}

impl DecodedPictureBuffer {
    pub fn new(max_size: usize) -> Self {
        DecodedPictureBuffer {
            pictures: VecDeque::with_capacity(max_size),
            max_size,
        }
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
        self.pictures.push_back(dpb_picture);
        output_pictures
    }

    /// Section C.4.5.3: Implements the "bumping" process to output frames when the DPB is full.
    pub fn get_pictures_for_output(&mut self) -> Vec<Picture> {
        let mut output = Vec::new();
        let mut i = 0;
        while i < self.pictures.len() {
            if self.pictures[i].marking == DpbMarking::UnusedForReference
                && self.pictures[i].needed_for_output
            {
                if let Some(dpb_pic) = self.pictures.remove(i) {
                    output.push(dpb_pic.picture);
                }
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
        self.pictures.iter().find(|p| {
            p.marking == DpbMarking::UsedForShortTermReference && p.picture.frame_num == pic_num
        })
    }

    pub fn find_long_term_ref(&self, long_term_pic_num: u32) -> Option<&DpbPicture> {
        self.pictures.iter().find(|p| match p.marking {
            DpbMarking::UsedForLongTermReference(idx) => idx == long_term_pic_num,
            _ => false,
        })
    }

    // Section 8.2.5 Decoded reference picture marking process
    pub fn mark_references(
        &mut self,
        header: &SliceHeader,
        nal_header: &NalHeader,
        sps: &SequenceParameterSet,
    ) {
        let curr_frame_num = header.frame_num as i32;
        let is_idr = nal_header.nal_unit_type == NalUnitType::IDRSlice;

        if is_idr {
            // Section 8.2.5.1
            for pic in self.pictures.iter_mut() {
                pic.marking = DpbMarking::UnusedForReference;
            }
            if let Some(dec_ref_pic_marking) = &header.dec_ref_pic_marking {
                if dec_ref_pic_marking.no_output_of_prior_pics_flag.unwrap_or(false) {
                    self.pictures.clear();
                }
                if dec_ref_pic_marking.long_term_reference_flag.unwrap_or(false) {
                    if let Some(last) = self.pictures.back_mut() {
                        last.marking = DpbMarking::UsedForLongTermReference(0);
                    }
                    // max_long_term_frame_idx = 0; (implicit)
                } else {
                    if let Some(last) = self.pictures.back_mut() {
                        last.marking = DpbMarking::UsedForShortTermReference;
                    }
                    // max_long_term_frame_idx = -1; (implicit)
                }
            }
        } else {
            if nal_header.nal_ref_idc == 0 {
                return;
            }

            let adaptive_ref_pic_marking_mode_flag = header
                .dec_ref_pic_marking
                .as_ref()
                .and_then(|m| m.adaptive_ref_pic_marking_mode_flag)
                .unwrap_or(false);

            if adaptive_ref_pic_marking_mode_flag {
                // Section 8.2.5.4 Adaptive memory control decoded reference picture marking process
                let ops = &header
                    .dec_ref_pic_marking
                    .as_ref()
                    .unwrap()
                    .memory_management_operations;
                for op in ops {
                    match op {
                        MemoryManagementControlOperation::MarkShortTermUnused {
                            difference_of_pic_nums_minus1,
                        } => {
                            let pic_num_x = curr_frame_num - (*difference_of_pic_nums_minus1 as i32 + 1);
                            let pic_num_x = if pic_num_x < 0 {
                                pic_num_x + (1 << (sps.log2_max_frame_num_minus4 + 4))
                            } else {
                                pic_num_x
                            };
                            self.mark_short_term_unused(pic_num_x, curr_frame_num, sps);
                        }
                        MemoryManagementControlOperation::MarkLongTermUnused {
                            long_term_pic_num,
                        } => {
                            self.mark_long_term_unused(*long_term_pic_num);
                        }
                        MemoryManagementControlOperation::MarkShortTermAsLongTerm {
                            difference_of_pic_nums_minus1,
                            long_term_frame_idx,
                        } => {
                            let pic_num_x = curr_frame_num - (*difference_of_pic_nums_minus1 as i32 + 1);
                            let pic_num_x = if pic_num_x < 0 {
                                pic_num_x + (1 << (sps.log2_max_frame_num_minus4 + 4))
                            } else {
                                pic_num_x
                            };

                            // First, free up the long term index if used
                            self.mark_long_term_unused(*long_term_frame_idx);

                            // Then assign
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
                             for pic in self.pictures.iter_mut() {
                                pic.marking = DpbMarking::UnusedForReference;
                            }
                            // The current picture is also affected (last one added)
                            if let Some(last) = self.pictures.back_mut() {
                                 last.marking = DpbMarking::UnusedForReference; // Will be potentially set by MMCO 6
                                 last.picture.frame_num = 0;
                                 last.picture.pic_order_cnt = 0; // Reset POC as well? Spec says FrameNum to 0.
                            }
                        }
                        MemoryManagementControlOperation::MarkCurrentAsLongTerm {
                            long_term_frame_idx,
                        } => {
                             self.mark_long_term_unused(*long_term_frame_idx);
                             if let Some(last) = self.pictures.back_mut() {
                                 last.marking = DpbMarking::UsedForLongTermReference(*long_term_frame_idx);
                             }
                        }
                    }
                }
            } else {
                // Section 8.2.5.3 Sliding window decoded reference picture marking process
                let num_ref = self.pictures.iter().filter(|p| p.marking != DpbMarking::UnusedForReference).count();
                if num_ref > sps.max_num_ref_frames as usize {
                    // Find the short-term reference picture with the smallest FrameNumWrap
                    // Since we store in decoding order, and FrameNum wraps, we need to be careful.
                    // But essentially, we are looking for the "oldest" short term reference.
                    // For simplicity in this implementation (and common cases), we can iterate
                    // and find one with UsedForShortTermReference that is not the current picture.
                    // A more robust implementation requires correct FrameNumWrap calculation.
                    
                    // We remove the first found short-term reference that is NOT the current picture (which is at the back)
                    let mut index_to_remove = None;
                    // Search from oldest (front) to newest (back)
                    for (i, pic) in self.pictures.iter().enumerate() {
                        if i == self.pictures.len() - 1 {
                            break; // Don't remove current
                        }
                        if pic.marking == DpbMarking::UsedForShortTermReference {
                             // Just pick the first one (FIFO behavior for sliding window roughly)
                             // Correct FrameNumWrap logic would be needed for technically correct behavior with gaps etc.
                             index_to_remove = Some(i);
                             break;
                        }
                    }
                    
                    if let Some(i) = index_to_remove {
                        self.pictures[i].marking = DpbMarking::UnusedForReference;
                    }
                }
            }
        }
    }

    fn find_short_term_index(&self, pic_num: i32, curr_frame_num: i32, sps: &SequenceParameterSet) -> Option<usize> {
        let max_frame_num = 1 << (sps.log2_max_frame_num_minus4 + 4);
        for (i, pic) in self.pictures.iter().enumerate() {
            if pic.marking == DpbMarking::UsedForShortTermReference {
                let pic_frame_num = pic.picture.frame_num as i32;
                let pn = if pic_frame_num > curr_frame_num {
                    pic_frame_num - max_frame_num
                } else {
                    pic_frame_num
                };
                if pn == pic_num {
                    return Some(i);
                }
            }
        }
        None
    }

    fn mark_short_term_unused(&mut self, pic_num: i32, curr_frame_num: i32, sps: &SequenceParameterSet) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use v_frame::pixel::ChromaSampling;

    fn create_dummy_picture(frame_num: u16, pic_order_cnt: u32) -> Picture {
        Picture {
            frame: VideoFrame::new_with_padding(16, 16, ChromaSampling::Cs420, 0),
            frame_num,
            pic_order_cnt,
        }
    }

    fn create_dummy_dpb_picture(
        frame_num: u16,
        pic_order_cnt: u32,
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
        let mut dpb = DecodedPictureBuffer::new(2);
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
        let mut dpb = DecodedPictureBuffer::new(2);

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
        let mut dpb = DecodedPictureBuffer::new(3);

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
        let mut dpb = DecodedPictureBuffer::new(3);

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
        let mut dpb = DecodedPictureBuffer::new(3);

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
}
