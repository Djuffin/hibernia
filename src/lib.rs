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

use log::info;

pub fn process_frames<W: std::io::Write>(
    frames: &[h264::decoder::VideoFrame],
    encoder: &mut y4m::Encoder<W>,
) {
    let mut planes = Vec::<Vec<u8>>::new();
    for (num, frame) in frames.iter().enumerate() {
        info!("Writing frame #{} to y4m", num);

        if planes.len() != frame.planes.len() {
            planes.resize(frame.planes.len(), Vec::new());
        }

        for (i, plane) in frame.planes.iter().enumerate() {
            let data_size = plane.cfg.width * plane.cfg.height;
            let data = &mut planes[i];
            if data.len() != data_size {
                data.resize(data_size, 0);
            }
            plane.copy_to_raw_u8(data, plane.cfg.width, 1);
        }

        let yuv_frame = y4m::Frame::new(
            [planes[0].as_slice(), planes[1].as_slice(), planes[2].as_slice()],
            None,
        );
        encoder.write_frame(&yuv_frame).unwrap();
    }
}
