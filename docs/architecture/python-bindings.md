# Python bindings

PyO3 0.28 + maturin 1.13. Pattern cribbed from `pydantic-core` / `huggingface/tokenizers`.

## Layout

```
bindings/python/
├── Cargo.toml               # cdylib, path-depends on xlstream-eval
├── pyproject.toml           # build-backend = maturin
├── src/
│   └── lib.rs               # #[pymodule] fn _xlstream
├── py_src/
│   └── xlstream/
│       ├── __init__.py      # imports from _xlstream
│       ├── _xlstream.pyi    # type stubs
│       └── py.typed         # PEP 561 marker
└── tests/
    └── test_evaluate.py
```

## Cargo.toml

```toml
[package]
name = "xlstream-python"
version.workspace = true
edition = "2021"

[lib]
name = "_xlstream"
crate-type = ["cdylib"]

[dependencies]
pyo3 = { version = "0.28", features = ["extension-module", "abi3-py39"] }
xlstream-core = { path = "../../crates/xlstream-core" }
xlstream-eval = { path = "../../crates/xlstream-eval" }
xlstream-io   = { path = "../../crates/xlstream-io" }
```

## pyproject.toml

```toml
[build-system]
requires = ["maturin>=1.13,<2.0"]
build-backend = "maturin"

[project]
name = "xlstream"
requires-python = ">=3.9"
dynamic = ["version"]
description = "Streaming Excel formula evaluation engine"
readme = "../../README.md"
license = { text = "MIT OR Apache-2.0" }
authors = [{ name = "Priscilla Emasoga" }]
classifiers = [
    "Programming Language :: Rust",
    "Programming Language :: Python :: 3",
    "License :: OSI Approved :: MIT License",
    "License :: OSI Approved :: Apache Software License",
]

[project.optional-dependencies]
dev = ["pytest>=8.0", "ruff>=0.6", "mypy>=1.10"]

[tool.maturin]
bindings = "pyo3"
module-name = "xlstream._xlstream"
python-source = "py_src"
features = ["pyo3/extension-module"]
strip = true
```

## `src/lib.rs`

```rust
use pyo3::prelude::*;
use pyo3::exceptions::{PyIOError, PyRuntimeError};
use pyo3::create_exception;

create_exception!(xlstream, XlStreamError, pyo3::exceptions::PyException);
create_exception!(xlstream, UnsupportedFormula, XlStreamError);
create_exception!(xlstream, FormulaParseError, XlStreamError);
create_exception!(xlstream, ClassificationError, XlStreamError);
create_exception!(xlstream, CircularReferenceError, XlStreamError);

impl From<xlstream_core::XlStreamError> for PyErr {
    fn from(e: xlstream_core::XlStreamError) -> PyErr {
        use xlstream_core::XlStreamError::*;
        let msg = e.to_string();
        match e {
            Io { .. } | Xlsx(_) | XlsxWrite(_) => PyIOError::new_err(msg),
            Unsupported { .. }       => UnsupportedFormula::new_err(msg),
            FormulaParse { .. }      => FormulaParseError::new_err(msg),
            Classification { .. }    => ClassificationError::new_err(msg),
            CircularReference { .. } => CircularReferenceError::new_err(msg),
            Internal(_)              => PyRuntimeError::new_err(msg),
        }
    }
}

/// Evaluate formulas in an xlsx file and write the result.
///
/// Arguments:
///   input_path: path to the source xlsx.
///   output_path: destination xlsx. If None, overwrite input_path.
///   workers: number of worker threads. Default: number of CPUs.
///
/// Returns: a dict with {rows_processed, duration_ms, peak_rss_bytes}.
#[pyfunction]
#[pyo3(signature = (input_path, output_path=None, *, workers=None))]
fn evaluate(
    py: Python<'_>,
    input_path: &str,
    output_path: Option<&str>,
    workers: Option<usize>,
) -> PyResult<Py<PyDict>> {
    let input = std::path::PathBuf::from(input_path);
    let output = output_path.map(std::path::PathBuf::from).unwrap_or_else(|| input.clone());

    // Release the GIL for the long-running Rust work.
    let summary = py.detach(move || {
        xlstream_eval::evaluate(&input, &output, workers)
    })?;

    // Acquire the GIL to build the return dict.
    let dict = pyo3::types::PyDict::new(py);
    dict.set_item("rows_processed", summary.rows_processed)?;
    dict.set_item("duration_ms", summary.duration_ms)?;
    dict.set_item("peak_rss_bytes", summary.peak_rss_bytes)?;
    Ok(dict.unbind())
}

#[pymodule]
fn _xlstream(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(evaluate, m)?)?;

    // Publish exception types on the module.
    m.add("XlStreamError",         m.py().get_type::<XlStreamError>())?;
    m.add("UnsupportedFormula",    m.py().get_type::<UnsupportedFormula>())?;
    m.add("FormulaParseError",     m.py().get_type::<FormulaParseError>())?;
    m.add("ClassificationError",   m.py().get_type::<ClassificationError>())?;
    m.add("CircularReferenceError",m.py().get_type::<CircularReferenceError>())?;

    Ok(())
}
```

