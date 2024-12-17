use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use stones::{apply_blinks, count_stones_after_blinks};

fn criterion_benchmark(c: &mut Criterion) {
    let input = [125, 17];
    let steps = 25;

    c.bench_function("With results", |b| {
        b.iter(|| black_box(apply_blinks(black_box(&input), black_box(steps))))
    });

    c.bench_function("Count only", |b| {
        b.iter(|| {
            black_box(count_stones_after_blinks(
                black_box(&input),
                black_box(steps),
            ))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
