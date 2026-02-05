use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hibernia::{h264, process_frames};
use std::fs;
use std::io;

struct Sink;
impl io::Write for Sink {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> { Ok(buf.len()) }
    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}

fn benchmark_process_frames(c: &mut Criterion) {
    let input_filename = "data/SVA_NL1_B.264";
    let encoded_video_buffer = fs::read(input_filename).expect("can't read file");
    let mut decoder = h264::decoder::Decoder::new();
    decoder.decode(&encoded_video_buffer).expect("Decoding error");
    let frames = decoder.get_frame_buffer();

    // We need the width and height for the encoder setup
    let first_frame = frames.first().unwrap();
    let y_plane = &first_frame.planes[0];
    let w = y_plane.cfg.width as u32;
    let h = y_plane.cfg.height as u32;

    c.bench_function("process_frames", |b| {
        b.iter(|| {
            let mut writer = Sink;
            let mut encoder = y4m::encode(w as usize, h as usize, y4m::Ratio { num: 15, den: 1 })
                .with_colorspace(y4m::Colorspace::C420)
                .write_header(&mut writer)
                .unwrap();

            process_frames(black_box(frames), black_box(&mut encoder));
        })
    });
}

criterion_group!(benches, benchmark_process_frames);
criterion_main!(benches);
