use std::cmp::{max, min, Ordering};
use std::collections::VecDeque;
use std::io::Read;
use std::sync::Arc;

use super::slice::{DeblockingFilterIdc, RefPicListModification, Slice, SliceHeader, SliceType};
use super::tables::mb_type_to_16x16_pred_mode;
use super::ColorPlane;

use super::dpb::{DecodedPictureBuffer, DpbMarking, DpbPicture, ReferenceDisposition};
use super::inter_pred::{
    render_chroma_inter_prediction, render_chroma_inter_prediction_b, render_luma_inter_prediction,
    render_luma_inter_prediction_b, InterpolationBuffer,
};
use super::intra_pred::{
    point_to_plane_offset, render_chroma_intra_prediction, render_luma_16x16_intra_prediction,
    render_luma_4x4_intra_prediction, render_luma_8x8_intra_prediction,
};
use super::macroblock::{
    self, get_4x4chroma_block_neighbor, get_4x4luma_block_location, get_4x4luma_block_neighbor,
    IMb, Intra_16x16_SamplePredMode, Intra_4x4_SamplePredMode, Intra_Chroma_Pred_Mode, Macroblock,
    MbAddr, MbNeighborName, MbPredictionMode, MotionVector, PartitionInfo,
};
use super::poc::PocState;
use super::residual::{level_scale_4x4_block, unzip_block_4x4, Block4x4, Residual};
use super::scaling_list::{
    resolve_pic_scaling_matrix, resolve_seq_scaling_matrix, ResolvedScalingMatrix,
};
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
    /// Decoded sample data. Wrapped in `Arc` so that bumping a reference
    /// picture for output (`DPB::bump_one`) is a refcount bump rather than
    /// a multi-MB memcpy. Mutated only during decode of the current picture
    /// (before any clone exists), via `Arc::get_mut`.
    pub frame: Arc<VideoFrame>,
    pub frame_num: u16,
    pub pic_order_cnt: i32,
    /// Per-MB motion field, stored after decoding for use in temporal direct
    /// prediction. Indexed by mb_addr. Only populated for reference pictures.
    /// Wrapped in `Arc` because the storage is read-only after attachment and
    /// is shared with each B-slice that picks this picture as colocated.
    pub motion_field: Option<Arc<MotionFieldStorage>>,
    pub crop: sps::CropDimensions,
}

/// Compact bit-packed boolean vector.
#[derive(Clone, Debug, Default)]
pub struct BitVec {
    words: Vec<u64>,
    len: usize,
}

impl BitVec {
    pub fn zeros(len: usize) -> Self {
        Self { words: vec![0u64; len.div_ceil(64)], len }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, i: usize) -> bool {
        self.words[i >> 6] & (1u64 << (i & 63)) != 0
    }

    pub fn set(&mut self, i: usize, v: bool) {
        let mask = 1u64 << (i & 63);
        if v {
            self.words[i >> 6] |= mask;
        } else {
            self.words[i >> 6] &= !mask;
        }
    }
}

/// Stores motion information from a decoded picture, needed for temporal direct prediction in B slices.
#[derive(Clone, Debug)]
pub struct MotionFieldStorage {
    /// Per-MB motion vectors and reference indices, indexed by mb_addr.
    pub mb_motion: Vec<macroblock::MbMotion>,
    /// Per-MB intra flag, indexed by mb_addr.
    pub mb_is_intra: BitVec,
    /// Slice index that decoded each MB, indexed by mb_addr. Used by
    /// temporal direct prediction to look up the right ref-list POCs in
    /// `slice_ref_pocs` -- each MB's `ref_idx_l0/l1` is interpreted in the
    /// context of *its own slice's* ref pic lists, which can differ across
    /// slices of a multi-slice picture.
    pub mb_slice_id: Vec<u16>,
    /// `(refPicList0_pocs, refPicList1_pocs)` for each slice in the picture,
    /// indexed by slice id.
    pub slice_ref_pocs: Vec<(Vec<i32>, Vec<i32>)>,
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

/// Per-slice deblocking parameters captured at slice-decode time and consumed
/// at picture finalize time. Needed because deblocking is a picture-level pass
/// (per Section 8.7), but each slice carries its own filter parameters.
#[derive(Clone, Debug)]
pub struct SliceDeblockParams {
    pub idc: DeblockingFilterIdc,
    pub alpha_c0_offset_div2: i32,
    pub beta_offset_div2: i32,
}

/// State accumulated across all slices of a single coded picture.
///
/// Multi-slice pictures are detected via Section 7.4.1.2.4: while consecutive
/// slice NAL units share the same `frame_num`, `pic_parameter_set_id`, POC fields,
/// etc., they belong to the same picture and accumulate into the same
/// `CurrentPicture`. When any of those fields change, the existing
/// `CurrentPicture` is finalized (deblocked, motion field built, stored in DPB,
/// emitted for output) and a fresh one is started.
#[derive(Clone, Debug)]
pub struct CurrentPicture {
    // The DPB-bound picture being assembled. Slice decoding writes pixels
    // directly into `dpb_pic.picture.frame`.
    pub dpb_pic: DpbPicture,
    pub disposition: ReferenceDisposition,

