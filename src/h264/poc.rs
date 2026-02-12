use crate::h264::dpb::ReferenceDisposition;
use crate::h264::slice::Slice;

#[derive(Clone, Debug, Default)]
pub struct PocState {
    // POC Type 0 state
    pub prev_pic_order_cnt_msb: i32,
    pub prev_pic_order_cnt_lsb: i32,

    // POC Type 1 & 2 state
    pub prev_frame_num: i32,
    pub prev_frame_num_offset: i32,
}

impl PocState {
    pub fn new() -> Self {
        Self {
            prev_pic_order_cnt_msb: 0,
            prev_pic_order_cnt_lsb: 0,
            prev_frame_num: 0,
            prev_frame_num_offset: 0,
        }
    }

    pub fn calculate_poc(&mut self, slice: &Slice, disposition: ReferenceDisposition) -> i32 {
        match slice.sps.pic_order_cnt_type {
            0 => self.calculate_poc_type0(slice, disposition),
            1 => self.calculate_poc_type1(slice, disposition),
            2 => self.calculate_poc_type2(slice, disposition),
            _ => 0,
        }
    }

    fn calculate_poc_type1(&mut self, slice: &Slice, disposition: ReferenceDisposition) -> i32 {
        // Section 8.2.1.2 Decoding process for picture order count type 1
        let (prev_frame_num_offset, prev_frame_num) = if disposition == ReferenceDisposition::Idr {
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

        let abs_frame_num = if slice.header.pic_parameter_set_id == 0 && abs_frame_num > 0 { // nal_ref_idc == 0 check is tricky here without NAL header
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

            let expected_delta_per_pic_order_cnt_cycle: i32 = slice.sps.offset_for_ref_frame.iter().sum();

            let mut expected_poc = pic_order_cnt_cycle_cnt * expected_delta_per_pic_order_cnt_cycle;
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

    fn calculate_poc_type0(&mut self, slice: &Slice, disposition: ReferenceDisposition) -> i32 {
        // Section 8.2.1.1 Decoding process for picture order count type 0
        let (prev_msb, prev_lsb) = if disposition == ReferenceDisposition::Idr {
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

    fn calculate_poc_type2(&mut self, slice: &Slice, disposition: ReferenceDisposition) -> i32 {
        // Section 8.2.1.3 Decoding process for picture order count type 2
        let (frame_num_offset, temp_pic_order_cnt) = if disposition == ReferenceDisposition::Idr {
            (0, 0)
        } else {
            let prev_frame_num = self.prev_frame_num;
            let prev_frame_num_offset = self.prev_frame_num_offset;
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
