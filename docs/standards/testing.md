# Testing

Two tiers for library crates. Every feature lands with both.

1. **Unit tests** — in-module `#[cfg(test)] mod tests`. Test one function, one behaviour. Live alongside the implementation.
2. **Conformance tests** — full pipeline through `evaluate()`, compared against LibreOffice cached values. The oracle is LibreOffice, not hand-computed assertions. One xlsx fixture per function.

Separately: **Benchmarks** via `criterion` and **Python tests** via `pytest`.

## Golden rules

1. **Every public function has at least one test.**
2. **Every builtin function has both a unit test and a conformance fixture.** Unit tests call the builtin directly. Conformance tests feed a formula through the full pipeline (parse, classify, prelude, evaluate, write) and compare against LibreOffice.
3. **Every bug fix has a regression test.** A conformance fixture in `fixtures/issues/` reproducing the bug, added before the fix.
4. **No test is "too trivial."** Trivial code is where bugs hide.
5. **Tests are code.** Reviewed as rigorously as library code.
6. **Tests must be deterministic.** No `std::thread::sleep`, no time-of-day dependence, no unseeded RNG.
7. **Tests must be fast.** Anything over 100 ms is flagged. Over 1 s requires `#[cfg_attr(not(feature = "slow-tests"), ignore)]`.

## Unit tests

Live in `#[cfg(test)] mod tests` at the bottom of each source file. Every crate has them.

### Rules

- Test names are full English sentences: `fn sum_of_numbers_with_bool_coerces_to_one`.
- Each test has exactly one `assert` per "thing tested." Multiple asserts fine if testing the same thing.
- Arrange / Act / Assert structure, visually separated.
- No test helpers that encode logic under test.

### Per-crate unit tests

| Crate | What's tested |
|-------|---------------|
| `xlstream-core` | Value types, coercion, error types, constants |
| `xlstream-parse` | Parser, classifier, rewriter, reference extraction, sets |
| `xlstream-eval` | Builtins (math, string, date, lookup, aggregate, conditional, info, financial), interpreter, prelude, topo sort |
| `xlstream-io` | Reader, writer, cell conversion |
| `xlstream-cli` | Argument parsing |

### xlstream-eval builtins

Each builtin file in `src/builtins/` has unit tests covering five shapes:

1. **Happy path** — normal inputs, correct output
2. **Empty / edge case** — empty args, zero, boundary values
3. **Error propagation** — error input produces error output
4. **Coercion** — boolean-to-number, text-to-number, empty-to-zero
5. **Type mismatch** — wrong input type produces appropriate `CellError`

Example:
```rust
#[test]
fn vlookup_exact_miss_returns_na() {
    let index = build_lookup_index(&[
        (LookupKey::text("EMEA"), 1),
        (LookupKey::text("APAC"), 2),
    ]);
    let result = vlookup_exact(&index, &LookupKey::text("NOT_THERE"));
    assert_eq!(result, Err(CellError::Na));
}
```

## Conformance tests

Full pipeline tests comparing xlstream output against LibreOffice cached values. One xlsx fixture per function or closely related group. **Only in `xlstream-eval`** — the other crates don't have a pipeline to test.

### Structure

```
crates/xlstream-eval/tests/
├── fixtures/                            <- LibreOffice-verified xlsx (one per function)
│   ├── operators/
│   │   ├── arithmetic.xlsx
│   │   ├── comparison.xlsx
│   │   └── concatenation.xlsx
│   ├── logical/
│   ├── math/
│   ├── text/
│   ├── date/
│   ├── lookup/
│   ├── aggregate/
│   ├── info/
│   ├── financial/
│   └── issues/                          <- per-issue regression fixtures
├── conformance/                         <- test modules
│   ├── mod.rs                           <- run_conformance() helper
│   ├── operators.rs
│   ├── logical.rs
│   ├── math.rs
│   ├── text.rs
│   ├── date.rs
│   ├── lookup.rs
│   ├── aggregate.rs
│   ├── info.rs
│   ├── financial.rs
│   └── issues.rs
└── conformance.rs                       <- test binary root
```

### How it works

