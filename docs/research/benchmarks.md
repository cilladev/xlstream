# Benchmarks — reference workloads and targets

## The reference workload

`benchmark_large_400k.xlsx`:
- Main sheet: 400,000 rows × 20 columns.
- 10 data columns + 10 formula columns.
- Formula mix: 2 simple arithmetic, 2 aggregate (SUM, SUMIF), 4 VLOOKUP (single-key into lookup sheets), 2 conditional (IFS, IF + VLOOKUP).
- Plus 2 lookup sheets (Thresholds: 25 rows with a pre-computed `RegionBusiness` helper column; Region Info: 5 rows).
- On-disk size: ~56 MB.

This mirrors a realistic trading/finance/retail analytics workbook.

## Scale tiers

| Tier | Rows | Data cols | Formula cols | On disk |
|---|---|---|---|---|
| Smoke | 1,000 | 10 | 10 | ~100 KB |
| Small | 10,000 | 20 | 10 | ~1 MB |
| Medium | 100,000 | 20 | 10 | ~15 MB |
| Reference | 400,000 | 20 | 10 | ~56 MB |
| Stress | 1,000,000 | 20 | 10 | ~140 MB |

## Targets for v0.1

| Tier | Peak RSS | Wall-clock (8-core) |
|---|---|---|
| Smoke | < 40 MB | < 0.5 s |
| Small | < 50 MB | < 2 s |
| Medium | < 150 MB | < 15 s |
| Reference | < 250 MB | < 180 s (3 min) |
| Stress | < 500 MB | < 450 s (7.5 min) |

Stretch goals (v0.2):
- Reference in < 60 s.
- Stress in < 180 s.

## Baseline comparison

xlstream vs. formualizer on identical hardware and identical workloads. The **Reference** row for formualizer is a real measurement (2026-04-17); other rows are extrapolated and will be confirmed by our benchmark harness. xlstream numbers are targets; will be measured once v0.1 is runnable.

| Workload | formualizer RSS | formualizer wall | xlstream RSS (target) | xlstream wall (target) |
|---|---|---|---|---|
| Smoke (1k) | ~400 MB | ~3 s | < 40 MB | < 0.5 s |
| Small (10k) | ~700 MB | ~1 min | < 50 MB | < 2 s |
| Medium (100k) | ~1.5 GB | ~20 min | < 150 MB | < 15 s |
| **Reference (400k)** | **3.3 GB (measured)** | **5h 40m (measured)** | **< 250 MB** | **< 3 min** |
| Stress (1M) | likely OOM / hour+ | very long | < 500 MB | < 7.5 min |

Reference-row measurement details: 23.4 MB raw data xlsx, 56.1 MB with formulas, 71.7 MB evaluated CSV. Load phase 2m 30s; evaluate + save phase 5h 38m. Ratio we're chasing: **≈ 13× less memory, ≈ 113× faster on wall-clock**.

## Methodology

### Hardware

- Apple M-series (for macOS and arm64 Linux), 8-core.
- x86_64 Linux CI runner (2 vCPU) for Linux benchmarks.
- Windows runs on CI only; target numbers same as Linux x86_64.

### Timing

`criterion` for micro-benchmarks. Wall-clock measured end-to-end via `std::time::Instant` in a small binary.

### Memory

Peak RSS via `memory-stats` crate. Reported as the maximum seen over the evaluation.

### Correctness

Every benchmark run also asserts output equality against a golden xlsx. Speed without correctness is not a pass.

## CI integration

### Every PR (fast)

- Smoke + Small benchmarks only. Must not regress by more than 10% RSS or 20% wall-clock vs main.

### Nightly

- Full tier. Plotted via `benchmark-action/github-action-benchmark@v1`, pushed to `gh-pages`.
- Reference tier wall-clock must stay < 180 s. RSS must stay < 250 MB.

### Release (pre-tag)

- Full tier + stress. Manually reviewed before shipping.

## Fixture generation

`benchmarks/fixtures/scripts/generate_reference.rs` produces `benchmark_large_400k.xlsx` deterministically (seeded RNG). Not committed to git — too large. CI regenerates before benchmarks run. Local: `cargo run -p xlstream-benchmarks --bin generate-fixtures`.

## Golden outputs

`benchmark_large_400k_golden.xlsx` — Excel-computed expected output. Generated once by opening `benchmark_large_400k.xlsx` in real Excel and saving. Committed to LFS (it's ~80 MB after evaluation). If Excel license isn't available, LibreOffice headless is acceptable for approximation; note that `=0.1+0.2=0.3` will differ.

## Sub-benchmarks

Per-component micro-benches in `benchmarks/benches/`:

- `parse_bench.rs` — formula parsing throughput.
- `lookup_bench.rs` — hash lookup on 10k-row table.
- `aggregate_bench.rs` — SUM/COUNT over 400k-cell column.
- `builtin_bench.rs` — per-builtin throughput (IF, VLOOKUP, SUM, etc.).
- `end_to_end_bench.rs` — full pipeline at each tier.

Targets, roughly:
- Parse: > 500k formulas/sec.
- Lookup: > 10M hits/sec on hot cache.
- Aggregate (numeric column): > 100M cells/sec single-threaded.
- Per-row eval overhead: < 1 microsecond/cell on row-local formula.

## Publishing results

`docs/research/benchmark-results.md` auto-updated by nightly CI with a summary table and links to criterion HTML output on gh-pages.

## Non-goals

- We don't publish benchmarks against HyperFormula (different platform) or Excel itself (not automatable).
- We don't claim to beat Excel's own engine. We beat every other publicly-automatable engine.

## When to add a new benchmark

Any PR that:
- Adds a new builtin function.
- Changes a hot-path structure.
- Modifies allocation strategy.

...must include at least one micro-bench for the changed thing. PR description includes the before/after numbers.
