# Testing

Five tiers. Every feature lands with the right ones.

1. **Unit tests** — in-module, `#[cfg(test)] mod tests`. Test one function, one behaviour.
2. **Integration tests** — in `tests/` of each crate. Test cross-module behaviour within a crate.
3. **Workspace integration tests** — in `tests/` at repo root. Test cross-crate end-to-end.
4. **Property tests** — `proptest` for functions with algebraic properties.
5. **Fuzz tests** — `cargo-fuzz` for the parser and I/O layer.
6. **Benchmarks** — `criterion` for perf-sensitive code.

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
├── arithmetic.rs
├── conditional.rs
├── lookups.rs
├── aggregates.rs
└── end_to_end.rs
```

Fixtures (small .xlsx files) live in `fixtures/canonical/`. Committed to the repo, each < 50 KB. Large fixtures go in `fixtures/generated/` and are reproduced by a build script.

### The two layers of integration testing

Every builtin lands with both:

1. **Evaluator-level integration** — drive the builtin via `xlstream_eval::Interpreter::eval` with a real AST node. Proves the builtin is reachable through dispatch, argument coercion, and error propagation. Lives in `crates/xlstream-eval/tests/<family>.rs`.
2. **End-to-end integration** (at least once per feature area) — drive the builtin via `xlstream_eval::evaluate(input, output, None)` on a tiny committed xlsx fixture. Proves the builtin works through the full pipeline: read → classify → prelude → stream → write. Lives in `tests/end_to_end.rs` at the repo root.

Evaluator-level integration is per-builtin. End-to-end is per-feature-area (one `IF` fixture covers all logical builtins conceptually; one `VLOOKUP` fixture covers the lookup family).

### Minimum bar per builtin PR

A PR that lands a new builtin is not mergeable until it includes:

- **≥ 1 unit test** at the builtin function level (direct `fn` call with a `&[Value]` slice).
- **≥ 1 evaluator-level integration test** that parses a formula string, evaluates it through `Interpreter`, and asserts the result.
- **Coverage of the five "shapes"** (happy path, empty args / edge case, error-in-argument propagation, coercion path, type mismatch → appropriate `CellError`). Can be split across unit + integration, but all five must appear somewhere for each builtin.

A PR that lands a new **feature area** (arithmetic, aggregates, lookups, etc.) adds:

- **≥ 1 end-to-end test** that evaluates a fixture xlsx containing formulas from that area and asserts the output matches an Excel-computed golden.

No exceptions without a design note in the PR explaining why.

## Workspace integration tests

`tests/` at the repo root. Test the whole pipeline: input xlsx → evaluate → output xlsx → read → compare.

```rust
#[test]
fn reference_workload_small() {
    let input = fixtures::dir().join("canonical/benchmark_small.xlsx");
    let output = tempfile::NamedTempFile::new().unwrap();
    xlstream_eval::evaluate(&input, output.path(), None).unwrap();

    let df_actual = read_output(output.path());
    let df_expected = read_golden(fixtures::dir().join("canonical/benchmark_small_golden.xlsx"));

    assert_frames_equal(&df_actual, &df_expected);
}
```

## Regression testing

Two complementary approaches live in `crates/xlstream-eval/tests/`. See `tests/README.md` for full details.

### Golden-file regression (`regression.rs`)

A single workbook (`tests/fixtures/regression.xlsx`) exercises all 117 supported surfaces. Excel is the oracle.

1. Generate: `cargo test -p xlstream-eval --test regression -- generate_fixture --ignored --nocapture`
2. Open in Excel, save (populates cached values), commit.
3. Run: `cargo test -p xlstream-eval --test regression`

The test evaluates the fixture through xlstream and compares every formula cell against Excel's cached value. Volatile functions (TODAY, NOW) are skipped. Float comparison uses epsilon 1e-6. Cell errors from Excel are matched against xlstream's error-string representation.

**When to regenerate:** after adding new formula columns to the `FORMULAS` spec in `regression.rs`.

### Base regression tests (`regression_base.rs`)

Per-bug regression tests. Each test builds a minimal fixture programmatically and asserts exact output. No Excel needed.

- Tests land BEFORE the fix (`#[ignore = "blocked: ..."]`).
- Fix un-ignores the test.
- Test + fix commit together.

