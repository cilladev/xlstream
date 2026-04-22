# Phase 13 — Documentation polish

**Goal:** every public API documented, examples work, `docs/` reviewed for typos/broken links, optional mdBook site.

**Estimated effort:** 3–4 days.

**Prerequisites:** API surface stable (Phases 5–11 complete).

**Reading:** [`docs/standards/documentation.md`](../standards/documentation.md).

**Output:** `cargo doc --no-deps --open` shows polished rustdoc. Every doctest passes. Optional: mdBook site published.

## Checklist

### Rustdoc

- [x] Every `pub` item has a doc comment. `#![warn(missing_docs)]` enforces at build.
- [x] Every doc comment has at least one `# Examples` block.
- [x] All examples compile under `cargo test --doc`.
- [x] Cross-references use `[TypeName]` / `[function]` syntax; broken links fail the build.
- [x] `# Errors` / `# Panics` sections on every fallible / non-total function.

### README files

- [x] Root `README.md`:
  - [x] Status badge.
  - [x] `pip install xlstream` instructions.
  - [x] Minimal Python example.
  - [x] Link to `docs/`.
  - [x] Link to crates.io / PyPI pages.
  - [x] Benchmark one-liner.
- [ ] Per-crate `README.md`:
  - [ ] One-sentence purpose.
  - [ ] Link to architecture doc.
  - [ ] Minimal Rust example.

### `docs/` review

- [x] All internal links resolve.
- [x] No stale "TODO" / "FIXME" markers.
- [x] Every phase doc has final tick marks reviewed for accuracy (only ticked if actually shipped).
- [x] Research docs reviewed — any outdated numbers? Recompute with real benchmarks from Phase 12.

### CHANGELOG

- [x] `[Unreleased]` has entries for every user-facing change made during Phases 1–12.
- [x] Sections: Added / Changed / Fixed / Removed / Deprecated / Security.
- [x] Ready to promote to `[0.1.0] - YYYY-MM-DD` in Phase 14.
- [x] Promoted to `[0.1.0] - 2026-04-20`.
- [x] `[0.1.1] - 2026-04-20` added with regression fixes.

### mdBook site (optional for v0.1)

- [ ] `docs-site/` with `book.toml`.
- [ ] Re-use `docs/` markdown via symlinks or a build step.
- [ ] `mdbook serve` locally renders.
- [ ] `mdbook build` publishes to `gh-pages`.
- [ ] Custom theme optional; default is fine.

> Deferred. Not blocking release.

### Migration guide

- [ ] `docs/migration-from-formualizer.md`:
  - [ ] API comparison (formualizer vs xlstream).
  - [ ] Feature parity table (what's supported, what's not).
  - [ ] Performance comparison on common workloads.
  - [ ] Refused-formula handling.

> Deferred. Not blocking release.

### Python docs

- [x] `bindings/python/README.md` polished.
- [x] Type stubs (`_xlstream.pyi`) match the actual API.
- [x] `py_src/xlstream/__init__.py` module docstring present.
- [x] `xlstream.__version__` works and matches.

### Review pass

- [x] Fresh pair of eyes reads `docs/brief.md` → `docs/architecture/overview.md` → `README.md` → installs and runs an example. Any friction = doc gap. Fix.

## Done when

- [x] `cargo doc --no-deps` builds without warnings.
- [x] `cargo test --doc` passes.
- [x] All internal doc links work.
- [ ] Migration guide written. (deferred)
- [x] CHANGELOG ready for release.
