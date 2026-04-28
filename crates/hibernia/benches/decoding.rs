use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hibernia::h264::decoder::{Decoder, DecoderContext};
use hibernia::h264::nal::NalUnitType;
use hibernia::h264::nal_parser::NalParser;
use hibernia::h264::parser::{
    parse_nal_header, parse_pps, parse_slice_data_cabac, parse_slice_header, parse_sps,
    remove_emulation_if_needed, BitReader,
};
use hibernia::h264::residual::ResidualPool;
use hibernia::h264::slice::SliceType;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::process::Command;

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap()
}

fn generate_ffmpeg_benchmark_data(path: &Path) {
    if path.exists() {
        return;
    }

    println!("Generating benchmark data with ffmpeg...");
    let status = Command::new("ffmpeg")
        .args(&[
            "-y",
            "-f",
            "lavfi",
            "-i",
            "mandelbrot=size=1280x720:rate=30",
            "-t",
            "3",
            "-c:v",
            "libx264",
            "-profile:v",
            "main",
            "-pix_fmt",
            "yuv420p",
            path.to_str().expect("non-UTF8 path"),
        ])
        .status()
        .expect("Failed to execute ffmpeg");

    assert!(status.success(), "ffmpeg failed to generate benchmark data");
}

fn bench_decoder(b: &mut criterion::Bencher, encoded_video_buffer: &[u8]) {
    b.iter(|| {
        let cursor = Cursor::new(black_box(encoded_video_buffer));
        let nal_parser = NalParser::new(cursor);
        let mut decoder = Decoder::new();

        for nal_result in nal_parser {
            let nal = nal_result.unwrap();
            decoder.decode(&nal).unwrap();
            while let Some(_frame) = decoder.retrieve_picture() {
                // consume frame
            }
        }
        decoder.flush().unwrap();
        while let Some(_frame) = decoder.retrieve_picture() {
            // consume frame
        }
    });
}

/// One-time setup for the CABAC parse-only bench. Walks the encoded stream,
/// loads SPS/PPS into a DecoderContext, and returns the de-emulated RBSPs of
/// every P- and B-slice in the stream (skipping the IDR/I leading slice).
fn prepare_cabac_slice_data(stream: &[u8]) -> (DecoderContext, Vec<Vec<u8>>) {
    let mut ctx = DecoderContext::default();
    let mut slice_rbsps: Vec<Vec<u8>> = Vec::new();
    let nal_parser = NalParser::new(Cursor::new(stream));

    for nal_result in nal_parser {
        let nal_data = nal_result.expect("NAL parse failed during CABAC bench setup");
        let nal_vec = remove_emulation_if_needed(&nal_data);
        let rbsp_data: &[u8] =
            if nal_vec.is_empty() { nal_data.as_slice() } else { nal_vec.as_slice() };

        let mut reader = BitReader::new(rbsp_data);
        let nal_header =
            parse_nal_header(&mut reader).expect("NAL header parse failed during CABAC bench setup");

        match nal_header.nal_unit_type {
            NalUnitType::SeqParameterSet => {
                let sps = parse_sps(&mut reader).expect("SPS parse failed");
                ctx.put_sps(sps);
            }
            NalUnitType::PicParameterSet => {
                let pps = parse_pps(&mut reader).expect("PPS parse failed");
                ctx.put_pps(pps);
            }
            NalUnitType::NonIDRSlice => {
                // Re-read from the start of the rbsp so the bench always sees the
                // slice header bits in the same position as when it runs.
                let mut hdr_reader = BitReader::new(rbsp_data);
                let nal_header_again =
                    parse_nal_header(&mut hdr_reader).expect("NAL header re-parse failed");
                let slice = parse_slice_header(&ctx, &nal_header_again, &mut hdr_reader)
                    .expect("slice header parse failed");
                if matches!(slice.header.slice_type, SliceType::P | SliceType::B) {
                    slice_rbsps.push(rbsp_data.to_vec());
                }
            }
            _ => {}
        }
    }

    if slice_rbsps.is_empty() {
        panic!("no P- or B-slice found in stream — bench setup needs a Main-profile stream");
    }
    (ctx, slice_rbsps)
}

fn bench_cabac_parse_slices(
    b: &mut criterion::Bencher,
    ctx: &DecoderContext,
    rbsps: &[Vec<u8>],
) {
    b.iter(|| {
        let mut pool = ResidualPool::default();
        for rbsp in rbsps {
            let mut reader = BitReader::new(black_box(rbsp.as_slice()));
            let nal_header = parse_nal_header(&mut reader).unwrap();
            let mut slice = parse_slice_header(ctx, &nal_header, &mut reader).unwrap();
            parse_slice_data_cabac(&mut reader, &mut slice, &mut pool).unwrap();
            black_box(slice);
        }
    });
}

pub fn decoding_benchmark(c: &mut Criterion) {
    let root = workspace_root();

    // Tests Baseline profile, CAVLC entropy coding, and no deblocking filter.
    // This serves as the baseline performance metric for the simplest decoding path.
    let nl2_buffer = fs::read(root.join("data/NL2_Sony_H/NL2_Sony_H.jsv")).expect("can't read NL2_Sony_H.jsv");
    c.bench_function("decode NL2_Sony_H", |b| bench_decoder(b, &nl2_buffer));

    // Tests Main profile, CABAC entropy coding, deblocking filter enabled, and B-slices.
    // This measures a heavy, realistic workload, evaluating CABAC parser efficiency,
    // memory bandwidth during deblocking, and sub-pixel interpolation in B-slices.
    let caba3_buffer = fs::read(root.join("data/CABA3_SVA_B/CABA3_SVA_B.264")).expect("can't read CABA3_SVA_B.264");
    c.bench_function("decode CABA3_SVA_B", |b| bench_decoder(b, &caba3_buffer));

    // Tests Constrained Baseline profile, CAVLC entropy coding, and deblocking filter enabled.
    // Short sequence (17 frames) useful for quick iterations while testing deblocking overhead.
    let sva_ba2_buffer = fs::read(root.join("data/SVA_BA2_D/SVA_BA2_D.264")).expect("can't read SVA_BA2_D.264");
    c.bench_function("decode SVA_BA2_D", |b| bench_decoder(b, &sva_ba2_buffer));

    // Tests 720p, Main Profile, 30fps generated by ffmpeg
    // Stresses decoder throughput with higher resolution and longer sequence
    let ffmpeg_720p_path: PathBuf = root.join("target/mandelbrot_720p_main.264");
    let _ = fs::create_dir_all(root.join("target"));
    generate_ffmpeg_benchmark_data(&ffmpeg_720p_path);
    let ffmpeg_720p_buffer =
        fs::read(&ffmpeg_720p_path).expect("can't read generated benchmark file");
    let _ = fs::remove_file(&ffmpeg_720p_path);

    let mut group = c.benchmark_group("720p_main");
    group.sample_size(10); // Reduce sample size as 720p 90 frames will take longer to decode
    group.bench_function("decode mandelbrot", |b| bench_decoder(b, &ffmpeg_720p_buffer));
    group.finish();

    // CABAC parsing bench. Re-uses the 720p_main mandelbrot bitstream because it's
    // Main profile (CABAC) with B/P frames.
    let (cabac_ctx, slice_rbsps) = prepare_cabac_slice_data(&ffmpeg_720p_buffer);
    let mut group = c.benchmark_group("cabac");
    group.bench_function("parse_slices mandelbrot", |b| {
        bench_cabac_parse_slices(b, &cabac_ctx, &slice_rbsps)
    });
    group.finish();
}

criterion_group!(benches, decoding_benchmark);
criterion_main!(benches);