## `py_src/xlstream/__init__.py`

```python
"""xlstream — streaming Excel formula evaluation engine."""

from ._xlstream import (
    evaluate,
    XlStreamError,
    UnsupportedFormula,
    FormulaParseError,
    ClassificationError,
    CircularReferenceError,
)

__all__ = [
    "evaluate",
    "XlStreamError",
    "UnsupportedFormula",
    "FormulaParseError",
    "ClassificationError",
    "CircularReferenceError",
]

__version__ = "0.1.0"
```

## `py_src/xlstream/_xlstream.pyi`

```python
from typing import Optional, TypedDict

class EvaluateSummary(TypedDict):
    rows_processed: int
    duration_ms: int
    peak_rss_bytes: int

def evaluate(
    input_path: str,
    output_path: Optional[str] = None,
    *,
    workers: Optional[int] = None,
) -> EvaluateSummary: ...

class XlStreamError(Exception): ...
class UnsupportedFormula(XlStreamError): ...
class FormulaParseError(XlStreamError): ...
class ClassificationError(XlStreamError): ...
class CircularReferenceError(XlStreamError): ...
```

## Development flow

```bash
# Dev install into a venv.
cd bindings/python
maturin develop --release

# Run Python tests.
pytest tests/

# Build a wheel for the current platform.
maturin build --release
```

## Why `abi3-py39`

- Builds a single wheel per platform that works on Python 3.9, 3.10, 3.11, 3.12, 3.13, 3.14.
- Matrix shrinks from 6 × 3 (py × os) to 1 × 3. CI and distribution simplified.
- Marginal runtime cost: small loss on a few hot-path PyO3 internals. Negligible for our workload (Rust does all the heavy lifting).

## GIL

Every call that does > 10 ms of Rust work wraps it in `py.detach(|| ...)`. For `evaluate`, that's the entire inner function — the GIL is released for the whole Rust eval.

Acquiring the GIL for the return dict is cheap (< 1 ms).

**Never call into Python types or PyO3 during `detach`.** The compiler catches this (you can't move a `Bound` / `Py<T>` into the closure), but it's worth re-stating.

## CI matrix

See [`../operations/ci.md`](../operations/ci.md) for the full GitHub Actions workflow. Summary: one job per OS (linux-x86_64, linux-aarch64, macos-universal2, windows-x64), each builds and tests against Python 3.12 (abi3 covers the rest).

## Python-side testing

Tests live in `bindings/python/tests/`. They run against an installed wheel (`maturin develop` during dev, `pip install dist/*.whl` in CI).

Example test:
```python
from pathlib import Path
import pytest
import xlstream

FIXTURES = Path(__file__).parent / "fixtures"

def test_simple_evaluation(tmp_path):
    result = xlstream.evaluate(
        str(FIXTURES / "arithmetic.xlsx"),
        str(tmp_path / "out.xlsx"),
    )
    assert result["rows_processed"] == 100

def test_unsupported_formula_raises(tmp_path):
    with pytest.raises(xlstream.UnsupportedFormula) as exc:
        xlstream.evaluate(
            str(FIXTURES / "has_offset.xlsx"),
            str(tmp_path / "out.xlsx"),
        )
    assert "OFFSET" in str(exc.value)
```

## Publishing

Tag a release → CI builds wheels across OS matrix → `maturin publish` uploads to PyPI. Rust crates go to crates.io via `cargo publish` in the same release job.

Full details: [`../operations/release.md`](../operations/release.md).
