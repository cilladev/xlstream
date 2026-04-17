# Phase 13 — Documentation polish

**Goal:** every public API documented, examples work, `docs/` reviewed for typos/broken links, optional mdBook site.

**Estimated effort:** 3–4 days.

**Prerequisites:** API surface stable (Phases 5–11 complete).

**Reading:** [`docs/standards/documentation.md`](../standards/documentation.md).

**Output:** `cargo doc --no-deps --open` shows polished rustdoc. Every doctest passes. Optional: mdBook site published.

## Checklist

### Rustdoc

- [ ] Every `pub` item has a doc comment. `#![warn(missing_docs)]` enforces at build.
- [ ] Every doc comment has at least one `# Examples` block.
- [ ] All examples compile under `cargo test --doc`.
- [ ] Cross-references use `[TypeName]` / `[function]` syntax; broken links fail the build.
- [ ] `# Errors` / `# Panics` sections on every fallible / non-total function.

### README files

- [ ] Root `README.md`:
  - [ ] Status badge.
  - [ ] `pip install xlstream` instructions.
  - [ ] Minimal Python example.
  - [ ] Link to `docs/`.
  - [ ] Link to crates.io / PyPI pages.
  - [ ] Benchmark one-liner ("evaluates 400k rows in 3 min on 60 MB RAM").
- [ ] Per-crate `README.md`:
  - [ ] One-sentence purpose.
  - [ ] Link to architecture doc.
  - [ ] Minimal Rust example.

### `docs/` review

- [ ] All internal links resolve.
- [ ] No stale "TODO" / "FIXME" markers.
- [ ] Every phase doc has final tick marks reviewed for accuracy (only ticked if actually shipped).
- [ ] Research docs reviewed — any outdated numbers? Recompute with real benchmarks from Phase 12.

### CHANGELOG

- [ ] `[Unreleased]` has entries for every user-facing change made during Phases 1–12.
- [ ] Sections: Added / Changed / Fixed / Removed / Deprecated / Security.
- [ ] Ready to promote to `[0.1.0] - YYYY-MM-DD` in Phase 14.

### mdBook site (optional for v0.1)

- [ ] `docs-site/` with `book.toml`.
- [ ] Re-use `docs/` markdown via symlinks or a build step.
- [ ] `mdbook serve` locally renders.
- [ ] `mdbook build` publishes to `gh-pages`.
- [ ] Custom theme optional; default is fine.

### Migration guide

- [ ] `docs/migration-from-formualizer.md`:
  - [ ] API comparison (formualizer vs xlstream).
  - [ ] Feature parity table (what's supported, what's not).
  - [ ] Performance comparison on common workloads.
  - [ ] Refused-formula handling — "if you got a `#REF!` before, you get a `ClassificationError` now, because the formula wasn't actually computable."

### Python docs

- [ ] `bindings/python/README.md` polished.
- [ ] Type stubs (`_xlstream.pyi`) match the actual API.
- [ ] `py_src/xlstream/__init__.py` module docstring present.
- [ ] `xlstream.__version__` works and matches.

### Review pass

- [ ] Fresh pair of eyes reads `docs/brief.md` → `docs/architecture/overview.md` → `README.md` → installs and runs an example. Any friction = doc gap. Fix.

## Done when

- `cargo doc --no-deps` builds without warnings.
- `cargo test --doc` passes.
- All internal doc links work.
- Migration guide written.
- CHANGELOG ready for release.
