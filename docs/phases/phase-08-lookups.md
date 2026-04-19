# Phase 8 — Lookups

**Goal:** `VLOOKUP`, `HLOOKUP`, `XLOOKUP`, `MATCH`, `XMATCH`, `INDEX`, `CHOOSE` — with hash-indexed exact match, binary-search approx match, wildcard fallback. Pure Excel only.

**Estimated effort:** ~1 week.

**Prerequisites:** Phases 3 + 4 + 5.

**Reading:** [`docs/architecture/lookups.md`](../architecture/lookups.md).

**Output:** lookups run in O(1) amortised per cell. 400k rows × 4 VLOOKUP evaluates in seconds, not minutes.

## Checklist

### Classification

- [x] Recognise each lookup function; record (lookup_sheet, key_col, value_col, match_mode) in the prelude plan.
- [x] Mark lookup sheets in pass 0 — sheets (any size, as long as they fit in memory) without formulas, which main-sheet formulas reference.
- [x] Multi-key lookups: user handles via concatenation at call site (`VLOOKUP(A&B, ...)`) against a helper column on the lookup sheet. Engine just sees a single text key. No composite-key logic in the classifier.

### `LookupKey`

In `xlstream-eval/src/lookup/value.rs` (named `LookupValue` to avoid collision with `xlstream_parse::LookupKey`):

- [x] `enum LookupValue { Number(OrderedF64), Text(CaseFoldedText), Bool(bool) }`.
- [x] Hash: type-aware (1 and "1" differ); text case-folded for Eq+Hash.
- [x] `impl Ord` for binary-search needs.

### `LookupIndex` builder

- [x] `LookupSheet::new(rows)` + `build_col_index(col)` / `build_row_index(row)`:
  - [x] Stream the sheet.
  - [x] For each row, take the cell at `key_col` as the key.
  - [x] Insert into `exact: HashMap<LookupValue, usize>` (first-match policy matches Excel).
  - [x] Store row values.
- [x] Sorted variant for approx match: `build_col_sorted(col)` collects into `Vec<(LookupValue, usize)>`, sorts once.

### Lookup builtins

In `xlstream-eval/src/builtins/lookup.rs` — all **stateful** (need prelude + AST args):

- [x] `VLOOKUP(key, range, col, match?)`:
  - [x] Exact: hash lookup.
  - [x] Approx: binary search (largest key <= lookup).
- [x] `HLOOKUP` — same but row-oriented (exact + approx).
- [x] `XLOOKUP(key, lookup_arr, return_arr, not_found?, match_mode?, search_mode?)`:
  - [x] Exact match with optional fallback.
  - [ ] Wildcard, approx — deferred to v0.2.
- [x] `MATCH(key, arr, match_type?)` — returns index (1-based).
- [x] `XMATCH` — XLOOKUP-flavoured MATCH (exact only).
- [x] `INDEX(array, row, col?)` — constant lookup into a pre-loaded range, no index build needed.
- [x] `CHOOSE(index, val1, val2, ...)` — returns `val[index]`. No table; just arg pick.

### Error cases

- [x] Key not found (exact) → `#N/A`.
- [x] Column index out of range → `#REF!`.
- [x] Invalid lookup range → `#REF!`.
- [x] Key arg is `#VALUE!` → propagate.
- [x] Approx match, lookup below first key → `#N/A`.

### Tests

For VLOOKUP (template for others):
- [x] Exact hit.
- [x] Exact miss → `#N/A`.
- [x] Approx (sorted) hit.
- [x] Approx miss below first key → `#N/A`.
- [x] Case-insensitive text match.
- [x] Numeric type match: `1` vs `"1"` — not equal (Excel semantics).
- [x] Error in key → propagate.
- [ ] Wildcard exact match (`"ap*"`) via XLOOKUP/XMATCH — linear scan, verify correctness and log warning on large tables.

For multi-key via `&` + helper column:
- [x] Lookup sheet has a helper column `=A&B`; main-sheet formula does `VLOOKUP(LeftCol & RightCol, helper_range, N, FALSE)`. End-to-end pass.
- [ ] Same idiom with `XLOOKUP`.
- [x] Key construction respects Excel `&` text-coercion semantics (numbers → trimmed text, bool → `"TRUE"`/`"FALSE"`, etc.).

### Perf smoke

- [x] Exact lookup on a 10k-row lookup table (#[ignore] smoke test).
- [ ] 400k rows × 4 VLOOKUP: total eval < 30 s single-threaded.

## Integration tests

- [x] Fixture: `VLOOKUP(Region, 'Region Info'!A:C, 2, FALSE)` with a 3-row lookup sheet. Assert 5 rows including miss and case-insensitive.
- [x] Fixture: `IF(VLOOKUP(...) > 150, "High", "Low")` — conditional + lookup combo.

## Done when

All listed functions work. Hash-exact lookups hit the perf target. The integration fixtures produce values matching Excel-computed golden.
