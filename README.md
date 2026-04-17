# xlstream

Streaming Excel formula evaluation engine. Rust core, Python bindings.

> **Status:** pre-alpha. Not yet published to crates.io or PyPI. See [`docs/phases/README.md`](docs/phases/README.md) for progress.

## What it does

Reads an `.xlsx` file row-by-row, evaluates formulas in a bounded-memory streaming traversal, and writes a new `.xlsx` with computed values. Built for workbooks where formulas are mostly row-local with small shared lookup tables and whole-column aggregates — the shape of ~90% of real business spreadsheets.

Target for v0.1: evaluate a 400,000-row × 20-column xlsx in **under 3 minutes wall-clock** with **peak RSS under 250 MB**.

## Why another engine

Existing Python-callable engines either hold the whole workbook in memory as a dependency graph (`formualizer`, 11 GB RSS on a 56 MB file) or are pure-Python and orders of magnitude slower (`pycel`, `xlcalculator`, `formulas`). xlstream trades feature breadth for architectural simplicity: streaming, two-pass, no circular refs, no iterative calc, no full dynamic-array spills. In return: 50–100× less memory and 5–10× more speed on the workloads that matter.

## Getting started (once v0.1 ships)

```bash
pip install xlstream
```

```python
import xlstream
xlstream.evaluate("input.xlsx", "output.xlsx")
```

## Development setup

Clone the repo, then from the project root:

```bash
make install
```

That's it. One command does everything:

- Verifies prerequisites (`git`, `python3`, `rustup`) are on PATH.
- Creates a Python virtualenv at `.venv/`.
- Installs the Rust toolchain + components from `rust-toolchain.toml`.
- Installs `maturin`, `pytest`, `ruff`, `pre-commit` into the venv.
- Installs git hooks (pre-commit, commit-msg, pre-push).

Then:

```bash
source .venv/bin/activate   # activate the venv
make check                  # validate: cargo fmt + clippy + tests + doctests
make help                   # see every available command
```

Prerequisites: `rustup` (https://rustup.rs), Python 3.9+, `git`, GNU make. Linux / macOS primary; Windows works via WSL.

## Documentation

- **Product brief:** [`docs/brief.md`](docs/brief.md)
- **Architecture:** [`docs/architecture/`](docs/architecture/)
- **Phased roadmap:** [`docs/phases/`](docs/phases/)
- **Research / competitive analysis:** [`docs/research/`](docs/research/)
- **Contributing:** [`CONTRIBUTING.md`](CONTRIBUTING.md) + [`docs/standards/`](docs/standards/)

## Licence

Dual-licensed under Apache-2.0 or MIT, at your option.
