use std::collections::VecDeque;

use super::decoder::{Picture, VideoFrame};

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
