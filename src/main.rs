#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::match_like_matches_macro)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::needless_late_init)]

#[macro_use]
extern crate num_derive;

pub mod diag;
pub mod h264;
pub mod y4m_cmp;

use std::env;
use std::fs;
use std::io;

use log::info;
use v_frame::plane::PlaneOffset;
use y4m_cmp::{compare_files, compare_frames};

fn main() {
    diag::init(false);
    let args: Vec<String> = env::args().collect();
    let mut input_filename = "data/NL1_Sony_D.jsv".to_string();
    if args.len() > 1 {
        input_filename = args[1].clone();
    }

    let buf = fs::read(input_filename).unwrap();
    let mut decoder = h264::decoder::Decoder::new();
    decoder.decode(&buf).expect("parsing error");

    let first_frame = decoder.get_frame_buffer().first().unwrap();
    let y_plane = &first_frame.planes[0];
    let w = y_plane.cfg.width as u32;
    let h = y_plane.cfg.height as u32;

    let mut writer = io::BufWriter::new(fs::File::create("output.y4m").unwrap());
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
    drop(encoder);

    if let Err(message) = compare_files("output.y4m", "data/NL1_Sony_D.y4m") {
        print!("File comparison result: {message}");
    }
}
