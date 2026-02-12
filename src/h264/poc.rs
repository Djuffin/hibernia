use super::dpb::ReferenceDisposition;
use super::slice::Slice;

#[derive(Debug, Default)]
pub struct PocState {
    // POC Type 0 state
    prev_pic_order_cnt_msb: i32,
    prev_pic_order_cnt_lsb: i32,
    /// Set to true if the previous *reference* picture included an MMCO 5 operation.
    /// Used for state resets in POC Type 0 (Section 8.2.1.1).
    prev_ref_has_mmco5: bool,

    // POC Type 1 & 2 state
    prev_frame_num: i32,
    prev_frame_num_offset: i32,
    /// Set to true if the previous picture (reference or non-reference) included an MMCO 5 operation.
    /// Used for state resets in POC Type 1 and 2 (Sections 8.2.1.2 and 8.2.1.3).
    prev_has_mmco5: bool,
}

impl PocState {
    pub fn new() -> Self {
        PocState::default()
    }

    #[inline]
    pub fn calculate_poc(&mut self, slice: &Slice, disposition: ReferenceDisposition) -> i32 {
        match slice.sps.pic_order_cnt_type {
            0 => self.calculate_poc_type0(slice, disposition),
            1 => self.calculate_poc_type1(slice, disposition),
            2 => self.calculate_poc_type2(slice, disposition),
            _ => 0,
        }
    }

    pub fn update_mmco5_state(&mut self, has_mmco5: bool, is_reference: bool) {
        self.prev_has_mmco5 = has_mmco5;
        if is_reference {
            self.prev_ref_has_mmco5 = has_mmco5;
        }
    }

    #[inline]
    fn calculate_poc_type0(&mut self, slice: &Slice, disposition: ReferenceDisposition) -> i32 {
        // Section 8.2.1.1 Decoding process for picture order count type 0
        let (prev_msb, prev_lsb) = if disposition == ReferenceDisposition::Idr {
            (0, 0)
        } else if self.prev_ref_has_mmco5 {
            // Section 8.2.1.1: If the previous reference picture included MMCO 5,
            // prevPicOrderCntMsb = 0 and prevPicOrderCntLsb = TopFieldOrderCnt of prev ref picture (0)
            (0, 0)
        } else {
            (self.prev_pic_order_cnt_msb, self.prev_pic_order_cnt_lsb)
        };

        let max_lsb = 1 << (slice.sps.log2_max_pic_order_cnt_lsb_minus4 + 4);
        let lsb = slice.header.pic_order_cnt_lsb.unwrap_or(0) as i32;

        // Check for MSB overflow/underflow (wrapping)
        let msb = if lsb < prev_lsb && (prev_lsb - lsb) >= (max_lsb / 2) {
            prev_msb + max_lsb
        } else if lsb > prev_lsb && (lsb - prev_lsb) > (max_lsb / 2) {
            prev_msb - max_lsb
        } else {
            prev_msb
        };

        if disposition != ReferenceDisposition::NonReference {
            self.prev_pic_order_cnt_lsb = lsb;
            self.prev_pic_order_cnt_msb = msb;
        }

        // TopFieldOrderCnt = PicOrderCntMsb + pic_order_cnt_lsb
        let top_field_order_cnt = msb + lsb;

        // TODO: Handle BottomFieldOrderCnt if bottom_field_flag is present (interlaced)
        // For progressive frames, POC is min(TopFieldOrderCnt, BottomFieldOrderCnt),
        // but effectively just TopFieldOrderCnt for now.

        top_field_order_cnt
    }

