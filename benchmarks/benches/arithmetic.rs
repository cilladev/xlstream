use criterion::{criterion_group, criterion_main, Criterion};

fn bench_arithmetic(_c: &mut Criterion) {}

criterion_group!(benches, bench_arithmetic);
criterion_main!(benches);
