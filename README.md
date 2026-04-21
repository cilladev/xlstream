# xlstream

![CI](https://github.com/cilladev/xlstream/actions/workflows/ci.yml/badge.svg)
![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)

**Streaming Excel formula evaluation engine.** Rust core with Python bindings.

Evaluates xlsx formulas in a single streaming pass -- no dependency graph, no full-workbook buffering. 117 Excel functions, row-parallel execution, bounded memory.

## Performance

| | xlstream | formualizer |
|---|---|---|
| 700k rows x 20 cols | **48s** | 5h 40m |
| Peak memory | **734 MB** | 3.3 GB |
| Architecture | Streaming (2-pass) | Full dependency graph |

**425x faster** on the same workbook. [Full benchmarks](docs/benchmarks.md)

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

### CLI

```bash
xlstream evaluate input.xlsx -o output.xlsx -w 8 --verbose
```

### Rust

```rust
let summary = xlstream_eval::evaluate(&input, &output, Some(8))?;
```

## What it supports

| Category | Count | Examples |
|---|---|---|
| Operators | 13 | `+`, `-`, `*`, `/`, `^`, `&`, `%`, comparisons |
| Conditionals / logical | 11 | IF, IFS, SWITCH, IFERROR, AND, OR, NOT |
| Aggregates | 15 | SUM, AVERAGE, SUMIF, COUNTIFS, MEDIAN |
| Lookups | 7 | VLOOKUP, XLOOKUP, INDEX/MATCH, CHOOSE |
| String | 19 | LEFT, UPPER, TRIM, CONCAT, TEXT, FIND |
| Date | 12 | TODAY, YEAR, EDATE, NETWORKDAYS |
| Math | 23 | ROUND, MOD, ABS, SQRT, LOG, SIN, PI |
| Info | 10 | ISNUMBER, ISTEXT, ISERROR, ISBLANK, NA, TYPE |
| Financial | 6 | PMT, PV, FV, NPV, IRR, RATE |
| **Total** | **117** | [Full list](docs/functions.md) |

## What it doesn't support

OFFSET, INDIRECT, FILTER, UNIQUE, SORT, LAMBDA, LET -- these require random cell access which breaks streaming. Table references are planned for v0.2. See [limitations](docs/architecture/streaming-model.md).

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

## Development

```bash
make install    # one-time: venv + rust + python + git hooks
make check      # fmt + clippy + tests + doctests
make bench      # benchmarks (rust + python)
make help       # all commands
```

## Documentation

- [Architecture](docs/architecture/) -- streaming model, crate layout, parallelism
- [Benchmarks](docs/benchmarks.md) -- measured performance across tiers
- [Functions](docs/functions.md) -- canonical list with status
- [Contributing](CONTRIBUTING.md) -- code standards, PR workflow

## License

Dual-licensed under Apache-2.0 or MIT, at your option.
