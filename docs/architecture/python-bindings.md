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
use std::path::PathBuf;

use pyo3::prelude::*;
use pyo3::exceptions::{PyOSError, PyRuntimeError};
use pyo3::types::PyDict;

use xlstream_core::XlStreamError as RustXlStreamError;

pyo3::create_exception!(xlstream, XlStreamError, pyo3::exceptions::PyException,
    "Base class for all xlstream errors.");
pyo3::create_exception!(xlstream, UnsupportedFormula, XlStreamError,
    "A formula cannot be evaluated in streaming mode.");
pyo3::create_exception!(xlstream, FormulaParseError, XlStreamError,
    "A formula could not be parsed.");
pyo3::create_exception!(xlstream, ClassificationError, XlStreamError,
    "A formula could not be classified.");
pyo3::create_exception!(xlstream, CircularReferenceError, XlStreamError,
    "Formula columns form a dependency cycle.");

fn to_pyerr(e: RustXlStreamError) -> PyErr {
    let msg = e.to_string();
    match e {
        RustXlStreamError::Io { .. }
        | RustXlStreamError::Xlsx(_)
        | RustXlStreamError::XlsxWrite(_) => PyOSError::new_err(msg),
        RustXlStreamError::Unsupported { .. }    => UnsupportedFormula::new_err(msg),
        RustXlStreamError::FormulaParse { .. }   => FormulaParseError::new_err(msg),
        RustXlStreamError::Classification { .. } => ClassificationError::new_err(msg),
        RustXlStreamError::CircularReference { .. } => CircularReferenceError::new_err(msg),
        RustXlStreamError::Internal(_)           => PyRuntimeError::new_err(msg),
    }
}

/// Evaluate formulas in an xlsx workbook and write the results.
///
/// The GIL is released for the entire Rust evaluation so other Python
/// threads can run concurrently.
#[pyfunction]
#[pyo3(signature = (input_path, output_path, *, workers=None))]
fn evaluate(
    py: Python<'_>,
    input_path: &str,
    output_path: &str,
    workers: Option<usize>,
) -> PyResult<Py<PyDict>> {
    let input = PathBuf::from(input_path);
    let output = PathBuf::from(output_path);

    // Release the GIL for the long-running Rust work.
    let summary =
        py.detach(move || xlstream_eval::evaluate(&input, &output, workers))
            .map_err(to_pyerr)?;

    // Acquire the GIL to build the return dict.
    let dict = PyDict::new(py);
    dict.set_item("rows_processed", summary.rows_processed)?;
    dict.set_item("formulas_evaluated", summary.formulas_evaluated)?;
    dict.set_item("duration_ms", summary.duration.as_millis() as u64)?;
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

class EvaluateResult(TypedDict):
    rows_processed: int
    formulas_evaluated: int
    duration_ms: int

def evaluate(
    input_path: str,
    output_path: str,
    *,
    workers: Optional[int] = None,
) -> EvaluateResult: ...

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

Every call that does > 10 ms of Rust work wraps it in `py.detach(|| ...)`. For `evaluate`, that's the entire inner function — the GIL is released for the whole Rust eval. The result is converted back via `.map_err(to_pyerr)?` since error conversion uses a free function rather than a `From` impl (avoids name collision between the Python `XlStreamError` exception class from `create_exception!` and the Rust `xlstream_core::XlStreamError`, which is aliased as `RustXlStreamError`).

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
