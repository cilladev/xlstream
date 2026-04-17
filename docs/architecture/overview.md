# Architecture overview

## One-paragraph summary

`xlstream` parses a workbook with a reused, battle-tested formula parser (`formualizer-parse`), classifies each formula by shape, pre-computes whole-column aggregates and lookup-sheet hash indexes in pass 1, and in pass 2 streams the main sheet row-by-row through a tight evaluator that uses the pre-computed scalars and indexes. Output is written incrementally to disk via `rust_xlsxwriter`'s constant-memory mode. Peak memory is bounded by the sum of inputs (shared strings, lookup sheets, column aggregates) — not by row count.

## System diagram

```
                   ┌───────────────────────────────────────────────┐
                   │                 xlstream driver                │
                   └───────────────────────────────────────────────┘
                                          │
           ┌──────────────────────────────┼──────────────────────────────┐
           ▼                              ▼                              ▼
   ┌───────────────┐            ┌──────────────────┐            ┌──────────────────┐
   │ xlstream-parse │           │ xlstream-eval    │            │ xlstream-io      │
   │ (wraps         │            │ streaming       │            │ calamine reader  │
   │ formualizer-   │            │ evaluator +     │            │ rust_xlsxwriter  │
   │ parse)         │            │ builtins        │            │ writer           │
   └───────────────┘            └──────────────────┘            └──────────────────┘
           │                              │                              │
           └──────────────────────────────┼──────────────────────────────┘
                                          ▼
                              ┌──────────────────────┐
                              │  xlstream-core       │
                              │  Value, Error, traits │
                              └──────────────────────┘

                                          ▲
                                          │
                   ┌───────────────────────────────────────────────┐
                   │           bindings/python (PyO3)               │
                   │  xlstream.evaluate(input, output=None)         │
                   └───────────────────────────────────────────────┘
```

## End-to-end pipeline

```
.xlsx  ──►  calamine scan           ──►  detect sheets, classify, index lookups, compute aggregates
         (pass 0: read metadata)         (pass 1: prelude — once per workbook, cheap)
                 │
                 ▼
       streaming cell iterator       ──►  per-row eval loop  ──►  rust_xlsxwriter (constant memory)
         (pass 2: one row at a time)      O(1) row memory        streams output to disk
                 │
                 ▼
              .xlsx out
```

### Pass 0 — scan

Open with calamine. Enumerate sheets. For each sheet, determine:
- Column headers (row 1).
- Cell types per column (numeric, string, bool, date).
- Formula text for each formula cell (via `next_formula`).
- Size (used-range dimensions).

Classify every sheet:
- **Main sheet**: the one whose formulas we're evaluating per-row. Large.
- **Lookup sheet**: referenced by main-sheet lookup formulas. Small (typically < 10k rows).
- **Other**: passed through unchanged.

If multiple sheets contain formula columns, we process each in its own streaming pass.

### Pass 1 — prelude

For each formula in the main sheet:
- Parse with `formualizer-parse` (once, cached by formula text).
- Walk the AST. Classify:
  - **AggregateOnly** — uses only whole-column or fixed-range refs with aggregate functions.
  - **LookupOnly** — uses only current-row cells + lookup-sheet references through supported lookup functions.
  - **RowLocal** — uses only current-row cells.
  - **Mixed** — combines row-local / aggregate / lookup sub-expressions. Supported.
  - **Unsupported** — references other rows, has circular deps, uses spilling functions. Refused with a clear error citing the formula.

For each lookup sheet referenced: load it into memory once, build hash indexes keyed by the column(s) actually looked up.