    #[inline]
    fn calculate_poc_type1(&mut self, slice: &Slice, disposition: ReferenceDisposition) -> i32 {
        // Section 8.2.1.2 Decoding process for picture order count type 1
        let (prev_frame_num_offset, prev_frame_num) = if disposition == ReferenceDisposition::Idr {
            (0, 0)
        } else if self.prev_has_mmco5 {
            // Section 8.2.1.2: If the previous picture included MMCO 5, prevFrameNumOffset = 0
            // and the previous picture is considered to have frame_num = 0.
            (0, 0)
        } else {
            (self.prev_frame_num_offset, self.prev_frame_num)
        };

        let frame_num_offset = if slice.header.frame_num < prev_frame_num as u16 {
            prev_frame_num_offset + (1 << (slice.sps.log2_max_frame_num_minus4 + 4))
        } else {
            prev_frame_num_offset
        };

        let num_ref_frames_in_pic_order_cnt_cycle = slice.sps.offset_for_ref_frame.len() as i32;
        let abs_frame_num = if num_ref_frames_in_pic_order_cnt_cycle != 0 {
            frame_num_offset + slice.header.frame_num as i32
        } else {
            0
        };

        // Section 8.2.1.2: if( nal_ref_idc == 0 && absFrameNum > 0 ) absFrameNum = absFrameNum - 1
        let abs_frame_num = if disposition == ReferenceDisposition::NonReference && abs_frame_num > 0
        {
            abs_frame_num - 1
        } else {
            abs_frame_num
        };

        let expected_pic_order_cnt = if abs_frame_num > 0 {
            let pic_order_cnt_cycle_cnt =
                (abs_frame_num - 1) / num_ref_frames_in_pic_order_cnt_cycle;
            let frame_num_in_pic_order_cnt_cycle =
                (abs_frame_num - 1) % num_ref_frames_in_pic_order_cnt_cycle;

            let expected_delta_per_pic_order_cnt_cycle: i32 =
                slice.sps.offset_for_ref_frame.iter().sum();

            let mut expected_poc =
                pic_order_cnt_cycle_cnt * expected_delta_per_pic_order_cnt_cycle;
            for i in 0..=frame_num_in_pic_order_cnt_cycle {
                expected_poc += slice.sps.offset_for_ref_frame[i as usize];
            }
            expected_poc
        } else {
            0
        };

        let expected_pic_order_cnt = if disposition == ReferenceDisposition::NonReference {
            expected_pic_order_cnt + slice.sps.offset_for_non_ref_pic
        } else {
            expected_pic_order_cnt
        };

        let top_field_order_cnt = expected_pic_order_cnt + slice.header.delta_pic_order_cnt[0];
        // TODO: Handle BottomFieldOrderCnt

        if disposition != ReferenceDisposition::NonReference {
            self.prev_frame_num = slice.header.frame_num as i32;
            self.prev_frame_num_offset = frame_num_offset;
        }

        top_field_order_cnt
    }

