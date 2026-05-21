# Benchmark Report — v0.3.1

**Date:** 2026-05-21
**Hardware:** iMac20,2 — Intel(R) Core(TM) i9-10910 CPU @ 3.60GHz, 128 GB
**Rust:** rustc 1.88.0
**Profile:** release (LTO fat, codegen-units=1)

## Tier results

| Tier | Rows | Workers | Wall-clock | Peak RSS |
|---|---|---|---|---|
| Small | 10,000 | 1 | 4.34 s | 74 MB |
| Small | 10,000 | 4 | 4.51 s | 70 MB |
| Medium | 100,000 | 1 | 42.91 s | 450 MB |
| Medium | 100,000 | 4 | 35.45 s | 485 MB |
| Large | 1,000,000 | 1 | 434.24 s | 1.5 GB |
| Large | 1,000,000 | 8 | 360.09 s | 1.7 GB |

## Comparison with v0.3.0

| Tier | v0.3.0 | v0.3.1 | Change |
|---|---|---|---|
| Small (1w) | 5.68 s | 4.34 s | -23.6% |
| Small (4w) | 6.57 s | 4.51 s | -31.4% |
| Medium (1w) | 56.43 s | 42.91 s | -24.0% |
| Medium (4w) | 46.04 s | 35.45 s | -23.0% |
| Large (1w) | 553.91 s | 434.24 s | -21.6% |
| Large (8w) | 481.21 s | 360.09 s | -25.2% |
