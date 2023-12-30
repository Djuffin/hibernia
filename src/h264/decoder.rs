use crate::h264::slice::SliceType;

use super::macroblock::{self, Macroblock};
use super::residual::{level_scale_4x4_block, transform_4x4, unzip_block_4x4};
use super::{nal, parser, pps, slice, sps, tables, ChromaFormat, Point};
use log::info;
use slice::Slice;
use v_frame::frame;
use v_frame::plane::PlaneOffset;

type VideoFrame = frame::Frame<u8>;

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
        let parse_error_handler = |e| DecodingError::MisformedData(e);
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
                    info!("Blocks: {:#?}", slice.get_block_count());
                    self.process_slice(&mut slice)?;
                }
                NalUnitType::IDRSlice => {
                    let mut slice =
                        parser::parse_slice_header(&self.context, &nal, &mut unit_input)
                            .map_err(parse_error_handler)?;

                    info!("IDR Slice: {:#?}", slice);
                    parser::parse_slice_data(&mut unit_input, &mut slice)
                        .map_err(parse_error_handler)?;
                    info!("Blocks: {:#?}", slice.get_block_count());
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
            let mb_location = slice.get_mb_location(mb_addr);
            if let Some(mb) = slice.get_mb(mb_addr) {
                match mb {
                    Macroblock::PCM(block) => {
                        let y_plane = &mut frame.planes[0];
                        let mut plane_slice =
                            y_plane.mut_slice(point_to_plain_offset(&mb_location));

                        for (idx, row) in
                            plane_slice.rows_iter_mut().take(tables::MB_HEIGHT).enumerate()
                        {
                            let row_range = idx * tables::MB_WIDTH..(idx + 1) * tables::MB_WIDTH;
                            row[..tables::MB_WIDTH]
                                .copy_from_slice(&block.pcm_sample_luma[row_range]);
                        }
                    }
                    Macroblock::I(imb) => {
                        qp += imb.mb_qp_delta;
                        let qp = qp.clamp(0, 51).try_into().unwrap();
                        if let Some(residual) = imb.residual.as_ref() {
                            let blocks = residual.restore(super::ColorPlane::Y, qp);
                            info!("MB {mb_addr} {qp} {:?}", blocks.len());

                            let y_plane = &mut frame.planes[0];

                            for (blk_idx, blk) in blocks.iter().enumerate() {
                                let mut origin =
                                    macroblock::get_4x4luma_block_location(blk_idx as u8);
                                origin.x += mb_location.x;
                                origin.y += mb_location.y;

                                let mut plane_slice =
                                    y_plane.mut_slice(point_to_plain_offset(&mb_location));

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
        return Ok(());
    }
}

fn point_to_plain_offset(p: &Point) -> PlaneOffset {
    PlaneOffset { x: p.x as isize, y: p.y as isize }
}
