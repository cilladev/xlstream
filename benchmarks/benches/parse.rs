use criterion::{criterion_group, criterion_main, Criterion};

fn bench_parse(_c: &mut Criterion) {}

criterion_group!(benches, bench_parse);
criterion_main!(benches);
