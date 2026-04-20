# xlstream-eval integration tests

## Structure

```
tests/
├── fixtures/
│   ├── .gitignore                        ← overrides root *.xlsx ignore
│   └── regression.xlsx                   ← Excel-verified golden file (see below)
├── helpers/
│   └── mod.rs                            ← shared fixture generators
├── regressions/
│   ├── mod.rs                            ← module root (declares issue files)
│   ├── issue_grouped_sumif.rs            ← SUMIF with row-local criteria
│   ├── issue_cross_sheet_aggregates.rs   ← cross-sheet SUMIF/COUNTIF/AVERAGEIF
│   └── issue_product_literals.rs         ← PRODUCT(2,3,4) returns #VALUE!
├── regression.rs                         ← golden-file comparison (all 117 surfaces)
├── regression_base.rs                    ← imports regressions/ module
├── README.md                             ← this file
└── *.rs                                  ← per-category integration tests
```

## Two regression approaches

### 1. Golden-file regression (`regression.rs`)

Broad coverage across all supported functions. Excel is the oracle.

**Workflow:**

1. Generate the fixture (one-time, or after adding formulas):
   ```sh
   cargo test -p xlstream-eval --test regression -- generate_fixture --ignored --nocapture
   ```
2. Open `tests/fixtures/regression.xlsx` in Excel.
3. Let formulas compute, then save. Excel populates cached values.
4. Commit the file.
5. Run:
   ```sh
   cargo test -p xlstream-eval --test regression
   ```

The test evaluates the fixture through xlstream and compares every cell
against Excel's cached values. Any mismatch fails with the cell address,
expected value, and actual value.

**What it covers:** all 117 implemented surfaces (115 functions + 13
operators minus 2 unsupported). Volatile functions (TODAY, NOW) are
skipped in comparison. IRR is omitted (needs range args; tested in unit
tests).

**When to regenerate:** when adding new formula columns to the spec.
Run the generator, re-save in Excel, commit.

### 2. Base regression tests (`regression_base.rs` + `regressions/`)

Per-bug regression tests. Each issue gets its own file under `regressions/`.
No Excel dependency — fixtures are built programmatically.

**Workflow:**

1. Bug is reported.
2. Create `regressions/issue_<description>.rs` with `#[ignore = "..."]` tests.
3. Add `mod issue_<description>;` to `regressions/mod.rs`.
4. Implement the fix.
5. Un-ignore the tests.
6. Commit test + fix together.

**Current issue files (all tests ignored, pending fixes):**

| File | Issue | Tests |
|------|-------|-------|
| `issue_grouped_sumif.rs` | SUMIF/COUNTIF with row-local criteria | 3 |
| `issue_cross_sheet_aggregates.rs` | Cross-sheet conditional aggregates return #VALUE! | 3 |
| `issue_product_literals.rs` | PRODUCT with literal args returns #VALUE! | 2 |

## Existing per-category tests

| File | Category | Coverage |
|------|----------|----------|
| `arithmetic.rs` | +, -, *, /, ^, % | operators on numbers, booleans, errors |
| `comparison.rs` | =, <>, <, >, <=, >= | cross-type comparisons |
| `concat.rs` | & | string/number concatenation |
| `conditional.rs` | IF, IFS, SWITCH, IFERROR, etc. | short-circuit, error catching |
| `aggregates.rs` | SUM, SUMIF, COUNTIF, etc. | prelude aggregates, conditional |
| `lookup_end_to_end.rs` | VLOOKUP, XLOOKUP, INDEX/MATCH | exact match, concat key, nested |
| `string.rs` | LEFT, RIGHT, UPPER, TRIM, etc. | text manipulation |
| `math.rs` | ROUND, MOD, ABS, SQRT, etc. | math builtins |
| `date.rs` | YEAR, MONTH, EDATE, etc. | date functions |
| `info_financial.rs` | ISBLANK, PMT, FV, etc. | info + financial |
| `precedence.rs` | operator ordering | unary minus, power |
| `unary.rs` | -, + (unary) | negate, plus |
| `parallel.rs` | multi-worker eval | parallel correctness |
| `eval_end_to_end.rs` | full pipeline | read-eval-write round-trip |
| `range_expansion.rs` | bounded range refs | cross-column refs |
| `perf_smoke.rs` | performance | 1k-row smoke test |

## Running tests

```sh
# All eval tests (excludes ignored)
cargo test -p xlstream-eval

# Include slow tests
cargo test -p xlstream-eval -- --include-ignored

# Just the golden-file test
cargo test -p xlstream-eval --test regression

# Just the base regression tests (includes ignored)
cargo test -p xlstream-eval --test regression_base -- --include-ignored
```
