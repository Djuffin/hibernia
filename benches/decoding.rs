use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hibernia::h264::decoder::Decoder;
use hibernia::h264::nal_parser::NalParser;
use std::fs;
use std::io::Cursor;

fn bench_decoder(b: &mut criterion::Bencher, encoded_video_buffer: &[u8]) {
    b.iter(|| {
        let cursor = Cursor::new(black_box(encoded_video_buffer));
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
    });
}

pub fn decoding_benchmark(c: &mut Criterion) {
    // Tests Baseline profile, CAVLC entropy coding, and no deblocking filter.
    // This serves as the baseline performance metric for the simplest decoding path.
    let nl2_buffer = fs::read("data/NL2_Sony_H.jsv").expect("can't read NL2_Sony_H.jsv");
    c.bench_function("decode NL2_Sony_H", |b| bench_decoder(b, &nl2_buffer));

    // Tests Main profile, CABAC entropy coding, deblocking filter enabled, and B-slices.
    // This measures a heavy, realistic workload, evaluating CABAC parser efficiency, 
    // memory bandwidth during deblocking, and sub-pixel interpolation in B-slices.
    let caba3_buffer = fs::read("data/CABA3_SVA_B.264").expect("can't read CABA3_SVA_B.264");
    c.bench_function("decode CABA3_SVA_B", |b| bench_decoder(b, &caba3_buffer));

    // Tests Constrained Baseline profile, CAVLC entropy coding, and deblocking filter enabled.
    // Short sequence (17 frames) useful for quick iterations while testing deblocking overhead.
    let sva_ba2_buffer = fs::read("data/SVA_BA2_D.264").expect("can't read SVA_BA2_D.264");
    c.bench_function("decode SVA_BA2_D", |b| bench_decoder(b, &sva_ba2_buffer));
}

criterion_group!(benches, decoding_benchmark);
criterion_main!(benches);
