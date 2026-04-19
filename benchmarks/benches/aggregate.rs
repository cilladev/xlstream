use criterion::{criterion_group, criterion_main, Criterion};
use std::path::Path;
use xlstream_eval::prelude_plan::execute_prelude;
use xlstream_io::Reader;
use xlstream_parse::{AggKind, AggregateKey};

fn bench_aggregate(c: &mut Criterion) {
    let keys = vec![
        AggregateKey { kind: AggKind::Sum, sheet: None, column: 2 },
        AggregateKey { kind: AggKind::Count, sheet: None, column: 2 },
        AggregateKey { kind: AggKind::Average, sheet: None, column: 2 },
    ];

    let small_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/bench_small.xlsx");
    let small_path = small_path.as_path();
    if small_path.exists() {
        c.bench_function("prelude_sum_count_avg_10k", |b| {
            b.iter(|| {
                let mut reader = Reader::open(small_path).unwrap();
                let (prelude, _count) =
                    execute_prelude(&mut reader, "Main", &keys, &[], &[]).unwrap();
                prelude
            });
        });
    } else {
        eprintln!(
            "skipping aggregate bench: bench_small.xlsx not found. Run generate-fixtures first."
        );
    }

    let medium_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/bench_medium.xlsx");
    let medium_path = medium_path.as_path();
    if medium_path.exists() {
        let mut group = c.benchmark_group("aggregate_medium");
        group.sample_size(10);
        group.bench_function("prelude_sum_count_avg_100k", |b| {
            b.iter(|| {
                let mut reader = Reader::open(medium_path).unwrap();
                let (prelude, _count) =
                    execute_prelude(&mut reader, "Main", &keys, &[], &[]).unwrap();
                prelude
            });
        });
        group.finish();
    }
}

criterion_group!(benches, bench_aggregate);
criterion_main!(benches);
