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

## Implementation sketch

```rust
fn stream_parallel(
    input: &Path,
    output: &Path,
    prelude: Prelude,
    num_workers: usize,
) -> Result<(), XlStreamError> {
    let total_rows = calamine_used_rows(input)?;
    let chunk_size = total_rows.div_ceil(num_workers);

    let (tx, rx) = bounded_channel::<(u32, Vec<Value>)>(num_workers * 1024);

    let workers: Vec<_> = (0..num_workers).map(|i| {
        let tx = tx.clone();
        let prelude = prelude.clone();  // Arc<Prelude> — cheap
        let input = input.to_path_buf();
        let start = 2 + i * chunk_size;
        let end = (start + chunk_size).min(total_rows + 1);

        thread::spawn(move || {
            let mut reader = Reader::open(&input)?;
            let mut cells = reader.cells("MainSheet")?;
            cells.seek_to_row(start as u32)?;

            let interp = Interpreter::new(&prelude);
            for row_idx in start..end {
                let raw_row = cells.next_row()?.expect("row exists");
                let mut row = raw_row;
                for &fcol in &topo_order {
                    row[fcol] = interp.eval(&asts[fcol], &RowScope { ... })?;
                }
                tx.send((row_idx, row)).unwrap();
            }
            Ok::<_, XlStreamError>(())
        })
    }).collect();

    drop(tx); // close channel when all workers finish

    let mut writer = Writer::create(output)?;
    writer.add_sheet("MainSheet")?;
    // Drain in row order via a reorder buffer.
    let mut expected = 2u32;
    let mut buffer = BTreeMap::new();
    while let Ok((row_idx, row)) = rx.recv() {
        buffer.insert(row_idx, row);
        while let Some(row) = buffer.remove(&expected) {
            writer.write_row(expected, &row)?;
            expected += 1;
        }
    }
    writer.finish()?;

    for w in workers { w.join().unwrap()?; }
    Ok(())
}
```

## calamine reader per worker

Each worker opens its own calamine `Xlsx` handle and seeks to its row range. We cannot share a single reader across threads (it's not `Sync`). Opening N readers is O(1) for shared-strings extraction since each parses `sharedStrings.xml`; we eat the N× shared-strings load cost as a floor. For the 400k-row 56 MB reference workload, shared-strings is a few MB — ~8× this is acceptable.

**Optimisation (v0.2):** load `sharedStrings.xml` once, share across workers via `Arc<Vec<String>>` injected into each reader. Requires a fork or PR to calamine.

## `seek_to_row`

calamine's `XlsxCellReader` doesn't natively support "skip to row N without emitting cells up to N." We build our own by consuming and discarding cells until the row index reaches the target. On the 400k-row reference workload, worker 8 skips 350k rows — measurable but acceptable (~0.5–1s of discard).

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
- Single-threaded streaming eval (pure per-row, no parallelism): ~8–12 minutes on 400k × 20 (conservative).
- 8-worker streaming: ~1.5–2 minutes (near-linear scaling for pure row-local; diluted by prelude + writer serialisation).

Benchmark harness is in `benchmarks/` — see [phase-12](../phases/phase-12-benchmarks.md).

## Thread pool ownership

`rayon::ThreadPoolBuilder::new().num_threads(n).build()` — local pool, not the global. We don't want our evaluation to contest with other rayon users in the same process (e.g., polars).

## Future: GPU? SIMD?

Not in v0.1. Revisit if benchmarks show specific kernels (aggregate sums, text matching) are bottlenecks.
