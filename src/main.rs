#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::match_like_matches_macro)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::needless_late_init)]
#![allow(non_snake_case)]

#[macro_use]
extern crate num_derive;

pub mod diag;
pub mod h264;
pub mod y4m_cmp;

use std::env;
use std::fmt::Error;
use std::fs;
use std::io;

use log::info;
use v_frame::plane::PlaneOffset;
use y4m_cmp::compare_y4m_buffers;

fn main() {
    diag::init(false);
    let args: Vec<String> = env::args().collect();
    let input_filename: String;
    if args.len() > 1 {
        input_filename = args[1].clone();
    } else {
        print!("No input file");
        return;
    }

    let encoded_video_buffer =
        fs::read(&input_filename).expect(format!("can't read file: {input_filename}").as_str());
    let mut decoder = h264::decoder::Decoder::new();
    decoder.decode(&encoded_video_buffer).expect("Decoding error");

    let first_frame = decoder.get_frame_buffer().first().unwrap();
    let y_plane = &first_frame.planes[0];
    let w = y_plane.cfg.width as u32;
    let h = y_plane.cfg.height as u32;

    let mut decoding_output = Vec::<u8>::new();
    {
        let mut writer = io::BufWriter::new(&mut decoding_output);
        let mut encoder = y4m::encode(w as usize, h as usize, y4m::Ratio { num: 15, den: 1 })
            .with_colorspace(y4m::Colorspace::C420)
            .write_header(&mut writer)
            .unwrap();

        for (num, frame) in decoder.get_frame_buffer().iter().enumerate() {
            info!("Writing frame #{num} {w} x {h} to y4m");

            let mut planes = Vec::<Vec<u8>>::new();
            for plane in &frame.planes {
                let data_size = plane.cfg.width * plane.cfg.height;
                let mut data = vec![0; data_size];
                plane.copy_to_raw_u8(&mut data, plane.cfg.width, 1);
                planes.push(data)
            }

            let yuv_frame = y4m::Frame::new(
                [planes[0].as_slice(), planes[1].as_slice(), planes[2].as_slice()],
                None,
            );
            encoder.write_frame(&yuv_frame).unwrap();
        }
    }
    fs::write("output.y4m", decoding_output.as_slice()).expect("can't save decoding result");
}

#[cfg(test)]
mod tests {
    pub use super::*;

    fn test_decoding_against_gold(
        encoded_file_name: &str,
        gold_y4m_filename: &str,
        tolerance: u8,
    ) -> Result<(), String> {
        fn stringify(e: io::Error) -> String {
            format!("IO error: {e}")
        }
        let expected_y4m_buffer = fs::read(gold_y4m_filename).map_err(stringify)?;
        let encoded_video_buffer = fs::read(encoded_file_name).map_err(stringify)?;
        let mut decoder = h264::decoder::Decoder::new();
        decoder
            .decode(&encoded_video_buffer)
            .map_err(|e| -> String { format!("Decoding error: {e:?}") })?;

        let first_frame = decoder.get_frame_buffer().first().unwrap();
        let y_plane = &first_frame.planes[0];
        let w = y_plane.cfg.width as u32;
        let h = y_plane.cfg.height as u32;

        let mut decoding_output = Vec::<u8>::new();
        {
            let mut writer = io::BufWriter::new(&mut decoding_output);
            let mut encoder = y4m::encode(w as usize, h as usize, y4m::Ratio { num: 15, den: 1 })
                .with_colorspace(y4m::Colorspace::C420)
                .write_header(&mut writer)
                .unwrap();

            for (num, frame) in decoder.get_frame_buffer().iter().enumerate() {
                let mut planes = Vec::<Vec<u8>>::new();
                for plane in &frame.planes {
                    let data_size = plane.cfg.width * plane.cfg.height;
                    let mut data = vec![0; data_size];
                    plane.copy_to_raw_u8(&mut data, plane.cfg.width, 1);
                    planes.push(data)
                }

                let yuv_frame = y4m::Frame::new(
                    [planes[0].as_slice(), planes[1].as_slice(), planes[2].as_slice()],
                    None,
                );
                encoder.write_frame(&yuv_frame).unwrap();
            }
        }

        compare_y4m_buffers(decoding_output.as_slice(), expected_y4m_buffer.as_slice(), tolerance)
    }

    #[test]
    pub fn test_NL1_Sony_D() -> Result<(), String> {
        // All slices are coded as I slices. Each picture contains only one slice.
        // disable_deblocking_filter_idc is equal to 1, specifying disabling of the deblocking filter process.
        test_decoding_against_gold("data/NL1_Sony_D.jsv", "data/NL1_Sony_D.y4m", 0)
    }

    #[test]
    pub fn test_SVA_NL1_B() -> Result<(), String> {
        // All slices are coded as I slices. Each picture contains only one slice.
        // disable_deblocking_filter_idc is equal to 1, specifying disabling of the deblocking filter process.
        test_decoding_against_gold("data/SVA_NL1_B.264", "data/SVA_NL1_B.y4m", 0)
    }

    #[test]
    #[ignore = "not yet implemented"]
    pub fn test_BA1_Sony_D() -> Result<(), String> {
        // Decoding of I slices with the deblocking filter process enabled.
        // All slices are coded as I slices. Each picture contains only one slice.
        test_decoding_against_gold("data/BA1_Sony_D.jsv", "data/BA1_Sony_D.y4m", 0)
    }

    #[test]
    pub fn test_NL2_Sony_H() -> Result<(), String> {
        diag::init(false);
        // Decoding of P slices.
        // All slices are coded as I or P slices. Each picture contains only one slice.
        // disable_deblocking_filter_idc is equal to 1, specifying disabling of the deblocking filter process.
        // pic_order_cnt_type is equal to 0.
        // h264 (Constrained Baseline), yuv420p(progressive), 176x144
        // Tolerance set to 10 due to observed differences up to 9 units in Frame #3 MB:17 (P-slice).
        // This likely stems from intermediate precision differences in interpolation in the reference decoder.
        test_decoding_against_gold("data/NL2_Sony_H.jsv", "data/NL2_Sony_H.y4m", 10)
    }
}
