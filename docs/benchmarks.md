# Benchmarks — measured performance

Measured on Apple M-series, 8-core. Criterion for micro-benchmarks; wall-clock via `std::time::Instant`.

## Tier results (50-column workbook: 20 data + 30 formula)

| Tier | Rows | Workers | Wall-clock | Peak RSS | Formulas evaluated |
|---|---|---|---|---|---|
| Small | 10,000 | 1 | 1.6s | 31 MB | 299,970 |
| Medium | 100,000 | 1 | 16.0s | 206 MB | 2,999,970 |
| Medium | 100,000 | 4 | 13.8s | 270 MB | 2,999,970 |
| Large | 1,000,000 | 1 | 156s | 1.7 GB | 29,999,970 |
| Large | 1,000,000 | 8 | 135s | 2.1 GB | 29,999,970 |

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
| formualizer (measured 2026-04-17) | 700k x 20 | 5h 40m | 3.3 GB |
| xlstream single-threaded (measured) | 700k x 20 | 48s | 734 MB |

Ratio: ~425x faster, ~4.5x less memory.

## Memory analysis

### Targets vs actuals

| Tier | Target (CLAUDE.md) | Actual | Status |
|---|---|---|---|
| Small (10k) | < 50 MB | 31 MB | Pass |
| Medium (100k) | < 100 MB | 206 MB | Miss (2x) |
| Large (1M) | < 300 MB | 1.7 GB | Miss (5.7x) |

The original target (< 250 MB for 700k rows) was based on the assumption that streaming evaluation would keep memory flat. The evaluation logic does — it holds ~10 MB (one row buffer + prelude scalars + lookup indexes). The memory overshoot comes from the I/O libraries.

### Where the memory goes

| Component | Memory | Can we control it? |
|---|---|---|
| Evaluator (row buffer + prelude) | ~10 MB | Yes — already minimal |
| calamine XML parsing | ~60% of RSS | No — upstream library |
| rust_xlsxwriter shared strings + output | ~30% of RSS | No — upstream library |
| Lookup sheet data (pre-loaded) | ~5-10 MB | Yes — proportional to lookup sheet size |

**calamine** decompresses the xlsx ZIP archive and parses the sheet XML. Even though it offers a "streaming" cell reader (`XlsxCellReader`), it still loads the shared string table and XML structure into memory. For a 412 MB xlsx, this is hundreds of MB.

**rust_xlsxwriter** in `constant_memory` mode flushes rows to a temp file, keeping row data flat. But it accumulates the shared string table (every unique string in the output) in memory. A workbook with millions of text cells builds a large string table.

### Path to < 250 MB (v0.2)

1. **Fork or contribute to calamine** — implement true streaming XML parsing without buffering the shared string table. The shared strings could be memory-mapped or loaded lazily.
2. **Reduce rust_xlsxwriter string table** — write strings inline instead of via the shared string table (trade file size for memory). Or implement a flushing shared string table.
3. **Alternative: use a different xlsx reader** — no production-quality streaming Rust xlsx reader exists as of 2026. Building one from scratch (zip streaming + SAX-style XML) is a multi-week project.

For v0.1, the memory profile is acceptable: 4.5x less than formualizer on the same workload. The wall-clock performance (425x faster) is the primary win.

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