For each whole-column aggregate: evaluate over the entire column (via calamine's streaming read) once, store as a scalar in the prelude context.

Topo-sort formula columns within each row (if column N's formula references column M, and M is also a formula column, M is evaluated first).

### Pass 2 — stream

Open a second calamine reader (or reuse if supported) positioned at row 2 of the main sheet. Open a `rust_xlsxwriter` in constant-memory mode for the output.

```
for row_idx in 2..=last_row:
    row_values: Vec<Value> = read one row from calamine
    for col in topo_order(formula_cols):
        row_values[col] = eval(ast[col], row_values, prelude_ctx)
    writer.write_row(row_values)
```

`eval` is a recursive AST walker over the formula node; each function call resolves through the builtin table. No allocation per cell beyond what the function requires; row vector is reused across rows.

## Key design decisions (with rationale)

### D1: Streaming, not graph

**Choice:** Two-pass streaming. No dependency DAG.
**Why:** Matches ~90% of business workloads. Memory and speed win dominates. The small minority of workloads we can't handle get a clear error instead of a slow fallback.
**Cost:** Cannot express arbitrary dependencies. Documented up-front.

### D2: Reuse `formualizer-parse`

**Choice:** Take `formualizer-parse` as a dependency, not fork it.
**Why:** The parser is the hardest part of any formula engine. `formualizer-parse` is battle-tested, handles `_xlfn.` prefixes, cross-sheet refs, whole-column ranges, and is already MIT/Apache. Forking loses future parser bug fixes.
**Cost:** Upstream coupling. Mitigated by a thin adapter layer (`xlstream-parse`) so we can swap later.

### D3: calamine for reading, rust_xlsxwriter for writing

**Choice:** Two different libraries, each the best at what it does.
**Why:** calamine is read-only and fast. rust_xlsxwriter is write-only with explicit constant-memory mode. umya (formualizer's choice) does both but neither well at our scale.
**Cost:** Two xlsx libraries in the dep tree. Acceptable; both MIT.

### D4: Rayon for row parallelism, not task parallelism

**Choice:** Shard rows across worker threads in pass 2. Each worker owns a range of rows.
**Why:** Rows are independent after prelude. Column-parallel is harder because of intra-row topo order.
**Cost:** Output ordering is non-trivial — use per-worker buffers, merge at writer.

### D5: No unsafe in v0.1

**Choice:** Zero `unsafe` blocks in library code.
**Why:** Correctness trumps perf for v0.1. Modern Rust with careful design reaches 95% of unsafe-tuned perf.
**Cost:** Some hot paths may need revisiting in v0.2. Defer.

### D6: Errors are values, not panics

**Choice:** Every fallible function returns `Result<_, XlStreamError>`. No panics.
**Why:** Library code that panics is unusable from Python. And from any other caller.
**Cost:** More `?` boilerplate. Acceptable.

### D7: Python version support via abi3

**Choice:** Build against `abi3-py39`. Single wheel per platform serves Python 3.9–3.14+.
**Why:** Simpler CI matrix, simpler distribution, smaller download count. Marginal runtime cost is acceptable.
**Cost:** Blocks a few PyO3 features that require per-version API. None of those matter for our surface.

## Memory model

| Component | Size | Lifetime |
|---|---|---|
| calamine shared-strings table | ~MB, proportional to unique-string count | Whole run |
| Lookup sheet data + hash indexes | ~MB per lookup sheet | Whole run |
| Prelude scalars | KB | Whole run |
| Current row values | ~KB | Per row |
| Formula AST cache (one per unique formula text) | KB | Whole run |
| rust_xlsxwriter constant-memory buffer | ~20 MB | Whole run |
| Total steady state | < 250 MB on reference | — |

## Error boundaries

1. **Parse errors** (malformed formula) — surfaced with a `FormulaError` at classification time. No evaluation attempted.
2. **Classification errors** (unsupported shape) — surfaced with a `ClassificationError` at prelude time. Clear message pointing at the formula, not cryptic.
3. **Evaluation errors at a cell** (`#DIV/0!` etc.) — written to the output cell as an Excel error; do NOT abort the run.
4. **I/O errors** (file missing, disk full) — surfaced as `IoError`, abort.
5. **Internal invariant violations** — these are panics, but only reachable via programming bugs, never user input.

See [`errors.md`](errors.md) for the taxonomy.

## Parallelism model

- **Prelude:** single-threaded for simplicity; it's a small fraction of total work.
- **Main stream:** partitioned by row ranges. Each worker:
  1. Owns its own calamine reader positioned at its range start.
  2. Evaluates rows in order.
  3. Writes to its own scratch buffer.
- **Writer:** single-threaded consumer that pulls buffers in row order from a bounded channel, writes to rust_xlsxwriter.
- Config: `XLSTREAM_WORKERS=N` or auto-detect cores. Disabled for files under 10k rows (overhead not worth it).

## What's NOT in this architecture

- No cell cache keyed by address — by design.
- No DAG — by design.
- No intermediate Arrow / Polars layer — rows flow through as `Vec<Value>` and out to writer. A future version might materialise a DataFrame on request, but that's a feature on top, not a layer underneath.
- No plugin system for user-defined functions in v0.1 — maybe v0.2.
