# Testing

Four tiers. Every feature lands with the right ones.

1. **Unit tests** — in-module, `#[cfg(test)] mod tests`. Test one function, one behaviour.
2. **Integration tests** — in `tests/` of each crate. Test cross-module behaviour within a crate.
3. **End-to-end tests** — full pipeline through `evaluate()`: read xlsx, parse, classify, prelude, stream, write, verify output.
4. **Benchmarks** — `criterion` for perf-sensitive code.

Separately: **Python tests** in `bindings/python/tests/` via `pytest`.

## Golden rules

1. **Every public function has at least one test.**
2. **Every builtin function has at least one integration test that exercises it via the evaluator.** Unit tests that call the builtin directly (`fn sum(&[Value]) -> ...`) are necessary but not sufficient. Integration tests must feed a formula *through the parser, classifier, and evaluator* — the same path a real user hits. A builtin with only direct-call tests is not considered complete.
3. **Every bug fix has a regression test.** The test is added before the fix. The test fails, then the fix lands, then the test passes.
4. **No test is "too trivial."** Trivial code is where bugs hide.
5. **Tests are code.** Reviewed as rigorously as library code. Poorly named or unassertive tests are rejected.
6. **Tests must be deterministic.** No `std::thread::sleep`, no time-of-day dependence, no unseeded RNG.
7. **Tests must be fast.** Anything over 100 ms is flagged. Over 1 s requires a `#[cfg_attr(not(feature = "slow-tests"), ignore)]` marker.

## Unit test rules

- Test names are a full English sentence: `fn sum_of_numbers_with_bool_coerces_to_one`.
- Each test has exactly one `assert` per "thing tested." Multiple asserts are fine if they test the same thing.
- Arrange / Act / Assert structure, visually separated.
- No test helpers that encode logic under test. Table-driven tests are fine; complex setup that inverts to the production code is not.

Example:
```rust
#[test]
fn vlookup_exact_miss_returns_na() {
    // Arrange
    let index = build_lookup_index(&[
        (LookupKey::text("EMEA"), 1),
        (LookupKey::text("APAC"), 2),
    ]);

    // Act
    let result = vlookup_exact(&index, &LookupKey::text("NOT_THERE"));

    // Assert
    assert_eq!(result, Err(CellError::Na));
}
```

## Integration test structure

Each crate's `tests/` directory holds integration tests. One file per feature area.

```
crates/xlstream-eval/tests/
├── end_to_end.rs                        ← test binary root (imports submodules)
├── end_to_end/
│   ├── helpers/mod.rs                   ← shared fixture generators
│   ├── pipeline.rs                      ← core pipeline (cell refs, chained formulas, errors)
│   ├── lookups.rs                       ← VLOOKUP, XLOOKUP, INDEX/MATCH
│   ├── cross_sheet_aggregates.rs        ← cross-sheet SUMIF/COUNTIF/AVERAGEIF
│   ├── grouped_sumif.rs                 ← SUMIF with row-local criteria
│   ├── product_literals.rs              ← PRODUCT(2,3,4) inline eval
│   ├── named_ranges.rs                  ← named range resolution
│   └── secondary_sheet_formulas.rs      ← multi-sheet formula evaluation
├── fixtures/
│   └── regression.xlsx                  ← Excel-verified golden file
├── regression_base.rs                   ← golden-file comparison (all surfaces)
└── *.rs                                 ← per-category unit-level tests
```

### The two layers of integration testing

Every builtin lands with both:

1. **Evaluator-level integration** — drive the builtin via `xlstream_eval::Interpreter::eval` with a real AST node. Proves the builtin is reachable through dispatch, argument coercion, and error propagation. Lives in `crates/xlstream-eval/tests/<family>.rs`.
2. **End-to-end integration** (at least once per feature area) — drive the builtin via `xlstream_eval::evaluate(input, output, None)` on a programmatic fixture. Proves the builtin works through the full pipeline: read, classify, prelude, stream, write. Lives in `crates/xlstream-eval/tests/end_to_end/<feature>.rs`.

Evaluator-level integration is per-builtin. End-to-end is per-feature-area.

### Minimum bar per builtin PR

A PR that lands a new builtin is not mergeable until it includes:

- **≥ 1 unit test** at the builtin function level (direct `fn` call with a `&[Value]` slice).
- **≥ 1 evaluator-level integration test** that parses a formula string, evaluates it through `Interpreter`, and asserts the result.
- **Coverage of the five "shapes"** (happy path, empty args / edge case, error-in-argument propagation, coercion path, type mismatch → appropriate `CellError`). Can be split across unit + integration, but all five must appear somewhere for each builtin.

A PR that lands a new **feature area** (arithmetic, aggregates, lookups, etc.) adds:

- **≥ 1 end-to-end test** that evaluates a fixture xlsx containing formulas from that area and asserts the output matches an Excel-computed golden.

No exceptions without a design note in the PR explaining why.

## Regression testing

Two complementary approaches live in `crates/xlstream-eval/tests/`. See `tests/README.md` for full details.

### Golden-file regression (`regression_base.rs`)

