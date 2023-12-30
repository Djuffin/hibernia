#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::match_like_matches_macro)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::needless_late_init)]

#[macro_use]
extern crate num_derive;

pub mod diag;
pub mod h264;

use std::env;
use std::fs;
use std::io;

use log::info;
use v_frame::plane::PlaneOffset;

fn main() {
    diag::init(false);
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let name = args[1].clone();
        let buf = fs::read(name).unwrap();
        let mut decoder = h264::decoder::Decoder::new();
        decoder.decode(&buf).expect("parsing error");
        let frame = decoder.get_frame_buffer().unwrap();
        let y_plane = &frame.planes[0];

        let w = y_plane.cfg.width as u32;
        let h = y_plane.cfg.height as u32;
        let data_size = (w * h) as usize;

        info!("Writing frame {w} x {h} to png");
        let mut writer = io::BufWriter::new(fs::File::create("output.png").unwrap());
        let mut encoder = png::Encoder::new(&mut writer, w, h);
        encoder.set_color(png::ColorType::Grayscale);
        let mut pixel_writer = encoder.write_header().unwrap();
        let mut data = Vec::<u8>::with_capacity(data_size);
        data.resize(data_size, 0);
        y_plane.copy_to_raw_u8(&mut data, w as usize, 1);
        let _ = pixel_writer.write_image_data(&data).unwrap();
    }
}
