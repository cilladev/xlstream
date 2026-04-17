# Phase 10 — Row parallelism

**Goal:** scale the row-evaluation pass across cores. Row-independent workloads approach linear speedup.

**Estimated effort:** 3–4 days.

**Prerequisites:** Phase 4 plus enough builtins to test on a realistic workload (Phases 5–8 at least).

**Reading:** [`docs/architecture/parallelism.md`](../architecture/parallelism.md).

**Output:** evaluating the reference 400k-row workload on 8 cores takes ~2 min instead of ~15 min single-threaded.

## Checklist

### Dispatch

- [ ] `evaluate(input, output, workers)`:
  - [ ] If `workers == Some(1)` or row count < 10,000: single-threaded.
  - [ ] Else: parallel path.
- [ ] `workers` resolves via `num_cpus::get()` when `None`.

### Worker fleet

- [ ] Spawn N threads. Each owns:
  - [ ] Its own `calamine::Xlsx` handle (re-opened from input path).
  - [ ] A seeked `XlsxCellReader` positioned at its row-range start.
  - [ ] A cloned `Interpreter` + shared `&Prelude` (`Arc<Prelude>`).
- [ ] Row ranges: `chunk_size = total_rows.div_ceil(workers)`; worker `i` owns rows `[start + i*chunk_size, start + (i+1)*chunk_size)`.
- [ ] Each worker iterates its range, sending `(row_idx, Vec<Value>)` into a shared bounded channel.

### Channel

- [ ] `crossbeam_channel::bounded(workers * 1024)` — back-pressure.
- [ ] Workers drop their TX; main-thread writer drops `rx` when loop finishes.

### Writer thread

- [ ] Single writer thread. Pulls from channel, inserts into a `BTreeMap<u32, Vec<Value>>` reorder buffer.
- [ ] Drains in row-order when contiguous rows are available.
- [ ] On channel close, drains remaining buffer.

### Reader seek

- [ ] calamine's `XlsxCellReader` doesn't natively seek. Implement:
  - [ ] `fn seek_to_row(&mut self, target: u32) -> Result<(), XlStreamError>` that discards cells until `row >= target`.
  - [ ] For worker 8 of 8 at 350k: the discard takes ~0.5–1 s. Document as known cost for v0.1.
- [ ] Optimisation for v0.2: pre-scan xlsx to build a row-offset index.

### Thread pool

- [ ] Use `rayon::ThreadPoolBuilder::new().num_threads(workers).build()` for the local pool.
- [ ] Don't use the global pool — avoids conflicting with polars or other rayon consumers in the same process.

### Determinism

- [ ] Parallelism must not change output order or values.
- [ ] Reorder buffer guarantees strictly increasing row output.
- [ ] RNG for `RAND()` / `RANDBETWEEN` must be deterministic when a seed is supplied; see next item.

### Volatile determinism

- [ ] `Prelude::volatile` carries a single `TODAY` / `NOW` / `RAND` per run. All workers share it.
- [ ] If `EvaluateOptions::random_seed: Option<u64>` is set, `RAND()` / `RANDBETWEEN` use a seeded `SmallRng`; otherwise use `thread_rng`.

### Tests

- [ ] Single-threaded vs parallel, same input → identical output.
- [ ] Benchmark: 400k × 20 single-threaded vs 8 workers. Record speedup. Target ≥ 6× on an 8-core machine for row-local workloads.
- [ ] Worker count = 1 equivalent to the single-threaded path.
- [ ] Fuzz-style: random workers ∈ {1, 2, 4, 8, 16}; outputs identical.

### Error propagation

- [ ] If any worker returns `Err`, the writer thread aborts, other workers are cancelled, the overall `evaluate` returns the first error.
- [ ] Test: inject a malformed row partway through; verify error is returned cleanly and no partial output remains.

## Done when

Parallelism delivers near-linear speedup on row-local workloads. Outputs identical across worker counts. Error handling clean.
