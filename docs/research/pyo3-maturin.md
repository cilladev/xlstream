# PyO3 + maturin in 2026

## What we picked

- **PyO3 0.28** with the Bound API.
- **maturin 1.13+** as build tool.
- **abi3-py39** for single-wheel-per-platform.
- **Layout**: Rust workspace + `bindings/python/` nested crate. Pattern from `huggingface/tokenizers`.

## Why maturin over setuptools-rust

As of 2026, maturin is the default. Four of the five major PyO3-using projects (polars, pydantic-core, ruff, tokenizers) use it. Only tiktoken still uses setuptools-rust — and only because it predates maturin maturity.

maturin:
- Handles wheel building across platforms without custom setup.py logic.
- Integrates with `PyO3/maturin-action@v1` GitHub Action for matrix builds.
- Supports abi3 out of the box.
- Reads version from `Cargo.toml` via `dynamic = ["version"]` in `pyproject.toml`, so Rust and Python versions stay in sync.

## Recommended layout (cribbed from tokenizers)

```
myproject/
├── Cargo.toml                    # [workspace] members = ["crates/*", "bindings/python"]
├── rust-toolchain.toml
├── crates/
│   └── myproject-core/           # pure-Rust library
│       └── Cargo.toml            # published to crates.io
├── bindings/
│   └── python/
│       ├── Cargo.toml            # cdylib, depends on ../../crates/myproject-core
│       ├── pyproject.toml        # build-backend = maturin
│       ├── src/lib.rs            # #[pymodule] fn _myproject(m)
│       ├── py_src/
│       │   └── myproject/
│       │       ├── __init__.py   # re-exports from _myproject
│       │       └── py.typed
│       └── tests/                # pytest
├── .github/workflows/
│   ├── ci.yml
│   └── release.yml
└── README.md
```

## Why `bindings/python/` nested

- Keeps the root clean. `Cargo.toml` at root is for Rust; anyone can clone and do `cargo build --workspace` without pip.
- `bindings/python/pyproject.toml` is self-contained. `maturin build` from that dir works.
- Python source co-located with its binding crate — trivial to find.
- Mirrors the pattern of hf tokenizers (battle-tested at scale).

## Minimal `pyproject.toml`

```toml
[build-system]
requires = ["maturin>=1.13,<2.0"]
build-backend = "maturin"

[project]
name = "myproject"
requires-python = ">=3.9"
dynamic = ["version"]
description = "Short project description"
readme = "../../README.md"
license = { text = "MIT OR Apache-2.0" }
authors = [{ name = "Your Name" }]
classifiers = [
    "Programming Language :: Rust",
    "Programming Language :: Python :: 3",
]

[project.optional-dependencies]
dev = ["pytest>=8", "ruff>=0.6", "mypy>=1.10"]

[tool.maturin]
bindings = "pyo3"
module-name = "myproject._myproject"    # native ext is a submodule
python-source = "py_src"                # pure-python wrapper dir
features = ["pyo3/extension-module"]
strip = true
```

## Minimal binding Cargo.toml

```toml
[package]
name = "myproject-python"
version.workspace = true
edition = "2021"

[lib]
name = "_myproject"
crate-type = ["cdylib"]

[dependencies]
pyo3 = { version = "0.28", features = ["extension-module", "abi3-py39"] }
myproject-core = { path = "../../crates/myproject-core" }
```

## PyO3 0.28 idioms (changed from older versions)

### Bound API (0.21+, mandatory 0.25+)

Old (`&PyAny`, `Py<T>`) is gone. New:

```rust
fn foo(py: Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<i64> { ... }
```

### GIL release — `py.detach()`, not `allow_threads`

`Python::allow_threads` was renamed to `Python::detach` in PyO3 0.27. The old name is a deprecated alias.

```rust
#[pyfunction]
fn heavy(py: Python<'_>, data: Vec<u8>) -> PyResult<u64> {
    py.detach(|| {
        // Long Rust work here. No PyO3 types allowed inside.
        Ok(expensive_rust(&data))
    })
}
```

### Module-level setup

```rust
#[pymodule]
fn _myproject(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(foo, m)?)?;
    m.add("SomeError", m.py().get_type::<SomeError>())?;
    Ok(())
}
```

Note: argument is `&Bound<'_, PyModule>`, not `&PyModule`.

### Error mapping

```rust
use pyo3::create_exception;
use pyo3::exceptions::PyValueError;

create_exception!(myproject, MyError, pyo3::exceptions::PyException);
create_exception!(myproject, ParseError, MyError);

impl From<crate::Error> for PyErr {
    fn from(e: crate::Error) -> PyErr {
        match e {
            crate::Error::Parse(msg) => ParseError::new_err(msg),
            crate::Error::Io(e)      => PyValueError::new_err(e.to_string()),
        }
    }
}
```

Then `#[pyfunction] fn foo() -> PyResult<T>` uses `?` on `crate::Error` naturally.

## CI matrix for wheel building

`PyO3/maturin-action@v1` is the standard workflow building block.

```yaml
- uses: PyO3/maturin-action@v1
  with:
    target: ${{ matrix.target }}
    manylinux: ${{ matrix.manylinux || 'auto' }}
    working-directory: ./bindings/python
    args: --release --out dist
```

With `abi3-py39`, one wheel per (os, arch) covers Python 3.9 through whatever CPython ships. Matrix typically:
- Linux x86_64
- Linux aarch64
- macOS universal2 (or separate x86_64 + arm64)
- Windows x64

## Publishing

Two-step on tag push:

1. **PyPI** via `maturin upload` or `maturin-action` `command: upload`.
2. **crates.io** via `cargo publish` in dependency order.

Use GitHub "environments" to gate each step on manual approval.

## Version sync

pydantic-core's trick: `pyproject.toml` reads version from `Cargo.toml` via `dynamic = ["version"]`. maturin respects this. Result: one place to bump, Rust crate and Python wheel always match.

## Free-threaded Python (3.13t, 3.14t)

PyO3 0.28 supports the free-threaded (GIL-less) CPython builds via `#[pymodule(gil_used = false)]`. Defer for v0.1 — add when users ask.

## Type stubs

`py_src/myproject/_myproject.pyi` provides Python type hints for the native extension. `py.typed` (empty file) marks the package PEP-561-compliant.

```python
# _myproject.pyi
from typing import Optional

def evaluate(input: str, output: Optional[str] = None) -> dict: ...

class XlStreamError(Exception): ...
```

## Testing

```bash
cd bindings/python
maturin develop --release   # builds + installs into active venv
pytest tests/
```

In CI: same. Or `uv pip install -e .` (pydantic-core style) for faster cold starts.

## Unresolved choices

### abi3 or per-version wheels?

**abi3** is our choice. Simpler; marginal runtime cost is acceptable. Reconsider if we need a PyO3 feature that requires per-version API (none anticipated).

### Publish binding crate to crates.io?

No. `xlstream-python` is only useful as the cdylib. Publishing would be noise.

### Free-threaded Python support in v0.1?

Defer to v0.2 unless a user asks.

## References

- https://pyo3.rs/
- https://www.maturin.rs/
- Example repos:
  - https://github.com/pola-rs/polars
  - https://github.com/pydantic/pydantic-core
  - https://github.com/openai/tiktoken
  - https://github.com/astral-sh/ruff
  - https://github.com/huggingface/tokenizers
