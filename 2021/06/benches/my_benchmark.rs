use criterion::{criterion_group, criterion_main, Criterion};
use day06::{array, hash_map, naive};

pub fn criterion_benchmark(c: &mut Criterion) {
    let fishes: Vec<u8> = vec![3, 4, 3, 1, 2];

    c.bench_function("naive", |b| b.iter(|| naive(256, fishes.clone())));
    c.bench_function("hash_map", |b| b.iter(|| hash_map(256, fishes.clone())));
    c.bench_function("array", |b| b.iter(|| array(256, fishes.clone())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