    // Section 7.4.1.2.4 boundary-detection fields, captured from first slice.
    pub pic_parameter_set_id: u8,
    pub frame_num: u16,
    pub field_pic_flag: bool,
    pub nal_ref_idc_nonzero: bool,
    pub nal_unit_type_is_idr: bool,
    pub idr_pic_id: Option<u32>,
    pub pic_order_cnt_lsb: Option<u32>,
    pub delta_pic_order_cnt_bottom: Option<i32>,
    pub delta_pic_order_cnt: [i32; 2],

    // First slice's header -- kept so MMCO / `dec_ref_pic_marking` is applied
    // once at finalize time using the picture-defining slice (per Section 7.4.3).
    pub first_slice_header: SliceHeader,
    // SPS active for this picture. Captured so finalize is self-contained:
    // when boundary detection triggers finalize, the new slice's SPS may differ
    // and the in-progress picture must MMCO-process under its own SPS.
    pub sps: sps::SequenceParameterSet,
    // PPS active for this picture. Captured at picture start because the
    // context-resident PPS with the same id can be replaced before finalize
    // runs (each picture in a stream may carry a fresh PPS update before its
    // first slice arrives), and the picture-level deblocking pass at finalize
    // needs this picture's `chroma_qp_index_offset`, not the next picture's.
    pub pps: pps::PicParameterSet,

    // Picture-wide accumulated state, length = pic_size_in_mbs.
    // Decoded macroblocks indexed by mb_addr. `None` until the owning slice
    // has been processed; finalize asserts no `None` remains.
    pub macroblocks: Vec<Option<Macroblock>>,
    // Slice index (0..slices_seen) that decoded each MB. Used by the
    // picture-level deblocking pass for `disable_deblocking_filter_idc=2`
    // (filter except across slice boundaries). `u16::MAX` means undecoded.
    pub mb_slice_id: Vec<u16>,
    // Per-MB motion info accumulated for `MotionFieldStorage` at finalize.
    // Empty for non-reference pictures: they can never be a colocated
    // reference, so the population work is skipped entirely.
    pub mb_motion: Vec<macroblock::MbMotion>,
    // Per-MB intra flag, packed 1 bit per MB. Empty for non-reference
    // pictures (same reason as `mb_motion`).
    pub mb_is_intra: BitVec,

    // Per-slice metadata, indexed by slice_id (0..slices_seen).
    // Deblocking parameters captured at slice-decode time, consumed by the
    // picture-level deblocking pass.
    pub slice_deblock: Vec<SliceDeblockParams>,
    // POCs of (refPicList0, refPicList1) for each slice, resolved at
    // slice-decode time *before* DPB mutation. Indexed by slice_id and
    // consumed when building the picture's `MotionFieldStorage`.
    pub slice_ref_pocs: Vec<(Vec<i32>, Vec<i32>)>,