`run_conformance()` in `conformance/mod.rs`:
1. Opens the LibreOffice-saved fixture (reads cached values as ground truth)
2. Identifies formula cells (skips data cells)
3. Evaluates through xlstream
4. Compares every formula cell against the cached value
5. Reports mismatches with cell address + expected + actual

Float comparison uses epsilon 1e-6. Volatile functions (TODAY, NOW) are not included in fixtures.

### Adding a new function

1. Create xlsx with openpyxl (data + formulas, no cached values):
   ```python
   import openpyxl
   wb = openpyxl.Workbook()
   ws = wb.active
   ws['A1'] = 'Value'
   ws['B1'] = 'Result'
   ws['A2'] = 10
   ws['B2'] = '=ROUND(A2, 1)'
   wb.save('/tmp/round.xlsx')
   ```

2. Recalculate with LibreOffice headless:
   ```sh
   /Applications/LibreOffice.app/Contents/MacOS/soffice \
     --headless --calc --convert-to xlsx \
     --outdir tests/fixtures/<category>/ /tmp/<function>.xlsx
   ```

3. Add `#[test]` in `conformance/<category>.rs`:
   ```rust
   #[test]
   fn round() {
       run_conformance("fixtures/math/round.xlsx");
   }
   ```

4. Commit fixture + test. No existing fixtures touched.

### Adding an issue regression

1. Create xlsx reproducing the issue
2. Recalculate with LibreOffice headless
3. Drop in `fixtures/issues/issue-NN-description.xlsx`
4. Add `#[test]` in `conformance/issues.rs`

### Minimum bar per builtin PR

- >= 1 unit test at the builtin function level
- >= 1 conformance fixture exercising the function
- Coverage of the five shapes (can be split across unit + conformance)

## Benchmarks

`criterion` benches live in `benchmarks/benches/`. Never in main crates.

| Code path | Benchmark file | Formulas using it |
|---|---|---|
| Parser | `parse.rs` | all formulas |
| Binary/unary ops | `arithmetic.rs` | +, -, *, /, ^, %, &, comparisons |
| String ops | `string.rs` | LEFT, RIGHT, MID, UPPER, CONCAT, TEXT |
| Date serial conversion | `date.rs` | YEAR, MONTH, EDATE, NETWORKDAYS |
| Pure math | `math.rs` | ROUND, MOD, SQRT, ABS, INT, POWER |
| Iterative solvers | `financial.rs` | IRR, RATE, PMT, NPV, FV |
| Short-circuit eval | `conditional.rs` | IF, IFS, SWITCH, IFERROR, AND, OR |
| Type checking | `info.rs` | ISBLANK, ISNUMBER, TYPE, ISERROR |
| Hash index probe | `lookup.rs` | VLOOKUP, XLOOKUP, INDEX/MATCH |
| Streaming fold | `aggregate.rs` | SUM, COUNT, AVERAGE, SUMIF, COUNTIF |
| Full pipeline | `pipeline.rs` | read+parse+classify+prelude+eval+write at 10k rows |

**Rule:** when adding a formula with a novel code path, add a criterion bench in the matching `benches/<category>.rs` file.

CI runs micro-benchmarks on every PR via `bench-gate` (fails on >20% regression).

## Memory profiling

| Test | Peak RSS |
|---|---|
| 10k x 20 | < 50 MB |
| 100k x 20 | < 150 MB |
| 400k x 20 | < 250 MB |

## Python tests

`bindings/python/tests/` via pytest.

- Test error type mapping (Rust errors -> Python exceptions)
- Test at least one evaluation path (small xlsx -> evaluate -> assert)
- Test GIL release (evaluate in thread pool, observe concurrency)
- Do NOT re-test what Rust tests cover. Test **the binding**, not the engine.

## Running everything

```bash
cargo test --all-features          # unit + conformance + doctests
cargo test --doc                   # doctests only
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
cargo bench                        # benchmarks (not CI-blocking)
cd bindings/python && maturin develop --release && pytest tests/
```

## CI gates

A PR merges only when:
- `cargo test --all-features` passes
- `cargo clippy -- -D warnings` is clean
- `cargo fmt --check` is clean
- `pytest` passes (if Python binding touched)
- Criterion bench-gate passes (no >20% regression)
