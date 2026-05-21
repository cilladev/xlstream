# xlstream

[![Crates.io](https://img.shields.io/crates/v/xlstream.svg)](https://crates.io/crates/xlstream)
[![docs.rs](https://docs.rs/xlstream/badge.svg)](https://docs.rs/xlstream)
[![CI](https://github.com/cilladev/xlstream/actions/workflows/ci.yml/badge.svg)](https://github.com/cilladev/xlstream/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/cilladev/xlstream#license)

A streaming Excel formula evaluation engine. Reads `.xlsx` files row-by-row, evaluates formulas in bounded memory, writes results to a new `.xlsx`. No dependency graph, no full-workbook buffering.

## Quick start

```toml
[dependencies]
xlstream = "0.2"
```

```rust
use std::path::Path;

fn main() -> Result<(), xlstream::XlStreamError> {
    let summary = xlstream::evaluate(
        Path::new("input.xlsx"),
        Path::new("output.xlsx"),
        &xlstream::EvaluateOptions::default(),
    )?;

    println!(
        "Evaluated {} formulas across {} rows in {:?}",
        summary.formulas_evaluated,
        summary.rows_processed,
        summary.duration,
    );

    Ok(())
}
```

## Performance

**3x less memory and 1.4x faster than LibreOffice. No install. Embeddable.**

Benchmark: 100k rows x 50 cols (20 data + 30 formula). Intel i9-10910, 128 GB RAM.

| Engine | Version | Wall-clock | Peak RSS | Architecture |
|---|---|---|---|---|
| **xlstream (1 worker)** | 0.2.1 | **26.5s** | **643 MB** | Streaming (2-pass) |
| **xlstream (4 workers)** | 0.2.1 | **23.0s** | **681 MB** | Streaming (2-pass) |
| LibreOffice | 26.2 | 31.9s | 2,081 MB | Graph |
| Excel (20 threads) | 16.108.2 | ~99s | ~430 MB | Graph |
| formualizer | 0.5.6 | 2h 8m | 11,322 MB | Full dependency graph |

Parallelism is automatic — rows are sharded across cores when the sheet has >= 10k data rows. Control it explicitly:

```rust
use std::path::Path;
use xlstream::EvaluateOptions;

let opts = EvaluateOptions { workers: Some(4), ..Default::default() };
xlstream::evaluate(Path::new("in.xlsx"), Path::new("out.xlsx"), &opts)?;
```

## 225 Excel-compatible functions

| Category   | Count | Examples                                          |
| ---------- | ----- | ------------------------------------------------- |
| Operators  | 13    | `+`, `-`, `*`, `/`, `^`, `&`, `%`, comparisons    |
| Logical    | 11    | IF, IFS, SWITCH, IFERROR, AND, OR, NOT, XOR       |
| Aggregates | 15    | SUM, SUMIF, SUMIFS, AVERAGE, COUNTIF, MEDIAN      |
| Lookups    | 7     | VLOOKUP, XLOOKUP, INDEX/MATCH, HLOOKUP, CHOOSE    |
| Text       | 19    | LEFT, UPPER, TRIM, CONCAT, TEXT, FIND, SUBSTITUTE |
| Date       | 12    | TODAY, YEAR, EDATE, EOMONTH, DATEDIF, NETWORKDAYS |
| Math       | 23    | ROUND, MOD, ABS, SQRT, LOG, SIN, PI, FLOOR        |
| Info       | 10    | ISNUMBER, ISTEXT, ISERROR, ISBLANK, ISREF, TYPE   |
| Financial  | 6     | PMT, PV, FV, NPV, IRR, RATE                       |

[Full cross-reference](https://github.com/cilladev/xlstream/blob/main/docs/functions.md) against Excel, formualizer, and xlcalculator.

## Error handling

```rust
use std::path::Path;
use xlstream::{evaluate, XlStreamError};

match evaluate(Path::new("in.xlsx"), Path::new("out.xlsx"), &Default::default()) {
    Ok(summary) => println!("{} formulas evaluated", summary.formulas_evaluated),
    Err(XlStreamError::Unsupported { address, formula, reason, .. }) => {
        eprintln!("Unsupported formula at {address}: {formula} ({reason})");
    }
    Err(e) => eprintln!("Error: {e}"),
}
```

Unsupported formulas (OFFSET, INDIRECT, LAMBDA, dynamic arrays) are rejected at classification with a specific reason and documentation link — not at runtime.

## Architecture

Two-pass streaming model:

1. **Prelude** — single-threaded scan. Computes whole-column aggregates (SUM, AVERAGE, COUNT), loads lookup sheets into hash-indexed memory, resolves named ranges.
2. **Row stream** — multi-threaded. Each row is evaluated using prelude scalars and the current row's cells. No row reads from future rows. Bounded memory regardless of file size.

Formulas that can't fit this model (OFFSET, INDIRECT, dynamic arrays, LAMBDA) are refused at classification with a clear error.

## Workspace crates

| Crate | Purpose |
|---|---|
| [`xlstream`](https://crates.io/crates/xlstream) | Facade — re-exports the public API (this crate) |
| [`xlstream-eval`](https://crates.io/crates/xlstream-eval) | Streaming evaluator, builtins, parallel dispatch |
| [`xlstream-parse`](https://crates.io/crates/xlstream-parse) | Formula parser adapter + streaming classifier |
| [`xlstream-io`](https://crates.io/crates/xlstream-io) | Calamine reader + rust_xlsxwriter writer |
| [`xlstream-core`](https://crates.io/crates/xlstream-core) | Value, error, and date types |

## Python bindings

Also available as a Python package with the same streaming architecture:

```bash
pip install xlstream
```

```python
import xlstream
result = xlstream.evaluate("input.xlsx", "output.xlsx")
```

## Minimum supported Rust version

1.88

## License

Dual-licensed under [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option.
