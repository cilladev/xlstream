#![allow(clippy::unwrap_used, clippy::cast_precision_loss)]

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::path::Path;
use xlstream_eval::evaluate;

fn bench_end_to_end(c: &mut Criterion) {
    let base = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tiers = [
        ("small", base.join("fixtures/bench_small.xlsx")),
        ("medium", base.join("fixtures/bench_medium.xlsx")),
    ];

    let mut group = c.benchmark_group("end_to_end");
    group.sample_size(10);

    for (name, path) in &tiers {
        if !path.exists() {
            eprintln!("skipping {name}: fixture not found at {}", path.display());
            continue;
        }

        group.bench_with_input(BenchmarkId::new("single_threaded", name), name, |b, _| {
            let output = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
            b.iter(|| evaluate(path, output.path(), Some(1)).unwrap());
        });

        group.bench_with_input(BenchmarkId::new("parallel_4", name), name, |b, _| {
            let output = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
            b.iter(|| evaluate(path, output.path(), Some(4)).unwrap());
        });
    }

    group.finish();
}

fn bench_rss(c: &mut Criterion) {
    let base = Path::new(env!("CARGO_MANIFEST_DIR"));
    let input = base.join("fixtures/bench_small.xlsx");
    if !input.exists() {
        eprintln!("skipping RSS bench: fixture not found");
        return;
    }

    c.bench_function("rss_small_tier", |b| {
        b.iter(|| {
            let output = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
            let rss_before = memory_stats::memory_stats().map_or(0, |s| s.physical_mem);
            let _ = evaluate(&input, output.path(), Some(1)).unwrap();
            let rss_after = memory_stats::memory_stats().map_or(0, |s| s.physical_mem);
            let rss_mb = rss_after.max(rss_before) / 1_000_000;
            assert!(rss_mb < 200, "RSS {rss_mb} MB exceeds budget");
        });
    });
}

criterion_group!(benches, bench_end_to_end, bench_rss);
criterion_main!(benches);
