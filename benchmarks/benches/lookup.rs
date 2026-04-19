use criterion::{criterion_group, criterion_main, Criterion};

fn bench_lookup(_c: &mut Criterion) {}

criterion_group!(benches, bench_lookup);
criterion_main!(benches);
