use criterion::{criterion_group, criterion_main, Criterion};
use day07::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let sample_crabs = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    c.bench_function("part1_naive", |b| b.iter(|| part1_naive(&sample_crabs)));
    c.bench_function("part1_functional", |b| {b.iter(|| part1_functional(&sample_crabs))});
    c.bench_function("part1_median", |b| b.iter(|| part1_median(&sample_crabs)));

    c.bench_function("part2_naive", |b| b.iter(|| part2_naive(&sample_crabs)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
