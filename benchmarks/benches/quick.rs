use criterion::{criterion_group, criterion_main, Criterion};

fn bench_quick(_c: &mut Criterion) {}

criterion_group!(benches, bench_quick);
criterion_main!(benches);