    // Number of slices appended to this picture so far. Equal to the next
    // slice_id to assign.
    pub slices_seen: u16,
    // Lowest mb_addr that hasn't been decoded by any slice yet. Used to
    // enforce the Section 7.4.3 raster-order rule on `first_mb_in_slice`
    // without an O(N) scan over `macroblocks`.
    pub next_mb_addr: macroblock::MbAddr,
}

/// Detects whether the incoming slice begins a new coded picture, per
/// Section 7.4.1.2.4 ("Detection of the first VCL NAL unit of a primary
/// coded picture").
///
/// Returns `true` if the slice belongs to a new picture (and the in-progress
/// `prev` should be finalized before this slice is processed). Returns `false`
/// if the slice is another part of `prev`.
fn is_new_picture(
    prev: &CurrentPicture,
    hdr: &SliceHeader,
    nal: &nal::NalHeader,
    sps: &sps::SequenceParameterSet,
) -> bool {
    // frame_num differs in value.
    if prev.frame_num != hdr.frame_num {
        return true;
    }

    // pic_parameter_set_id differs in value.
    if prev.pic_parameter_set_id != hdr.pic_parameter_set_id {
        return true;
    }

    // field_pic_flag differs in value. Field coding is rejected upstream, so
    // in practice both are always `false` -- checked anyway for spec correctness.
    if prev.field_pic_flag != hdr.field_pic_flag {
        return true;
    }

    // nal_ref_idc differs in value, with one of the values being equal to 0.
    let hdr_nal_ref_idc_nonzero = nal.nal_ref_idc != 0;
    if prev.nal_ref_idc_nonzero != hdr_nal_ref_idc_nonzero {
        return true;
    }

    // nal_unit_type differs in value, with one of the values being equal to 5
    // (IDR slice).
    let hdr_is_idr = nal.nal_unit_type == nal::NalUnitType::IDRSlice;
    if prev.nal_unit_type_is_idr != hdr_is_idr {
        return true;
    }

    // nal_unit_type is equal to 5 for both, and idr_pic_id differs in value.
    if prev.nal_unit_type_is_idr && hdr_is_idr && prev.idr_pic_id != hdr.idr_pic_id {
        return true;
    }

    // pic_order_cnt_type is equal to 0 for both, and either pic_order_cnt_lsb
    // differs or delta_pic_order_cnt_bottom differs.
    if sps.pic_order_cnt_type == 0 {
        if prev.pic_order_cnt_lsb != hdr.pic_order_cnt_lsb {
            return true;
        }
        if prev.delta_pic_order_cnt_bottom != hdr.delta_pic_order_cnt_bottom {
            return true;
        }
    }

    // pic_order_cnt_type is equal to 1 for both, and either delta_pic_order_cnt[0]
    // differs or delta_pic_order_cnt[1] differs.
    if sps.pic_order_cnt_type == 1 && prev.delta_pic_order_cnt != hdr.delta_pic_order_cnt {
        return true;
    }

    false
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
/// if let Some(pic) = decoder.retrieve_picture() {
///     println!("Decoded {}x{} frame", pic.crop.display_width, pic.crop.display_height);
/// }
/// ```
pub struct Decoder {
    context: DecoderContext,
    dpb: DecodedPictureBuffer,
    output_pictures: VecDeque<Picture>,
    interpolation_buffer: InterpolationBuffer,
    poc_state: PocState,
    // Picture currently being assembled across one or more slices. `None`
    // before the first slice of a stream and immediately after a picture is
    // finalized.
    current_picture: Option<CurrentPicture>,
    residual_pool: super::residual::ResidualPool,
    mb_pool: macroblock::MacroblockPool,
}

impl std::fmt::Debug for Decoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Decoder")
            .field("context", &self.context)
            .field("dpb", &self.dpb)
            .field("output_pictures", &self.output_pictures)
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
            output_pictures: VecDeque::new(),
            interpolation_buffer: InterpolationBuffer::new(),
            poc_state: PocState::new(),
            current_picture: None,
            residual_pool: super::residual::ResidualPool::default(),
            mb_pool: macroblock::MacroblockPool::default(),
        }
    }

    pub fn decode(&mut self, nal_data: &[u8]) -> Result<(), DecodingError> {
        use nal::NalUnitType;

        let rbsp_data = parser::remove_emulation_if_needed(nal_data);
        let mut input = parser::BitReader::new(&rbsp_data);
        let parse_error_handler = DecodingError::MisformedData;

        let nal = parser::parse_nal_header(&mut input).map_err(parse_error_handler)?;
        debug_assert!(input.is_aligned());
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

                // Section 7.4.1.2.4: finalize the in-progress picture if this
                // slice begins a new one. With `current_picture == None` (first
                // slice of the stream, or just-finalized) we always start fresh.
                let starts_new_picture = self
                    .current_picture
                    .as_ref()
                    .map(|prev| is_new_picture(prev, &slice.header, &nal, &slice.sps))
                    .unwrap_or(true);
                if starts_new_picture {
                    self.finalize_pending_picture()?;
                    self.current_picture = Some(self.start_new_picture(&slice, &nal)?);
                }

                // Decoding methods need `&mut self`, which conflicts with a
                // `self.current_picture.as_mut()` borrow. Take the picture out,
                // decode into it, then put it back. On decode error the `?`
                // drops `current` -- partial pictures are not retained.
                let mut current = self.current_picture.take().ok_or_else(|| {
                    DecodingError::MisformedData(
                        "internal: current_picture missing after start".into(),
                    )
                })?;
                self.decode_slice_into_current(&mut current, &mut slice, &mut input)?;
                self.current_picture = Some(current);
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
            NalUnitType::EndOfSeq => {
                self.finalize_pending_picture()?;
            }
            NalUnitType::EndOfStream => {
                self.finalize_pending_picture()?;
            }
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

    /// Allocates a fresh `CurrentPicture` for the picture that begins with
    /// this slice: computes POC, builds an empty `DpbPicture` frame, and
    /// pre-allocates picture-wide per-MB state. Does not mutate the DPB.
    fn start_new_picture(
        &mut self,
        slice: &Slice,
        nal: &nal::NalHeader,
    ) -> Result<CurrentPicture, DecodingError> {
        let sps = &slice.sps;
        let header = &slice.header;

        let frame = Arc::new(VideoFrame::new_with_padding(
            sps.pic_width(),
            sps.pic_height(),
            v_frame::pixel::ChromaSampling::Cs420,
            16,
        ));

        let disposition = if nal.nal_unit_type == nal::NalUnitType::IDRSlice {
            ReferenceDisposition::Idr
        } else if nal.nal_ref_idc != 0 {
            ReferenceDisposition::NonIdrReference
        } else {
            ReferenceDisposition::NonReference
        };

        let pic_order_cnt = self.poc_state.calculate_poc(slice, disposition);
        let crop_dims = sps.crop_dimensions();

        let pic = Picture {
            frame,
            frame_num: header.frame_num,
            pic_order_cnt,
            motion_field: None,
            crop: crop_dims,
        };
        let dpb_pic = DpbPicture {
            picture: pic,
            marking: if nal.nal_ref_idc != 0 {
                DpbMarking::UsedForShortTermReference
            } else {
                DpbMarking::UnusedForReference
            },
            is_idr: nal.nal_unit_type == nal::NalUnitType::IDRSlice,
            structure: super::dpb::DpbPictureStructure::Frame,
            needed_for_output: true,
        };

        let pic_size = sps.pic_size_in_mbs();

        Ok(CurrentPicture {
            dpb_pic,
            disposition,
            pic_parameter_set_id: header.pic_parameter_set_id,
            frame_num: header.frame_num,
            field_pic_flag: header.field_pic_flag,
            nal_ref_idc_nonzero: nal.nal_ref_idc != 0,
            nal_unit_type_is_idr: nal.nal_unit_type == nal::NalUnitType::IDRSlice,
            idr_pic_id: header.idr_pic_id,
            pic_order_cnt_lsb: header.pic_order_cnt_lsb,
            delta_pic_order_cnt_bottom: header.delta_pic_order_cnt_bottom,
            delta_pic_order_cnt: header.delta_pic_order_cnt,
            first_slice_header: header.clone(),
            sps: sps.clone(),
            pps: slice.pps.clone(),
            macroblocks: self.mb_pool.acquire(pic_size),
            mb_slice_id: vec![u16::MAX; pic_size],
            // Motion-field arrays are only consulted when this picture is later
            // used as a colocated reference for B-slice temporal direct
            // prediction. Non-reference pictures can never be colocated, so
            // skip the allocation and population work entirely.
            mb_motion: if disposition == ReferenceDisposition::NonReference {
                Vec::new()
            } else {
                vec![macroblock::MbMotion::default(); pic_size]
            },
            mb_is_intra: if disposition == ReferenceDisposition::NonReference {
                BitVec::default()
            } else {
                BitVec::zeros(pic_size)
            },
            slice_deblock: Vec::new(),
            slice_ref_pocs: Vec::new(),
            slices_seen: 0,
            next_mb_addr: 0,
        })
    }

    /// Builds reference lists, parses the slice's coded data, decodes its
    /// macroblocks into the current picture's frame, and captures per-slice
    /// metadata (deblock parameters, ref-list POCs) while DPB indices are
    /// still valid -- finalize-time consumers (picture-level deblocking and
    /// motion field) need POCs that reflect the DPB state at decode time.
    fn decode_slice_into_current(
        &mut self,
        current: &mut CurrentPicture,
        slice: &mut Slice,
        input: &mut parser::BitReader,
    ) -> Result<(), DecodingError> {
        let pic_order_cnt = current.dpb_pic.picture.pic_order_cnt;

        // Section 7.4.3: `first_mb_in_slice` is strictly increasing across
        // slices of a picture (raster order). Reject streams whose slices
        // overlap or move backwards -- they would silently overwrite already
        // decoded MBs.
        if slice.header.first_mb_in_slice < current.next_mb_addr {
            return Err(DecodingError::MisformedData(format!(
                "slice first_mb_in_slice={} would overlap already-decoded MBs (next expected {})",
                slice.header.first_mb_in_slice, current.next_mb_addr
            )));
        }

        // Section 7.4.1.2.4: picture-scope fields must agree across all slices
        // of a picture. `is_new_picture` would have started a fresh picture
        // when these differ, so a mismatch here means a decoder bug rather
        // than a bitstream issue -- debug-only check.
        debug_assert!(
            current.slices_seen == 0
                || (slice.header.frame_num == current.frame_num
                    && slice.header.pic_parameter_set_id == current.pic_parameter_set_id
                    && slice.header.field_pic_flag == current.field_pic_flag),
            "picture-scope slice header fields drifted across slices"
        );

        // Section 7.4.3: when present in multiple slices of the same picture,
        // `dec_ref_pic_marking` must be identical. Only the first slice's
        // marking is consulted at finalize time, so this catches malformed
        // streams that disagree across slices.
        debug_assert!(
            current.slices_seen == 0
                || slice.header.dec_ref_pic_marking
                    == current.first_slice_header.dec_ref_pic_marking,
            "dec_ref_pic_marking mismatch across slices of the same picture"
        );

        // Section 8.2.4: reference picture lists are constructed before
        // parsing -- temporal direct prediction (used during B-slice parsing)
        // needs the colocated picture, and the current picture is not yet in
        // the DPB.
        self.construct_ref_pic_list0(slice, pic_order_cnt)?;
        if slice.header.slice_type == SliceType::B {
            self.construct_ref_pic_list1(slice, pic_order_cnt)?;
            self.setup_colocated_pic_info(slice, pic_order_cnt);
        }

        // Resolve the constructed ref-list indices to POCs while DPB indices
        // are still valid (MMCO at finalize may invalidate them). L0 lives on
        // the slice because `map_col_to_list0` consults it during the parse
        // below; L1 has no in-parse consumer, so it goes straight into a
        // local that ends up in `current.slice_ref_pocs` for later use by
        // deblocking and future colocated-picture lookups.
        slice.ref_pic_list0_pocs = slice
            .ref_pic_list0
            .iter()
            .filter_map(|&idx| self.dpb.pictures.get(idx).map(|p| p.picture.pic_order_cnt))
            .collect();
        let l1_pocs: Vec<i32> = slice
            .ref_pic_list1
            .iter()
            .filter_map(|&idx| self.dpb.pictures.get(idx).map(|p| p.picture.pic_order_cnt))
            .collect();

        parser::parse_slice_data(input, slice, &mut self.residual_pool)
            .map_err(DecodingError::MisformedData)?;
        let pic_size = current.macroblocks.len();
        let mb_count = slice.get_macroblock_count();
        let end_mb = (slice.header.first_mb_in_slice as usize).saturating_add(mb_count);
        if end_mb > pic_size {
            return Err(DecodingError::MisformedData(format!(
                "slice extends past picture: first_mb_in_slice={}, count={}, pic_size_in_mbs={}",
                slice.header.first_mb_in_slice, mb_count, pic_size
            )));
        }

        self.process_slice_into_picture(slice, current)?;

        current.slice_deblock.push(SliceDeblockParams {
            idc: slice.header.deblocking_filter_idc,
            alpha_c0_offset_div2: slice.header.slice_alpha_c0_offset_div2,
            beta_offset_div2: slice.header.slice_beta_offset_div2,
        });
        current
            .slice_ref_pocs
            .push((std::mem::take(&mut slice.ref_pic_list0_pocs), l1_pocs));
        current.slices_seen += 1;
        current.next_mb_addr =
            slice.header.first_mb_in_slice + slice.get_macroblock_count() as MbAddr;

        Ok(())
    }

    /// Applies MMCO / `dec_ref_pic_marking`, drops dead references, stores the
    /// picture in the DPB (with output bumping), and updates POC state.
    /// Consumes the `CurrentPicture`.
    fn finalize_picture(&mut self, mut current: CurrentPicture) -> Result<(), DecodingError> {
        // Every MB must have been decoded by some slice. Truncated slices are
        // caught earlier by `parse_slice_data`, but a missing slice (multi-slice
        // picture where one slice never arrived) leaves sentinels behind that
        // would silently produce a corrupted frame. `macroblocks` and
        // `mb_slice_id` are written together, so checking either suffices.
        if current.mb_slice_id.iter().any(|&id| id == u16::MAX) {
            return Err(DecodingError::MisformedData(
                "picture has macroblocks with no slice owner at finalize".into(),
            ));
        }
        debug_assert!(current.macroblocks.iter().all(Option::is_some));

        // Section 8.7: deblocking is a picture-level pass. Run it before any
        // DPB mutation so the stored frame reflects the filtered samples.
        // Use the PPS captured at picture start; the context-resident PPS
        // with the same id may have been overwritten by the next picture's
        // PPS update by the time finalize runs.
        let pic_width_in_mbs = current.sps.pic_width_in_mbs();
        let pic_height_in_mbs = (current.sps.pic_height_in_map_units_minus1 + 1) as usize;
        let deblock_input = deblocking::PictureDeblockInput {
            sps: &current.sps,
            pps: &current.pps,
            macroblocks: &current.macroblocks,
            mb_slice_id: &current.mb_slice_id,
            slice_deblock: &current.slice_deblock,
            slice_ref_pocs: &current.slice_ref_pocs,
            pic_width_in_mbs,
            pic_height_in_mbs,
        };
        deblocking::filter_picture(
            &deblock_input,
            Arc::get_mut(&mut current.dpb_pic.picture.frame)
                .expect("frame uniquely owned during deblocking"),
        );

        // Attach motion field for use as a colocated picture in future B
        // slices. POCs were captured per-slice in `decode_slice_into_current`
        // before any DPB mutation, so the lookups they enable remain valid
        // after MMCO runs below.
        current.dpb_pic.picture.motion_field = self.build_motion_field(&mut current).map(Arc::new);

        // Section C.2.3: Mark references + remove dead pictures (before storage).
        // Per Section 7.4.3, `dec_ref_pic_marking` (and the rest of the
        // picture-scope slice-header fields) must agree across all slices of
        // a picture, so applying the first slice's header here is well-defined
        // for both single- and multi-slice pictures.
        let (has_mmco5, flushed) = self.dpb.mark_prior_references(
            &current.first_slice_header,
            current.disposition,
            &current.sps,
            &mut current.dpb_pic,
        );
        self.output_pictures.extend(flushed);
        self.dpb.remove_dead_pictures();

        // Section C.2.4: Store current picture (with bumping if DPB is full).
        let pictures = self.dpb.store_picture(current.dpb_pic);
        self.output_pictures.extend(pictures);

        // Reclaim per-MB Residual boxes and the macroblock array allocation
        // before `current` drops. The DPB only keeps the decoded frame and
        // motion field; the array itself isn't retained, so its capacity
        // returns to the pool for the next picture.
        for slot in current.macroblocks.iter_mut() {
            if let Some(mb) = slot {
                let residual = match mb {
                    Macroblock::I(m) => m.residual.take(),
                    Macroblock::P(m) => m.residual.take(),
                    Macroblock::B(m) => m.residual.take(),
                    Macroblock::PCM(_) => None,
                };
                if let Some(r) = residual {
                    self.residual_pool.release(r);
                }
            }
        }
        self.mb_pool.release(std::mem::take(&mut current.macroblocks));

        self.poc_state.update_mmco5_state(
            has_mmco5,
            current.disposition != ReferenceDisposition::NonReference,
        );
        Ok(())
    }

    /// Retrieves the next available picture from the decoder's output queue.
    /// Returns `Some(Picture)` if a picture is available, or `None` if the queue is empty.
    pub fn retrieve_picture(&mut self) -> Option<Picture> {
        self.output_pictures.pop_front()
    }

    /// Flushes the decoder, forcing any remaining frames in the DPB to be output.
    /// This is necessary because some frames may be held in the DPB (Decoded Picture Buffer)
    /// for reference or reordering (e.g., B-frames) and won't be output immediately.
    /// This should be called at the end of the stream.
    /// Call `retrieve_picture` repeatedly after flushing until it returns `None`.
    pub fn flush(&mut self) -> Result<(), DecodingError> {
        self.finalize_pending_picture()?;
        let pictures = self.dpb.flush();
        self.output_pictures.extend(pictures.into_iter().map(|p| p.picture));
        Ok(())
    }

    /// Finalizes the in-progress picture, if any. No-op when no picture is
    /// being assembled. Used to drain the boundary state machine on flush,
    /// `EndOfSeq`, or `EndOfStream`.
    fn finalize_pending_picture(&mut self) -> Result<(), DecodingError> {
        if let Some(cur) = self.current_picture.take() {
            self.finalize_picture(cur)?;
        }
        Ok(())
    }

    /// Performs prediction and reconstruction for the slice, writing samples
    /// into the in-progress picture's frame and recording per-MB state into
    /// `current` so later picture-level passes (deblocking, motion field) can
    /// see all slices of the picture together.
    fn process_slice_into_picture(
        &mut self,
        slice: &mut Slice,
        current: &mut CurrentPicture,
    ) -> Result<(), DecodingError> {
        let frame = Arc::get_mut(&mut current.dpb_pic.picture.frame)
            .expect("frame uniquely owned during decode");
        let qp_bd_offset_y = 6 * slice.sps.bit_depth_luma_minus8 as i32;
        let qp_bd_offset_c = 6 * slice.sps.bit_depth_chroma_minus8 as i32;
        let mut qp = slice.pps.pic_init_qp_minus26 + 26 + slice.header.slice_qp_delta;
        let ref_pics_l0 =
            resolve_ref_pic_list(&slice.ref_pic_list0, &self.dpb.pictures, "ref_pic_list0")?;
        let ref_pics_l1 =
            resolve_ref_pic_list(&slice.ref_pic_list1, &self.dpb.pictures, "ref_pic_list1")?;

        // Resolve the active (picture-level) scaling matrix from SPS + PPS per
        // clauses 7.4.2.1.1.1 (rule A) and 7.4.2.2.1 (rule B). Falls back to
        // all-flat-16 when neither signals a custom matrix.
        let active_scaling_matrix: ResolvedScalingMatrix = {
            let sps_resolved = resolve_seq_scaling_matrix(
                slice.sps.seq_scaling_matrix.as_ref(),
                slice.sps.ChromaArrayType(),
            );
            resolve_pic_scaling_matrix(
                &sps_resolved,
                slice.sps.seq_scaling_matrix.is_some(),
                slice.pps.pic_scaling_matrix.as_ref(),
                slice.pps.transform_8x8_mode_flag,
                slice.sps.ChromaArrayType(),
            )
        };
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
                            return Err(DecodingError::FeatureNotSupported(
                                "monochrome chroma format is not supported".into(),
                            ));
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
                        qp = next_qp(qp, imb.mb_qp_delta, qp_bd_offset_y);
                        let residuals = restore_residuals(
                            imb.residual.as_deref(),
                            ColorPlane::Y,
                            qp as u8,
                            &active_scaling_matrix,
                        );

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
                                render_luma_8x8_intra_prediction(
                                    slice, mb_addr, imb, mb_loc, luma_plane, &residuals,
                                );
                            }
                            MbPredictionMode::Intra_16x16 => {
                                render_luma_16x16_intra_prediction(
                                    slice,
                                    mb_addr,
                                    mb_loc,
                                    luma_plane,
                                    mb_type_to_16x16_pred_mode(imb.mb_type).ok_or_else(|| {
                                        DecodingError::OutOfRange(format!(
                                            "no 16x16 pred mode for mb_type {:?}",
                                            imb.mb_type
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
                            let chroma_qp = get_chroma_qp(qp, qp_offset, qp_bd_offset_c);
                            let chroma_plane = &mut frame.planes[plane_name as usize];
                            let residuals = restore_residuals(
                                imb.residual.as_deref(),
                                plane_name,
                                chroma_qp,
                                &active_scaling_matrix,
                            );
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
                        qp = next_qp(qp, block.mb_qp_delta, qp_bd_offset_y);
                        let residuals = restore_residuals(
                            block.residual.as_deref(),
                            ColorPlane::Y,
                            qp as u8,
                            &active_scaling_matrix,
                        );

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
                            let chroma_qp = get_chroma_qp(qp, qp_offset, qp_bd_offset_c);
                            let residuals = restore_residuals(
                                block.residual.as_deref(),
                                plane_name,
                                chroma_qp,
                                &active_scaling_matrix,
                            );
                            render_chroma_inter_prediction(
                                slice,
                                block,
                                mb_loc,
                                plane_name,
                                frame,
                                &residuals,
                                &ref_pics_l0,
                            )?;
                        }
                    }
                    Macroblock::B(block) => {
                        qp = next_qp(qp, block.mb_qp_delta, qp_bd_offset_y);
                        let residuals = restore_residuals(
                            block.residual.as_deref(),
                            ColorPlane::Y,
                            qp as u8,
                            &active_scaling_matrix,
                        );

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
                            let chroma_qp = get_chroma_qp(qp, qp_offset, qp_bd_offset_c);
                            let residuals = restore_residuals(
                                block.residual.as_deref(),
                                plane_name,
                                chroma_qp,
                                &active_scaling_matrix,
                            );
                            render_chroma_inter_prediction_b(
                                slice,
                                block,
                                mb_loc,
                                plane_name,
                                frame,
                                &residuals,
                                &ref_pics_l0,
                                &ref_pics_l1,
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

        // Drain decoded macroblocks into picture-wide state consumed at
        // finalize: `mb_slice_id` drives cross-slice deblocking suppression
        // and `mb_motion` / `mb_is_intra` feed picture-level motion field
        // assembly. Nothing reads `slice.macroblocks` after this point.
        // Motion-field writes are skipped for non-reference pictures; they
        // can never be used as a colocated reference.
        let first_mb_addr = slice.header.first_mb_in_slice as usize;
        let slices_seen = current.slices_seen;
        let needs_motion_field = current.disposition != ReferenceDisposition::NonReference;
        for (i, mb) in slice.take_macroblocks().into_iter().enumerate() {
            let mb_addr = first_mb_addr + i;
            if needs_motion_field {
                current.mb_motion[mb_addr] = mb.get_motion_info();
                current
                    .mb_is_intra
                    .set(mb_addr, matches!(&mb, Macroblock::I(_) | Macroblock::PCM(_)));
            }
            current.mb_slice_id[mb_addr] = slices_seen;
            current.macroblocks[mb_addr] = Some(mb);
        }

        Ok(())
    }

    // Section 8.2.4 Decoding process for reference picture lists construction (List 0):
    // initialize per 8.2.4.2 then apply ref_pic_list_modification reordering per 8.2.4.3.
    // Picture numbers (8.2.4.1) are computed inline in the initializers.
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
        self.modify_ref_pic_list(
            slice,
            &slice.header.ref_pic_list_modification.list0,
            &mut ref_list0,
        );

        // Truncate to num_ref_idx_l0_active_minus1 + 1
        let len = (slice.header.num_ref_idx_l0_active_minus1 + 1) as usize;
        if ref_list0.len() > len {
            ref_list0.truncate(len);
        }

        slice.ref_pic_list0 = ref_list0.into_vec();
        Ok(())
    }

    // Section 8.2.4.2 Initialization process for reference picture lists
    fn initialize_ref_pic_list0(&self, slice: &Slice) -> SmallVec<[usize; 16]> {
        let max_frame_num = 1 << (slice.sps.log2_max_frame_num_minus4 + 4);
        let curr_frame_num = slice.header.frame_num as i32;

        let mut short_term_refs: SmallVec<[(usize, i32); 16]> = SmallVec::new();
        let mut long_term_refs: SmallVec<[(usize, usize); 16]> = SmallVec::new();

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
                    long_term_refs.push((i, lt_idx as usize));
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

    // Section 8.2.4.3 Reordering process for reference picture lists. Same
    // algorithm for list0 and list1 -- caller passes the matching modifications
    // and target list.
    fn modify_ref_pic_list(
        &self,
        slice: &Slice,
        modifications: &[RefPicListModification],
        ref_list: &mut SmallVec<[usize; 16]>,
    ) {
        if modifications.is_empty() {
            return;
        }

        let max_frame_num = 1 << (slice.sps.log2_max_frame_num_minus4 + 4);
        let curr_frame_num = slice.header.frame_num as i32;
        let mut pic_num_lx_pred = curr_frame_num;
        let mut ref_idx = 0;

        for modification in modifications {
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

                    if let Some(idx) = self.find_short_term_in_dpb(pic_num_lx) {
                        self.place_picture_in_list(ref_list, idx, ref_idx);
                        ref_idx += 1;
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

                    if let Some(idx) = self.find_short_term_in_dpb(pic_num_lx) {
                        self.place_picture_in_list(ref_list, idx, ref_idx);
                        ref_idx += 1;
                    }
                }
                RefPicListModification::RemapLongTerm(long_term_pic_num) => {
                    if let Some(idx) = self.find_long_term_in_dpb(*long_term_pic_num) {
                        self.place_picture_in_list(ref_list, idx, ref_idx);
                        ref_idx += 1;
                    }
                }
            }
        }
    }

    fn place_picture_in_list(&self, list: &mut SmallVec<[usize; 16]>, pic_idx: usize, ref_idx: usize) {
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
            // refIdxL0 starts at 0 and increments per spec, so an out-of-bounds
            // index must be exactly `list.len()` -- append.
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
    fn initialize_ref_pic_list0_b(&self, slice: &Slice, current_poc: i32) -> SmallVec<[usize; 16]> {
        let mut short_term_le: SmallVec<[(usize, i32); 16]> = SmallVec::new(); // POC <= current
        let mut short_term_gt: SmallVec<[(usize, i32); 16]> = SmallVec::new(); // POC > current
        let mut long_term_refs: SmallVec<[(usize, usize); 16]> = SmallVec::new();

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
                    long_term_refs.push((i, lt_idx as usize));
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
    fn initialize_ref_pic_list1(&self, slice: &Slice, current_poc: i32) -> SmallVec<[usize; 16]> {
        let mut short_term_gt: SmallVec<[(usize, i32); 16]> = SmallVec::new(); // POC > current
        let mut short_term_le: SmallVec<[(usize, i32); 16]> = SmallVec::new(); // POC <= current
        let mut long_term_refs: SmallVec<[(usize, usize); 16]> = SmallVec::new();

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
                    long_term_refs.push((i, lt_idx as usize));
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
        if ref_list1.len() > 1 && ref_list1.as_slice() == slice.ref_pic_list0.as_slice() {
            ref_list1.swap(0, 1);
        }

        self.modify_ref_pic_list(
            slice,
            &slice.header.ref_pic_list_modification.list1,
            &mut ref_list1,
        );

        let len = (slice.header.num_ref_idx_l1_active_minus1 + 1) as usize;
        if ref_list1.len() > len {
            ref_list1.truncate(len);
        }

        slice.ref_pic_list1 = ref_list1.into_vec();
        Ok(())
    }

    /// Set up colocated picture info on the slice for temporal direct prediction.
    fn setup_colocated_pic_info(&self, slice: &mut Slice, current_poc: i32) {
        slice.current_pic_poc = current_poc;

        if slice.ref_pic_list1.is_empty() {
            return;
        }
        let col_dpb_idx = slice.ref_pic_list1[0];
        if let Some(col_pic) = self.dpb.pictures.get(col_dpb_idx) {
            if let Some(ref mf) = col_pic.picture.motion_field {
                slice.col_pic = Some(slice::ColPicInfo {
                    motion: Arc::clone(mf),
                    pic_poc: col_pic.picture.pic_order_cnt,
                    ref_l1_0_is_short_term: col_pic.marking.is_short_term(),
                });
            }
        }
    }

    /// Drains `current`'s per-MB motion arrays into a `MotionFieldStorage`
    /// stashed on the DPB picture for later use as a colocated reference by
    /// B-slice temporal direct prediction. Returns `None` for non-reference
    /// pictures. After return, `current.mb_motion` etc. are empty.
    fn build_motion_field(&self, current: &mut CurrentPicture) -> Option<MotionFieldStorage> {
        if current.disposition == ReferenceDisposition::NonReference {
            return None;
        }
        Some(MotionFieldStorage {
            mb_motion: std::mem::take(&mut current.mb_motion),
            mb_is_intra: std::mem::take(&mut current.mb_is_intra),
            mb_slice_id: std::mem::take(&mut current.mb_slice_id),
            slice_ref_pocs: std::mem::take(&mut current.slice_ref_pocs),
        })
    }
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

// Section 8.5: produce restored residual blocks for one plane, or empty if
// the macroblock carries no residual (skipped, or all-zero CBP).
#[inline]
fn restore_residuals(
    residual: Option<&Residual>,
    plane: ColorPlane,
    qp: u8,
    scaling: &ResolvedScalingMatrix,
) -> SmallVec<[Block4x4; 16]> {
    match residual {
        Some(r) => r.restore(plane, qp, scaling),
        None => SmallVec::new(),
    }
}

// Section 7.4.5.1 / 8.5.10: apply mb_qp_delta with wraparound across the
// quantizer range [-qp_bd_offset_y, 51].
#[inline]
fn next_qp(qp: i32, mb_qp_delta: i32, qp_bd_offset_y: i32) -> i32 {
    (qp + mb_qp_delta + 52 + 2 * qp_bd_offset_y) % (52 + qp_bd_offset_y) - qp_bd_offset_y
}

// Section 8.5.8 Derivation process for chroma quantization parameters.
// The result is statically in u8 range: qp_c is in [-qp_bd_offset_c, 51], and
// qp_bd_offset_c = 6 * bit_depth_chroma_minus8 in [0, 36], so qp_c + qp_bd_offset_c
// in [0, 87].
pub fn get_chroma_qp(luma_qp: i32, chroma_qp_offset: i32, qp_bd_offset_c: i32) -> u8 {
    // Table 8-15: identity for qp_i in 0..30, then the spec mapping for 30..52.
    const TABLE: [u8; 52] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 29, 30, 31, 32, 32, 33, 34, 34, 35, 35, 36, 36, 37, 37, 38, 38, 38, 39,
        39, 39, 39, 39,
    ];
    let qp_i = (luma_qp + chroma_qp_offset).clamp(-qp_bd_offset_c, 51);
    let qp_c = if qp_i < 0 { qp_i } else { TABLE[qp_i as usize] as i32 };
    (qp_c + qp_bd_offset_c) as u8
}

