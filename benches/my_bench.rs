use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Day 14", |b| b.iter(|| { days::day14::solve() }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
