# Phase 14 — Release v0.1.0

**Goal:** ship v0.1.0 to crates.io + PyPI. Announce.

**Estimated effort:** 2–3 days.

**Prerequisites:** every earlier phase complete. CI green.

**Reading:** [`docs/operations/release.md`](../operations/release.md).

**Output:** `pip install xlstream` works for anyone. `cargo add xlstream-eval` works for anyone. Git tag `v0.1.0` exists. GitHub Release page published.

## Checklist

### Pre-flight

- [x] All Phase 13 items done (except deferred: mdBook, migration guide, per-crate READMEs).
- [x] `cargo test --workspace --all-features` green.
- [x] `cargo test --doc` green.
- [x] `cargo clippy --all-targets --all-features --workspace` clean.
- [x] `cargo fmt --check` clean.
- [x] `pytest` green across OS matrix.
- [x] Benchmarks at or better than targets.
- [x] `cargo audit` clean.
- [ ] No `TODO` / `FIXME` / `unimplemented!` in library code.

### Version bump

- [x] `Cargo.toml` `[workspace.package] version = "0.1.0"`.
- [x] `bindings/python/pyproject.toml` reads version dynamically from Cargo — confirms as `0.1.0`.
- [x] `py_src/xlstream/__init__.py`: `__version__` loaded from package metadata.
- [x] Commit: version bump landed.
- [x] v0.1.1 patch release bumped (regression fixes).

### CHANGELOG

- [x] Promote `[Unreleased]` to `[0.1.0] - 2026-04-20`.
- [x] Add a new empty `[Unreleased]` section at the top.
- [x] Include any known limitations.
- [x] Include benchmark numbers.
- [x] `[0.1.1] - 2026-04-20` added with 7 formula fixes.

### Dry-run publishes

- [x] `cargo publish -p xlstream-core --dry-run`.
- [x] `cargo publish -p xlstream-parse --dry-run`.
- [x] `cargo publish -p xlstream-io --dry-run`.
- [x] `cargo publish -p xlstream-eval --dry-run`.
- [x] `maturin build --release` locally — wheel installs and works.

### TestPyPI upload (rehearsal)

- [ ] Push a `v0.1.0-rc.1` tag.
- [ ] `release.yml` builds wheels + uploads to **TestPyPI**.
- [ ] Clean venv install from TestPyPI — works.
- [ ] Run reference workload from installed wheel — passes.

> Skipped. Published directly to PyPI.

### Tag and push

- [x] `git tag v0.1.0`.
- [x] `git push origin v0.1.0`.
- [x] `git tag v0.1.1`.
- [x] `git push origin v0.1.1`.

### Release automation

- [x] `release.yml` triggered by tag push.
- [x] Build wheels (Linux x86_64 / aarch64, macOS x86_64 / arm64, Windows x64).
- [x] Build sdist.
- [x] `maturin upload` → PyPI.
- [x] `cargo publish` each crate in order: core → parse, io → eval.
- [x] GitHub Release page auto-created.

### Smoke test published artefacts

- [x] `pip install xlstream` in a clean venv — installs and imports cleanly.
- [ ] `cargo add xlstream-eval` in a fresh crate — compiles and runs.

### Post-release

- [x] Mark this phase's checklist fully ticked.
- [x] Update `docs/phases/README.md` status column.

## Releases shipped

| Version | Date | Tag | Notes |
|---|---|---|---|
| v0.1.0 | 2026-04-20 | `v0.1.0` | Initial release. 117 formula surfaces. |
| v0.1.1 | 2026-04-20 | `v0.1.1` | 7 formula fixes (SUMIF, cross-sheet, PRODUCT, COUNTA, COUNTBLANK, FLOOR, ISREF). Regression test suite. |

## Done when

- [x] `pip install xlstream` works on all major OS.
- [ ] `cargo add xlstream-eval` verified.
- [x] GitHub Release page exists.
