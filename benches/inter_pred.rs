use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hibernia::h264::inter_pred::{interpolate_luma, InterpolationBuffer};
use hibernia::h264::macroblock::MotionVector;
use v_frame::plane::Plane;

fn create_test_plane(width: usize, height: usize, fill: u8) -> Plane<u8> {
    let mut p = Plane::new(width, height, 0, 0, 16, 16);
    // Fill with random-ish data to avoid branch prediction anomalies if data matters
    // but here we just fill with a pattern
    for i in 0..p.data.len() {
        p.data[i] = ((i + fill as usize) % 256) as u8;
    }
    p
}

pub fn inter_pred_benchmark(c: &mut Criterion) {
    let plane = create_test_plane(640, 480, 42);
    let mut dst = [0u8; 16 * 16]; // Max block size
    let mut buffer = InterpolationBuffer::new();

    let mut group = c.benchmark_group("interpolate_luma");

    // Integer position (0, 0)
    group.bench_function("integer (0,0)", |b| {
        let mv = MotionVector { x: 0, y: 0 };
        b.iter(|| {
            interpolate_luma(
                black_box(&plane),
                black_box(100),
                black_box(100),
                black_box(0),
                black_box(0),
                black_box(16),
                black_box(16),
                black_box(mv),
                black_box(&mut dst),
                black_box(16),
                black_box(&mut buffer),
            )
        })
    });

    // Half-pel horizontal (2, 0) - position 'b'
    group.bench_function("half-pel (2,0)", |b| {
        let mv = MotionVector { x: 2, y: 0 };
        b.iter(|| {
            interpolate_luma(
                black_box(&plane),
                black_box(100),
                black_box(100),
                black_box(0),
                black_box(0),
                black_box(16),
                black_box(16),
                black_box(mv),
                black_box(&mut dst),
                black_box(16),
                black_box(&mut buffer),
            )
        })
    });

    // Half-pel vertical (0, 2) - position 'h'
    group.bench_function("half-pel (0,2)", |b| {
        let mv = MotionVector { x: 0, y: 2 };
        b.iter(|| {
            interpolate_luma(
                black_box(&plane),
                black_box(100),
                black_box(100),
                black_box(0),
                black_box(0),
                black_box(16),
                black_box(16),
                black_box(mv),
                black_box(&mut dst),
                black_box(16),
                black_box(&mut buffer),
            )
        })
    });

    // Half-pel center (2, 2) - position 'j'
    group.bench_function("half-pel (2,2)", |b| {
        let mv = MotionVector { x: 2, y: 2 };
        b.iter(|| {
            interpolate_luma(
                black_box(&plane),
                black_box(100),
                black_box(100),
                black_box(0),
                black_box(0),
                black_box(16),
                black_box(16),
                black_box(mv),
                black_box(&mut dst),
                black_box(16),
                black_box(&mut buffer),
            )
        })
    });

    // Quarter-pel (1, 0) - position 'a'
    group.bench_function("quarter-pel (1,0)", |b| {
        let mv = MotionVector { x: 1, y: 0 };
        b.iter(|| {
            interpolate_luma(
                black_box(&plane),
                black_box(100),
                black_box(100),
                black_box(0),
                black_box(0),
                black_box(16),
                black_box(16),
                black_box(mv),
                black_box(&mut dst),
                black_box(16),
                black_box(&mut buffer),
            )
        })
    });

    // Quarter-pel (1, 1) - position 'e' (needs b and h)
    group.bench_function("quarter-pel (1,1)", |b| {
        let mv = MotionVector { x: 1, y: 1 };
        b.iter(|| {
            interpolate_luma(
                black_box(&plane),
                black_box(100),
                black_box(100),
                black_box(0),
                black_box(0),
                black_box(16),
                black_box(16),
                black_box(mv),
                black_box(&mut dst),
                black_box(16),
                black_box(&mut buffer),
            )
        })
    });

    // Quarter-pel (2, 1) - position 'f' (needs b and j)
    group.bench_function("quarter-pel (2,1)", |b| {
        let mv = MotionVector { x: 2, y: 1 };
        b.iter(|| {
            interpolate_luma(
                black_box(&plane),
                black_box(100),
                black_box(100),
                black_box(0),
                black_box(0),
                black_box(16),
                black_box(16),
                black_box(mv),
                black_box(&mut dst),
                black_box(16),
                black_box(&mut buffer),
            )
        })
    });

    group.finish();
}

criterion_group!(benches, inter_pred_benchmark);
criterion_main!(benches);
