use std::cmp::{max, min, Ordering};
use std::collections::VecDeque;
use std::io::Read;

use super::slice::{RefPicListModification, SliceType, Slice};
use super::tables::mb_type_to_16x16_pred_mode;
use super::ColorPlane;

use super::dpb::{DecodedPictureBuffer, DpbPicture, DpbMarking, ReferenceDisposition};
use super::inter_pred::{interpolate_chroma, interpolate_luma, InterpolationBuffer};
use super::intra_pred::{
    point_to_plane_offset, render_chroma_intra_prediction, render_luma_16x16_intra_prediction,
    render_luma_4x4_intra_prediction,
};
use super::macroblock::{
    self, get_4x4chroma_block_location, get_4x4chroma_block_neighbor, get_4x4luma_block_location,
    get_4x4luma_block_neighbor, IMb, Intra_16x16_SamplePredMode, Intra_4x4_SamplePredMode,
    Intra_Chroma_Pred_Mode, Macroblock, MbAddr, MbNeighborName, MbPredictionMode, MotionVector,
    PMb, PartitionInfo,
};
use super::poc::PocState;
use super::residual::{level_scale_4x4_block, unzip_block_4x4, Block4x4};
use super::tables::{MB_HEIGHT, MB_WIDTH};
use super::{deblocking, nal, parser, pps, slice, sps, tables, ChromaFormat, Point};
use log::{info, trace};
use smallvec::SmallVec;
use v_frame::frame;
use v_frame::plane::{self, PlaneOffset, PlaneSlice};

pub type VideoFrame = frame::Frame<u8>;

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
    sps: Vec<sps::SequenceParameterSet>,
    pps: Vec<pps::PicParameterSet>,
}

impl DecoderContext {
    pub fn get_sps(&self, id: u8) -> Option<&sps::SequenceParameterSet> {
        self.sps.iter().find(|x| x.seq_parameter_set_id == id)
    }

    pub fn put_sps(&mut self, sps: sps::SequenceParameterSet) {
        let id = sps.seq_parameter_set_id;
        self.sps.retain(|x| x.seq_parameter_set_id != id);
        self.sps.push(sps);
    }

    pub fn get_pps(&self, id: u8) -> Option<&pps::PicParameterSet> {
        self.pps.iter().find(|x| x.seq_parameter_set_id == id)
    }

    pub fn put_pps(&mut self, pps: pps::PicParameterSet) {
        let id = pps.seq_parameter_set_id;
        self.pps.retain(|x| x.seq_parameter_set_id != id);
        self.pps.push(pps);
    }
}

/// A standards-compliant H.264 (AVC) video decoder.
///
/// This decoder implements the ITU-T H.264 specification, supporting the parsing of NAL units
/// and the reconstruction of video frames. It currently maintains internal state for
/// Sequence Parameter Sets (SPS), Picture Parameter Sets (PPS), and the Decoded Picture Buffer (DPB).
///
/// # Usage
///
/// The decoder works by feeding it individual NAL units (Network Abstraction Layer) extracted
/// from a byte stream. You must use a separate parser (like [`crate::h264::nal_parser::NalParser`])
/// to split the raw byte stream into NAL units before passing them to [`Decoder::decode`].
///
/// # Example
///
/// ```rust
/// use hibernia::h264::decoder::Decoder;
///
/// // Valid NAL units from SVA_BA2_D.264
/// // SPS (Sequence Parameter Set)
/// let sps = vec![
///     0x67, 0x42, 0xE0, 0x15, 0x8D, 0x66, 0x0B, 0x13, 0x90
/// ];
/// // PPS (Picture Parameter Set)
/// let pps = vec![
///     0x68, 0xCE, 0x38, 0x80
/// ];
/// // IDR Slice (First 40 bytes)
/// // Note: This is truncated, so full decoding might fail or produce a partial frame,
/// // but it's enough to pass header parsing.
/// let slice = vec![
///     0x65, 0x88, 0x80, 0x00, 0x41, 0x98, 0xD2, 0x2F, 0x01, 0x07, 0xDD, 0xF8, 0x03, 0x30, 0x20,
///     0xFE, 0xC3, 0xA8, 0x8B, 0xEF, 0x0B, 0xFC, 0xCF, 0x9C, 0x7A, 0x07, 0xDF, 0x9F, 0x15, 0x00,
///     0x6E, 0x18, 0xFA, 0x14, 0x43, 0x03, 0x9F, 0xA8, 0xFC
/// ];
///
/// let mut decoder = Decoder::new();
///
/// // 1. Decode Parameter Sets first
/// decoder.decode(&sps).expect("SPS should be valid");
/// decoder.decode(&pps).expect("PPS should be valid");
///
/// // 2. Decode Slice Data
/// // Since we provided a truncated slice, we use .ok() to ignore potential EOF errors
/// // for this demonstration. In a real scenario, you'd provide the full NAL.
/// let _ = decoder.decode(&slice);
///
/// // 3. Retrieve decoded frames (if any are ready)
/// if let Some(frame) = decoder.retrieve_frame() {
///     println!("Decoded {}x{} frame", frame.planes[0].cfg.width, frame.planes[0].cfg.height);
/// }
/// ```
pub struct Decoder {
    context: DecoderContext,
    dpb: DecodedPictureBuffer,
    output_frames: VecDeque<VideoFrame>,
    interpolation_buffer: InterpolationBuffer,
    poc_state: PocState,
}

