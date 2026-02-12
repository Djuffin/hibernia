use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hibernia::h264::decoder::Decoder;
use hibernia::h264::nal_parser::NalParser;
use std::fs;
use std::io::Cursor;

pub fn decoding_benchmark(c: &mut Criterion) {
    let input_filename = "data/NL2_Sony_H.jsv";
    let encoded_video_buffer = fs::read(input_filename).expect("can't read file");

    c.bench_function("decode NL2_Sony_H", |b| {
        b.iter(|| {
            let cursor = Cursor::new(black_box(&encoded_video_buffer));
            let nal_parser = NalParser::new(cursor);
            let mut decoder = Decoder::new();

            for nal_result in nal_parser {
                let nal = nal_result.unwrap();
                decoder.decode(&nal).unwrap();
                while let Some(_frame) = decoder.retrieve_frame() {
                    // consume frame
                }
            }
            decoder.flush().unwrap();
            while let Some(_frame) = decoder.retrieve_frame() {
                // consume frame
            }
        })
    });
}

criterion_group!(benches, decoding_benchmark);
criterion_main!(benches);
