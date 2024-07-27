#![allow(dead_code)]

use std::path::Path;

use criterion::{criterion_group, criterion_main, Criterion};

use benchmarking::{read_lines_collect, read_lines_push};


const BOOK_PATH: &str = "../data/voyage-of-the-beagle.txt";


/// Benchmark reading file into a vector of lines
fn read_lines_into_vector(c: &mut Criterion) {
    let path = Path::new(BOOK_PATH);
    let mut group = c.benchmark_group("Read lines into Vector");
    group.bench_function(
        "Iterator::collect()",
        |b| b.iter(|| read_lines_collect(path))
    );
    group.bench_function(
        "Vec::push()",
        |b| b.iter(|| read_lines_push(path, None)),
    );
    group.bench_function(
        "Vec::push() w/ preallocate",
        |b| b.iter(|| read_lines_push(path, Some(20_000)))
    );
    group.finish();
}


criterion_group!(benches, read_lines_into_vector);
criterion_main!(benches);
