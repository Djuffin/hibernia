use std::cmp::{max, min, Ordering};
use std::collections::HashMap;
use std::io::Read;

use crate::h264::dpb::{DpbMarking, ReferenceDisposition};
use crate::h264::slice::{RefPicListModification, SliceType};
use crate::h264::tables::mb_type_to_16x16_pred_mode;
use crate::h264::ColorPlane;

use super::dpb::{DecodedPictureBuffer, DpbPicture};
use super::inter_pred::{interpolate_chroma, interpolate_luma, InterpolationBuffer};
use super::macroblock::{
    self, get_4x4chroma_block_location, get_4x4chroma_block_neighbor, get_4x4luma_block_location,
    get_4x4luma_block_neighbor, IMb, Intra_16x16_SamplePredMode, Intra_4x4_SamplePredMode,
    Intra_Chroma_Pred_Mode, Macroblock, MbAddr, MbNeighborName, MbPredictionMode, MotionVector,
    PMb, PartitionInfo,
};
use super::residual::{level_scale_4x4_block, transform_4x4, unzip_block_4x4, Block4x4};
use super::tables::{MB_HEIGHT, MB_WIDTH};
use super::{nal, parser, pps, slice, sps, tables, ChromaFormat, Point};
use log::info;
use slice::Slice;
use v_frame::frame;
use v_frame::plane::{self, PlaneMutSlice, PlaneOffset};

pub type VideoFrame = frame::Frame<u8>;
type Plane = v_frame::plane::Plane<u8>;

#[derive(Clone, Debug)]
pub struct Picture {
    pub frame: VideoFrame,
    pub frame_num: u16,
    pub pic_order_cnt: i32,
}

#[derive(Debug, Clone)]
pub enum DecodingError {
    MisformedData(String),
    OutOfRange(String),
    Wtf,
}

#[derive(Clone, Debug, Default)]
pub struct DecoderContext {
    sps: HashMap<u8, sps::SequenceParameterSet>,
    pps: HashMap<u8, pps::PicParameterSet>,
}

impl DecoderContext {
    pub fn get_sps(&self, id: u8) -> Option<&sps::SequenceParameterSet> {
        self.sps.get(&id)
    }

    pub fn put_sps(&mut self, sps: sps::SequenceParameterSet) {
        self.sps.insert(sps.seq_parameter_set_id, sps);
    }

    pub fn get_pps(&self, id: u8) -> Option<&pps::PicParameterSet> {
        self.pps.get(&id)
    }

    pub fn put_pps(&mut self, pps: pps::PicParameterSet) {
        self.pps.insert(pps.pic_parameter_set_id, pps);
    }
}

pub struct Decoder {
    context: DecoderContext,
    dpb: DecodedPictureBuffer,
    output_frames: Vec<VideoFrame>,
    interpolation_buffer: InterpolationBuffer,

    // POC Type 0 state
    prev_pic_order_cnt_msb: i32,
    prev_pic_order_cnt_lsb: i32,
}

impl std::fmt::Debug for Decoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Decoder")
            .field("context", &self.context)
            .field("dpb", &self.dpb)
            .field("output_frames", &self.output_frames)
            .field("prev_pic_order_cnt_msb", &self.prev_pic_order_cnt_msb)
            .field("prev_pic_order_cnt_lsb", &self.prev_pic_order_cnt_lsb)
            .finish()
    }
}

impl Default for Decoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Decoder {
    pub fn new() -> Decoder {
        Decoder {
            context: DecoderContext::default(),
            dpb: DecodedPictureBuffer::new(1),
            output_frames: Vec::new(),
            interpolation_buffer: InterpolationBuffer::new(),
            prev_pic_order_cnt_msb: 0,
            prev_pic_order_cnt_lsb: 0,
        }
    }

    pub fn decode(&mut self, data: &[u8]) -> Result<(), DecodingError> {
        use nal::NalUnitType;
        let mut input = parser::BitReader::new(data);
        let parse_error_handler = DecodingError::MisformedData;
        loop {
            if input.remaining() < 4 * 8 {
                info!("End of data");
                break;
            }
            info!("---------------------------------------------------");
            let nal = parser::parse_nal_header(&mut input).map_err(parse_error_handler)?;
            assert!(input.is_aligned());
            info!("NAL {:?}", nal);
            let cur_byte_index = (input.position() / 8) as usize;
            let nal_size_bytes =
                if let Some(bytes) = parser::count_bytes_till_start_code(&data[cur_byte_index..]) {
                    bytes
                } else {
                    data.len() - cur_byte_index
                };
            let nal_buffer = &data[cur_byte_index..cur_byte_index + nal_size_bytes];
            let nal_cow = parser::remove_emulation_if_needed(nal_buffer);
            let mut unit_input = parser::BitReader::new(&nal_cow);
            input.skip((nal_size_bytes * 8) as u32).map_err(parse_error_handler)?;

            match nal.nal_unit_type {
                NalUnitType::Unspecified => {}
                NalUnitType::SliceDataA => {}
                NalUnitType::SliceDataB => {}
                NalUnitType::SliceDataC => {}
                NalUnitType::IDRSlice | NalUnitType::NonIDRSlice => {
                    let mut slice =
                        parser::parse_slice_header(&self.context, &nal, &mut unit_input)
                            .map_err(parse_error_handler)?;

                    info!("{:?} {:#?}", nal.nal_unit_type, slice);
                    let frame = VideoFrame::new_with_padding(
                        slice.sps.pic_width(),
                        slice.sps.pic_hight(),
                        v_frame::pixel::ChromaSampling::Cs420,
                        16,
                    );

                    let disposition = if nal.nal_unit_type == NalUnitType::IDRSlice {
                        ReferenceDisposition::Idr
                    } else if nal.nal_ref_idc != 0 {
                        ReferenceDisposition::NonIdrReference
                    } else {
                        ReferenceDisposition::NonReference
                    };

                    let pic_order_cnt = self.calculate_poc(&slice, disposition);

                    let pic = Picture { frame, frame_num: slice.header.frame_num, pic_order_cnt };
                    let dpb_pic = DpbPicture {
                        picture: pic,
                        marking: if nal.nal_ref_idc != 0 {
                            super::dpb::DpbMarking::UsedForShortTermReference
                        } else {
                            super::dpb::DpbMarking::UnusedForReference
                        },
                        is_idr: nal.nal_unit_type == NalUnitType::IDRSlice,
                        structure: super::dpb::DpbPictureStructure::Frame,
                        needed_for_output: true,
                    };
                    let pictures = self.dpb.store_picture(dpb_pic);
                    self.output_frames.extend(pictures.into_iter().map(|p| p.frame));

                    parser::parse_slice_data(&mut unit_input, &mut slice)
                        .map_err(parse_error_handler)?;
                    info!("Blocks: {:#?}", slice.get_macroblock_count());
                    self.process_slice(&mut slice)?;
                    self.dpb.mark_references(&slice.header, disposition, &slice.sps);
                }
                NalUnitType::SupplementalEnhancementInfo => {}
                NalUnitType::SeqParameterSet => {
                    let sps = parser::parse_sps(&mut unit_input).map_err(parse_error_handler)?;
                    info!("SPS: {:#?}", sps);
                    assert_eq!(sps.ChromaArrayType(), ChromaFormat::YUV420);
                    self.context.put_sps(sps);
                }
                NalUnitType::PicParameterSet => {
                    let pps = parser::parse_pps(&mut unit_input).map_err(parse_error_handler)?;
                    info!("PPS: {:#?}", pps);
                    self.context.put_pps(pps);
                }
                NalUnitType::AccessUnitDelimiter => {}
                NalUnitType::EndOfSeq => {}
                NalUnitType::EndOfStream => {}
                NalUnitType::FillerData => {}
                NalUnitType::SeqParameterSetExtension => {}
                NalUnitType::Prefix => {}
                NalUnitType::SubsetSeqParameterSet => {}
                NalUnitType::DepthParameterSet => {}
                NalUnitType::CodedSliceAux => {}
                NalUnitType::CodedSliceExtension => {}
                NalUnitType::CodedSliceExtensionForDepthView => {}
                NalUnitType::Reserved => {}
            }
        }
        Ok(())
    }

