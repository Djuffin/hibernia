#![allow(unused)]
#![allow(clippy::match_like_matches_macro)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::needless_late_init)]

#[macro_use]
extern crate num_derive;

pub mod diag;
pub mod h264;

use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
use std::env;
use std::fs;

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

        let mut img =
            ImageBuffer::from_fn(y_plane.cfg.width as u32, y_plane.cfg.height as u32, |x, y| {
                image::Luma([y_plane.p(x as usize, y as usize)])
            });
        img.save("output.png");
    }
}
