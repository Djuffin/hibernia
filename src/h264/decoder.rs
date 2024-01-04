use crate::h264::slice::SliceType;
use crate::h264::ColorPlane;

use super::macroblock::{
    self, get_4x4luma_block_location, get_4x4luma_block_neighbor, IMb, Intra_16x16_SamplePredMode,
    Intra_4x4_SamplePredMode, Intra_Chroma_Pred_Mode, Macroblock, MbAddr, MbNeighborName,
    MbPredictionMode,
};
use super::residual::{level_scale_4x4_block, transform_4x4, unzip_block_4x4};
use super::{nal, parser, pps, slice, sps, tables, ChromaFormat, Point};
use log::info;
use slice::Slice;
use v_frame::frame;
use v_frame::plane::{self, PlaneOffset};

type VideoFrame = frame::Frame<u8>;
type Plane = v_frame::plane::Plane<u8>;

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

#[derive(Debug, Default)]
pub struct Decoder {
    context: DecoderContext,
    frame_buffer: Option<VideoFrame>,
}

impl Decoder {
    pub fn new() -> Decoder {
        Decoder::default()
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
            let nal_vec = parser::remove_emulation_if_needed(nal_buffer);
            let mut unit_input = if nal_vec.is_empty() {
                parser::BitReader::new(nal_buffer)
            } else {
                parser::BitReader::new(nal_vec.as_slice())
            };
            input.skip((nal_size_bytes * 8) as u32).map_err(parse_error_handler)?;

            match nal.nal_unit_type {
                NalUnitType::Unspecified => {}
                NalUnitType::SliceDataA => {}
                NalUnitType::SliceDataB => {}
                NalUnitType::SliceDataC => {}
                NalUnitType::NonIDRSlice => {
                    let mut slice =
                        parser::parse_slice_header(&self.context, &nal, &mut unit_input)
                            .map_err(parse_error_handler)?;

                    info!("non-IDR Slice: {:#?}", slice);
                    if slice.header.slice_type != SliceType::I {
                        break;
                    }
                    parser::parse_slice_data(&mut unit_input, &mut slice)
                        .map_err(parse_error_handler)?;
                    info!("Blocks: {:#?}", slice.get_macroblock_count());
                    self.process_slice(&mut slice)?;
                }
                NalUnitType::IDRSlice => {
                    let mut slice =
                        parser::parse_slice_header(&self.context, &nal, &mut unit_input)
                            .map_err(parse_error_handler)?;

                    info!("IDR Slice: {:#?}", slice);
                    parser::parse_slice_data(&mut unit_input, &mut slice)
                        .map_err(parse_error_handler)?;
                    info!("Blocks: {:#?}", slice.get_macroblock_count());
                    return self.process_slice(&mut slice); // Temporarily stop after first slice
                }
                NalUnitType::SupplementalEnhancementInfo => {}
                NalUnitType::SeqParameterSet => {
                    let sps = parser::parse_sps(&mut unit_input).map_err(parse_error_handler)?;
                    info!("SPS: {:#?}", sps);
                    info!("Data {:?}", nal_buffer);
                    assert_eq!(sps.ChromaArrayType(), ChromaFormat::YUV420);
                    let frame = VideoFrame::new_with_padding(
                        sps.pic_width(),
                        sps.pic_hight(),
                        v_frame::pixel::ChromaSampling::Cs420,
                        0,
                    );
                    self.frame_buffer = Some(frame);
                    self.context.put_sps(sps);
                }
                NalUnitType::PicParameterSet => {
                    let pps = parser::parse_pps(&mut unit_input).map_err(parse_error_handler)?;
                    info!("PPS: {:#?}", pps);
                    info!("Data {:?}", nal_buffer);
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

    pub fn get_frame_buffer(&self) -> Option<&VideoFrame> {
        self.frame_buffer.as_ref()
    }

    fn process_slice(&mut self, slice: &mut Slice) -> Result<(), DecodingError> {
        if self.frame_buffer.is_none() {
            return Err(DecodingError::Wtf);
        }
        let mut qp = slice.pps.pic_init_qp_minus26 + 26 + slice.header.slice_qp_delta;
        let frame = self.frame_buffer.as_mut().unwrap();
        for mb_addr in 0..(slice.sps.pic_size_in_mbs() as u32) {
            let mb_loc = slice.get_mb_location(mb_addr);
            if let Some(mb) = slice.get_mb(mb_addr) {
                match mb {
                    Macroblock::PCM(block) => {
                        let y_plane = &mut frame.planes[0];
                        let mut plane_slice = y_plane.mut_slice(point_to_plain_offset(mb_loc));

                        for (idx, row) in
                            plane_slice.rows_iter_mut().take(tables::MB_HEIGHT).enumerate()
                        {
                            let row_range = idx * tables::MB_WIDTH..(idx + 1) * tables::MB_WIDTH;
                            row[..tables::MB_WIDTH]
                                .copy_from_slice(&block.pcm_sample_luma[row_range]);
                        }
                    }
                    Macroblock::I(imb) => {
                        qp = (qp + imb.mb_qp_delta).clamp(0, 51);
                        let qp = qp.try_into().unwrap();
                        if let Some(residual) = imb.residual.as_ref() {
                            let blocks = residual.restore(ColorPlane::Y, qp);
                            info!(
                                "MB {mb_addr} {qp} {:?} {:?}",
                                residual.prediction_mode,
                                blocks.len()
                            );

                            let y_plane = &mut frame.planes[0];

                            for (blk_idx, blk) in blocks.iter().enumerate() {
                                let mut blk_loc = get_4x4luma_block_location(blk_idx as u8);
                                blk_loc.x += mb_loc.x;
                                blk_loc.y += mb_loc.y;

                                let mut plane_slice =
                                    y_plane.mut_slice(point_to_plain_offset(blk_loc));
                                info!("  blk:{blk_idx} {blk_loc:?} {:?} ", blk.samples);
                                for (idx, row) in plane_slice.rows_iter_mut().take(4).enumerate() {
                                    for i in 0..4 {
                                        row[i] = blk.samples[idx][i]
                                            .abs()
                                            .clamp(0, 255)
                                            .try_into()
                                            .unwrap();
                                    }
                                }
                            }
                        }
                    }
                    Macroblock::P(block) => {
                        todo!("implement P blocks!");
                    }
                }
            }
        }
        Ok(())
    }
}

fn point_to_plain_offset(p: Point) -> PlaneOffset {
    PlaneOffset { x: p.x as isize, y: p.y as isize }
}

pub fn render_luma_16x16_intra_prediction(
    slice: &Slice,
    mb_addr: MbAddr,
    loc: Point,
    target: &mut Plane,
    mode: Intra_16x16_SamplePredMode,
) {
    let x = loc.x as usize;
    let y = loc.y as usize;
    match mode {
        Intra_16x16_SamplePredMode::Intra_16x16_Vertical => {
            // Section 8.3.3.1 Specification of Intra_16x16_Vertical prediction mode
            let mut src_row = [0; 16];
            let offset = point_to_plain_offset(loc);
            src_row.copy_from_slice(&target.row(y as isize - 1)[x..(x + 16)]);
            let mut target_slice = target.mut_slice(point_to_plain_offset(loc));
            for row in target_slice.rows_iter_mut().take(16) {
                row.copy_from_slice(&src_row);
            }
        }
        Intra_16x16_SamplePredMode::Intra_16x16_Horizontal => {
            // Section 8.3.3.2 Specification of Intra_16x16_Horizontal prediction mode
            let mut offset = point_to_plain_offset(loc);
            offset.x -= 1;
            let mut target_slice = target.mut_slice(offset);
            for row in target_slice.rows_iter_mut().take(16) {
                let src = row[0];
                row[1..=16].fill(src);
            }
        }
        Intra_16x16_SamplePredMode::Intra_16x16_DC => {
            // Section 8.3.3.3 Specification of Intra_16x16_DC prediction mode
            let offset = point_to_plain_offset(loc);

            // Calculate the sum of all the values at the left of the current macroblock
            let sum_a = if slice.get_mb_neighbor(mb_addr, MbNeighborName::A).is_some() {
                let target_slice = target.slice(PlaneOffset { x: offset.x - 1, ..offset });
                Some(target_slice.rows_iter().take(16).map(|r| r[0] as u32).sum::<u32>())
            } else {
                None
            };

            // Calculate the sum of all the values at the top of the current macroblock
            let sum_b = if slice.get_mb_neighbor(mb_addr, MbNeighborName::B).is_some() {
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
        Intra_16x16_SamplePredMode::Intra_16x16_Plane => todo!(),
    }
}
