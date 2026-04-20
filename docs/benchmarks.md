# Benchmarks — measured performance

Measured on Apple M-series, 8-core. Criterion for micro-benchmarks; wall-clock via `std::time::Instant`.

## Tier results (50-column workbook: 20 data + 30 formula)

| Tier | Rows | Workers | Wall-clock | Peak RSS | Formulas evaluated |
|---|---|---|---|---|---|
| Small | 10,000 | 1 | 1.6s | — | 299,970 |
| Medium | 100,000 | 1 | 15.9s | — | 2,999,970 |
| Medium | 100,000 | 4 | 13.3s | — | 2,999,970 |
| Medium | 100,000 | 8 | 13.8s | — | 2,999,970 |
| Large | 1,000,000 | 1 | 158s | 2.3 GB | 29,999,970 |
| Large | 1,000,000 | 8 | 136s | — | 29,999,970 |

## Parallel scaling (medium tier, 100k rows)

| Workers | Wall-clock | Speedup |
|---|---|---|
| 1 | 15.9s | 1.0x |
| 2 | 13.7s | 1.16x |
| 4 | 13.3s | 1.20x |
| 8 | 13.8s | 1.15x |

Modest speedup on this workload. Bottleneck is I/O: each worker re-opens the xlsx and seeks to its row range (calamine shared-strings parse + XML cursor seek). Row-local-only workloads show better scaling.

## Micro-benchmarks

| Benchmark | Result |
|---|---|
| Parse 30 formulas | ~35 us/batch |
| Arithmetic ops (add/mul/div) | 16-45 ns/eval (22-62M ops/sec) |
| String concat | ~280 ns/eval (heap allocation) |
| VLOOKUP exact (1k table) | sub-microsecond |

## Comparative summary

| Engine | Workload | Wall-clock | Peak RSS |
|---|---|---|---|
| formualizer (measured 2026-04-17) | 400k x 20 | 5h 40m | 3.3 GB |
| xlstream single-threaded (measured) | 700k x 20 | 48s | 734 MB |

Ratio: ~425x faster, ~4.5x less memory.

## Running benchmarks

```bash
# Generate fixtures (10k + 100k tiers)
make bench-generate

# Run all Rust criterion benchmarks
make bench-rust

# Run Python API benchmark
make bench-python

# Run everything
make bench
```

Criterion HTML reports: `target/criterion/report/index.html`.

## Fixture generation

`cargo run -p xlstream-benchmarks --release --bin generate-fixtures -- --tier all`

Deterministic (seed=42). Produces `bench_small.xlsx`, `bench_medium.xlsx`, `bench_large.xlsx`. Not committed — regenerated before bench runs.
