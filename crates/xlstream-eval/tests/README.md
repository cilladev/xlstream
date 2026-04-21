# xlstream-eval integration tests

## Structure

```
tests/
├── fixtures/
│   ├── .gitignore                        ← overrides root *.xlsx ignore
│   └── regression.xlsx                   ← Excel-verified golden file
├── end_to_end/
│   ├── helpers/mod.rs                    ← shared fixture generators
│   ├── pipeline.rs                       ← core pipeline (cell refs, chained formulas, errors)
│   ├── lookups.rs                        ← VLOOKUP, XLOOKUP, INDEX/MATCH
│   ├── cross_sheet_aggregates.rs         ← cross-sheet SUMIF/COUNTIF/AVERAGEIF
│   ├── grouped_sumif.rs                  ← SUMIF with row-local criteria
│   ├── product_literals.rs              ← PRODUCT(2,3,4) inline eval
│   └── named_ranges.rs                  ← named range resolution
├── end_to_end.rs                        ← test binary root (imports all above)
├── regression_base.rs                   ← golden-file comparison (all surfaces)
├── README.md                            ← this file
└── *.rs                                 ← per-category unit-level tests
```

## Two test layers

### Golden-file regression (`regression_base.rs`)

Broad coverage. One fixture with all supported formulas, Excel as oracle.

```sh
# Generate fixture (one-time)
cargo test -p xlstream-eval --test regression_base -- generate_fixture --ignored --nocapture
# Open in Excel, save, commit, then:
cargo test -p xlstream-eval --test regression_base
```

### End-to-end integration (`end_to_end.rs`)

Per-feature tests through the full pipeline: read → parse → classify → prelude → evaluate → write → verify.

```sh
# All end-to-end tests
cargo test -p xlstream-eval --test end_to_end

# Specific feature
cargo test -p xlstream-eval --test end_to_end -- named_ranges
cargo test -p xlstream-eval --test end_to_end -- lookups
cargo test -p xlstream-eval --test end_to_end -- pipeline
```

### Adding a new feature's tests

1. Create `end_to_end/<feature_name>.rs`
2. Add `#[path = "end_to_end/<feature_name>.rs"] mod <feature_name>;` to `end_to_end.rs`
3. Write tests using programmatic fixtures (no Excel dependency)

## Per-category unit tests

| File | Category |
|------|----------|
| `arithmetic.rs` | +, -, *, /, ^, % |
| `comparison.rs` | =, <>, <, >, <=, >= |
| `concat.rs` | & |
| `conditional.rs` | IF, IFS, SWITCH, IFERROR |
| `aggregates.rs` | SUM, SUMIF, COUNTIF |
| `string.rs` | LEFT, RIGHT, UPPER, TRIM |
| `math.rs` | ROUND, MOD, ABS, SQRT |
| `date.rs` | YEAR, MONTH, EDATE |
| `info_financial.rs` | ISBLANK, PMT, FV |
| `precedence.rs` | operator ordering |
| `unary.rs` | -, + (unary) |
| `parallel.rs` | multi-worker eval |
| `range_expansion.rs` | bounded range refs |
| `perf_smoke.rs` | 1k-row smoke test |
