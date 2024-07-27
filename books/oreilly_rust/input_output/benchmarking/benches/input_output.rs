
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use benchmarking::*;


fn criterion_benchmark(c: &mut Criterion) {
    let input = 1_000_000;

    // Compare individial function
    //~ c.bench_function("series", |b| b.iter(|| euler1_series(black_box(input))));

    // Compare functions by grouping them
    let mut group = c.benchmark_group("Euler 1");
    group.bench_function("simple", |b| b.iter(|| euler1_simple(black_box(input))));
    group.bench_function("series", |b| b.iter(|| euler1_series(black_box(input))));
    group.finish();
}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
