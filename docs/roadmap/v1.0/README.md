# v1.0 Roadmap

**Status:** planning
**Target:** 2027 Q2
**Theme:** API stability, performance, documentation

No new functions. v1.0 is the stability commitment — after this release, the public API is frozen for 1 year (semver).

## API freeze

- [ ] **`evaluate()` signature locked** — `fn evaluate(input, output, &EvaluateOptions) -> Result<EvaluateSummary>`. No breaking changes.
- [ ] **`EvaluateOptions` fields locked** — `workers`, `iterative_calc`, `max_iterations`, `max_change`, `output_mode`. New fields are additive only (with defaults).
- [ ] **`EvaluateSummary` fields locked** — `rows_processed`, `formulas_evaluated`, `duration`. New fields additive only.
- [ ] **`OutputMode` enum locked** — `PreserveFormulas`, `ValuesOnly`. New variants are additive only.
- [ ] **Python API locked** — `xlstream.evaluate()` signature, kwargs, return dict shape, exception types.
- [ ] **CLI interface locked** — `xlstream evaluate`, `xlstream classify`. Flags and output format.
- [ ] **Error types locked** — `XlStreamError` variants. New variants additive only.
- [ ] **Audit all `pub` items** — remove or hide any pub item not intended as stable API. Anything pub at v1.0 stays pub.

## Performance hardening

- [ ] **Memory target: < 250 MB for 100k rows** — investigate and reduce calamine shared-strings buffering and rust_xlsxwriter string table overhead. Current: 643 MB (1w) / 681 MB (4w), evaluator itself ~10 MB. Measured 2026-05-13, bench_medium.xlsx (100k × 50).
- [ ] **Wall-clock target: < 15s for 100k rows** — profile and optimize hot paths. Current: 26.5s (1w) / 23.0s (4w). Measured 2026-05-13.
- [ ] **Benchmark regression gate** — CI blocks merge on >10% RSS or >20% wall-clock regression. Currently only micro-benchmarks are gated.
- [ ] **Memory benchmark in CI** — add tier benchmarks (small/medium/large) to CI with RSS tracking.

## Documentation

- [ ] **mdBook site** — hosted documentation covering architecture, API reference, streaming model, function reference. Replaces the `docs/` folder for external users.
- [ ] **Per-crate README audit** — verify each crate README is accurate and has a working example.
- [ ] **Python documentation** — docstrings on all public Python functions, published to readthedocs or equivalent.
- [ ] **Migration guide v0.x -> v1.0** — document any breaking changes from the v0.x series.

## Quality

- [ ] **100% conformance coverage** — every implemented function has a conformance fixture with >=15 formulas.
- [ ] **Fuzz testing** — fuzz the formula parser with cargo-fuzz. Target: 1M iterations without panic.
- [ ] **Property-based testing** — proptest for `extract_row_refs` / `reconstruct` round-trips and `FormulaTemplate` correctness.
- [ ] **Error message audit** — every `XlStreamError` variant produces an actionable error message with enough context to diagnose without reading source.

## Packaging

- [ ] **PyPI metadata** — correct project URLs, description, classifiers, license.
- [ ] **Cargo publish readiness** — all crates publishable to crates.io with correct metadata.
- [ ] **GitHub release automation** — tag triggers: build wheels, publish to PyPI, create GitHub release with changelog.
- [ ] **Supported platforms documented** — linux x86_64, macOS arm64/x86_64, Windows x86_64. Python 3.9+.

## Out of scope (post v1.0)

- New functions beyond the ~434 shipped in v0.1–v0.5
- `.xlsb`, `.xls`, `.ods` input formats
- External workbook references
- Cell formatting preservation
- RAND/RANDBETWEEN with deterministic seeding
- Incremental re-evaluation
- Web API / server mode

## Done when

- All boxes ticked
- `make check` passes
- All conformance tests pass
- Benchmark report meets targets (< 250 MB, < 15s for 100k rows)
- mdBook site published
- CHANGELOG promoted to `[1.0.0]`
- Tagged and released
- Blog post / announcement
