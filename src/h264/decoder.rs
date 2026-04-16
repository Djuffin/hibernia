use std::cmp::{max, min, Ordering};
use std::collections::VecDeque;
use std::io::Read;

use super::slice::{RefPicListModification, Slice, SliceType};
use super::tables::mb_type_to_16x16_pred_mode;
use super::ColorPlane;

use super::dpb::{DecodedPictureBuffer, DpbMarking, DpbPicture, ReferenceDisposition};
use super::inter_pred::{interpolate_chroma, interpolate_luma, InterpolationBuffer};
use super::intra_pred::{
    point_to_plane_offset, render_chroma_intra_prediction, render_luma_16x16_intra_prediction,
    render_luma_4x4_intra_prediction,
};
use super::macroblock::{
    self, get_4x4chroma_block_location, get_4x4chroma_block_neighbor, get_4x4luma_block_location,
    get_4x4luma_block_neighbor, BMb, IMb, Intra_16x16_SamplePredMode, Intra_4x4_SamplePredMode,
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
pub type RefPicList<'a> = Vec<&'a DpbPicture>;

#[derive(Clone, Debug)]
pub struct Picture {
    pub frame: VideoFrame,
    pub frame_num: u16,
    pub pic_order_cnt: i32,
    /// Per-MB motion field, stored after decoding for use in temporal direct prediction.
    /// Indexed by mb_addr. Only populated for reference pictures.
    pub motion_field: Option<MotionFieldStorage>,
    pub crop: sps::CropDimensions,
}

/// Stores motion information from a decoded picture, needed for temporal direct prediction in B slices.
#[derive(Clone, Debug)]
pub struct MotionFieldStorage {
    /// Motion vectors and ref indices for each MB, indexed by mb_addr.
    pub mb_motion: Vec<macroblock::MbMotion>,
    /// Whether each MB was intra-coded.
    pub mb_is_intra: Vec<bool>,
    /// POC of each reference in the picture's refPicList0.
    pub ref_pic_l0_pocs: Vec<i32>,
    /// POC of each reference in the picture's refPicList1.
    pub ref_pic_l1_pocs: Vec<i32>,
}

#[derive(Debug, Clone)]
pub enum DecodingError {
    MisformedData(String),
    OutOfRange(String),
    FeatureNotSupported(String),
    ReferenceNotFound(String),
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
        self.pps.iter().find(|x| x.pic_parameter_set_id == id)
    }

    pub fn put_pps(&mut self, pps: pps::PicParameterSet) {
        let id = pps.pic_parameter_set_id;
        self.pps.retain(|x| x.pic_parameter_set_id != id);
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
/// if let Some(pic) = decoder.retrieve_frame() {
///     println!("Decoded {}x{} frame", pic.crop.display_width, pic.crop.display_height);
/// }
/// ```
pub struct Decoder {
    context: DecoderContext,
    dpb: DecodedPictureBuffer,
    output_frames: VecDeque<Picture>,
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

                let crop_dims = slice.sps.crop_dimensions();

                let pic = Picture {
                    frame,
                    frame_num: slice.header.frame_num,
                    pic_order_cnt,
                    motion_field: None,
                    crop: crop_dims,
                };
                let mut dpb_pic = DpbPicture {
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

                // --- C.2.2: Picture decoding (current picture NOT in DPB) ---

                // Construct reference picture lists before parsing, because temporal
                // direct prediction (used during B-slice parsing) needs the colocated picture.
                self.construct_ref_pic_list0(&mut slice, pic_order_cnt)?;
                if slice.header.slice_type == SliceType::B {
                    self.construct_ref_pic_list1(&mut slice, pic_order_cnt)?;
                    self.setup_colocated_pic_info(&mut slice, pic_order_cnt);
                }

                parser::parse_slice_data(&mut input, &mut slice).map_err(parse_error_handler)?;
                self.process_slice(&mut slice, &mut dpb_pic.picture.frame)?;

                // Build motion field while DPB indices are still valid (before mutations).
                self.save_motion_field(&slice, disposition, &mut dpb_pic);

                // --- C.2.3: Mark references + remove dead pictures (before storage) ---
                let (has_mmco5, flushed) = self.dpb.mark_prior_references(
                    &slice.header,
                    disposition,
                    &slice.sps,
                    &mut dpb_pic,
                );
                self.output_frames.extend(flushed.into_iter());
                self.dpb.remove_dead_pictures();

                // --- C.2.4: Store current picture (with bumping if DPB is full) ---
                let pictures = self.dpb.store_picture(dpb_pic);
                self.output_frames.extend(pictures.into_iter());

                self.poc_state.update_mmco5_state(
                    has_mmco5,
                    disposition != ReferenceDisposition::NonReference,
                );
            }
            NalUnitType::SupplementalEnhancementInfo => {}
            NalUnitType::SeqParameterSet => {
                let sps = parser::parse_sps(&mut input).map_err(parse_error_handler)?;
                info!("SPS: {:#?}", sps);
                if sps.ChromaArrayType() != ChromaFormat::YUV420 {
                    return Err(DecodingError::FeatureNotSupported(format!(
                        "chroma format {:?} is not supported, only YUV420",
                        sps.ChromaArrayType()
                    )));
                }
                if !sps.frame_mbs_only_flag {
                    return Err(DecodingError::FeatureNotSupported(
                        "interlaced video (frame_mbs_only_flag=0) is not supported".into(),
                    ));
                }
                if sps.gaps_in_frame_num_value_allowed_flag {
                    return Err(DecodingError::FeatureNotSupported(
                        "gaps_in_frame_num_value_allowed_flag=1 is not supported".into(),
                    ));
                }
                if sps.seq_scaling_matrix_present_flag {
                    return Err(DecodingError::FeatureNotSupported(
                        "custom scaling matrices are not supported".into(),
                    ));
                }

                // Update DPB size: use level-derived MaxDpbFrames per A.3.1,
                // or VUI max_dec_frame_buffering if bitstream_restriction_flag is set.
                let max_dpb_frames = super::dpb::max_dpb_frames(&sps);
                let mut max_dpb_size = max(sps.max_num_ref_frames as usize, 1);
                if let Some(vui) = &sps.vui_parameters {
                    if vui.bitstream_restriction_flag {
                        max_dpb_size = max(max_dpb_size, vui.max_dec_frame_buffering as usize);
                    }
                }
                max_dpb_size = max(max_dpb_size, max_dpb_frames);
                self.dpb.set_max_size(max_dpb_size);

                self.context.put_sps(sps);
            }
            NalUnitType::PicParameterSet => {
                let pps = parser::parse_pps(&mut input).map_err(parse_error_handler)?;
                info!("PPS: {:#?}", pps);
                if pps.transform_8x8_mode_flag {
                    return Err(DecodingError::FeatureNotSupported(
                        "8x8 transform is not supported".into(),
                    ));
                }
                if pps.slice_group.is_some() {
                    return Err(DecodingError::FeatureNotSupported(
                        "slice groups are not supported".into(),
                    ));
                }
                if pps.constrained_intra_pred_flag {
                    return Err(DecodingError::FeatureNotSupported(
                        "constrained intra prediction is not supported".into(),
                    ));
                }
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

    /// Retrieves the next available picture from the decoder's output queue.
    /// Returns `Some(Picture)` if a picture is available, or `None` if the queue is empty.
    pub fn retrieve_frame(&mut self) -> Option<Picture> {
        self.output_frames.pop_front()
    }

    /// Flushes the decoder, forcing any remaining frames in the DPB to be output.
    /// This is necessary because some frames may be held in the DPB (Decoded Picture Buffer)
    /// for reference or reordering (e.g., B-frames) and won't be output immediately.
    /// This should be called at the end of the stream.
    /// Call `retrieve_frame` repeatedly after flushing until it returns `None`.
    pub fn flush(&mut self) -> Result<(), DecodingError> {
        let pictures = self.dpb.flush();
        self.output_frames.extend(pictures.into_iter().map(|p| p.picture));
        Ok(())
    }

    /// Process the decoded slice, performing prediction and reconstruction.
    /// The current picture's frame is passed separately (it is not yet in the DPB).
    fn process_slice(
        &mut self,
        slice: &mut Slice,
        frame: &mut VideoFrame,
    ) -> Result<(), DecodingError> {
        let qp_bd_offset_y = 6 * slice.sps.bit_depth_luma_minus8 as i32;
        let qp_bd_offset_c = 6 * slice.sps.bit_depth_chroma_minus8 as i32;
        let mut qp = slice.pps.pic_init_qp_minus26 + 26 + slice.header.slice_qp_delta;
        let ref_pics_l0 = resolve_ref_pic_list(&slice.ref_pic_list0, &self.dpb.pictures, "ref_pic_list0")?;
        let ref_pics_l1 = resolve_ref_pic_list(&slice.ref_pic_list1, &self.dpb.pictures, "ref_pic_list1")?;
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

                        let chroma_format = slice.sps.ChromaArrayType();
                        if chroma_format == super::ChromaFormat::Monochrome {
                            break;
                        }
                        let shift = chroma_format.get_chroma_shift();
                        let chroma_loc =
                            Point { x: mb_loc.x >> shift.width, y: mb_loc.y >> shift.height };
                        let chroma_width = tables::MB_WIDTH >> shift.width;
                        let chroma_height = tables::MB_HEIGHT >> shift.height;

                        for (plane, samples) in [
                            (ColorPlane::Cb, &block.pcm_sample_chroma_cb),
                            (ColorPlane::Cr, &block.pcm_sample_chroma_cr),
                        ] {
                            let chroma_plane = &mut frame.planes[plane as usize];
                            let mut chroma_slice =
                                chroma_plane.mut_slice(point_to_plane_offset(chroma_loc));
                            for (idx, row) in
                                chroma_slice.rows_iter_mut().take(chroma_height).enumerate()
                            {
                                let row_range = idx * chroma_width..(idx + 1) * chroma_width;
                                row[..chroma_width].copy_from_slice(&samples[row_range]);
                            }
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
                            MbPredictionMode::Intra_8x8 => {
                                return Err(DecodingError::FeatureNotSupported(
                                    "Intra_8x8 prediction is not supported".into(),
                                ));
                            }
                            MbPredictionMode::Intra_16x16 => {
                                render_luma_16x16_intra_prediction(
                                    slice,
                                    mb_addr,
                                    mb_loc,
                                    luma_plane,
                                    mb_type_to_16x16_pred_mode(imb.mb_type).ok_or_else(|| {
                                        DecodingError::OutOfRange(format!(
                                            "no 16x16 pred mode for mb_type {:?}", imb.mb_type
                                        ))
                                    })?,
                                    &residuals,
                                );
                            }
                            MbPredictionMode::Pred_L0
                            | MbPredictionMode::Pred_L1
                            | MbPredictionMode::BiPred
                            | MbPredictionMode::Direct => {
                                unreachable!("Inter prediction mode on I macroblock")
                            }
                        }

                        for plane_name in [ColorPlane::Cb, ColorPlane::Cr] {
                            let qp_offset = slice.pps.get_chroma_qp_index_offset(plane_name);
                            let chroma_qp =
                                get_chroma_qp(qp, qp_offset, qp_bd_offset_c).try_into().map_err(|_| {
                                DecodingError::OutOfRange("chroma QP out of u8 range".into())
                            })?;
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
                            &ref_pics_l0,
                            &mut self.interpolation_buffer,
                        )?;

                        for plane_name in [ColorPlane::Cb, ColorPlane::Cr] {
                            let qp_offset = slice.pps.get_chroma_qp_index_offset(plane_name);
                            let chroma_qp =
                                get_chroma_qp(qp, qp_offset, qp_bd_offset_c).try_into().map_err(|_| {
                                DecodingError::OutOfRange("chroma QP out of u8 range".into())
                            })?;
                            let residuals = if let Some(residual) = block.residual.as_ref() {
                                residual.restore(plane_name, chroma_qp)
                            } else {
                                SmallVec::new()
                            };
                            render_chroma_inter_prediction(
                                slice, block, mb_loc, plane_name, frame, &residuals, &ref_pics_l0,
                            )?;
                        }
                    }
                    Macroblock::B(block) => {
                        qp = (qp + block.mb_qp_delta + 52 + 2 * qp_bd_offset_y)
                            % (52 + qp_bd_offset_y)
                            - qp_bd_offset_y;
                        let residuals = if let Some(residual) = block.residual.as_ref() {
                            residual.restore(ColorPlane::Y, qp as u8)
                        } else {
                            SmallVec::new()
                        };

                        render_luma_inter_prediction_b(
                            slice,
                            block,
                            mb_loc,
                            frame,
                            &residuals,
                            &ref_pics_l0,
                            &ref_pics_l1,
                            &mut self.interpolation_buffer,
                        )?;

                        for plane_name in [ColorPlane::Cb, ColorPlane::Cr] {
                            let qp_offset = slice.pps.get_chroma_qp_index_offset(plane_name);
                            let chroma_qp =
                                get_chroma_qp(qp, qp_offset, qp_bd_offset_c).try_into().map_err(|_| {
                                DecodingError::OutOfRange("chroma QP out of u8 range".into())
                            })?;
                            let residuals = if let Some(residual) = block.residual.as_ref() {
                                residual.restore(plane_name, chroma_qp)
                            } else {
                                SmallVec::new()
                            };
                            render_chroma_inter_prediction_b(
                                slice, block, mb_loc, plane_name, frame, &residuals,
                                &ref_pics_l0, &ref_pics_l1,
                            )?;
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
    fn construct_ref_pic_list0(
        &self,
        slice: &mut Slice,
        current_poc: i32,
    ) -> Result<(), DecodingError> {
        if slice.header.slice_type == SliceType::I || slice.header.slice_type == SliceType::SI {
            slice.ref_pic_list0.clear();
            return Ok(());
        }

        let mut ref_list0 = if slice.header.slice_type == SliceType::B {
            self.initialize_ref_pic_list0_b(slice, current_poc)
        } else {
            self.initialize_ref_pic_list0(slice)
        };
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

        let mut short_term_refs = Vec::new();
        let mut long_term_refs = Vec::new();

        // The current picture is not yet in the DPB (stored after decoding per C.2.4),
        // so all DPB entries are valid reference candidates.
        for (i, pic) in self.dpb.pictures.iter().enumerate() {
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
                        self.find_short_term_in_dpb(pic_num_lx)
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
                        self.find_short_term_in_dpb(pic_num_lx)
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

    fn find_short_term_in_dpb(&self, frame_num: i32) -> Option<usize> {
        self.dpb.pictures.iter().position(|pic| {
            pic.marking.is_short_term() && pic.picture.frame_num as i32 == frame_num
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

    // Section 8.2.4.2.3 Initialization process for reference picture lists for B slices
    // List 0 for B slices: short-term with POC <= current (desc), then POC > current (asc), then long-term (asc)
    fn initialize_ref_pic_list0_b(&self, slice: &Slice, current_poc: i32) -> Vec<usize> {
        let mut short_term_le = Vec::new(); // POC <= current
        let mut short_term_gt = Vec::new(); // POC > current
        let mut long_term_refs = Vec::new();

        for (i, pic) in self.dpb.pictures.iter().enumerate() {
            match pic.marking {
                DpbMarking::UsedForShortTermReference => {
                    if pic.picture.pic_order_cnt <= current_poc {
                        short_term_le.push((i, pic.picture.pic_order_cnt));
                    } else {
                        short_term_gt.push((i, pic.picture.pic_order_cnt));
                    }
                }
                DpbMarking::UsedForLongTermReference(lt_idx) => {
                    long_term_refs.push((i, lt_idx));
                }
                _ => {}
            }
        }

        short_term_le.sort_by_key(|k| std::cmp::Reverse(k.1)); // Descending POC
        short_term_gt.sort_by_key(|k| k.1); // Ascending POC
        long_term_refs.sort_by_key(|k| k.1); // Ascending LongTermPicNum

        short_term_le
            .iter()
            .map(|x| x.0)
            .chain(short_term_gt.iter().map(|x| x.0))
            .chain(long_term_refs.iter().map(|x| x.0))
            .collect()
    }

    // Section 8.2.4.2.3 Initialization process for reference picture lists for B slices
    // List 1 for B slices: short-term with POC > current (asc), then POC <= current (desc), then long-term (asc)
    fn initialize_ref_pic_list1(&self, slice: &Slice, current_poc: i32) -> Vec<usize> {
        let mut short_term_gt = Vec::new(); // POC > current
        let mut short_term_le = Vec::new(); // POC <= current
        let mut long_term_refs = Vec::new();

        for (i, pic) in self.dpb.pictures.iter().enumerate() {
            match pic.marking {
                DpbMarking::UsedForShortTermReference => {
                    if pic.picture.pic_order_cnt > current_poc {
                        short_term_gt.push((i, pic.picture.pic_order_cnt));
                    } else {
                        short_term_le.push((i, pic.picture.pic_order_cnt));
                    }
                }
                DpbMarking::UsedForLongTermReference(lt_idx) => {
                    long_term_refs.push((i, lt_idx));
                }
                _ => {}
            }
        }

        short_term_gt.sort_by_key(|k| k.1); // Ascending POC
        short_term_le.sort_by_key(|k| std::cmp::Reverse(k.1)); // Descending POC
        long_term_refs.sort_by_key(|k| k.1); // Ascending LongTermPicNum

        short_term_gt
            .iter()
            .map(|x| x.0)
            .chain(short_term_le.iter().map(|x| x.0))
            .chain(long_term_refs.iter().map(|x| x.0))
            .collect()
    }

    fn construct_ref_pic_list1(
        &self,
        slice: &mut Slice,
        current_poc: i32,
    ) -> Result<(), DecodingError> {
        let ref_list0 = &slice.ref_pic_list0;
        let mut ref_list1 = self.initialize_ref_pic_list1(slice, current_poc);

        // Section 8.2.4.2.3: When list1 is identical to list0 and has more than one entry, swap first two
        if ref_list1.len() > 1 && ref_list1 == *ref_list0 {
            ref_list1.swap(0, 1);
        }

        self.modify_ref_pic_list1(slice, &mut ref_list1);

        let len = (slice.header.num_ref_idx_l1_active_minus1 + 1) as usize;
        if ref_list1.len() > len {
            ref_list1.truncate(len);
        }

        slice.ref_pic_list1 = ref_list1;
        Ok(())
    }

    // Section 8.2.4.3 Reordering process for reference picture lists (list 1)
    fn modify_ref_pic_list1(&self, slice: &Slice, ref_list1: &mut Vec<usize>) {
        if slice.header.ref_pic_list_modification.list1.is_empty() {
            return;
        }

        let max_frame_num = 1 << (slice.sps.log2_max_frame_num_minus4 + 4);
        let curr_frame_num = slice.header.frame_num as i32;
        let mut pic_num_lx_pred = curr_frame_num;
        let mut ref_idx_l1 = 0;

        for modification in &slice.header.ref_pic_list_modification.list1 {
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
                        self.find_short_term_in_dpb(pic_num_lx)
                    {
                        self.place_picture_in_list(ref_list1, idx, ref_idx_l1);
                        ref_idx_l1 += 1;
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
                        self.find_short_term_in_dpb(pic_num_lx)
                    {
                        self.place_picture_in_list(ref_list1, idx, ref_idx_l1);
                        ref_idx_l1 += 1;
                    }
                }
                RefPicListModification::RemapLongTerm(long_term_pic_num) => {
                    if let Some(idx) = self.find_long_term_in_dpb(*long_term_pic_num) {
                        self.place_picture_in_list(ref_list1, idx, ref_idx_l1);
                        ref_idx_l1 += 1;
                    }
                }
            }
        }
    }

    /// Set up colocated picture info on the slice for temporal direct prediction.
    fn setup_colocated_pic_info(&self, slice: &mut Slice, current_poc: i32) {
        slice.current_pic_poc = current_poc;

        // Set ref_pic_list0 POCs
        slice.ref_pic_list0_pocs = slice
            .ref_pic_list0
            .iter()
            .filter_map(|&idx| self.dpb.pictures.get(idx).map(|p| p.picture.pic_order_cnt))
            .collect();

        // Set ref_pic_list1 POCs
        slice.ref_pic_list1_pocs = slice
            .ref_pic_list1
            .iter()
            .filter_map(|&idx| self.dpb.pictures.get(idx).map(|p| p.picture.pic_order_cnt))
            .collect();

        if slice.ref_pic_list1.is_empty() {
            return;
        }
        let col_dpb_idx = slice.ref_pic_list1[0];
        if let Some(col_pic) = self.dpb.pictures.get(col_dpb_idx) {
            if let Some(ref mf) = col_pic.picture.motion_field {
                slice.col_pic = Some(slice::ColPicInfo {
                    mb_motion: mf.mb_motion.clone(),
                    mb_is_intra: mf.mb_is_intra.clone(),
                    ref_pic_l0_pocs: mf.ref_pic_l0_pocs.clone(),
                    ref_pic_l1_pocs: mf.ref_pic_l1_pocs.clone(),
                    pic_poc: col_pic.picture.pic_order_cnt,
                    ref_l1_0_is_short_term: col_pic.marking.is_short_term(),
                });
            }
        }
    }

    /// Build motion field storage and attach it to the current picture.
    /// Must be called while DPB indices (from ref lists) are still valid,
    /// i.e., before mark_prior_references / remove_dead_pictures / store_picture.
    fn save_motion_field(
        &self,
        slice: &Slice,
        disposition: ReferenceDisposition,
        dpb_pic: &mut DpbPicture,
    ) {
        if disposition == ReferenceDisposition::NonReference {
            return;
        }
        let mb_count = slice.get_macroblock_count();
        let mut mb_motion = Vec::with_capacity(mb_count);
        let mut mb_is_intra = Vec::with_capacity(mb_count);
        let first_mb_addr = slice.header.first_mb_in_slice;
        for i in 0..mb_count {
            let mb_addr = first_mb_addr + i as u32;
            if let Some(mb) = slice.get_mb(mb_addr) {
                mb_motion.push(mb.get_motion_info());
                mb_is_intra.push(matches!(mb, Macroblock::I(_) | Macroblock::PCM(_)));
            } else {
                mb_motion.push(macroblock::MbMotion::default());
                mb_is_intra.push(true);
            }
        }
        // Resolve DPB indices to POCs now, before DPB mutations invalidate indices.
        let ref_pic_l0_pocs: Vec<i32> = slice
            .ref_pic_list0
            .iter()
            .filter_map(|&idx| self.dpb.pictures.get(idx).map(|p| p.picture.pic_order_cnt))
            .collect();
        let ref_pic_l1_pocs: Vec<i32> = slice
            .ref_pic_list1
            .iter()
            .filter_map(|&idx| self.dpb.pictures.get(idx).map(|p| p.picture.pic_order_cnt))
            .collect();

        dpb_pic.picture.motion_field = Some(MotionFieldStorage {
            mb_motion,
            mb_is_intra,
            ref_pic_l0_pocs,
            ref_pic_l1_pocs,
        });
    }
}

// Section 8.4.2.3: Weighted prediction mode for the current slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WeightedPredMode {
    Default,
    Explicit,
    Implicit,
}

// Resolved weighting parameters for one prediction direction (Section 8.4.3 outputs).
struct WeightParams {
    log_wd: u32,
    w0: i32,
    o0: i32,
    w1: i32,
    o1: i32,
}

/// Determine the weighted prediction mode for the current slice (Section 8.4.2.3).
fn get_weighted_pred_mode(slice: &Slice) -> WeightedPredMode {
    match slice.header.slice_type {
        SliceType::P | SliceType::SP => {
            if slice.pps.weighted_pred_flag {
                WeightedPredMode::Explicit
            } else {
                WeightedPredMode::Default
            }
        }
        SliceType::B => match slice.pps.weighted_bipred_idc {
            1 => WeightedPredMode::Explicit,
            2 => WeightedPredMode::Implicit,
            _ => WeightedPredMode::Default,
        },
        _ => WeightedPredMode::Default,
    }
}

// Section 8.4.2.3.2, Eq 8-274/8-275: Weighted sample prediction for uni-prediction.
#[inline]
fn weighted_uni_pred(pred: u8, w: i32, o: i32, log_wd: u32) -> u8 {
    let val = if log_wd >= 1 {
        ((i32::from(pred) * w + (1 << (log_wd - 1))) >> log_wd) + o
    } else {
        i32::from(pred) * w + o
    };
    val.clamp(0, 255) as u8
}

// Section 8.4.2.3.2, Eq 8-276: Weighted sample prediction for bi-prediction.
#[inline]
fn weighted_bi_pred(pred_l0: u8, pred_l1: u8, wp: &WeightParams) -> u8 {
    let val = ((i32::from(pred_l0) * wp.w0 + i32::from(pred_l1) * wp.w1
        + (1 << wp.log_wd))
        >> (wp.log_wd + 1))
        + ((wp.o0 + wp.o1 + 1) >> 1);
    val.clamp(0, 255) as u8
}

// Section 8.4.3: Derive explicit luma weighting parameters from pred_weight_table.
fn get_explicit_luma_weights(slice: &Slice, ref_idx_l0: usize, ref_idx_l1: usize) -> WeightParams {
    let table = slice.header.pred_weight_table.as_ref().unwrap();
    let log_wd = table.luma_log2_weight_denom;
    let (w0, o0) = table
        .list0
        .get(ref_idx_l0)
        .map_or((1 << log_wd, 0), |f| (f.luma_weight, f.luma_offset));
    let (w1, o1) = table
        .list1
        .get(ref_idx_l1)
        .map_or((1 << log_wd, 0), |f| (f.luma_weight, f.luma_offset));
    WeightParams { log_wd, w0, o0, w1, o1 }
}

// Section 8.4.3: Derive explicit chroma weighting parameters from pred_weight_table.
// chroma_idx: 0 = Cb, 1 = Cr.
fn get_explicit_chroma_weights(
    slice: &Slice,
    ref_idx_l0: usize,
    ref_idx_l1: usize,
    chroma_idx: usize,
) -> WeightParams {
    let table = slice.header.pred_weight_table.as_ref().unwrap();
    let log_wd = table.chroma_log2_weight_denom;
    let (w0, o0) = table
        .list0
        .get(ref_idx_l0)
        .map_or((1 << log_wd, 0), |f| {
            (f.chroma_weights[chroma_idx], f.chroma_offsets[chroma_idx])
        });
    let (w1, o1) = table
        .list1
        .get(ref_idx_l1)
        .map_or((1 << log_wd, 0), |f| {
            (f.chroma_weights[chroma_idx], f.chroma_offsets[chroma_idx])
        });
    WeightParams { log_wd, w0, o0, w1, o1 }
}

fn resolve_ref_pic_list<'a>(
    ref_list: &[usize],
    dpb_pictures: &'a [DpbPicture],
    list_name: &str,
) -> Result<RefPicList<'a>, DecodingError> {
    ref_list
        .iter()
        .enumerate()
        .map(|(i, &dpb_idx)| {
            dpb_pictures.get(dpb_idx).ok_or_else(|| {
                DecodingError::ReferenceNotFound(format!(
                    "{list_name}[{i}]: DPB index {dpb_idx} not in DPB (size {})",
                    dpb_pictures.len()
                ))
            })
        })
        .collect()
}

// Section 8.4.3: Derive implicit weighting parameters from POC distances (Eq 8-277 to 8-283).
// Same weights are used for luma and chroma in implicit mode.
fn get_implicit_weights(
    ref_pic_l0: &DpbPicture,
    ref_pic_l1: &DpbPicture,
    current_poc: i32,
) -> WeightParams {
    let log_wd: u32 = 5;
    let default = WeightParams { log_wd, w0: 32, o0: 0, w1: 32, o1: 0 };

    // Fallback if either reference is long-term (Eq 8-280)
    if ref_pic_l0.marking.is_long_term() || ref_pic_l1.marking.is_long_term() {
        return default;
    }

    let poc_l0 = ref_pic_l0.picture.pic_order_cnt;
    let poc_l1 = ref_pic_l1.picture.pic_order_cnt;

    // DiffPicOrderCnt(pic1, pic0) — Eq 8-197/8-198
    let diff_poc_l1_l0 = poc_l1 - poc_l0;
    if diff_poc_l1_l0 == 0 {
        return default;
    }

    // Eq 8-201, 8-202: DistScaleFactor
    let td = (poc_l1 - poc_l0).clamp(-128, 127);
    let tb = (current_poc - poc_l0).clamp(-128, 127);
    let tx = (16384 + (td.abs() >> 1)) / td;
    let dist_scale_factor = ((tb * tx + 32) >> 6).clamp(-1024, 1023);

    let w1 = dist_scale_factor >> 2;
    if w1 < -64 || w1 > 128 {
        return default;
    }
    let w0 = 64 - w1;

    WeightParams { log_wd, w0, o0: 0, w1, o1: 0 }
}


pub fn render_luma_inter_prediction(
    slice: &Slice,
    mb: &PMb,
    mb_loc: Point,
    frame: &mut VideoFrame,
    residuals: &[Block4x4],
    ref_pics_l0: &[&DpbPicture],
    buffer: &mut InterpolationBuffer,
) -> Result<(), DecodingError> {
    let y_plane = &mut frame.planes[0];
    let wp_mode = get_weighted_pred_mode(slice);

    for raster_idx in 0..16 {
        let (grid_x, grid_y) = (raster_idx % 4, raster_idx / 4);
        let partition = mb.motion.partitions[grid_y as usize][grid_x as usize];

        let ref_idx = partition.ref_idx_l0;
        let mv = partition.mv_l0;

        let ref_pic = *ref_pics_l0.get(ref_idx as usize).ok_or_else(|| {
            DecodingError::ReferenceNotFound(format!(
                "ref_idx_l0 {} out of bounds (list length {})", ref_idx, ref_pics_l0.len()
            ))
        })?;
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

        // Section 8.4.2.3: Apply weighted prediction before residual addition
        if wp_mode == WeightedPredMode::Explicit {
            let wp = get_explicit_luma_weights(slice, ref_idx as usize, 0);
            for sample in &mut dst {
                *sample = weighted_uni_pred(*sample, wp.w0, wp.o0, wp.log_wd);
            }
        }

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
    Ok(())
}

pub fn render_chroma_inter_prediction(
    slice: &Slice,
    mb: &PMb,
    mb_loc: Point,
    plane: ColorPlane,
    frame: &mut VideoFrame,
    residuals: &[Block4x4],
    ref_pics_l0: &[&DpbPicture],
) -> Result<(), DecodingError> {
    let chroma_plane = &mut frame.planes[plane as usize];
    let mb_x_chroma = mb_loc.x >> 1;
    let mb_y_chroma = mb_loc.y >> 1;
    let wp_mode = get_weighted_pred_mode(slice);
    let chroma_idx = plane as usize - 1; // Cb=0, Cr=1

    // 1. Prediction (Block by block 2x2)
    for blk_idx in 0..16 {
        let (grid_x, grid_y) = (blk_idx % 4, blk_idx / 4);
        let partition = mb.motion.partitions[grid_y as usize][grid_x as usize];
        let ref_idx = partition.ref_idx_l0;
        let mv = partition.mv_l0;

        let ref_pic = *ref_pics_l0.get(ref_idx as usize).ok_or_else(|| {
            DecodingError::ReferenceNotFound(format!(
                "ref_idx_l0 {} out of bounds (list length {})", ref_idx, ref_pics_l0.len()
            ))
        })?;
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

        // Section 8.4.2.3: Apply weighted prediction
        if wp_mode == WeightedPredMode::Explicit {
            let wp =
                get_explicit_chroma_weights(slice, ref_idx as usize, 0, chroma_idx);
            for sample in &mut dst {
                *sample = weighted_uni_pred(*sample, wp.w0, wp.o0, wp.log_wd);
            }
        }

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
    Ok(())
}

pub fn render_luma_inter_prediction_b(
    slice: &Slice,
    mb: &BMb,
    mb_loc: Point,
    frame: &mut VideoFrame,
    residuals: &[Block4x4],
    ref_pics_l0: &[&DpbPicture],
    ref_pics_l1: &[&DpbPicture],
    buffer: &mut InterpolationBuffer,
) -> Result<(), DecodingError> {
    let y_plane = &mut frame.planes[0];
    let wp_mode = get_weighted_pred_mode(slice);

    for raster_idx in 0..16 {
        let (grid_x, grid_y) = (raster_idx % 4, raster_idx / 4);
        let partition = mb.motion.partitions[grid_y as usize][grid_x as usize];
        let pred_mode = partition.pred_mode;

        let blk_x = grid_x * 4;
        let blk_y = grid_y * 4;

        let mut dst = [0u8; 16];

        let has_l0 = pred_mode == MbPredictionMode::Pred_L0 || pred_mode == MbPredictionMode::BiPred;
        let has_l1 = pred_mode == MbPredictionMode::Pred_L1 || pred_mode == MbPredictionMode::BiPred;

        let mut pred_l0 = [0u8; 16];
        let mut pred_l1 = [0u8; 16];

        if has_l0 {
            let ref_pic = ref_pics_l0.get(partition.ref_idx_l0 as usize).ok_or_else(|| {
                DecodingError::ReferenceNotFound(format!(
                    "ref_idx_l0 {} out of bounds (list length {})", partition.ref_idx_l0, ref_pics_l0.len()
                ))
            })?;
            let ref_plane = &ref_pic.picture.frame.planes[0];
            interpolate_luma(
                ref_plane,
                mb_loc.x,
                mb_loc.y,
                blk_x as u8,
                blk_y as u8,
                4,
                4,
                partition.mv_l0,
                &mut pred_l0,
                4,
                buffer,
            );
        }

        if has_l1 {
            let ref_pic = ref_pics_l1.get(partition.ref_idx_l1 as usize).ok_or_else(|| {
                DecodingError::ReferenceNotFound(format!(
                    "ref_idx_l1 {} out of bounds (list length {})", partition.ref_idx_l1, ref_pics_l1.len()
                ))
            })?;
            let ref_plane = &ref_pic.picture.frame.planes[0];
            interpolate_luma(
                ref_plane,
                mb_loc.x,
                mb_loc.y,
                blk_x as u8,
                blk_y as u8,
                4,
                4,
                partition.mv_l1,
                &mut pred_l1,
                4,
                buffer,
            );
        }

        // Section 8.4.2.3: Combine predictions according to weighted prediction mode
        match wp_mode {
            WeightedPredMode::Explicit => {
                let wp = get_explicit_luma_weights(
                    slice,
                    partition.ref_idx_l0 as usize,
                    partition.ref_idx_l1 as usize,
                );
                if has_l0 && has_l1 {
                    for i in 0..16 {
                        dst[i] = weighted_bi_pred(pred_l0[i], pred_l1[i], &wp);
                    }
                } else if has_l0 {
                    for i in 0..16 {
                        dst[i] = weighted_uni_pred(pred_l0[i], wp.w0, wp.o0, wp.log_wd);
                    }
                } else if has_l1 {
                    for i in 0..16 {
                        dst[i] = weighted_uni_pred(pred_l1[i], wp.w1, wp.o1, wp.log_wd);
                    }
                }
            }
            WeightedPredMode::Implicit => {
                if has_l0 && has_l1 {
                    let ref_l0 = ref_pics_l0.get(partition.ref_idx_l0 as usize).ok_or_else(|| {
                        DecodingError::ReferenceNotFound(format!(
                            "ref_idx_l0 {} out of bounds (list length {})", partition.ref_idx_l0, ref_pics_l0.len()
                        ))
                    })?;
                    let ref_l1 = ref_pics_l1.get(partition.ref_idx_l1 as usize).ok_or_else(|| {
                        DecodingError::ReferenceNotFound(format!(
                            "ref_idx_l1 {} out of bounds (list length {})", partition.ref_idx_l1, ref_pics_l1.len()
                        ))
                    })?;
                    let wp = get_implicit_weights(ref_l0, ref_l1, slice.current_pic_poc);
                    for i in 0..16 {
                        dst[i] = weighted_bi_pred(pred_l0[i], pred_l1[i], &wp);
                    }
                } else if has_l0 {
                    // Section 8.4.2.3: implicit mode with only one list falls back to default
                    dst = pred_l0;
                } else if has_l1 {
                    dst = pred_l1;
                }
            }
            WeightedPredMode::Default => {
                if has_l0 && has_l1 {
                    for i in 0..16 {
                        dst[i] = ((pred_l0[i] as u16 + pred_l1[i] as u16 + 1) >> 1) as u8;
                    }
                } else if has_l0 {
                    dst = pred_l0;
                } else if has_l1 {
                    dst = pred_l1;
                }
            }
        }

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
    Ok(())
}

pub fn render_chroma_inter_prediction_b(
    slice: &Slice,
    mb: &BMb,
    mb_loc: Point,
    plane: ColorPlane,
    frame: &mut VideoFrame,
    residuals: &[Block4x4],
    ref_pics_l0: &[&DpbPicture],
    ref_pics_l1: &[&DpbPicture],
) -> Result<(), DecodingError> {
    let chroma_plane = &mut frame.planes[plane as usize];
    let mb_x_chroma = mb_loc.x >> 1;
    let mb_y_chroma = mb_loc.y >> 1;
    let wp_mode = get_weighted_pred_mode(slice);
    let chroma_idx = plane as usize - 1; // Cb=0, Cr=1

    // 1. Prediction (2x2 blocks corresponding to each 4x4 luma block)
    for blk_idx in 0..16 {
        let (grid_x, grid_y) = (blk_idx % 4, blk_idx / 4);
        let partition = mb.motion.partitions[grid_y as usize][grid_x as usize];
        let pred_mode = partition.pred_mode;

        let has_l0 = pred_mode == MbPredictionMode::Pred_L0 || pred_mode == MbPredictionMode::BiPred;
        let has_l1 = pred_mode == MbPredictionMode::Pred_L1 || pred_mode == MbPredictionMode::BiPred;

        let blk_x = (grid_x * 4) >> 1;
        let blk_y = (grid_y * 4) >> 1;

        let mut pred_l0 = [0u8; 4];
        let mut pred_l1 = [0u8; 4];

        if has_l0 {
            let ref_pic = ref_pics_l0.get(partition.ref_idx_l0 as usize).ok_or_else(|| {
                DecodingError::ReferenceNotFound(format!(
                    "ref_idx_l0 {} out of bounds (list length {})", partition.ref_idx_l0, ref_pics_l0.len()
                ))
            })?;
            let ref_plane = &ref_pic.picture.frame.planes[plane as usize];
            interpolate_chroma(
                ref_plane,
                mb_x_chroma,
                mb_y_chroma,
                blk_x as u8,
                blk_y as u8,
                2,
                2,
                partition.mv_l0,
                &mut pred_l0,
                2,
            );
        }

        if has_l1 {
            let ref_pic = ref_pics_l1.get(partition.ref_idx_l1 as usize).ok_or_else(|| {
                DecodingError::ReferenceNotFound(format!(
                    "ref_idx_l1 {} out of bounds (list length {})", partition.ref_idx_l1, ref_pics_l1.len()
                ))
            })?;
            let ref_plane = &ref_pic.picture.frame.planes[plane as usize];
            interpolate_chroma(
                ref_plane,
                mb_x_chroma,
                mb_y_chroma,
                blk_x as u8,
                blk_y as u8,
                2,
                2,
                partition.mv_l1,
                &mut pred_l1,
                2,
            );
        }

        // Section 8.4.2.3: Combine predictions according to weighted prediction mode
        let mut dst = [0u8; 4];
        match wp_mode {
            WeightedPredMode::Explicit => {
                let wp = get_explicit_chroma_weights(
                    slice,
                    partition.ref_idx_l0 as usize,
                    partition.ref_idx_l1 as usize,
                    chroma_idx,
                );
                if has_l0 && has_l1 {
                    for i in 0..4 {
                        dst[i] = weighted_bi_pred(pred_l0[i], pred_l1[i], &wp);
                    }
                } else if has_l0 {
                    for i in 0..4 {
                        dst[i] = weighted_uni_pred(pred_l0[i], wp.w0, wp.o0, wp.log_wd);
                    }
                } else if has_l1 {
                    for i in 0..4 {
                        dst[i] = weighted_uni_pred(pred_l1[i], wp.w1, wp.o1, wp.log_wd);
                    }
                }
            }
            WeightedPredMode::Implicit => {
                if has_l0 && has_l1 {
                    // Implicit mode uses same weights for luma and chroma
                    let ref_l0 = ref_pics_l0.get(partition.ref_idx_l0 as usize).ok_or_else(|| {
                        DecodingError::ReferenceNotFound(format!(
                            "ref_idx_l0 {} out of bounds (list length {})", partition.ref_idx_l0, ref_pics_l0.len()
                        ))
                    })?;
                    let ref_l1 = ref_pics_l1.get(partition.ref_idx_l1 as usize).ok_or_else(|| {
                        DecodingError::ReferenceNotFound(format!(
                            "ref_idx_l1 {} out of bounds (list length {})", partition.ref_idx_l1, ref_pics_l1.len()
                        ))
                    })?;
                    let wp = get_implicit_weights(ref_l0, ref_l1, slice.current_pic_poc);
                    for i in 0..4 {
                        dst[i] = weighted_bi_pred(pred_l0[i], pred_l1[i], &wp);
                    }
                } else if has_l0 {
                    dst = pred_l0;
                } else if has_l1 {
                    dst = pred_l1;
                }
            }
            WeightedPredMode::Default => {
                if has_l0 && has_l1 {
                    for i in 0..4 {
                        dst[i] = ((pred_l0[i] as u16 + pred_l1[i] as u16 + 1) >> 1) as u8;
                    }
                } else if has_l0 {
                    dst = pred_l0;
                } else if has_l1 {
                    dst = pred_l1;
                }
            }
        }

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

    // 2. Residuals
    for (blk_idx, residual_blk) in residuals.iter().enumerate() {
        let blk_loc = get_4x4chroma_block_location(blk_idx as u8);

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
    Ok(())
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

#[cfg(test)]
mod weighted_pred_tests {
    use super::*;

    // --- weighted_uni_pred tests (Eq 8-274/8-275) ---

    #[test]
    fn uni_pred_identity() {
        // w = 1 << log_wd, o = 0 should produce the same value (default weight)
        for log_wd in 0..8u32 {
            let w = 1i32 << log_wd;
            for pred in [0u8, 1, 127, 128, 254, 255] {
                assert_eq!(
                    weighted_uni_pred(pred, w, 0, log_wd),
                    pred,
                    "identity failed for pred={pred}, log_wd={log_wd}"
                );
            }
        }
    }

    #[test]
    fn uni_pred_log_wd_zero() {
        // log_wd = 0: result = pred * w + o, clamped
        assert_eq!(weighted_uni_pred(100, 2, 10, 0), 210);
        assert_eq!(weighted_uni_pred(200, 2, 0, 0), 255); // clamped
        assert_eq!(weighted_uni_pred(10, -1, 0, 0), 0); // clamped negative
    }

    #[test]
    fn uni_pred_with_offset() {
        // log_wd = 7, w = 128 (=1<<7), o = 10 => pred + 10
        assert_eq!(weighted_uni_pred(100, 128, 10, 7), 110);
        // Clamping to 255
        assert_eq!(weighted_uni_pred(250, 128, 10, 7), 255);
    }

    #[test]
    fn uni_pred_clamping() {
        // Should clamp to [0, 255]
        assert_eq!(weighted_uni_pred(0, 128, -50, 7), 0);
        assert_eq!(weighted_uni_pred(255, 256, 100, 7), 255);
    }

    // --- weighted_bi_pred tests (Eq 8-276) ---

    #[test]
    fn bi_pred_equal_weights() {
        // w0=w1=32, log_wd=5, o0=o1=0 => same as default (l0+l1+1)>>1
        let wp = WeightParams { log_wd: 5, w0: 32, o0: 0, w1: 32, o1: 0 };
        assert_eq!(weighted_bi_pred(100, 200, &wp), 150);
        assert_eq!(weighted_bi_pred(0, 0, &wp), 0);
        assert_eq!(weighted_bi_pred(255, 255, &wp), 255);
        assert_eq!(weighted_bi_pred(1, 0, &wp), 1); // (1*32 + 0*32 + 32) >> 6 = 64 >> 6 = 1
    }

    #[test]
    fn bi_pred_arithmetic() {
        // Manual calculation: log_wd=5, w0=32, w1=32, o0=0, o1=0
        // (100*32 + 200*32 + 32) >> 6 + 0 = (3200 + 6400 + 32) >> 6 = 9632 >> 6 = 150
        let wp = WeightParams { log_wd: 5, w0: 32, o0: 0, w1: 32, o1: 0 };
        assert_eq!(weighted_bi_pred(100, 200, &wp), 150);

        // w0=64, w1=0 => effectively uni-pred from L0
        // (100*64 + 200*0 + 32) >> 6 + 0 = 6432 >> 6 = 100
        let wp2 = WeightParams { log_wd: 5, w0: 64, o0: 0, w1: 0, o1: 0 };
        assert_eq!(weighted_bi_pred(100, 200, &wp2), 100);
    }

    #[test]
    fn bi_pred_with_offsets() {
        // log_wd=5, w0=32, w1=32, o0=20, o1=10
        // (100*32 + 100*32 + 32) >> 6 + (20+10+1)>>1 = (6400+32)>>6 + 15 = 100 + 15 = 115
        let wp = WeightParams { log_wd: 5, w0: 32, o0: 20, w1: 32, o1: 10 };
        assert_eq!(weighted_bi_pred(100, 100, &wp), 115);
    }

    #[test]
    fn bi_pred_clamping() {
        let wp = WeightParams { log_wd: 5, w0: 64, o0: 127, w1: 64, o1: 127 };
        assert_eq!(weighted_bi_pred(255, 255, &wp), 255); // clamped
        let wp2 = WeightParams { log_wd: 5, w0: 64, o0: -128, w1: 64, o1: -128 };
        assert_eq!(weighted_bi_pred(0, 0, &wp2), 0); // clamped
    }

    // --- Default weight identity ---

    #[test]
    fn default_weight_is_identity() {
        // Explicit weights with w = 1 << log_wd, o = 0 should match unweighted copy
        for log_wd in 0..8u32 {
            let w = 1i32 << log_wd;
            for pred in [0u8, 64, 128, 200, 255] {
                let weighted = weighted_uni_pred(pred, w, 0, log_wd);
                assert_eq!(
                    weighted, pred,
                    "default weight not identity: pred={pred}, log_wd={log_wd}"
                );
            }
        }
    }
}