impl std::fmt::Debug for Decoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Decoder")
            .field("context", &self.context)
            .field("dpb", &self.dpb)
            .field("output_frames", &self.output_frames)
            .field("poc_state", &self.poc_state)
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
            dpb: DecodedPictureBuffer::new(),
            output_frames: VecDeque::new(),
            interpolation_buffer: InterpolationBuffer::new(),
            poc_state: PocState::new(),
        }
    }

    pub fn decode(&mut self, nal_data: &[u8]) -> Result<(), DecodingError> {
        use nal::NalUnitType;

        let nal_vec = parser::remove_emulation_if_needed(nal_data);
        let rbsp_data = if nal_vec.is_empty() { nal_data } else { nal_vec.as_slice() };

        let mut input = parser::BitReader::new(rbsp_data);
        let parse_error_handler = DecodingError::MisformedData;

        let nal = parser::parse_nal_header(&mut input).map_err(parse_error_handler)?;
        assert!(input.is_aligned());
        trace!("NAL {:?}", nal);

        match nal.nal_unit_type {
            NalUnitType::Unspecified => {}
            NalUnitType::SliceDataA => {}
            NalUnitType::SliceDataB => {}
            NalUnitType::SliceDataC => {}
            NalUnitType::IDRSlice | NalUnitType::NonIDRSlice => {
                let mut slice = parser::parse_slice_header(&self.context, &nal, &mut input)
                    .map_err(parse_error_handler)?;

                trace!("{:?} {:#?}", nal.nal_unit_type, slice);
                let frame = VideoFrame::new_with_padding(
                    slice.sps.pic_width(),
                    slice.sps.pic_height(),
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

                let pic_order_cnt = self.poc_state.calculate_poc(&slice, disposition);

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

                parser::parse_slice_data(&mut input, &mut slice).map_err(parse_error_handler)?;
                self.process_slice(&mut slice)?;
                // MMCO 5 (Memory Management Control Operation 5) marks all reference pictures
                // as "unused for reference" and sets the current frame's frame_num and POC to 0.
                let has_mmco5 = self.dpb.mark_references(&slice.header, disposition, &slice.sps);
                self.poc_state.update_mmco5_state(
                    has_mmco5,
                    disposition != ReferenceDisposition::NonReference,
                );
            }
            NalUnitType::SupplementalEnhancementInfo => {}
            NalUnitType::SeqParameterSet => {
                let sps = parser::parse_sps(&mut input).map_err(parse_error_handler)?;
                info!("SPS: {:#?}", sps);
                assert_eq!(sps.ChromaArrayType(), ChromaFormat::YUV420);

                // Update DPB size based on SPS and VUI parameters
                let mut max_dpb_size = max(sps.max_num_ref_frames as usize, 1);
                if let Some(vui) = &sps.vui_parameters {
                    if vui.bitstream_restriction_flag {
                        max_dpb_size = max(max_dpb_size, vui.max_dec_frame_buffering as usize);
                    }
                }
                self.dpb.set_max_size(max_dpb_size);

                self.context.put_sps(sps);
            }
            NalUnitType::PicParameterSet => {
                let pps = parser::parse_pps(&mut input).map_err(parse_error_handler)?;
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

        Ok(())
    }

    /// Retrieves the next available frame from the decoder's output queue.
    /// Returns `Some(VideoFrame)` if a frame is available, or `None` if the queue is empty.
    pub fn retrieve_frame(&mut self) -> Option<VideoFrame> {
        self.output_frames.pop_front()
    }

    /// Flushes the decoder, forcing any remaining frames in the DPB to be output.
    /// This is necessary because some frames may be held in the DPB (Decoded Picture Buffer)
    /// for reference or reordering (e.g., B-frames) and won't be output immediately.
    /// This should be called at the end of the stream.
    /// Call `retrieve_frame` repeatedly after flushing until it returns `None`.
    pub fn flush(&mut self) -> Result<(), DecodingError> {
        let pictures = self.dpb.flush();
        self.output_frames.extend(pictures.into_iter().map(|p| p.picture.frame));
        Ok(())
    }

    fn process_slice(&mut self, slice: &mut Slice) -> Result<(), DecodingError> {
        if self.dpb.pictures.is_empty() {
            return Err(DecodingError::Wtf);
        }

        self.construct_ref_pic_list0(slice)?;

        let qp_bd_offset_y = 6 * slice.sps.bit_depth_luma_minus8 as i32;
        let qp_bd_offset_c = 6 * slice.sps.bit_depth_chroma_minus8 as i32;
        let mut qp = slice.pps.pic_init_qp_minus26 + 26 + slice.header.slice_qp_delta;
        let split_idx = self.dpb.pictures.len() - 1;
        let (references, current) = self.dpb.pictures.split_at_mut(split_idx);
        let frame = &mut current[0].picture.frame;
        let first_mb_addr = slice.header.first_mb_in_slice;
        for i in 0..slice.get_macroblock_count() {
            let mb_addr = first_mb_addr + i as u32;
            let mb_loc = slice.get_mb_location(mb_addr);

            if let Some(mb) = slice.get_mb(mb_addr) {
                match mb {
                    Macroblock::PCM(block) => {
                        qp = 0;
                        let y_plane = &mut frame.planes[0];
                        let mut plane_slice = y_plane.mut_slice(point_to_plane_offset(mb_loc));

                        for (idx, row) in
                            plane_slice.rows_iter_mut().take(tables::MB_HEIGHT).enumerate()
                        {
                            let row_range = idx * tables::MB_WIDTH..(idx + 1) * tables::MB_WIDTH;
                            row[..tables::MB_WIDTH]
                                .copy_from_slice(&block.pcm_sample_luma[row_range]);
                        }
                    }
                    Macroblock::I(imb) => {
                        qp = (qp + imb.mb_qp_delta + 52 + 2 * qp_bd_offset_y)
                            % (52 + qp_bd_offset_y)
                            - qp_bd_offset_y;
                        let residuals = if let Some(residual) = imb.residual.as_ref() {
                            residual.restore(ColorPlane::Y, qp as u8)
                        } else {
                            SmallVec::new()
                        };

                        let luma_plane = &mut frame.planes[0];
                        let luma_prediction_mode = imb.MbPartPredMode(0);
                        match luma_prediction_mode {
                            MbPredictionMode::None => panic!("impossible pred mode"),
                            MbPredictionMode::Intra_4x4 => {
                                render_luma_4x4_intra_prediction(
                                    slice, mb_addr, imb, mb_loc, luma_plane, &residuals,
                                );
                            }
                            MbPredictionMode::Intra_8x8 => todo!("8x8 pred mode"),
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
                            MbPredictionMode::Pred_L0 => todo!(),
                            MbPredictionMode::Pred_L1 => todo!(),
                        }

                        for plane_name in [ColorPlane::Cb, ColorPlane::Cr] {
                            let qp_offset = slice.pps.get_chroma_qp_index_offset(plane_name);
                            let chroma_qp =
                                get_chroma_qp(qp, qp_offset, qp_bd_offset_c).try_into().unwrap();
                            let chroma_plane = &mut frame.planes[plane_name as usize];
                            let residuals = if let Some(residual) = imb.residual.as_ref() {
                                residual.restore(plane_name, chroma_qp)
                            } else {
                                SmallVec::new()
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
                    }
                    Macroblock::P(block) => {
                        qp = (qp + block.mb_qp_delta + 52 + 2 * qp_bd_offset_y)
                            % (52 + qp_bd_offset_y)
                            - qp_bd_offset_y;
                        let residuals = if let Some(residual) = block.residual.as_ref() {
                            residual.restore(ColorPlane::Y, qp as u8)
                        } else {
                            SmallVec::new()
                        };

                        render_luma_inter_prediction(
                            slice,
                            block,
                            mb_loc,
                            frame,
                            &residuals,
                            references,
                            &mut self.interpolation_buffer,
                        );

                        for plane_name in [ColorPlane::Cb, ColorPlane::Cr] {
                            let qp_offset = slice.pps.get_chroma_qp_index_offset(plane_name);
                            let chroma_qp =
                                get_chroma_qp(qp, qp_offset, qp_bd_offset_c).try_into().unwrap();
                            let residuals = if let Some(residual) = block.residual.as_ref() {
                                residual.restore(plane_name, chroma_qp)
                            } else {
                                SmallVec::new()
                            };
                            render_chroma_inter_prediction(
                                slice, block, mb_loc, plane_name, frame, &residuals, references,
                            );
                        }
                    }
                }
            }

            if let Some(mb) = slice.get_mb_mut(mb_addr) {
                mb.set_qp(qp as u8);
                trace!("MB {mb_addr}: {mb:?}");
            }
        }

        deblocking::filter_slice(slice, frame);

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
                let blk_idx =
                    macroblock::get_4x4luma_block_index(Point { x: blk_x as u32, y: blk_y as u32 });
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