    #[inline]
    fn calculate_poc_type2(&mut self, slice: &Slice, disposition: ReferenceDisposition) -> i32 {
        // Section 8.2.1.3 Decoding process for picture order count type 2
        let (frame_num_offset, temp_pic_order_cnt) = if disposition == ReferenceDisposition::Idr {
            (0, 0)
        } else {
            let (prev_frame_num, prev_frame_num_offset) = if self.prev_has_mmco5 {
                // Section 8.2.1.3: If the previous picture included MMCO 5, prevFrameNumOffset = 0
                // and the previous picture is considered to have frame_num = 0.
                (0, 0)
            } else {
                (self.prev_frame_num, self.prev_frame_num_offset)
            };
            let frame_num = slice.header.frame_num as i32;

            let frame_num_offset = if prev_frame_num > frame_num {
                let max_frame_num = 1 << (slice.sps.log2_max_frame_num_minus4 + 4);
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
            self.prev_frame_num_offset = frame_num_offset;
            self.prev_frame_num = slice.header.frame_num as i32;
        }

        temp_pic_order_cnt
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::h264::pps::PicParameterSet;
    use crate::h264::slice::SliceHeader;
    use crate::h264::sps::{SequenceParameterSet, VuiParameters};
    use crate::h264::{ChromaFormat, Profile};

    fn prepare_slice() -> Slice {
        let sps = SequenceParameterSet {
            profile: Profile::Baseline,
            level_idc: 20,
            seq_parameter_set_id: 0,
            chroma_format_idc: ChromaFormat::YUV420,
            separate_color_plane_flag: false,
            log2_max_frame_num_minus4: 0, // max_frame_num = 16
            log2_max_pic_order_cnt_lsb_minus4: 4,
            max_num_ref_frames: 1,
            pic_width_in_mbs_minus1: 3,
            pic_height_in_map_units_minus1: 3,
            frame_mbs_only_flag: true,
            pic_order_cnt_type: 2, // Important for Type 2 testing
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

        let header = SliceHeader { first_mb_in_slice: 0, ..SliceHeader::default() };

        Slice::new(sps, pps, header)
    }

    #[test]
    fn test_poc_type2_non_ref() {
        let mut poc_state = PocState::new();
        let mut slice = prepare_slice();

        // IDR
        poc_state.calculate_poc(&slice, ReferenceDisposition::Idr);

        // Frame 1 (Ref)
        slice.header.frame_num = 1;
        poc_state.calculate_poc(&slice, ReferenceDisposition::NonIdrReference);

        // Frame 2 (Non-Ref)
        // Non-ref usually shares frame_num with previous ref, or increments?
        // Spec says: "If the current picture is not a reference picture, frame_num shall be equal to FrameNum of the preceding reference picture."
        // But let's assume valid bitstream where frame_num matches logic.
        // If we provide frame_num = 2, and non-ref.
        slice.header.frame_num = 2;
        let poc = poc_state.calculate_poc(&slice, ReferenceDisposition::NonReference);
        // tempPOC = 2 * (0 + 2) - 1 = 3
        assert_eq!(poc, 3);

        // Decoder state should NOT update for non-ref
        assert_eq!(poc_state.prev_frame_num, 1);
    }

    #[test]
    fn test_poc_type2_wrapping() {
        let mut poc_state = PocState::new();
        let mut slice = prepare_slice();
        // log2_max_frame_num_minus4 = 0 => max_frame_num = 1 << 4 = 16.

        // IDR
        poc_state.calculate_poc(&slice, ReferenceDisposition::Idr);

        // Frame 15 (Ref)
        slice.header.frame_num = 15;
        let poc = poc_state.calculate_poc(&slice, ReferenceDisposition::NonIdrReference);
        assert_eq!(poc, 30); // 2 * 15
        assert_eq!(poc_state.prev_frame_num, 15);

        // Frame 0 (Ref) - Wrap around
        slice.header.frame_num = 0;
        let poc = poc_state.calculate_poc(&slice, ReferenceDisposition::NonIdrReference);
        // prev_frame_num (15) > frame_num (0) => offset += 16 => offset = 16.
        // POC = 2 * (16 + 0) = 32.
        assert_eq!(poc, 32);
        assert_eq!(poc_state.prev_frame_num, 0);
        assert_eq!(poc_state.prev_frame_num_offset, 16);

        // Frame 1 (Ref)
        slice.header.frame_num = 1;
        let poc = poc_state.calculate_poc(&slice, ReferenceDisposition::NonIdrReference);
        // prev_frame_num (0) > frame_num (1) is False. Offset stays 16.
        // POC = 2 * (16 + 1) = 34.
        assert_eq!(poc, 34);
    }

    #[test]
    fn test_poc_mmco5_reset() {
        let mut poc_state = PocState::new();
        let mut slice = prepare_slice();
        slice.sps.pic_order_cnt_type = 0;
        slice.sps.log2_max_pic_order_cnt_lsb_minus4 = 4; // max_lsb = 16

        // IDR
        poc_state.calculate_poc(&slice, ReferenceDisposition::Idr);

        // Frame 0 (Ref)
        slice.header.pic_order_cnt_lsb = Some(0);
        poc_state.calculate_poc(&slice, ReferenceDisposition::NonIdrReference);

        // Frame 2 (Ref) with MMCO 5
        slice.header.pic_order_cnt_lsb = Some(2);
        poc_state.calculate_poc(&slice, ReferenceDisposition::NonIdrReference);
        // Simulate MMCO 5 execution
        poc_state.update_mmco5_state(true, true);

        // Next Frame (Ref)
        slice.header.pic_order_cnt_lsb = Some(4);
        let poc = poc_state.calculate_poc(&slice, ReferenceDisposition::NonIdrReference);
        // Since prev_ref_has_mmco5 is true:
        // prev_msb = 0, prev_lsb = 0
        // msb = 0 (no wrapping)
        // POC = 0 + 4 = 4
        assert_eq!(poc, 4);
    }
}
