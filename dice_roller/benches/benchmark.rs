
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use dice_roller::{build_array_manually, build_array_stdlib};


fn bench_fibonaccis(c: &mut Criterion) {
    c.bench_function("manual", |b| b.iter(|| build_array_manually(black_box(20))));
    c.bench_function("stdlib", |b| b.iter(|| build_array_stdlib(black_box(20))));
}

criterion_group!(benches, bench_fibonaccis);
criterion_main!(benches);
