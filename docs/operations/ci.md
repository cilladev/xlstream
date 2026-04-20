# CI

GitHub Actions. **Three** workflows: `pre-commit`, `ci`, `release`.

## Division of labour

- **`pre-commit.yml`** — runs the same hooks contributors run locally (fmt, clippy, test, doctests, ruff, typos, commit-msg format). Catches cases where a contributor skipped local hooks or used `--no-verify`. One OS (Ubuntu) — this is correctness, not platform matrix.
- **`ci.yml`** — platform matrix: (Linux / macOS / Windows) × (cargo test, cargo audit, Python wheel build + pytest). What pre-commit can't cover.
- **`release.yml`** — tag-triggered wheel builds + crates.io + PyPI publish.
- Benchmarks run per-PR via the `bench-smoke` job in `ci.yml`.

The `pre-commit` + `ci` split avoids running clippy and tests twice. See the file for the exact job list.

## `pre-commit.yml` — runs contributor-equivalent hooks

Already wired at `.github/workflows/pre-commit.yml`. Runs on every push + PR. Single Ubuntu job. Covers fmt, clippy, tests, doctests, ruff, typos, commit-msg via the same `.pre-commit-config.yaml` contributors use locally.

## `ci.yml` — platform matrix

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:

concurrency:
  group: ci-${{ github.ref }}
  cancel-in-progress: true

env:
  CARGO_TERM_COLOR: always
  RUSTFLAGS: -D warnings
  RUST_BACKTRACE: 1

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      fail-fast: false
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    steps:
      - uses: actions/checkout@v5
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo test --workspace --all-features
      - run: cargo test --workspace --doc

  python:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    steps:
      - uses: actions/checkout@v5
      - uses: actions/setup-python@v6
        with: { python-version: "3.12" }
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - name: Install maturin
        run: pip install maturin pytest
      - name: Build and install wheel
        working-directory: bindings/python
        run: maturin develop --release
      - name: Run pytest
        working-directory: bindings/python
        run: pytest -v

  audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v5
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo install cargo-audit --locked
      - run: cargo audit

  bench-smoke:
    runs-on: ubuntu-latest
    # Quick sanity benchmark on every PR.
    steps:
      - uses: actions/checkout@v5
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo bench --bench quick -- --sample-size 10
```

All PR-blocking jobs must pass. No merges past red CI.

## `release.yml` — on tag push

```yaml
name: Release

on:
  push:
    tags: ["v*"]

jobs:
  build-wheels:
    name: Build wheels (${{ matrix.os }}-${{ matrix.target }})
    runs-on: ${{ matrix.os }}
    strategy:
      fail-fast: false
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64
            manylinux: auto
          - os: ubuntu-latest
            target: aarch64
            manylinux: auto
          - os: macos-latest
            target: x86_64
          - os: macos-latest
            target: aarch64
          - os: windows-latest
            target: x64
    steps:
      - uses: actions/checkout@v5
      - uses: actions/setup-python@v6
        with: { python-version: "3.12" }
      - uses: PyO3/maturin-action@v1
        with:
          target: ${{ matrix.target }}
          manylinux: ${{ matrix.manylinux || 'auto' }}
          working-directory: ./bindings/python
          args: --release --out dist
      - uses: actions/upload-artifact@v5
        with:
          name: wheels-${{ matrix.os }}-${{ matrix.target }}
          path: bindings/python/dist/*.whl

  sdist:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v5
      - uses: PyO3/maturin-action@v1
        with:
          command: sdist
          working-directory: ./bindings/python
          args: --out dist
      - uses: actions/upload-artifact@v5
        with:
          name: sdist
          path: bindings/python/dist/*.tar.gz

  publish-pypi:
    needs: [build-wheels, sdist]
    runs-on: ubuntu-latest
    environment: release-pypi   # require manual approval
    steps:
      - uses: actions/download-artifact@v5
        with:
          pattern: wheels-*
          path: dist
          merge-multiple: true
      - uses: actions/download-artifact@v5
        with: { name: sdist, path: dist }
      - uses: PyO3/maturin-action@v1
        with:
          command: upload
          args: --non-interactive --skip-existing dist/*
        env:
          MATURIN_PYPI_TOKEN: ${{ secrets.PYPI_API_TOKEN }}

  publish-crates:
    needs: [build-wheels]
    runs-on: ubuntu-latest
    environment: release-crates
    steps:
      - uses: actions/checkout@v5
      - uses: dtolnay/rust-toolchain@stable
      - name: Publish core
        run: cargo publish -p xlstream-core --token ${{ secrets.CARGO_REGISTRY_TOKEN }}
      - name: Publish parse
        run: cargo publish -p xlstream-parse --token ${{ secrets.CARGO_REGISTRY_TOKEN }}
      - name: Publish io
        run: cargo publish -p xlstream-io --token ${{ secrets.CARGO_REGISTRY_TOKEN }}
      - name: Publish eval
        run: cargo publish -p xlstream-eval --token ${{ secrets.CARGO_REGISTRY_TOKEN }}
```

Crate publish order matters — dependents must wait for dependencies. `xlstream-core` first, then `parse` / `io` (which depend on core), then `eval` (which depends on all three).

## Required secrets

| Secret | Used by | Scope |
|---|---|---|
| `CARGO_REGISTRY_TOKEN` | `release.yml` | `environment: release-crates` |
| `PYPI_API_TOKEN` | `release.yml` | `environment: release-pypi` |

`environment: release-*` gates on manual approval. No automatic publish without an admin clicking approve.

## Branch protection rules (main)

- Require PR with 1 approval.
- Require status checks: `fmt`, `clippy`, `test`, `python`, `audit`.
- Require branches up-to-date before merge.
- No direct pushes.
- No force pushes.
- No deletion.

## Concurrency

Every workflow has `concurrency: { group: <name>-${{ github.ref }}, cancel-in-progress: true }` so pushing to a branch cancels the previous run. Saves time and minutes.

## Caching

`Swatinem/rust-cache@v2` for target directories. Roughly 50% CI speedup on incremental changes. Hash-keyed on `Cargo.lock`.

## Python version coverage

abi3-py39 means we build **one** wheel per (os, arch) that serves Python 3.9 through whatever CPython releases. CI builds against 3.12; if we ever want version-specific tests, add a `strategy.matrix.python-version` axis.

## Failing CI

A failing job is a bug. Fix the bug, don't mute the check. If a check is genuinely flaky, open an issue, investigate, and either fix the underlying issue or mark the test as known-flaky with a link to the tracking issue.
