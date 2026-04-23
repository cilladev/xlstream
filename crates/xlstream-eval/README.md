# xlstream-eval

[![Crates.io](https://img.shields.io/crates/v/xlstream-eval.svg)](https://crates.io/crates/xlstream-eval)
[![docs.rs](https://docs.rs/xlstream-eval/badge.svg)](https://docs.rs/xlstream-eval)

Streaming Excel formula evaluator for the [xlstream](https://github.com/cilladev/xlstream) engine. Two-pass architecture: prelude (aggregates + lookups), then row-by-row streaming with automatic parallelism.

This is the main Rust entry point. For Python, use `pip install xlstream`.

## Usage

```rust
use std::path::Path;
use xlstream_eval::evaluate;

let summary = evaluate(
    Path::new("input.xlsx"),
    Path::new("output.xlsx"),
    None, // auto-detect worker count
)?;
println!("Evaluated {} formulas in {:?}", summary.formulas_evaluated, summary.duration);
```

## What it provides

- **`evaluate(input, output, workers) -> Result<EvaluateSummary>`** -- reads xlsx, evaluates all formulas on all sheets, writes results
- **`EvaluateSummary`** -- `rows_processed`, `formulas_evaluated`, `duration`
- **`Interpreter`** -- AST walker for custom evaluation pipelines
- **`Prelude`** -- pre-computed aggregates, lookup indexes, cached ranges

## 103 Excel-compatible functions

Logical (IF, IFS, SWITCH, IFERROR), aggregate (SUM, COUNT, AVERAGE, MIN, MAX, MEDIAN), conditional aggregate (SUMIF, SUMIFS, COUNTIF, COUNTIFS, AVERAGEIF, AVERAGEIFS), lookup (VLOOKUP, XLOOKUP, HLOOKUP, INDEX, MATCH), string (LEFT, RIGHT, MID, CONCAT, TEXTJOIN, SUBSTITUTE, TEXT), date (DATE, YEAR, MONTH, NETWORKDAYS, EDATE), math (ROUND, ABS, SQRT, MOD, POWER), financial (PMT, FV, PV, IRR, RATE), info (ISBLANK, ISNUMBER, ISTEXT, TYPE).

Full list: [functions.md](https://github.com/cilladev/xlstream/blob/main/docs/functions.md)

## Architecture

**Pass 1 (prelude):** single-threaded scan. Parse formulas, classify, compute whole-column aggregates, load lookup sheets, build hash indexes. Runs once per formula-bearing sheet.

**Pass 2 (row stream):** multi-threaded if >= 10k rows. Each worker reads its row range, evaluates formulas in topological order, sends results through a bounded channel. Main thread drains in row order to the writer.

See [streaming-model.md](https://github.com/cilladev/xlstream/blob/main/docs/architecture/streaming-model.md).

## Dependencies

`xlstream-core`, `xlstream-parse`, `xlstream-io`, `rayon`, `crossbeam-channel`, `num_cpus`, `phf`, `tracing`.

## License

Dual-licensed under Apache-2.0 or MIT, at your option.
