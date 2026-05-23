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

use hibernia::api::{
    create_decoder, Codec, DecodedPicture, DecoderConfig, DecoderError, DefaultAllocator,
    EncodedPacket, FlushMode, StreamFormat, VideoDecoder, VideoDecoderCallbacks, VideoPlane,
};
use hibernia::diag;
use hibernia::h264::nal_parser::NalParser;

use std::env;
use std::fs;
use std::io::{self, BufReader};
use std::sync::Arc;
use std::time::Instant;

use log::info;

struct CliCallbacks;

impl VideoDecoderCallbacks for CliCallbacks {
    fn on_picture_available(&self) {}
    fn on_format_changed(&self, _format: StreamFormat) {}
}

fn main() {
    diag::init(false);
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    let input_filename: String;
    let output_filename: Option<String>;
    if args.len() > 1 {
        input_filename = args[1].clone();
        output_filename = if args.len() > 2 { Some(args[2].clone()) } else { None };
    } else {
        println!("Usage: hibernia <input.h264> [output.y4m]");
        return;
    }

    let file = fs::File::open(&input_filename)
        .unwrap_or_else(|_| panic!("can't read file: {input_filename}"));
    let nal_parser = NalParser::new(BufReader::new(file));

    let mut decoder = create_decoder(
        DecoderConfig::new(Codec::H264),
        Arc::new(DefaultAllocator),
        Arc::new(CliCallbacks),
    )
    .expect("create_decoder");

    let mut frame_count = 0;
    {
        let mut writer_opt = output_filename.map(|f| {
            io::BufWriter::new(fs::File::create(&f).unwrap_or_else(|_| panic!("can't create {f}")))
        });
        let mut encoder_opt: Option<y4m::Encoder<io::BufWriter<fs::File>>> = None;

        let mut process_frame = |pic: DecodedPicture, frame_count: &mut usize| {
            let format = &pic.format;
            let display_width = format.display_width;
            let display_height = format.display_height;
            let crop_left = format.crop_left;
            let crop_top = format.crop_top;

            if writer_opt.is_none() && encoder_opt.is_none() {
                info!("Decoded frame #{} {} x {}", frame_count, display_width, display_height);
                *frame_count += 1;
                return;
            }

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

            info!(
                "Writing frame #{} {} x {} to y4m",
                frame_count, display_width, display_height
            );
            *frame_count += 1;

            // Copy each plane's visible cropped region into a tight buffer
            // the y4m encoder expects.
            let mut planes: [Vec<u8>; 3] = [Vec::new(), Vec::new(), Vec::new()];
            for (i, channel) in [VideoPlane::Y, VideoPlane::U, VideoPlane::V].iter().enumerate() {
                let view = pic.frame.plane(*channel).expect("plane present");
                let (cw, ch, cx, cy) = if i == 0 {
                    (display_width, display_height, crop_left, crop_top)
                } else {
                    (display_width / 2, display_height / 2, crop_left / 2, crop_top / 2)
                };
                planes[i].resize(cw * ch, 0);
                for row in 0..ch {
                    let src_base = (cy + row) * view.stride + cx;
                    let dst_base = row * cw;
                    planes[i][dst_base..dst_base + cw]
                        .copy_from_slice(&view.data[src_base..src_base + cw]);
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

        // Re-wrap each NAL with an Annex-B start code prefix so the
        // adapter's splitter can parse it.
        for nal_result in nal_parser {
            let nal = nal_result.expect("nal parse");
            let mut buf = Vec::with_capacity(nal.len() + 4);
            buf.extend_from_slice(&[0, 0, 0, 1]);
            buf.extend_from_slice(&nal);
            decoder.decode(EncodedPacket::from_vec(buf)).expect("decode");
            while let Some(pic) = decoder.get_picture().expect("get_picture") {
                process_frame(pic, &mut frame_count);
            }
        }
        decoder.flush(FlushMode::Drain).expect("flush");
        while let Some(pic) = decoder.get_picture().expect("get_picture") {
            process_frame(pic, &mut frame_count);
        }
    }

    let elapsed = start.elapsed();
    let fps = frame_count as f64 / elapsed.as_secs_f64();
    println!("Decoded {frame_count} frames in {elapsed:.3?} ({fps:.2} fps)");
}
