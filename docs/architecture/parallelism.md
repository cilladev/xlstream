# Parallelism

Row sharding via rayon. Default: one worker per logical core. Disabled for rows < 10,000 — overhead not worth it.

## Model

```
        calamine reader opens input N times
                       │
        ┌──────────────┼──────────────┐
        ▼              ▼              ▼
  worker 1         worker 2         worker N
  rows 2..100k    rows 100k..200k  ...
        │              │              │
        └──────────────┼──────────────┘
                       ▼
              bounded MPSC channel
                       │
                       ▼
             single writer thread
             (rust_xlsxwriter constant memory)
```

Each worker evaluates its row range independently and pushes `(row_idx, Vec<Value>)` tuples into a bounded channel. A single writer thread drains the channel in row-index order (small reorder buffer) and writes each row.

## Why single writer

`rust_xlsxwriter` constant-memory mode requires strictly increasing row indices per sheet. Multi-writer to the same sheet is not supported.

## Why row sharding and not column sharding

Within a row, formula columns may depend on each other (topo order). Sharding by column within a row serialises — negates the win. Sharding by row is embarrassingly parallel after prelude.

## Implementation (Phase 10)

```rust
// evaluate() dispatches based on worker count and row threshold:
//   workers > 1 && formulas exist && total_data_rows >= 10,000 → parallel
//   otherwise → single-threaded (stream_single_threaded)

fn stream_parallel(
    input: &Path,
    output: &mut Writer,
    prelude: Arc<Prelude>,
    col_asts: Arc<HashMap<u32, Ast>>,
    topo_order: Vec<u32>,
    main_sheet: &str,
    total_data_rows: u32,
    num_workers: usize,
) -> Result<(u64, u64), XlStreamError> {
    let chunk_size = (total_data_rows as usize).div_ceil(num_workers);

    // Header: read from fresh reader, write before spawning workers.
    // Non-main sheets: written by caller before this function.

    let (tx, rx) = crossbeam_channel::bounded(num_workers * 1024);

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_workers)
        .build()?;

    for worker_id in 0..num_workers {
        let start_row = 1 + (worker_id * chunk_size) as u32;
        let end_row = (1 + ((worker_id + 1) * chunk_size) as u32)
            .min(1 + total_data_rows);
        // Each worker: open Reader, seek_to_row, eval, send row-by-row.
        pool.spawn(move || {
            if let Err(e) = run_worker(..., start_row, end_row, &tx) {
                let _ = tx.send(Err(e));
            }
        });
    }
    drop(tx); // channel closes when all workers finish

    // Reorder buffer: two-phase drain.
    // Phase 1 (channel open): drain contiguous rows from expected_row.
    // Phase 2 (channel closed): drain remaining rows via pop_first.
    // This handles sparse sheets (non-contiguous row indices) correctly.
    let mut buffer: BTreeMap<u32, (Vec<Value>, u64)> = BTreeMap::new();
    // ... drain loop ...
}

// Workers stream row-by-row into the channel. The bounded capacity
// provides back-pressure — if the writer falls behind, workers block
// on tx.send() rather than buffering unboundedly.
fn run_worker(
    input: &Path, main_sheet: &str, prelude: &Prelude,
    col_asts: &HashMap<u32, Ast>, topo_order: &[u32],
    start_row: u32, end_row: u32,
    tx: &Sender<Result<(u32, Vec<Value>, u64), XlStreamError>>,
) -> Result<(), XlStreamError> {
    let mut reader = Reader::open(input)?;
    let mut stream = reader.cells(main_sheet)?;
    stream.seek_to_row(start_row)?;
    let interp = Interpreter::new(prelude);
    while let Some((row_idx, mut row_values)) = stream.next_row()? {
        if row_idx >= end_row { break; }
        // evaluate formula cols in topo order, then send
        tx.send(Ok((row_idx, row_values, formula_count)))?;
    }
    Ok(())
}
```

## calamine reader per worker

Each worker opens its own calamine `Xlsx` handle and seeks to its row range. We cannot share a single reader across threads (it's not `Sync`). Opening N readers is O(1) for shared-strings extraction since each parses `sharedStrings.xml`; we eat the N× shared-strings load cost as a floor. For the 700k-row 56 MB reference workload, shared-strings is a few MB — ~8× this is acceptable.

**Optimisation (v0.2):** load `sharedStrings.xml` once, share across workers via `Arc<Vec<String>>` injected into each reader. Requires a fork or PR to calamine.

## `seek_to_row`

calamine's `XlsxCellReader` doesn't natively support "skip to row N without emitting cells up to N." We build our own by consuming and discarding cells until the row index reaches the target. On the 700k-row reference workload, worker 8 skips 350k rows — measurable but acceptable (~0.5–1s of discard).

**Optimisation (v0.2):** index the xlsx's sheet XML offsets by row (one pass; stored in memory) — O(1) seek.

## Output ordering guarantee

Reorder buffer is `BTreeMap<u32, Vec<Value>>`. Worst-case depth: the number of rows in flight ≈ num_workers × channel_buffer. For num_workers=8, buffer=1024: 8192 rows × ~160 bytes = ~1.3 MB. Flat, predictable.

## Disabling parallelism

```
XLSTREAM_WORKERS=1      # force single-threaded
XLSTREAM_WORKERS=4      # force 4 workers
XLSTREAM_WORKERS=auto   # num_cpus::get() (default)
```

Python API: `xlstream.evaluate(input, output, workers=4)`.

## Measured expectations

From formualizer-comparable numbers in the brief:
- Single-threaded streaming eval (pure per-row, no parallelism): ~8–12 minutes on 700k × 20 (conservative).
- 8-worker streaming: ~1.5–2 minutes (near-linear scaling for pure row-local; diluted by prelude + writer serialisation).

Benchmark harness is in `benchmarks/` — see [benchmarks reports](../../benchmarks/reports/).

## Thread pool ownership

`rayon::ThreadPoolBuilder::new().num_threads(n).build()` — local pool, not the global. We don't want our evaluation to contest with other rayon users in the same process (e.g., polars).

## Future: GPU? SIMD?

Not in v0.1. Revisit if benchmarks show specific kernels (aggregate sums, text matching) are bottlenecks.
