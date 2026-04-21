# Benchmarks — research and pre-build analysis

Pre-build investigation: formualizer baseline, workload design, methodology decisions.

For measured xlstream performance numbers, see [`benchmarks/reports/`](../../benchmarks/reports/).

## The reference workload

`benchmark_large_formulas.xlsx`:
- Main sheet: 700,001 rows x 20 columns.
- 10 data columns + 10 formula columns.
- Formula mix: 2 simple arithmetic, 2 aggregate (SUM, SUMIF), 4 VLOOKUP (single-key into lookup sheets), 2 conditional (IFS, IF + VLOOKUP).
- Plus 2 lookup sheets (Thresholds: 25 rows with a pre-computed `RegionBusiness` helper column; Region Info: 5 rows).
- On-disk size: ~56 MB.

This mirrors a realistic trading/finance/retail analytics workbook.

## Formualizer baseline (measured 2026-04-17)

| Workload | formualizer RSS | formualizer wall |
|---|---|---|
| **Reference (700k)** | **3.3 GB** | **5h 40m** |

Reference-row details: 23.4 MB raw data xlsx, 56.1 MB with formulas, 71.7 MB evaluated CSV. Load phase 2m 30s; evaluate + save phase 5h 38m.

Extrapolated formualizer numbers (unconfirmed):

| Workload | formualizer RSS | formualizer wall |
|---|---|---|
| Smoke (1k) | ~400 MB | ~3 s |
| Small (10k) | ~700 MB | ~1 min |
| Medium (100k) | ~1.5 GB | ~20 min |
| Stress (1M) | likely OOM / hour+ | very long |

## Pre-build targets for v0.1

| Tier | Peak RSS | Wall-clock (8-core) |
|---|---|---|
| Smoke | < 40 MB | < 0.5 s |
| Small | < 50 MB | < 2 s |
| Medium | < 150 MB | < 15 s |
| Reference | < 250 MB | < 180 s (3 min) |
| Stress | < 500 MB | < 450 s (7.5 min) |

## Methodology

### Hardware

- Apple M-series (for macOS and arm64 Linux), 8-core.
- x86_64 Linux CI runner (2 vCPU) for Linux benchmarks.

### Timing

`criterion` for micro-benchmarks. Wall-clock measured end-to-end via `std::time::Instant`.

### Memory

Peak RSS via `memory-stats` crate.

### Fixture generation

`cargo run -p xlstream-benchmarks --bin generate-fixtures` produces workbooks deterministically (seeded RNG). Not committed to git. CI regenerates before benchmarks run.

## Non-goals

- We don't publish benchmarks against HyperFormula (different platform) or Excel itself (not automatable).
- We don't claim to beat Excel's own engine. We beat every other publicly-automatable engine.

## When to add a new benchmark

Any PR that adds a builtin, changes a hot path, or modifies allocation strategy must include a micro-bench. PR description includes before/after numbers.
