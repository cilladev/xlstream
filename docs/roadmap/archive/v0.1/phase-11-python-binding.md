# Phase 11 — Python binding

**Goal:** `pip install xlstream` works. `xlstream.evaluate(...)` from Python drives the Rust engine.

**Estimated effort:** ~1 week.

**Prerequisites:** Phase 10 complete. Phases 5–9 complete for a useful feature set.

**Reading:** [`docs/architecture/python-bindings.md`](../architecture/python-bindings.md), [`docs/research/pyo3-maturin.md`](../research/pyo3-maturin.md).

**Output:** PyPI wheels for Linux / macOS / Windows. A Python script can call `xlstream.evaluate("in.xlsx", "out.xlsx")`.

## Checklist

### Crate setup

- [x] `bindings/python/Cargo.toml`: cdylib, depends on `xlstream-core`, `xlstream-eval`, `xlstream-io`.
- [x] `bindings/python/pyproject.toml`: maturin backend, module-name `xlstream._xlstream`, python-source `py_src`.
- [x] `py_src/xlstream/__init__.py` re-exports the native API.
- [x] `py_src/xlstream/py.typed` — PEP 561 marker.
- [x] `py_src/xlstream/_xlstream.pyi` — type stubs.

### Native extension (`src/lib.rs`)

- [x] `#[pymodule] fn _xlstream(m: &Bound<'_, PyModule>) -> PyResult<()>`:
  - [x] Exposes `evaluate` function.
  - [x] Creates and publishes exception classes (`XlStreamError`, `UnsupportedFormula`, `FormulaParseError`, `ClassificationError`, `CircularReferenceError`).
- [x] `to_pyerr(e: RustXlStreamError) -> PyErr` — map every variant. Uses type alias to avoid naming collision with `create_exception!`.
- [x] `#[pyfunction] fn evaluate(py, input_path, output_path, *, workers=None)`:
  - [x] Wraps the inner Rust `evaluate` inside `py.detach(|| ...)` for GIL release.
  - [x] Returns a `dict` with `rows_processed`, `formulas_evaluated`, `duration_ms`.
- [x] Signature uses `Bound<'_, PyDict>` etc.; avoid deprecated `&PyDict`.

### Exception hierarchy

- [x] `create_exception!(xlstream, XlStreamError, pyo3::exceptions::PyException)`.
- [x] Subclasses: `UnsupportedFormula`, `FormulaParseError`, `ClassificationError`, `CircularReferenceError`.
- [x] `IOError`-class failures map to Python's built-in `OSError` to match Pythonic expectations.
- [x] Python `test_exceptions.py` verifies each class is raised on an appropriate input.

### GIL release

- [x] Every call that does > 10 ms of Rust work wraps in `py.detach`.
- [x] Verify via a test: run `evaluate` from multiple threads via `threading.Barrier` + `ThreadPoolExecutor`.

### Version sync

- [x] `pyproject.toml` uses `dynamic = ["version"]` so maturin reads from Cargo.
- [x] Version is set once in workspace `Cargo.toml` `[workspace.package]`.

### Development flow

```bash
cd bindings/python
pip install maturin pytest
maturin develop --release
pytest tests/
```

- [x] Document these commands in `bindings/python/README.md`.
- [x] Local dev target compiles in under 30 s after warm cache.

### Python tests

In `bindings/python/tests/`:

- [x] `test_evaluate.py`:
  - [x] Basic run on a tiny fixture (5 rows, 1 formula col).
  - [x] Missing input -> `OSError`.
  - [x] Unsupported formula -> `UnsupportedFormula`.
  - [x] Returned dict has all three keys with correct types.
- [x] `test_exceptions.py`:
  - [x] Each exception class is importable.
  - [x] Inheritance: `UnsupportedFormula` subclasses `XlStreamError` subclasses `Exception`.
- [x] `test_concurrency.py`:
  - [x] `evaluate` called from multiple threads runs concurrently.
  - [x] GIL release verified.

### CI wheels

- [x] `.github/workflows/ci.yml` has a `python` job per OS that runs `maturin develop` + pytest (already scaffolded with guard).
- [ ] `.github/workflows/release.yml` uses `PyO3/maturin-action@v1` to build wheels on: (deferred: CI platform matrix)
  - [ ] Linux x86_64 (manylinux). (deferred: CI platform matrix)
  - [ ] Linux aarch64. (deferred: CI platform matrix)
  - [ ] macOS x86_64. (deferred: CI platform matrix)
  - [ ] macOS arm64. (deferred: CI platform matrix)
  - [ ] Windows x64. (deferred: CI platform matrix)
- [ ] abi3-py39 means one wheel per (os, arch), covering 3.9+. (deferred: CI platform matrix)
- [ ] sdist job. (deferred: CI platform matrix)

### Wheel testing

- [ ] In release.yml, after building wheels, run pytest against the built wheel (not against `maturin develop`). Ensures the released artefact works. (deferred: CI platform matrix)
- [ ] Clean Python environment per OS; `pip install dist/*.whl`; `pytest`. (deferred: CI platform matrix)

### Documentation

- [x] `bindings/python/README.md`: install, usage, link to main docs.
- [x] Add Python usage section to root `README.md`.
- [x] `docs/architecture/python-bindings.md` updated to match implementation.

## Done when

- `pip install xlstream` works on a clean Python 3.9–3.14 across Linux / macOS / Windows (via TestPyPI first).
- Python tests pass in CI across the matrix.
- GIL release verified.
- Exception hierarchy complete.
- End-to-end: a Python script evaluates the 400k reference workload and returns a summary dict.
