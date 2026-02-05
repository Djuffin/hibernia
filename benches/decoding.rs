use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hibernia::h264::decoder::Decoder;
use std::fs;

pub fn decoding_benchmark(c: &mut Criterion) {
    let input_filename = "data/NL2_Sony_H.jsv";
    let encoded_video_buffer = fs::read(input_filename).expect("can't read file");

    c.bench_function("decode NL2_Sony_H", |b| {
        b.iter(|| {
            let mut decoder = Decoder::new();
            decoder.decode(black_box(&encoded_video_buffer)).unwrap();
        })
    });
}

criterion_group!(benches, decoding_benchmark);
criterion_main!(benches);
