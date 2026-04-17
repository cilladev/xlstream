# Phase 11 — Python binding

**Goal:** `pip install xlstream` works. `xlstream.evaluate(...)` from Python drives the Rust engine.

**Estimated effort:** ~1 week.

**Prerequisites:** Phase 10 complete. Phases 5–9 complete for a useful feature set.

**Reading:** [`docs/architecture/python-bindings.md`](../architecture/python-bindings.md), [`docs/research/pyo3-maturin.md`](../research/pyo3-maturin.md).

**Output:** PyPI wheels for Linux / macOS / Windows. A Python script can call `xlstream.evaluate("in.xlsx", "out.xlsx")`.

## Checklist

### Crate setup

- [ ] `bindings/python/Cargo.toml`: cdylib, depends on `xlstream-core`, `xlstream-eval`, `xlstream-io`.
- [ ] `bindings/python/pyproject.toml`: maturin backend, module-name `xlstream._xlstream`, python-source `py_src`.
- [ ] `py_src/xlstream/__init__.py` re-exports the native API.
- [ ] `py_src/xlstream/py.typed` — PEP 561 marker.
- [ ] `py_src/xlstream/_xlstream.pyi` — type stubs.

### Native extension (`src/lib.rs`)

- [ ] `#[pymodule] fn _xlstream(m: &Bound<'_, PyModule>) -> PyResult<()>`:
  - [ ] Exposes `evaluate` function.
  - [ ] Creates and publishes exception classes (`XlStreamError`, `UnsupportedFormula`, `FormulaParseError`, `ClassificationError`, `CircularReferenceError`).
- [ ] `impl From<xlstream_core::XlStreamError> for PyErr` — map every variant.
- [ ] `#[pyfunction] fn evaluate(py, input_path, output_path=None, *, workers=None)`:
  - [ ] Wraps the inner Rust `evaluate` inside `py.detach(|| ...)` for GIL release.
  - [ ] Returns a `dict` with `rows_processed`, `duration_ms`, `peak_rss_bytes`.
- [ ] Signature uses `Bound<'_, PyDict>` etc.; avoid deprecated `&PyDict`.

### Exception hierarchy

- [ ] `create_exception!(xlstream, XlStreamError, pyo3::exceptions::PyException)`.
- [ ] Subclasses: `UnsupportedFormula`, `FormulaParseError`, `ClassificationError`, `CircularReferenceError`.
- [ ] `IOError`-class failures map to Python's built-in `OSError` / `IOError` to match Pythonic expectations.
- [ ] Python `test_exceptions.py` verifies each class is raised on an appropriate input.

### GIL release

- [ ] Every call that does > 10 ms of Rust work wraps in `py.detach`.
- [ ] Verify via a test: run `evaluate` in a `ThreadPoolExecutor`, check that N threads complete in roughly the same wall-clock as 1.

### Version sync

- [ ] `pyproject.toml` uses `dynamic = ["version"]` so maturin reads from Cargo.
- [ ] Version is set once in workspace `Cargo.toml` `[workspace.package]`.

### Development flow

```bash
cd bindings/python
pip install maturin pytest
maturin develop --release
pytest tests/
```

- [ ] Document these commands in `bindings/python/README.md`.
- [ ] Local dev target compiles in under 30 s after warm cache.

### Python tests

In `bindings/python/tests/`:

- [ ] `test_evaluate.py`:
  - [ ] Basic run on a tiny fixture (5 rows, 2 formula cols).
  - [ ] Missing input → `IOError`.
  - [ ] Unsupported formula → `UnsupportedFormula`.
  - [ ] Returned dict has all three keys with correct types.
- [ ] `test_exceptions.py`:
  - [ ] Each exception class is importable.
  - [ ] Inheritance: `UnsupportedFormula` subclasses `XlStreamError` subclasses `Exception`.
- [ ] `test_concurrency.py`:
  - [ ] `evaluate` called from multiple threads runs concurrently.
  - [ ] GIL release verified.

### CI wheels

- [ ] `.github/workflows/ci.yml` gains a `python` job per OS that runs `maturin develop` + pytest.
- [ ] `.github/workflows/release.yml` uses `PyO3/maturin-action@v1` to build wheels on:
  - [ ] Linux x86_64 (manylinux).
  - [ ] Linux aarch64.
  - [ ] macOS x86_64.
  - [ ] macOS arm64.
  - [ ] Windows x64.
- [ ] abi3-py39 means one wheel per (os, arch), covering 3.9+.
- [ ] sdist job.

### Wheel testing

- [ ] In release.yml, after building wheels, run pytest against the built wheel (not against `maturin develop`). Ensures the released artefact works.
- [ ] Clean Python environment per OS; `pip install dist/*.whl`; `pytest`.

### Documentation

- [ ] `bindings/python/README.md`: install, usage, link to main docs.
- [ ] Add Python usage section to root `README.md`.
- [ ] `docs/architecture/python-bindings.md` already has the API reference; verify it matches.

## Done when

- `pip install xlstream` works on a clean Python 3.9–3.14 across Linux / macOS / Windows (via TestPyPI first).
- Python tests pass in CI across the matrix.
- GIL release verified.
- Exception hierarchy complete.
- End-to-end: a Python script evaluates the 400k reference workload and returns a summary dict.
