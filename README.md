# xlstream

![CI](https://github.com/cilladev/xlstream/actions/workflows/ci.yml/badge.svg)
![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)

`xlstream` is a **streaming Excel formula evaluation engine** written in Rust, with Python bindings. It reads `.xlsx` files row-by-row, evaluates formulas in a single (or two-pass) streaming traversal (no dependency graph, no full-workbook buffering), and writes the results to a new `.xlsx` — all in bounded memory regardless of file size.

We are not building a general-purpose spreadsheet engine. We are building the *fastest, leanest* Excel formula evaluator — supporting ~465 of Excel's ~500 functions, everything that fits a streaming architecture.

## Performance

**3x less memory and 1.4x faster than LibreOffice. No install. Embeddable.** [Full benchmarks](benchmarks/reports)

Benchmark: 100k rows x 50 cols (20 data + 30 formula). Intel i9-10910, 128 GB RAM.

| Engine | Version | Wall-clock | Peak RSS | Architecture |
|---|---|---|---|---|
| **xlstream (1 worker)** | 0.2.1 | **26.5s** | **643 MB** | Streaming (2-pass) |
| **xlstream (4 workers)** | 0.2.1 | **23.0s** | **681 MB** | Streaming (2-pass) |
| LibreOffice | 26.2 | 31.9s | 2,081 MB | Graph |
| Excel (20 threads) | 16.108.2 | ~99s | ~430 MB | Graph |
| formualizer | 0.5.6 | 2h 8m | 11,322 MB | Full dependency graph |


## Supported functions

225 functions + 13 operators across 12 categories. [Full list with cross-reference](docs/functions.md).

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

Excel has ~500 functions. ~465 are streaming-compatible. We're adding more each release — see the [roadmap](docs/roadmap/README.md).


## Install

**Rust:**
```bash
cargo add xlstream
```

**Python:**
```bash
pip install xlstream
```

## Usage

### Python

```python
import xlstream

result = xlstream.evaluate("input.xlsx", "output.xlsx")
# {'rows_processed': 100000, 'formulas_evaluated': 2999970, 'duration_ms': 23000}

# Parallel (row-sharded across cores)
result = xlstream.evaluate("input.xlsx", "output.xlsx", workers=8)
```

### Rust

```rust
use std::path::Path;

let summary = xlstream::evaluate(
    Path::new("input.xlsx"),
    Path::new("output.xlsx"),
    &xlstream::EvaluateOptions::default(),
)?;
```

## Error handling

```python
try:
    xlstream.evaluate("input.xlsx", "output.xlsx")
except xlstream.UnsupportedFormula as e:
    print(e)  # names the cell, formula, and reason
except xlstream.FormulaParseError as e:
    print(e)
except OSError as e:
    print(e)  # file not found, corrupt xlsx
```


## License

Dual-licensed under Apache-2.0 or MIT, at your option.
