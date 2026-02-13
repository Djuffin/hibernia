#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::match_like_matches_macro)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::needless_late_init)]
#![allow(non_snake_case)]

use hibernia::diag;
use hibernia::h264;

use std::env;
use std::fmt::Error;
use std::fs;
use std::io;

use hibernia::h264::nal_parser::NalParser;
use log::info;
use std::io::BufReader;
use v_frame::plane::PlaneOffset;

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

    let file =
        fs::File::open(&input_filename).expect(&format!("can't read file: {input_filename}"));
    let reader = BufReader::new(file);
    let nal_parser = NalParser::new(reader);
    let mut decoder = h264::decoder::Decoder::new();

    let mut decoding_output = Vec::<u8>::new();
    let mut frame_count = 0;

    {
        let mut writer_opt = Some(io::BufWriter::new(&mut decoding_output));
        let mut encoder_opt: Option<y4m::Encoder<io::BufWriter<&mut Vec<u8>>>> = None;

        let mut process_frame = |frame: h264::decoder::VideoFrame| {
            if encoder_opt.is_none() {
                let y_plane = &frame.planes[0];
                let w = y_plane.cfg.width;
                let h = y_plane.cfg.height;
                if let Some(writer) = writer_opt.take() {
                    encoder_opt = Some(
                        y4m::encode(w, h, y4m::Ratio { num: 15, den: 1 })
                            .with_colorspace(y4m::Colorspace::C420)
                            .write_header(writer)
                            .unwrap(),
                    );
                }
            }

            info!(
                "Writing frame #{} {} x {} to y4m",
                frame_count, frame.planes[0].cfg.width, frame.planes[0].cfg.height
            );
            frame_count += 1;

            let mut planes = Vec::<Vec<u8>>::new();
            if planes.len() < frame.planes.len() {
                planes.resize_with(frame.planes.len(), Vec::new);
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

            if let Some(enc) = &mut encoder_opt {
                enc.write_frame(&yuv_frame).unwrap();
            }
        };

        for nal_result in nal_parser {
            let nal_data = nal_result.expect("Error parsing NAL");
            decoder.decode(&nal_data).expect("Decoding error");

            while let Some(frame) = decoder.retrieve_frame() {
                process_frame(frame);
            }
        }

        decoder.flush().expect("Flush error");
        while let Some(frame) = decoder.retrieve_frame() {
            process_frame(frame);
        }
    }
    fs::write("output.y4m", decoding_output.as_slice()).expect("can't save decoding result");
}