Naming: `test_<short_description>`.

## Excel parity

Every builtin function has at least one test asserting its output against values produced by real Excel. The golden-file regression suite (`regression.rs`) is the primary mechanism — it covers all 117 surfaces in a single workbook verified by Excel.

## The 1900 leap year test

Write it early. It's the most commonly-missed Excel compatibility trap.

```rust
#[test]
fn excel_treats_1900_02_29_as_valid() {
    // Excel serial 60 = Feb 29 1900 (not a real date; legacy Lotus bug).
    let v = ExcelDate::from_serial(60.0);
    assert_eq!(v.year_month_day(), (1900, 2, 29));
}

#[test]
fn dates_after_march_1900_match_excel() {
    let v = ExcelDate::from_serial(61.0);
    assert_eq!(v.year_month_day(), (1900, 3, 1));
}
```

## Accuracy test list (do early)

From the research:
- Text comparison case-insensitivity: `"a"="A"` → TRUE (except `EXACT`).
- Boolean coercion in arithmetic: `TRUE + 1` → 2.
- Empty cell vs 0 vs "": `ISBLANK(empty)` → TRUE, `ISBLANK(0)` → FALSE.
- Error propagation through arithmetic.
- Error interception by `IFERROR` / `IFNA`.
- Whole-column ranges don't allocate 1M cells.
- `VLOOKUP` approximate match on unsorted data returns Excel-compatible garbage (not our place to fix).
- Operator precedence: unary minus before `^` (Excel: `-2^2 = 4`, not `-4`).
- 1900 leap year (above).
- 1904 date system — workbook flag respected.

Each becomes a named test in `tests/accuracy_parity.rs`.

## Property tests

`proptest` for functions with algebraic properties.

```rust
proptest! {
    #[test]
    fn sum_is_commutative(a: f64, b: f64) {
        prop_assert_eq!(
            sum(&[Value::Number(a), Value::Number(b)]),
            sum(&[Value::Number(b), Value::Number(a)]),
        );
    }

    #[test]
    fn parsed_formula_round_trips(ast in ast_strategy()) {
        let s = ast.to_string();
        let parsed = parse(&s).unwrap();
        prop_assert_eq!(parsed, ast);
    }
}
```

Especially useful for: parser round-trip, arithmetic associativity, coercion idempotence.

## Fuzz tests

`cargo-fuzz` targets live in `fuzz/` at the workspace root.

- `fuzz_parser` — feeds arbitrary bytes to the parser. Must not panic.
- `fuzz_xlsx_reader` — feeds malformed xlsx files to the reader. Must return `Err`, not crash.
- `fuzz_classifier` — random ASTs into classification. Must return a valid `Classification` or `Unsupported`.

Run in CI weekly (scheduled workflow). Shorter run in per-PR CI (1 minute each target) to catch obvious regressions.

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

### Reference workload

`benchmarks/fixtures/reference_400k.xlsx` is the ground truth. Every perf claim in the docs comes from this workload.

CI runs the `quick` bench (5k rows) on every PR. Full tier benchmarks run locally via `make bench`.

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
- `cargo test --all-features` passes.
- `cargo test --doc` passes.
- `cargo clippy --all-targets --all-features -- -D warnings` is clean.
- `cargo fmt --check` is clean.
- `pytest` passes (for PRs touching the Python binding).
- Code coverage didn't drop (target: 85%+ line coverage on library crates).

## Test data

- Small (< 50 KB) xlsx fixtures → committed in `fixtures/canonical/`.
- Large xlsx fixtures → generated by a script, gitignored, reproduced on CI.
- Expected outputs → committed as JSON next to the input, one fixture per test.

Scripts to (re)generate fixtures live in `fixtures/scripts/`. Each is a small Rust binary or a Python openpyxl script, invoked by `cargo xtask regenerate-fixtures` (or a `Makefile` target).
