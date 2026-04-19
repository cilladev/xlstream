# Phase 10 — Row parallelism

**Goal:** scale the row-evaluation pass across cores. Row-independent workloads approach linear speedup.

**Estimated effort:** 3–4 days.

**Prerequisites:** Phase 4 plus enough builtins to test on a realistic workload (Phases 5–8 at least).

**Reading:** [`docs/architecture/parallelism.md`](../architecture/parallelism.md).

**Output:** evaluating the reference 400k-row workload on 8 cores takes ~2 min instead of ~15 min single-threaded.

## Checklist

### Dispatch

- [x] `evaluate(input, output, workers)`:
  - [x] If `workers == Some(1)` or row count < 10,000: single-threaded.
  - [x] Else: parallel path.
- [x] `workers` resolves via `num_cpus::get()` when `None`.

### Worker fleet

- [x] Spawn N threads. Each owns:
  - [x] Its own `calamine::Xlsx` handle (re-opened from input path).
  - [x] A seeked `XlsxCellReader` positioned at its row-range start.
  - [x] A cloned `Interpreter` + shared `&Prelude` (`Arc<Prelude>`).
- [x] Row ranges: `chunk_size = total_rows.div_ceil(workers)`; worker `i` owns rows `[start + i*chunk_size, start + (i+1)*chunk_size)`.
- [x] Each worker iterates its range, sending `(row_idx, Vec<Value>)` into a shared bounded channel.

### Channel

- [x] `crossbeam_channel::bounded(workers * 1024)` — back-pressure.
- [x] Workers drop their TX; main-thread writer drops `rx` when loop finishes.

### Writer thread

- [x] Single writer thread. Pulls from channel, inserts into a `BTreeMap<u32, Vec<Value>>` reorder buffer.
- [x] Drains in row-order when contiguous rows are available.
- [x] On channel close, drains remaining buffer.

### Reader seek

- [x] calamine's `XlsxCellReader` doesn't natively seek. Implement:
  - [x] `fn seek_to_row(&mut self, target: u32) -> Result<(), XlStreamError>` that discards cells until `row >= target`.
  - [x] For worker 8 of 8 at 350k: the discard takes ~0.5–1 s. Document as known cost for v0.1.
- [ ] Optimisation for v0.2: pre-scan xlsx to build a row-offset index.

### Thread pool

- [x] Use `rayon::ThreadPoolBuilder::new().num_threads(workers).build()` for the local pool.
- [x] Don't use the global pool — avoids conflicting with polars or other rayon consumers in the same process.

### Determinism

- [x] Parallelism must not change output order or values.
- [x] Reorder buffer guarantees strictly increasing row output.
- [ ] RNG for `RAND()` / `RANDBETWEEN` must be deterministic when a seed is supplied; see next item.

### Volatile determinism

- [x] `Prelude::volatile` carries a single `TODAY` / `NOW` per run. All workers share it via `Arc<Prelude>`.
- [ ] `RAND()` / `RANDBETWEEN` deterministic seeding — deferred: no RAND/RANDBETWEEN builtins implemented. Will add deterministic seeding when those builtins land.

### Tests

- [x] Single-threaded vs parallel, same input → identical output.
- [ ] Benchmark: 400k × 20 single-threaded vs 8 workers. Record speedup. Target ≥ 6× on an 8-core machine for row-local workloads. (Requires reference workbook.)
- [x] Worker count = 1 equivalent to the single-threaded path.
- [x] Fuzz-style: random workers ∈ {1, 2, 4, 8, 16}; outputs identical.

### Error propagation

- [x] If any worker returns `Err`, the writer thread aborts, other workers are cancelled, the overall `evaluate` returns the first error.
- [x] Test: nonexistent file returns error cleanly.

## Done when

Parallelism delivers near-linear speedup on row-local workloads. Outputs identical across worker counts. Error handling clean.
