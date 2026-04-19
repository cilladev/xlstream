use criterion::{criterion_group, criterion_main, Criterion};

fn bench_aggregate(_c: &mut Criterion) {}

criterion_group!(benches, bench_aggregate);
criterion_main!(benches);
