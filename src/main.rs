#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::match_like_matches_macro)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::manual_is_multiple_of)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::large_enum_variant)]
#![allow(non_snake_case)]

use hibernia::diag;
use hibernia::h264;

use std::env;
use std::fmt::Error;
use std::fs;
use std::io;
use std::time::Instant;

use hibernia::h264::nal_parser::NalParser;
use log::info;
use std::io::BufReader;
use v_frame::plane::PlaneOffset;

fn main() {
    diag::init(false);
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    let input_filename: String;
    if args.len() > 1 {
        input_filename = args[1].clone();
    } else {
        print!("No input file");
        return;
    }

    let file =
        fs::File::open(&input_filename).unwrap_or_else(|_| panic!("can't read file: {input_filename}"));
    let reader = BufReader::new(file);
    let nal_parser = NalParser::new(reader);
    let mut decoder = h264::decoder::Decoder::new();

    let mut frame_count = 0;

    {
        let output_file = fs::File::create("output.y4m").expect("can't create output.y4m");
        let mut writer_opt = Some(io::BufWriter::new(output_file));
        let mut encoder_opt: Option<y4m::Encoder<io::BufWriter<fs::File>>> = None;

        let mut process_frame = |pic: h264::decoder::Picture| {
            let frame = pic.frame;
            let display_width = pic.crop.display_width;
            let display_height = pic.crop.display_height;
            let crop_left = pic.crop.crop_left;
            let crop_top = pic.crop.crop_top;

            if encoder_opt.is_none() {
                if let Some(writer) = writer_opt.take() {
                    encoder_opt = Some(
                        y4m::encode(display_width, display_height, y4m::Ratio { num: 15, den: 1 })
                            .with_colorspace(y4m::Colorspace::C420)
                            .write_header(writer)
                            .unwrap(),
                    );
                }
            }

            info!("Writing frame #{} {} x {} to y4m", frame_count, display_width, display_height);
            frame_count += 1;

            let mut planes = Vec::<Vec<u8>>::new();
            if planes.len() < frame.planes.len() {
                planes.resize_with(frame.planes.len(), Vec::new);
            }

            for (i, plane) in frame.planes.iter().enumerate() {
                let (cw, ch, cx, cy) = if i == 0 {
                    (display_width, display_height, crop_left, crop_top)
                } else {
                    (display_width / 2, display_height / 2, crop_left / 2, crop_top / 2)
                };

                let data_size = cw * ch;
                let data = &mut planes[i];
                if data.len() != data_size {
                    data.resize(data_size, 0);
                }

                for row in 0..ch {
                    let src_offset =
                        (plane.cfg.yorigin + cy + row) * plane.cfg.stride + plane.cfg.xorigin + cx;
                    let dst_offset = row * cw;
                    data[dst_offset..dst_offset + cw]
                        .copy_from_slice(&plane.data[src_offset..src_offset + cw]);
                }
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

            while let Some(pic) = decoder.retrieve_picture() {
                process_frame(pic);
            }
        }

        decoder.flush().expect("Flush error");
        while let Some(pic) = decoder.retrieve_picture() {
            process_frame(pic);
        }
    }

    let elapsed = start.elapsed();
    let fps = frame_count as f64 / elapsed.as_secs_f64();
    println!("Decoded {frame_count} frames in {elapsed:.3?} ({fps:.2} fps)");
}
