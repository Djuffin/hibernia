use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hibernia::h264::decoder::Decoder;
use hibernia::h264::nal_parser::NalParser;
use std::fs::File;
use std::io::BufReader;

pub fn decoding_benchmark(c: &mut Criterion) {
    let input_filename = "data/NL2_Sony_H.jsv";
    // We pre-read the file to avoid disk I/O in benchmark loop?
    // NalParser takes a reader.
    // If we want to benchmark decoding speed, we should probably read from memory.
    // So BufReader<Cursor<Vec<u8>>>.

    let encoded_video_buffer = std::fs::read(input_filename).expect("can't read file");

    c.bench_function("decode NL2_Sony_H", |b| {
        b.iter(|| {
            let cursor = std::io::Cursor::new(&encoded_video_buffer);
            let reader = BufReader::new(cursor);
            let parser = NalParser::new(reader);
            let mut decoder = Decoder::new();

            for nal in parser {
                let nal = nal.unwrap();
                decoder.decode(black_box(&nal)).unwrap();
                while decoder.retrieve_frame().is_some() {}
            }
            decoder.flush().unwrap();
            while decoder.retrieve_frame().is_some() {}
        })
    });
}

criterion_group!(benches, decoding_benchmark);
criterion_main!(benches);
