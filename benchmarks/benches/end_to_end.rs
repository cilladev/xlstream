use criterion::{criterion_group, criterion_main, Criterion};

fn bench_end_to_end(_c: &mut Criterion) {}

criterion_group!(benches, bench_end_to_end);
criterion_main!(benches);
