# xlstream-eval

Streaming Excel formula evaluator. Two-pass architecture: prelude (aggregates + lookups), then row-by-row streaming with optional parallelism.

## Public API

- **`evaluate(input, output, workers) -> Result<EvaluateSummary>`** -- main entry point. Reads xlsx, evaluates all formulas, writes results. `workers=None` auto-detects cores; `Some(n)` uses n threads if >= 10k rows.
- **`EvaluateSummary`** -- `rows_processed`, `formulas_evaluated`, `duration`
- **`Interpreter`** -- AST walker. `eval(node, scope) -> Value`
- **`Prelude`** -- pre-computed aggregates, lookup indexes, cached ranges, volatile data
- **`RowScope`** -- current row's cell values for evaluation context

## Builtins (70+)

Conditional (IF, IFS, SWITCH, IFERROR), aggregate (SUM, COUNT, AVERAGE, MIN, MAX), conditional aggregate (SUMIF, SUMIFS, COUNTIF, COUNTIFS, AVERAGEIF, AVERAGEIFS), lookup (VLOOKUP, XLOOKUP, HLOOKUP, INDEX, MATCH), string (LEFT, RIGHT, MID, CONCAT, TEXTJOIN, SUBSTITUTE, TEXT), date (DATE, YEAR, MONTH, NETWORKDAYS, WORKDAY, EDATE), math (ROUND, ABS, SQRT, MOD, POWER), financial (PMT, FV, PV, RATE), info (ISBLANK, ISNUMBER, ISTEXT, TYPE).

Full list: [`docs/functions.md`](../../docs/functions.md)

## What it does NOT own

- xlsx I/O (that's `xlstream-io`).
- Value types (that's `xlstream-core`).
- Formula parsing (that's `xlstream-parse`).

## Dependencies

`xlstream-core`, `xlstream-parse`, `xlstream-io`, `rayon`, `crossbeam-channel`, `num_cpus`, `phf`, `tracing`.

## Architecture

Pass 1 (prelude): single-threaded scan. Parse formulas, classify, compute whole-column aggregates, load lookup sheets, build hash indexes.

Pass 2 (row stream): multi-threaded if >= 10k rows. Each worker reads its row range, evaluates formulas in topological order, sends results through a bounded channel. Main thread drains in row order to the writer.

See [`docs/architecture/streaming-model.md`](../../docs/architecture/streaming-model.md).
