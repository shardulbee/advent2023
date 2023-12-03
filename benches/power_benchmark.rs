use advent2023::solutions::DayTwo;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn power_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("power");
    group.bench_function("power", |b| {
        b.iter(|| DayTwo::run(black_box(false), black_box(None)))
    });
    group.bench_function("power_optimized", |b| {
        b.iter(|| DayTwo::run(black_box(false), black_box(Some(true))))
    });
    group.finish();
}

criterion_group!(benches, power_benchmark);
criterion_main!(benches);
