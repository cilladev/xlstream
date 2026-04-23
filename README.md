# xlstream

![CI](https://github.com/cilladev/xlstream/actions/workflows/ci.yml/badge.svg)
![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)

`xlstream` is a **streaming Excel formula evaluation engine** written in Rust, with Python bindings. It reads `.xlsx` files row-by-row, evaluates formulas in a single (or two-pass) streaming traversal (no dependency graph, no full-workbook buffering), and writes the results to a new `.xlsx` — all in bounded memory regardless of file size.

We are not building a general-purpose spreadsheet engine. We are building the *fastest, leanest* Excel formula evaluator — supporting ~465 of Excel's ~500 functions, everything that fits a streaming architecture.

## Performance

**425x faster** on the same workbook. [Full benchmarks](benchmarks/reports)

|                     | xlstream           | formualizer           |
| ------------------- | ------------------ | --------------------- |
| 700k rows x 20 cols | **48s**            | 5h 40m                |
| Peak memory         | **734 MB**         | 3.3 GB                |
| Architecture        | Streaming (2-pass) | Full dependency graph |


## Supported functions

103 functions + 13 operators across 9 categories. [Full list with cross-reference](docs/functions.md).

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

```bash
pip install xlstream
```

## Usage

### Python

```python
import xlstream

result = xlstream.evaluate("input.xlsx", "output.xlsx")
# {'rows_processed': 700001, 'formulas_evaluated': 7000000, 'duration_ms': 48000}

# Parallel (row-sharded across cores)
result = xlstream.evaluate("input.xlsx", "output.xlsx", workers=8)
```

### Rust

```rust
let summary = xlstream_eval::evaluate(&input, &output, Some(8))?;
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