    fn calculate_poc(&mut self, slice: &Slice, disposition: ReferenceDisposition) -> i32 {
        match slice.sps.pic_order_cnt_type {
            0 => self.calculate_poc_type0(slice, disposition),
            1 => 0, // TODO: Implement Type 1
            2 => 0, // TODO: Implement Type 2
            _ => 0,
        }
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

    pub fn get_frame_buffer(&self) -> &[VideoFrame] {
        self.output_frames.as_slice()
    }

    pub fn clear_frame_buffer(&mut self) {
        self.output_frames.clear();
    }

    fn process_pcm_mb(
        frame: &mut VideoFrame,
        mb_loc: Point,
        block: &macroblock::PcmMb,
    ) {
        let y_plane = &mut frame.planes[0];
        let mut plane_slice = y_plane.mut_slice(point_to_plain_offset(mb_loc));

        for (idx, row) in plane_slice.rows_iter_mut().take(tables::MB_HEIGHT).enumerate() {
            let row_range = idx * tables::MB_WIDTH..(idx + 1) * tables::MB_WIDTH;
            row[..tables::MB_WIDTH].copy_from_slice(&block.pcm_sample_luma[row_range]);
        }
    }

    fn process_i_mb(
        slice: &Slice,
        mb_addr: u32,
        mb_loc: Point,
        frame: &mut VideoFrame,
        imb: &IMb,
        qp: &mut i32,
    ) -> Result<(), DecodingError> {
        *qp = (*qp + imb.mb_qp_delta).clamp(0, 51);
        let qp_val: u8 = (*qp).try_into().unwrap();
        let residuals = if let Some(residual) = imb.residual.as_ref() {
            residual.restore(ColorPlane::Y, qp_val)
        } else {
            Vec::new()
        };

        let luma_plane = &mut frame.planes[0];
        let luma_prediction_mode = imb.MbPartPredMode(0);
        info!(
            "MB {mb_addr} {qp} Luma: {:?} Chroma: {:?}",
            luma_prediction_mode, imb.intra_chroma_pred_mode
        );
        match luma_prediction_mode {
            MbPredictionMode::None => panic!("impossible pred mode"),
            MbPredictionMode::Intra_4x4 => {
                render_luma_4x4_intra_prediction(
                    slice, mb_addr, imb, mb_loc, luma_plane, &residuals,
                );
            }
            MbPredictionMode::Intra_8x8 => return Err(DecodingError::MisformedData("8x8 pred mode not supported".to_string())),
            MbPredictionMode::Intra_16x16 => {
                render_luma_16x16_intra_prediction(
                    slice,
                    mb_addr,
                    mb_loc,
                    luma_plane,
                    mb_type_to_16x16_pred_mode(imb.mb_type).unwrap(),
                    &residuals,
                );
            }
            MbPredictionMode::Pred_L0 => return Err(DecodingError::MisformedData("Pred_L0 in I_MB".to_string())),
            MbPredictionMode::Pred_L1 => return Err(DecodingError::MisformedData("Pred_L1 in I_MB".to_string())),
        }

        for plane_name in [ColorPlane::Cb, ColorPlane::Cr] {
            let chroma_qp =
                get_chroma_qp(*qp, slice.pps.chroma_qp_index_offset, 0)
                    .try_into()
                    .unwrap();
            let chroma_plane = &mut frame.planes[plane_name as usize];
            let residuals = if let Some(residual) = imb.residual.as_ref() {
                residual.restore(plane_name, chroma_qp)
            } else {
                Vec::new()
            };
            render_chroma_intra_prediction(
                slice,
                mb_addr,
                mb_loc,
                chroma_plane,
                imb.intra_chroma_pred_mode,
                &residuals,
            )
        }
        Ok(())
    }

    fn process_p_mb(
        slice: &Slice,
        mb_loc: Point,
        frame: &mut VideoFrame,
        block: &PMb,
        references: &[DpbPicture],
        qp: &mut i32,
        interpolation_buffer: &mut InterpolationBuffer,
    ) {
        *qp = (*qp + block.mb_qp_delta).clamp(0, 51);
        let qp_val: u8 = (*qp).try_into().unwrap();
        let residuals = if let Some(residual) = block.residual.as_ref() {
            residual.restore(ColorPlane::Y, qp_val)
        } else {
            Vec::new()
        };

        render_luma_inter_prediction(
            slice,
            block,
            mb_loc,
            frame,
            &residuals,
            references,
            interpolation_buffer,
        );

        for plane_name in [ColorPlane::Cb, ColorPlane::Cr] {
            let chroma_qp =
                get_chroma_qp(*qp, slice.pps.chroma_qp_index_offset, 0)
                    .try_into()
                    .unwrap();
            let residuals = if let Some(residual) = block.residual.as_ref() {
                residual.restore(plane_name, chroma_qp)
            } else {
                Vec::new()
            };
            render_chroma_inter_prediction(
                slice, block, mb_loc, plane_name, frame, &residuals,
                references,
            );
        }
    }

    fn process_slice(&mut self, slice: &mut Slice) -> Result<(), DecodingError> {
        if self.dpb.pictures.is_empty() {
            return Err(DecodingError::Wtf);
        }

        self.construct_ref_pic_list0(slice)?;

        let mut qp = slice.pps.pic_init_qp_minus26 + 26 + slice.header.slice_qp_delta;
        let split_idx = self.dpb.pictures.len() - 1;
        let (references, current) = self.dpb.pictures.split_at_mut(split_idx);
        let frame = &mut current[0].picture.frame;
        for mb_addr in 0..(slice.sps.pic_size_in_mbs() as u32) {
            let mb_loc = slice.get_mb_location(mb_addr);
            if let Some(mb) = slice.get_mb(mb_addr) {
                match mb {
                    Macroblock::PCM(block) => Self::process_pcm_mb(frame, mb_loc, block),
                    Macroblock::I(imb) => {
                        Self::process_i_mb(slice, mb_addr, mb_loc, frame, imb, &mut qp)?
                    }
                    Macroblock::P(block) => Self::process_p_mb(
                        slice,
                        mb_loc,
                        frame,
                        block,
                        references,
                        &mut qp,
                        &mut self.interpolation_buffer,
                    ),
                }
            }
        }
        Ok(())
    }

    // Section 8.2.4.1 Decoding process for picture numbers
    fn construct_ref_pic_list0(&self, slice: &mut Slice) -> Result<(), DecodingError> {
        // We only support P slices for now (and I/B later).
        // If I slice, list is empty? Spec says "invoked when decoding P, SP or B slice".
        if slice.header.slice_type == SliceType::I || slice.header.slice_type == SliceType::SI {
            slice.ref_pic_list0.clear();
            return Ok(());
        }

        let mut ref_list0 = self.initialize_ref_pic_list0(slice);
        self.modify_ref_pic_list0(slice, &mut ref_list0);

        // Truncate to num_ref_idx_l0_active_minus1 + 1
        let len = (slice.header.num_ref_idx_l0_active_minus1 + 1) as usize;
        if ref_list0.len() > len {
            ref_list0.truncate(len);
        }

        slice.ref_pic_list0 = ref_list0;
        Ok(())
    }

    // Section 8.2.4.2 Initialization process for reference picture lists
    fn initialize_ref_pic_list0(&self, slice: &Slice) -> Vec<usize> {
        let max_frame_num = 1 << (slice.sps.log2_max_frame_num_minus4 + 4);
        let curr_frame_num = slice.header.frame_num as i32;

        // Collect valid reference pictures
        let mut short_term_refs = Vec::new();
        let mut long_term_refs = Vec::new();

        // We use indices into the DPB
        // The last picture in the DPB is the current picture being decoded,
        // so we exclude it from the reference list.
        let candidates_count = self.dpb.pictures.len().saturating_sub(1);
        for (i, pic) in self.dpb.pictures.iter().enumerate().take(candidates_count) {
            match pic.marking {
                DpbMarking::UsedForShortTermReference => {
                    let frame_num = pic.picture.frame_num as i32;
                    let pic_num = if frame_num > curr_frame_num {
                        frame_num - max_frame_num
                    } else {
                        frame_num
                    };
                    short_term_refs.push((i, pic_num));
                }
                DpbMarking::UsedForLongTermReference(lt_idx) => {
                    long_term_refs.push((i, lt_idx));
                }
                _ => {}
            }
        }

        // Sort
        short_term_refs.sort_by_key(|k| std::cmp::Reverse(k.1)); // Descending PicNum
        long_term_refs.sort_by_key(|k| k.1); // Ascending LongTermPicNum

        // Initial List 0
        short_term_refs.iter().map(|x| x.0).chain(long_term_refs.iter().map(|x| x.0)).collect()
    }

    // Section 8.2.4.3 Reordering process for reference picture lists
    fn modify_ref_pic_list0(&self, slice: &Slice, ref_list0: &mut Vec<usize>) {
        if slice.header.ref_pic_list_modification.list0.is_empty() {
            return;
        }

        let max_frame_num = 1 << (slice.sps.log2_max_frame_num_minus4 + 4);
        let curr_frame_num = slice.header.frame_num as i32;
        let mut pic_num_lx_pred = curr_frame_num;
        let mut ref_idx_l0 = 0;

        for modification in &slice.header.ref_pic_list_modification.list0 {
            match modification {
                RefPicListModification::RemapShortTermNegative(abs_diff_minus1) => {
                    let abs_diff = (abs_diff_minus1 + 1) as i32;
                    let pic_num_lx_no_wrap = pic_num_lx_pred - abs_diff;
                    let pic_num_lx = if pic_num_lx_no_wrap < 0 {
                        pic_num_lx_no_wrap + max_frame_num
                    } else {
                        pic_num_lx_no_wrap
                    };
                    pic_num_lx_pred = pic_num_lx;

                    if let Some(idx) =
                        self.find_short_term_in_dpb(pic_num_lx, curr_frame_num, max_frame_num)
                    {
                        self.place_picture_in_list(ref_list0, idx, ref_idx_l0);
                        ref_idx_l0 += 1;
                    }
                }
                RefPicListModification::RemapShortTermPositive(abs_diff_minus1) => {
                    let abs_diff = (abs_diff_minus1 + 1) as i32;
                    let pic_num_lx_no_wrap = pic_num_lx_pred + abs_diff;
                    let pic_num_lx = if pic_num_lx_no_wrap >= max_frame_num {
                        pic_num_lx_no_wrap - max_frame_num
                    } else {
                        pic_num_lx_no_wrap
                    };
                    pic_num_lx_pred = pic_num_lx;

                    if let Some(idx) =
                        self.find_short_term_in_dpb(pic_num_lx, curr_frame_num, max_frame_num)
                    {
                        self.place_picture_in_list(ref_list0, idx, ref_idx_l0);
                        ref_idx_l0 += 1;
                    }
                }
                RefPicListModification::RemapLongTerm(long_term_pic_num) => {
                    if let Some(idx) = self.find_long_term_in_dpb(*long_term_pic_num) {
                        self.place_picture_in_list(ref_list0, idx, ref_idx_l0);
                        ref_idx_l0 += 1;
                    }
                }
            }
        }
    }

    fn place_picture_in_list(&self, list: &mut Vec<usize>, pic_idx: usize, ref_idx: usize) {
        if ref_idx < list.len() {
            list.insert(ref_idx, pic_idx);

            let mut i = ref_idx + 1;
            while i < list.len() {
                if list[i] == pic_idx {
                    list.remove(i);
                } else {
                    i += 1;
                }
            }
        } else {
            // If ref_idx is out of bounds, we just push it? Spec implies refIdxL0 starts at 0 and increments.
            // So it should usually be valid for insertion (possibly at end).
            list.push(pic_idx);
        }
    }

    // Find short term picture in DPB and return its index.
    fn find_short_term_in_dpb(
        &self,
        pic_num: i32,
        curr_frame_num: i32,
        max_frame_num: i32,
    ) -> Option<usize> {
        self.dpb.pictures.iter().position(|pic| {
            if pic.marking.is_short_term() {
                let frame_num = pic.picture.frame_num as i32;
                let pn =
                    if frame_num > curr_frame_num { frame_num - max_frame_num } else { frame_num };
                pn == pic_num
            } else {
                false
            }
        })
    }

    // Find long term picture in DPB and return its index.
    fn find_long_term_in_dpb(&self, long_term_pic_num: u32) -> Option<usize> {
        self.dpb.pictures.iter().position(|pic| {
            if let DpbMarking::UsedForLongTermReference(idx) = pic.marking {
                idx == long_term_pic_num
            } else {
                false
            }
        })
    }
}

#[inline]
fn point_to_plain_offset(p: Point) -> PlaneOffset {
    PlaneOffset { x: p.x as isize, y: p.y as isize }
}

#[derive(Default)]
struct Surroundings4x4 {
    pub offset: PlaneOffset,
    pub top_row: [u8; 9],
    pub left_column: [u8; 5],
}

impl Surroundings4x4 {
    pub fn load(&mut self, plane: &Plane, blk_loc: Point, substitute_right: bool) {
        let mut offset = point_to_plain_offset(blk_loc);
        offset.x -= 1;
        offset.y -= 1;
        let mut target_slice = plane.slice(offset);

        if offset.y > 0 {
            if substitute_right {
                // Section 8.3.1.2 Intra_4x4 sample prediction
                // When samples p[ x, −1 ], with x = 4..7, are marked as "not available" ...
                self.top_row[0..5].copy_from_slice(&target_slice[0][0..5]);
                let filler = self.top_row[4];
                self.top_row[5..9].fill(filler);
            } else {
                self.top_row.copy_from_slice(&target_slice[0][0..9]);
            }
        } else {
            self.top_row.fill(0);
        }

        self.left_column[0] = self.top_row[0];
        offset.y += 1;
        target_slice = plane.slice(offset);
        if offset.x > 0 {
            for (idx, v) in target_slice.rows_iter().take(4).enumerate() {
                self.left_column[idx + 1] = v[0];
            }
        } else {
            self.left_column[1..5].fill(0);
        }

        offset.x += 1;
        self.offset = offset;
    }