A single workbook (`tests/fixtures/regression.xlsx`) exercises all 117 supported surfaces. Excel is the oracle.

1. Generate: `cargo test -p xlstream-eval --test regression_base -- generate_fixture --ignored --nocapture`
2. Open in Excel, save (populates cached values), commit.
3. Run: `cargo test -p xlstream-eval --test regression_base`

The test evaluates the fixture through xlstream and compares every formula cell against Excel's cached value. Volatile functions (TODAY, NOW) are skipped. Float comparison uses epsilon 1e-6. Cell errors from Excel are matched against xlstream's error-string representation.

**When to regenerate:** after adding new formula columns to the `FORMULAS` spec in `regression_base.rs`.

## Excel parity

Every builtin function has at least one test asserting its output against values produced by real Excel. The golden-file regression suite (`regression_base.rs`) is the primary mechanism — it covers all 117 surfaces in a single workbook verified by Excel. Edge cases (1900 leap year, boolean coercion, case-insensitive comparison, error propagation, operator precedence) are covered by the golden file and per-category integration tests.

## Benchmarks

`criterion` benches live in `benchmarks/benches/`. Never in the main crates.

```rust
fn bench_vlookup_10k(c: &mut Criterion) {
    let index = build_10k_lookup_index();
    c.bench_function("vlookup_exact_hit", |b| {
        b.iter(|| vlookup_exact(black_box(&index), black_box(&key)))
    });
}
```

### Benchmark coverage by code path

Every formula maps to one of these code paths. Each path has a criterion benchmark.

| Code path | Benchmark file | Formulas using it |
|---|---|---|
| Parser | `parse.rs` | all formulas |
| Binary/unary ops | `arithmetic.rs` | +, -, *, /, ^, %, &, comparisons |
| String ops | `string.rs` | LEFT, RIGHT, MID, UPPER, CONCAT, TEXT, etc. |
| Date serial conversion | `date.rs` | YEAR, MONTH, EDATE, NETWORKDAYS, etc. |
| Pure math | `math.rs` | ROUND, MOD, SQRT, ABS, INT, POWER, etc. |
| Iterative solvers | `financial.rs` | IRR, RATE, PMT, NPV, FV |
| Short-circuit eval | `conditional.rs` | IF, IFS, SWITCH, IFERROR, AND, OR |
| Type checking | `info.rs` | ISBLANK, ISNUMBER, TYPE, ISERROR |
| Hash index probe | `lookup.rs` | VLOOKUP, XLOOKUP, INDEX/MATCH |
| Streaming fold | `aggregate.rs` | SUM, COUNT, AVERAGE, SUMIF, COUNTIF |
| Full pipeline | `end_to_end.rs` | read+parse+classify+prelude+eval+write at 10k rows |

**Rule: when you add a formula with a novel code path** (not just dispatch to existing machinery), add a criterion bench for it in the matching `benches/<category>.rs` file. If no category fits, create a new file and add a `[[bench]]` entry to `benchmarks/Cargo.toml`.

### Reference workload

CI runs criterion micro-benchmarks on every PR via `bench-gate` (fails on >15% regression). Full tier benchmarks (100k+) run locally via `scripts/bench-report.sh`. Reports are stored in `benchmarks/reports/`.

## Memory profiling

Tests that assert on peak RSS use the `memory-stats` crate (or `procfs` on Linux, custom on macOS). We enforce:

| Test | Peak RSS |
|---|---|
| 10k × 20 | < 50 MB |
| 100k × 20 | < 150 MB |
| 400k × 20 | < 250 MB |

Tolerances are generous now; will tighten for v1.

## Python tests

`bindings/python/tests/test_evaluate.py` and siblings. Use pytest.

- Each Rust error type maps to a Python exception. Test the mapping directly.
- Test at least one evaluation path end-to-end (small xlsx → evaluate → assert output).
- Test that the GIL is released — by running evaluate in a thread pool and observing concurrency.
- Do NOT re-test what the Rust tests already cover. The Python layer tests **the binding**, not the engine.

## Running everything locally

```bash
# All Rust tests.
cargo test --all-features

# Doctests.
cargo test --doc

# Lints.
cargo clippy --all-targets --all-features -- -D warnings

# Format check.
cargo fmt --check

# Python tests.
cd bindings/python && maturin develop --release && pytest tests/

# Benchmarks (don't run in CI blocking path).
cargo bench
```

## CI gates

A PR merges only when:
- `cargo test --all-features` passes (unit + integration + doctests).
- `cargo clippy --all-targets --all-features -- -D warnings` is clean.
- `cargo fmt --check` is clean.
- `pytest` passes (for PRs touching the Python binding).
- Criterion bench-gate passes (no >15% regression on micro-benchmarks).

## Test data

- Golden-file fixture (`tests/fixtures/regression.xlsx`) — committed, Excel-verified.
- Programmatic fixtures — generated in test code via `rust_xlsxwriter`, no committed xlsx needed.
- Large fixtures for benchmarks — generated by `benchmarks/src/bin/generate_fixtures.rs`, gitignored.
