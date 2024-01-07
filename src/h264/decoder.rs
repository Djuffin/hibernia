use std::cmp::{max, min};
use std::io::Read;

use crate::h264::slice::SliceType;
use crate::h264::tables::mb_type_to_16x16_pred_mode;
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
use v_frame::plane::{self, PlaneOffset, PlaneSlice};

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
                        16,
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
                            info!(
                                "MB {mb_addr} {qp} {:?}",
                                residual.prediction_mode
                            );

                            let y_plane = &mut frame.planes[0];

                            let mode = imb.MbPartPredMode(0);
                            match mode {
                                MbPredictionMode::None => panic!("impossible pred mode"),
                                MbPredictionMode::Intra_4x4 => {
                                    render_luma_4x4_intra_prediction(
                                        &slice, mb_addr, &imb, mb_loc, y_plane,
                                    );
                                }
                                MbPredictionMode::Intra_8x8 => todo!("8x8 pred mode"),
                                MbPredictionMode::Intra_16x16 => {
                                    render_luma_16x16_intra_prediction(
                                        slice,
                                        mb_addr,
                                        mb_loc,
                                        y_plane,
                                        mb_type_to_16x16_pred_mode(imb.mb_type).unwrap(),
                                    );
                                }
                                MbPredictionMode::Pred_L0 => todo!(),
                                MbPredictionMode::Pred_L1 => todo!(),
                            }

                            let blocks = residual.restore(ColorPlane::Y, qp);
                            for (blk_idx, blk) in blocks.iter().enumerate() {
                                let mut blk_loc = get_4x4luma_block_location(blk_idx as u8);
                                blk_loc.x += mb_loc.x;
                                blk_loc.y += mb_loc.y;

                                let mut plane_slice =
                                    y_plane.mut_slice(point_to_plain_offset(blk_loc));
                                info!("  blk:{blk_idx} {blk_loc:?} {:?} ", blk.samples);
                                for (y, row) in plane_slice.rows_iter_mut().take(4).enumerate() {
                                    for (x, pixel) in row.iter_mut().take(4).enumerate() {
                                        *pixel = (*pixel as i32 + blk.samples[y][x])
                                            .abs()
                                            .clamp(0, 255)
                                            as u8;
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
    pub fn load(&mut self, plane: &Plane, blk_loc: Point) {
        let mut offset = point_to_plain_offset(blk_loc);
        offset.x -= 1;
        offset.y -= 1;

        if offset.y > 0 {
            let range = offset.y as usize..(offset.y as usize + 9);
            self.top_row.copy_from_slice(&plane.row(offset.y)[range]);
        } else {
            self.top_row.fill(0);
        }

        self.left_column[0] = self.top_row[0];
        offset.y += 1;

        if offset.x > 0 {
            let target_slice = plane.slice(offset);
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

// Section 8.3.1.1 Derivation process for Intra4x4PredMode
pub fn render_luma_4x4_intra_prediction(
    slice: &Slice,
    mb_addr: MbAddr,
    mb: &IMb,
    mb_loc: Point,
    target: &mut Plane,
) {
    #[inline]
    fn weighted_avg(double: u8, single_a: u8, single_b: u8) -> u8 {
        (2 * (double as u16) + (single_a as u16) + (single_b as u16) + 2 >> 2) as u8
    }

    #[inline]
    fn avg(a: u8, b: u8) -> u8 {
        (((a as u16) + (b as u16) + 1) >> 1) as u8
    }

    let mut ctx = Surroundings4x4::default();
    for blk_idx in 0..16u8 {
        let mut blk_loc = get_4x4luma_block_location(blk_idx as u8);
        blk_loc.x += mb_loc.x;
        blk_loc.y += mb_loc.y;
        let x = blk_loc.x as usize;
        let y = blk_loc.y as usize;
        ctx.load(target, blk_loc);

        let mode = mb.rem_intra4x4_pred_mode[blk_idx as usize];
        info!(" >{blk_idx}: {mode}");
        match mode {
            Intra_4x4_SamplePredMode::Vertical => {
                // Section 8.3.1.2.1 Specification of Intra_4x4_Vertical prediction mode
                let mut target_slice = target.mut_slice(ctx.offset);
                let src = ctx.top4();
                info!("   >Ver src: {src:?}");
                for row in target_slice.rows_iter_mut().take(4) {
                    row[0..4].copy_from_slice(src);
                }
            }
            Intra_4x4_SamplePredMode::Horizontal => {
                // Section 8.3.1.2.2 Specification of Intra_4x4_Horizontal prediction mode
                let mut target_slice = target.mut_slice(ctx.offset);
                let src = ctx.left4();
                info!("   >Hor src: {src:?}");
                for (idx, row) in target_slice.rows_iter_mut().take(4).enumerate() {
                    row[0..4].fill(src[idx]);
                }
            }
            Intra_4x4_SamplePredMode::DC => {
                // Section 8.3.1.2.3 Specification of Intra_4x4_DC prediction mode
                let offset = point_to_plain_offset(blk_loc);

                // Calculate the sum of all the values at the left of the current macroblock
                let same_mb = get_4x4luma_block_neighbor(blk_idx, MbNeighborName::A).1.is_none();
                let sum_a = if same_mb || slice.has_mb_neighbor(mb_addr, MbNeighborName::A) {
                    Some(ctx.left4().iter().map(|v| *v as u32).sum::<u32>())
                } else {
                    None
                };

                // Calculate the sum of all the values at the top of the current macroblock
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

                info!("   >DC sum: {sum}");
                let mut target_slice = target.mut_slice(offset);
                for row in target_slice.rows_iter_mut().take(4) {
                    row[0..4].fill(sum as u8);
                }
            }
            Intra_4x4_SamplePredMode::Diagonal_Down_Left => {
                // Section 8.3.1.2.4 Specification of Intra_4x4_Diagonal_Down_Left prediction mode
                let mut target_slice = target.mut_slice(ctx.offset);
                let top_row = &ctx.top_row[1..];
                for (y, row) in target_slice.rows_iter_mut().take(4).enumerate() {
                    for (x, value) in row.iter_mut().take(4).enumerate() {
                        let i = x + y;
                        *value = if i < 6 {
                            weighted_avg(top_row[i + 1], top_row[i], top_row[i + 2])
                        } else {
                            weighted_avg(top_row[7], top_row[7], top_row[6])
                        };
                    }
                }
            }
            Intra_4x4_SamplePredMode::Diagonal_Down_Right => {
                // Section 8.3.1.2.5 Specification of Intra_4x4_Diagonal_Down_Right prediction mode
                let mut target_slice = target.mut_slice(ctx.offset);
                let top = &ctx.top_row;
                let left = &ctx.left_column;
                for (y, row) in target_slice.rows_iter_mut().take(4).enumerate() {
                    for (x, value) in row.iter_mut().take(4).enumerate() {
                        *value = if x > y {
                            let i = 1 + x - y;
                            weighted_avg(top[i - 1], top[i - 2], top[i])
                        } else if y < x {
                            let i = 1 + y - x;
                            weighted_avg(left[i - 1], left[i - 2], left[i])
                        } else {
                            weighted_avg(top[0], top[1], left[1])
                        }
                    }
                }
            }
            Intra_4x4_SamplePredMode::Vertical_Right => {
                // Section 8.3.1.2.6 Specification of Intra_4x4_Vertical_Right prediction mode
                let mut target_slice = target.mut_slice(ctx.offset);
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
            Intra_4x4_SamplePredMode::Horizontal_Down => {
                // Section 8.3.1.2.7 Specification of Intra_4x4_Horizontal_Down prediction mode
                let mut target_slice = target.mut_slice(ctx.offset);
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
            Intra_4x4_SamplePredMode::Vertical_Left => {
                // Section 8.3.1.2.8 Specification of Intra_4x4_Vertical_Left prediction mode
                let mut target_slice = target.mut_slice(ctx.offset);
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
            Intra_4x4_SamplePredMode::Horizontal_Up => {
                // Section 8.3.1.2.9 Specification of Intra_4x4_Horizontal_Up prediction mode
                let mut target_slice = target.mut_slice(ctx.offset);
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
        }
    }
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
    let offset = point_to_plain_offset(loc);
    info!(" >{mode:?}");
    match mode {
        Intra_16x16_SamplePredMode::Intra_16x16_Vertical => {
            // Section 8.3.3.1 Specification of Intra_16x16_Vertical prediction mode
            let mut src_row = [0; 16];
            src_row.copy_from_slice(&target.row(y as isize - 1)[x..(x + 16)]);
            let mut target_slice = target.mut_slice(point_to_plain_offset(loc));
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
            let offset = point_to_plain_offset(loc);

            // Calculate the sum of all the values at the left of the current macroblock
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
            for x in 0..8 {
                h += (x as i32 + 1) * (top[1 + x + 8] as i32 - top[1 + 6 - x] as i32);
            }

            let mut v = 0i32;
            for y in 0..8 {
                v += (y as i32 + 1) * (left[1 + y + 8] as i32 - left[1 + 6 - y] as i32);
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
}