    #[inline]
    pub fn top4(&self) -> &[u8] {
        &self.top_row[1..5]
    }

    #[inline]
    pub fn left4(&self) -> &[u8] {
        &self.left_column[1..5]
    }
}

pub fn render_luma_inter_prediction(
    slice: &Slice,
    mb: &PMb,
    mb_loc: Point,
    frame: &mut VideoFrame,
    residuals: &[Block4x4],
    references: &[DpbPicture],
    buffer: &mut InterpolationBuffer,
) {
    let y_plane = &mut frame.planes[0];

    for raster_idx in 0..16 {
        let (grid_x, grid_y) = (raster_idx % 4, raster_idx / 4);
        let partition = mb.motion.partitions[grid_y as usize][grid_x as usize];

        let ref_idx = partition.ref_idx_l0;
        let mv = partition.mv_l0;

        if let Some(&dpb_idx) = slice.ref_pic_list0.get(ref_idx as usize) {
            if let Some(ref_pic) = references.get(dpb_idx) {
                let ref_plane = &ref_pic.picture.frame.planes[0];

                let blk_x = grid_x * 4;
                let blk_y = grid_y * 4;

                let mut dst = [0u8; 16]; // 4x4 block

                interpolate_luma(
                    ref_plane,
                    mb_loc.x,
                    mb_loc.y,
                    blk_x as u8,
                    blk_y as u8,
                    4,
                    4,
                    mv,
                    &mut dst,
                    4, // stride for 4x4 block buffer
                    buffer,
                );

                // Add residual
                let blk_idx = macroblock::get_4x4luma_block_index(Point {
                    x: blk_x as u32,
                    y: blk_y as u32,
                });
                if let Some(residual_blk) = residuals.get(blk_idx as usize) {
                    for y in 0..4 {
                        for x in 0..4 {
                            let res = residual_blk.samples[y][x];
                            let pred = dst[y * 4 + x] as i32;
                            dst[y * 4 + x] = (pred + res).clamp(0, 255) as u8;
                        }
                    }
                }

                // Copy to frame
                let mut plane_slice = y_plane.mut_slice(PlaneOffset {
                    x: (mb_loc.x + blk_x as u32) as isize,
                    y: (mb_loc.y + blk_y as u32) as isize,
                });

                for (y, row) in plane_slice.rows_iter_mut().take(4).enumerate() {
                    let row_data = &dst[y * 4..(y + 1) * 4];
                    row[..4].copy_from_slice(row_data);
                }
            }
        }
    }
}

pub fn render_chroma_inter_prediction(
    slice: &Slice,
    mb: &PMb,
    mb_loc: Point,
    plane: ColorPlane,
    frame: &mut VideoFrame,
    residuals: &[Block4x4],
    references: &[DpbPicture],
) {
    let chroma_plane = &mut frame.planes[plane as usize];
    let mb_x_chroma = mb_loc.x >> 1;
    let mb_y_chroma = mb_loc.y >> 1;

    // 1. Prediction (Block by block 2x2)
    for blk_idx in 0..16 {
        let (grid_x, grid_y) = (blk_idx % 4, blk_idx / 4);
        let partition = mb.motion.partitions[grid_y as usize][grid_x as usize];
        let ref_idx = partition.ref_idx_l0;
        let mv = partition.mv_l0;

        if let Some(&dpb_idx) = slice.ref_pic_list0.get(ref_idx as usize) {
            if let Some(ref_pic) = references.get(dpb_idx) {
                let ref_plane = &ref_pic.picture.frame.planes[plane as usize];

                let blk_x = (grid_x * 4) >> 1; // 2
                let blk_y = (grid_y * 4) >> 1; // 2

                let mut dst = [0u8; 4]; // 2x2 = 4 pixels

                interpolate_chroma(
                    ref_plane,
                    mb_x_chroma,
                    mb_y_chroma,
                    blk_x as u8,
                    blk_y as u8,
                    2,
                    2,
                    mv,
                    &mut dst,
                    2, // stride
                );

                // Write to frame
                let mut plane_slice = chroma_plane.mut_slice(PlaneOffset {
                    x: (mb_x_chroma + blk_x as u32) as isize,
                    y: (mb_y_chroma + blk_y as u32) as isize,
                });

                for (y, row) in plane_slice.rows_iter_mut().take(2).enumerate() {
                    row[0] = dst[y * 2];
                    row[1] = dst[y * 2 + 1];
                }
            }
        }
    }

    // 2. Residuals (Block by block 4x4)
    for (blk_idx, residual_blk) in residuals.iter().enumerate() {
        let blk_loc = get_4x4chroma_block_location(blk_idx as u8);
        // blk_loc is relative to MB top-left in chroma samples

        let mut plane_slice = chroma_plane.mut_slice(PlaneOffset {
            x: (mb_x_chroma + blk_loc.x) as isize,
            y: (mb_y_chroma + blk_loc.y) as isize,
        });

        for (y, row) in plane_slice.rows_iter_mut().take(4).enumerate() {
            for x in 0..4 {
                let res = residual_blk.samples[y][x];
                let pred = row[x] as i32;
                row[x] = (pred + res).clamp(0, 255) as u8;
            }
        }
    }
}

// Section 8.5.8 Derivation process for chroma quantization parameters
pub fn get_chroma_qp(luma_qp: i32, chroma_qp_offset: i32, qp_bd_offset_c: i32) -> i32 {
    let qp_i = (luma_qp + chroma_qp_offset).clamp(-qp_bd_offset_c, 51);

    // 2. Look up qP_C from qP_I using Table 8-15
    let qp_c = match qp_i {
        i if i < 30 => i,
        30 => 29,
        31 => 30,
        32 => 31,
        33 => 32,
        34 => 32,
        35 => 33,
        36 => 34,
        37 => 34,
        38 => 35,
        39 => 35,
        40 => 36,
        41 => 36,
        42 => 37,
        43 => 37,
        44 => 38,
        45 => 38,
        46 => 38,
        47 => 39,
        48 => 39,
        49 => 39,
        50 => 39,
        51 => 39,
        _ => unreachable!(),
    };

    qp_c + qp_bd_offset_c
}

#[inline]
fn weighted_avg(double: u8, single_a: u8, single_b: u8) -> u8 {
    ((2 * (double as u16) + (single_a as u16) + (single_b as u16) + 2) >> 2) as u8
}

#[inline]
fn avg(a: u8, b: u8) -> u8 {
    (((a as u16) + (b as u16) + 1) >> 1) as u8
}

fn pred_luma_4x4_vertical(ctx: &Surroundings4x4, target_slice: &mut PlaneMutSlice<u8>) {
    let src = ctx.top4();
    for row in target_slice.rows_iter_mut().take(4) {
        row[0..4].copy_from_slice(src);
    }
}

fn pred_luma_4x4_horizontal(ctx: &Surroundings4x4, target_slice: &mut PlaneMutSlice<u8>) {
    let src = ctx.left4();
    for (idx, row) in target_slice.rows_iter_mut().take(4).enumerate() {
        row[0..4].fill(src[idx]);
    }
}

fn pred_luma_4x4_dc(
    slice: &Slice,
    mb_addr: MbAddr,
    blk_idx: u8,
    ctx: &Surroundings4x4,
    target_slice: &mut PlaneMutSlice<u8>,
) {
    let same_mb = get_4x4luma_block_neighbor(blk_idx, MbNeighborName::A).1.is_none();
    let sum_a = if same_mb || slice.has_mb_neighbor(mb_addr, MbNeighborName::A) {
        Some(ctx.left4().iter().map(|v| *v as u32).sum::<u32>())
    } else {
        None
    };

    let same_mb = get_4x4luma_block_neighbor(blk_idx, MbNeighborName::B).1.is_none();
    let sum_b = if same_mb || slice.has_mb_neighbor(mb_addr, MbNeighborName::B) {
        Some(ctx.top4().iter().map(|v| *v as u32).sum::<u32>())
    } else {
        None
    };

    let mut sum = sum_a.unwrap_or(0) + sum_b.unwrap_or(0);
    if sum_a.is_some() && sum_b.is_some() {
        sum = (sum + 4) >> 3;
    } else if sum_a.is_some() != sum_b.is_some() {
        sum = (sum + 2) >> 2;
    } else {
        sum = 1 << 7;
    }

    for row in target_slice.rows_iter_mut().take(4) {
        row[0..4].fill(sum as u8);
    }
}

fn pred_luma_4x4_diagonal_down_left(ctx: &Surroundings4x4, target_slice: &mut PlaneMutSlice<u8>) {
    let top_row = &ctx.top_row[1..=8];
    for (y, row) in target_slice.rows_iter_mut().take(4).enumerate() {
        for (x, value) in row.iter_mut().take(4).enumerate() {
            let i = x + y;
            *value = if i == 6 {
                weighted_avg(top_row[7], top_row[7], top_row[6])
            } else {
                weighted_avg(top_row[i + 1], top_row[i], top_row[i + 2])
            };
        }
    }
}

fn pred_luma_4x4_diagonal_down_right(ctx: &Surroundings4x4, target_slice: &mut PlaneMutSlice<u8>) {
    let top = &ctx.top_row;
    let left = &ctx.left_column;
    for (y, row) in target_slice.rows_iter_mut().take(4).enumerate() {
        for (x, value) in row.iter_mut().take(4).enumerate() {
            *value = match x.cmp(&y) {
                Ordering::Greater => {
                    let i = 1 + x - y;
                    weighted_avg(top[i - 1], top[i - 2], top[i])
                }
                Ordering::Less => {
                    let i = 1 + y - x;
                    weighted_avg(left[i - 1], left[i - 2], left[i])
                }
                Ordering::Equal => weighted_avg(top[0], top[1], left[1]),
            }
        }
    }
}

fn pred_luma_4x4_vertical_right(ctx: &Surroundings4x4, target_slice: &mut PlaneMutSlice<u8>) {
    let left = &ctx.left_column;
    let top = &ctx.top_row;
    for (y, row) in target_slice.rows_iter_mut().take(4).enumerate() {
        for (x, value) in row.iter_mut().take(4).enumerate() {
            let z = 2 * (x as isize) - (y as isize);
            let i = 1 + x - (y >> 1);
            *value = match z {
                0 | 2 | 4 | 6 => avg(top[i], top[i - 1]),
                1 | 3 | 5 => weighted_avg(top[i - 1], top[i - 2], top[i]),
                -1 => weighted_avg(top[0], top[1], left[1]),
                _ => {
                    let y = y + 1;
                    weighted_avg(left[y - 2], left[y - 1], left[y - 3])
                }
            };
        }
    }
}

fn pred_luma_4x4_horizontal_down(ctx: &Surroundings4x4, target_slice: &mut PlaneMutSlice<u8>) {
    let left = &ctx.left_column;
    let top = &ctx.top_row;
    for (y, row) in target_slice.rows_iter_mut().take(4).enumerate() {
        for (x, value) in row.iter_mut().take(4).enumerate() {
            let z = 2 * (y as isize) - (x as isize);
            let i = 1 + y - (x >> 1);
            *value = match z {
                0 | 2 | 4 | 6 => avg(left[i - 1], left[i]),
                1 | 3 | 5 => weighted_avg(left[i - 1], left[i - 2], left[i]),
                -1 => weighted_avg(top[0], left[1], top[1]),
                _ => weighted_avg(top[x - 1], top[x], top[x - 2]),
            };
        }
    }
}

fn pred_luma_4x4_vertical_left(ctx: &Surroundings4x4, target_slice: &mut PlaneMutSlice<u8>) {
    let top_row = &ctx.top_row[1..];
    for (y, row) in target_slice.rows_iter_mut().take(4).enumerate() {
        for (x, value) in row.iter_mut().take(4).enumerate() {
            let i = x + (y >> 1);
            *value = if y % 2 == 0 {
                avg(top_row[i], top_row[i + 1])
            } else {
                weighted_avg(top_row[i + 1], top_row[i + 2], top_row[i])
            };
        }
    }
}

fn pred_luma_4x4_horizontal_up(ctx: &Surroundings4x4, target_slice: &mut PlaneMutSlice<u8>) {
    let left = ctx.left4();
    for (y, row) in target_slice.rows_iter_mut().take(4).enumerate() {
        for (x, value) in row.iter_mut().take(4).enumerate() {
            let z = x + 2 * y;
            let i = y + (x >> 1);
            *value = match z {
                0 | 2 | 4 => avg(left[i], left[i + 1]),
                1 | 3 => weighted_avg(left[i + 1], left[i], left[i + 2]),
                5 => weighted_avg(left[3], left[3], left[2]),
                _ => left[3],
            };
        }
    }
}

// Section 8.3.1.1 Derivation process for Intra4x4PredMode
pub fn render_luma_4x4_intra_prediction(
    slice: &Slice,
    mb_addr: MbAddr,
    mb: &IMb,
    mb_loc: Point,
    target: &mut Plane,
    residuals: &[Block4x4],
) {
    let mut ctx = Surroundings4x4::default();
    let has_c_mb_neighbor = slice.has_mb_neighbor(mb_addr, MbNeighborName::C);
    for blk_idx in 0..16 {
        let mut blk_loc = get_4x4luma_block_location(blk_idx);
        blk_loc.x += mb_loc.x;
        blk_loc.y += mb_loc.y;
        let substitute_right = match blk_idx {
            3 | 7 | 11 | 13 | 15 => true,
            5 => !has_c_mb_neighbor,
            _ => false,
        };
        ctx.load(target, blk_loc, substitute_right);
        let mut target_slice = target.mut_slice(ctx.offset);

        let mode = mb.rem_intra4x4_pred_mode[blk_idx as usize];
        match mode {
            Intra_4x4_SamplePredMode::Vertical => {
                pred_luma_4x4_vertical(&ctx, &mut target_slice);
            }
            Intra_4x4_SamplePredMode::Horizontal => {
                pred_luma_4x4_horizontal(&ctx, &mut target_slice);
            }
            Intra_4x4_SamplePredMode::DC => {
                pred_luma_4x4_dc(slice, mb_addr, blk_idx, &ctx, &mut target_slice);
            }
            Intra_4x4_SamplePredMode::Diagonal_Down_Left => {
                pred_luma_4x4_diagonal_down_left(&ctx, &mut target_slice);
            }
            Intra_4x4_SamplePredMode::Diagonal_Down_Right => {
                pred_luma_4x4_diagonal_down_right(&ctx, &mut target_slice);
            }
            Intra_4x4_SamplePredMode::Vertical_Right => {
                pred_luma_4x4_vertical_right(&ctx, &mut target_slice);
            }
            Intra_4x4_SamplePredMode::Horizontal_Down => {
                pred_luma_4x4_horizontal_down(&ctx, &mut target_slice);
            }
            Intra_4x4_SamplePredMode::Vertical_Left => {
                pred_luma_4x4_vertical_left(&ctx, &mut target_slice);
            }
            Intra_4x4_SamplePredMode::Horizontal_Up => {
                pred_luma_4x4_horizontal_up(&ctx, &mut target_slice);
            }
        }

        if let Some(residual) = residuals.get(blk_idx as usize) {
            for (y, row) in target_slice.rows_iter_mut().take(4).enumerate() {
                for (x, pixel) in row.iter_mut().take(4).enumerate() {
                    *pixel = (*pixel as i32 + residual.samples[y][x]).clamp(0, 255) as u8;
                }
            }
        }
    }
}

pub fn render_luma_16x16_intra_prediction(
    slice: &Slice,
    mb_addr: MbAddr,
    loc: Point,
    target: &mut Plane,
    mode: Intra_16x16_SamplePredMode,
    residuals: &[Block4x4],
) {
    let x = loc.x as usize;
    let y = loc.y as usize;
    let offset = point_to_plain_offset(loc);
    info!("luma 16x16 prediction: {mode:?}");
    match mode {
        Intra_16x16_SamplePredMode::Intra_16x16_Vertical => {
            // Section 8.3.3.1 Specification of Intra_16x16_Vertical prediction mode
            let mut src_row = [0; 16];
            src_row.copy_from_slice(&target.row(y as isize - 1)[x..(x + 16)]);
            let mut target_slice = target.mut_slice(offset);
            for row in target_slice.rows_iter_mut().take(16) {
                row[0..16].copy_from_slice(&src_row);
            }
        }
        Intra_16x16_SamplePredMode::Intra_16x16_Horizontal => {
            // Section 8.3.3.2 Specification of Intra_16x16_Horizontal prediction mode
            let mut target_slice = target.mut_slice(PlaneOffset { x: offset.x - 1, ..offset });
            for row in target_slice.rows_iter_mut().take(16) {
                let src = row[0];
                row[1..=16].fill(src);
            }
        }
        Intra_16x16_SamplePredMode::Intra_16x16_DC => {
            // Section 8.3.3.3 Specification of Intra_16x16_DC prediction mode
            let sum_a = if slice.has_mb_neighbor(mb_addr, MbNeighborName::A) {
                let target_slice = target.slice(PlaneOffset { x: offset.x - 1, ..offset });
                Some(target_slice.rows_iter().take(16).map(|r| r[0] as u32).sum::<u32>())
            } else {
                None
            };

            // Calculate the sum of all the values at the top of the current macroblock
            let sum_b = if slice.has_mb_neighbor(mb_addr, MbNeighborName::B) {
                let row = &target.row(y as isize - 1)[x..(x + 16)];
                Some(row.iter().map(|r| *r as u32).sum::<u32>())
            } else {
                None
            };

            let mut sum = sum_a.unwrap_or(0) + sum_b.unwrap_or(0);
            if sum_a.is_some() && sum_b.is_some() {
                sum = (sum + 16) >> 5;
            } else if sum_a.is_some() != sum_b.is_some() {
                sum = (sum + 8) >> 4;
            } else {
                sum = 1 << 7;
            }

            let mut target_slice = target.mut_slice(offset);
            for row in target_slice.rows_iter_mut().take(16) {
                row[0..16].fill(sum as u8);
            }
        }
        Intra_16x16_SamplePredMode::Intra_16x16_Plane => {
            // Section 8.3.3.4 Specification of Intra_16x16_Plane prediction mode
            let slice = target.slice(PlaneOffset { x: offset.x - 1, y: offset.y - 1 });
            let mut top = [0; 17];
            top.copy_from_slice(&slice[0][0..17]);
            let mut left = [0; 17];
            for (idx, row) in slice.rows_iter().take(17).enumerate() {
                left[idx] = row[0];
            }

            let mut h = 0i32;
            let mut v = 0i32;
            for i in 1..=8 {
                h += (i as i32) * (top[i + 8] as i32 - top[8 - i] as i32);
                v += (i as i32) * (left[i + 8] as i32 - left[8 - i] as i32);
            }
            let a = (top[16] as i32 + left[16] as i32) * 16;
            let b = (5 * h + 32) >> 6;
            let c = (5 * v + 32) >> 6;

            let mut target_slice = target.mut_slice(offset);
            for (y, row) in target_slice.rows_iter_mut().take(16).enumerate() {
                for (x, pixel) in row.iter_mut().take(16).enumerate() {
                    let x = x as i32;
                    let y = y as i32;
                    let value = (a + b * (x - 7) + c * (y - 7) + 16) >> 5;
                    *pixel = value.clamp(0, 255) as u8;
                }
            }
        }
    }

    for (blk_idx, blk) in residuals.iter().enumerate() {
        let mut blk_loc = get_4x4luma_block_location(blk_idx as u8);
        blk_loc.x += loc.x;
        blk_loc.y += loc.y;

        let mut plane_slice = target.mut_slice(point_to_plain_offset(blk_loc));
        for (y, row) in plane_slice.rows_iter_mut().take(4).enumerate() {
            for (x, pixel) in row.iter_mut().take(4).enumerate() {
                *pixel = (*pixel as i32 + blk.samples[y][x]).clamp(0, 255) as u8;
            }
        }
    }
}

pub fn render_chroma_intra_prediction(
    slice: &Slice,
    mb_addr: MbAddr,
    loc: Point,
    target: &mut Plane,
    mode: Intra_Chroma_Pred_Mode,
    residuals: &[Block4x4],
) {
    // Section 8.3.4 Intra prediction process for chroma samples
    let chroma_shift = slice.sps.ChromaArrayType().get_chroma_shift();
    let loc = Point { x: loc.x >> chroma_shift.width, y: loc.y >> chroma_shift.width };
    let mb_width = MB_WIDTH >> chroma_shift.width;
    let mb_height = MB_HEIGHT >> chroma_shift.height;
    let offset = point_to_plain_offset(loc);

    #[inline]
    fn sum(slice: &[u8]) -> u32 {
        slice.iter().map(|v| *v as u32).sum::<u32>()
    }

    match mode {
        Intra_Chroma_Pred_Mode::Vertical => {
            // Section 8.3.4.3 Specification of Intra_Chroma_Vertical prediction mode
            let x = loc.x as usize;
            let y = loc.y as usize;
            let mut src_row = [0; 16];
            src_row[0..mb_width].copy_from_slice(&target.row(y as isize - 1)[x..(x + mb_width)]);
            let mut target_slice = target.mut_slice(offset);
            for row in target_slice.rows_iter_mut().take(mb_height) {
                row[0..mb_width].copy_from_slice(&src_row[0..mb_width]);
            }
        }
        Intra_Chroma_Pred_Mode::Horizontal => {
            // Section 8.3.4.2 Specification of Intra_Chroma_Horizontal prediction mode
            let mut target_slice = target.mut_slice(PlaneOffset { x: offset.x - 1, ..offset });
            for row in target_slice.rows_iter_mut().take(mb_height) {
                let src = row[0];
                row[1..=mb_width].fill(src);
            }
        }
        Intra_Chroma_Pred_Mode::DC => {
            // Section 8.3.4.1 Specification of Intra_Chroma_DC prediction mode

            // Calculate the sum of all the values at the top of the current block
            let mut top_left = None;
            let mut top_right = None;
            if slice.has_mb_neighbor(mb_addr, MbNeighborName::B) {
                let target_slice = target.slice(PlaneOffset { y: offset.y - 1, ..offset });
                top_left = Some(sum(&target_slice[0][0..4]));
                top_right = Some(sum(&target_slice[0][4..8]));
            }

            // Calculate the sum of all the values at the left of the current block
            let mut left_top = None;
            let mut left_bottom = None;
            if slice.has_mb_neighbor(mb_addr, MbNeighborName::A) {
                let target_slice = target.slice(PlaneOffset { x: offset.x - 1, ..offset });
                let mut left_column = [0u8; 8];
                for (idx, row) in target_slice.rows_iter().take(8).enumerate() {
                    left_column[idx] = row[0];
                }
                left_top = Some(sum(&left_column[0..4]));
                left_bottom = Some(sum(&left_column[4..8]));
            }

            for blk_idx in 0..4 {
                const DEFAULT_VALUE: u32 = 1 << 7; // = 1 << ( BitDepthC − 1 )
                let result = match blk_idx {
                    0 => {
                        // If ( xO, yO ) is equal to ( 0, 0 ) or xO and yO are greater than 0
                        if let (Some(left), Some(top)) = (left_top, top_left) {
                            (left + top + 4) >> 3
                        } else if let Some(s) = top_left {
                            (s + 2) >> 2
                        } else if let Some(s) = left_top {
                            (s + 2) >> 2
                        } else {
                            DEFAULT_VALUE
                        }
                    }
                    1 => {
                        // If xO is greater than 0 and yO is equal to 0
                        if let Some(s) = top_right {
                            (s + 2) >> 2
                        } else if let Some(s) = left_top {
                            (s + 2) >> 2
                        } else {
                            DEFAULT_VALUE
                        }
                    }
                    2 => {
                        // If xO is equal to 0 and yO is greater than 0
                        if let Some(s) = left_bottom {
                            (s + 2) >> 2
                        } else if let Some(s) = top_left {
                            (s + 2) >> 2
                        } else {
                            DEFAULT_VALUE
                        }
                    }
                    3 => {
                        if let (Some(left), Some(top)) = (left_bottom, top_right) {
                            (left + top + 4) >> 3
                        } else if let Some(s) = top_right {
                            (s + 2) >> 2
                        } else if let Some(s) = left_bottom {
                            (s + 2) >> 2
                        } else {
                            DEFAULT_VALUE
                        }
                    }
                    _ => unreachable!(),
                };

                let mut blk_loc = get_4x4chroma_block_location(blk_idx);
                blk_loc.x += loc.x;
                blk_loc.y += loc.y;
                let mut target_slice = target.mut_slice(point_to_plain_offset(blk_loc));
                for row in target_slice.rows_iter_mut().take(4) {
                    row[0..4].fill(result as u8);
                }
            }
        }
        Intra_Chroma_Pred_Mode::Plane => {
            // Section 8.3.4.4 Specification of Intra_Chroma_Plane prediction mode
            // yCF = 0 and xCF = 0
            let target_slice = target.slice(PlaneOffset { x: offset.x - 1, y: offset.y - 1 });
            let mut h = 0;
            let mut top_row = [0u8; 9];
            top_row.copy_from_slice(&target_slice[0][0..9]);
            for x in 0..4usize {
                h += (x as isize + 1) * (top_row[4 + 1 + x] as isize - top_row[2 + 1 - x] as isize);
            }

            let mut v = 0;
            let mut left_column = [0u8; 9];
            for (idx, row) in target_slice.rows_iter().take(9).enumerate() {
                left_column[idx] = row[0];
            }
            for y in 0..4usize {
                v += (y as isize + 1)
                    * (left_column[4 + 1 + y] as isize - left_column[2 + 1 - y] as isize);
            }

            let a = 16 * (left_column[8] as isize + top_row[8] as isize);
            let b = (34 * h + 32) >> 6;
            let c = (34 * v + 32) >> 6;

            let mut target_slice = target.mut_slice(offset);
            for (y, row) in target_slice.rows_iter_mut().take(mb_height).enumerate() {
                for (x, pixel) in row.iter_mut().take(mb_width).enumerate() {
                    let x = x as isize;
                    let y = y as isize;
                    *pixel = ((a + b * (x - 3) + c * (y - 3) + 16) >> 5) as u8;
                }
            }
        }
    }

    for (blk_idx, residual) in residuals.iter().enumerate() {
        let mut blk_loc = get_4x4chroma_block_location(blk_idx as u8);
        blk_loc.x += loc.x;
        blk_loc.y += loc.y;
        let mut target_slice = target.mut_slice(point_to_plain_offset(blk_loc));
        for (y, row) in target_slice.rows_iter_mut().take(4).enumerate() {
            for (x, pixel) in row.iter_mut().take(4).enumerate() {
                *pixel = (*pixel as i32 + residual.samples[y][x]).clamp(0, 255) as u8;
            }
        }
    }
}
