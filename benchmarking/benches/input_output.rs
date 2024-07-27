
use criterion::{BenchmarkId, black_box, criterion_group, criterion_main, Criterion};
use benchmarking::*;


fn criterion_benchmark(c: &mut Criterion) {
    // Compare individial function
    let input = 1_000_000;
    c.bench_function("simple", |b| b.iter(|| euler1_simple(black_box(input))));

    // Compare functions by grouping them
    let mut group = c.benchmark_group("Euler 1");
    let input = 1_000_000;
    group.bench_function("simple", |b| b.iter(|| euler1_simple(black_box(input))));
    group.bench_function("series", |b| b.iter(|| euler1_series(black_box(input))));
    group.finish();

    // Groups vs various inputs
    let inputs = [100, 1000, 10000, 100000, 1000000];
    let mut group = c.benchmark_group("Multiple inputs");
    for i in inputs {
        group.bench_with_input(
            BenchmarkId::new("euler1_simple", i),
            &i,
            |b, &i| { b.iter(|| euler1_simple(black_box(i))) }
        );

        group.bench_with_input(
            BenchmarkId::new("euler1_series", i),
            &i,
            |b, &i| { b.iter(|| euler1_series(black_box(i))) }
        );
    }
    group.finish();
}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
