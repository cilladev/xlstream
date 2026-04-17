# Phase 8 — Lookups

**Goal:** `VLOOKUP`, `HLOOKUP`, `XLOOKUP`, `MATCH`, `XMATCH`, `INDEX`, `CHOOSE` — with hash-indexed exact match, binary-search approx match, wildcard fallback. Pure Excel only.

**Estimated effort:** ~1 week.

**Prerequisites:** Phases 3 + 4 + 5.

**Reading:** [`docs/architecture/lookups.md`](../architecture/lookups.md).

**Output:** lookups run in O(1) amortised per cell. 400k rows × 4 VLOOKUP evaluates in seconds, not minutes.

## Checklist

### Classification

- [ ] Recognise each lookup function; record (lookup_sheet, key_col, value_col, match_mode) in the prelude plan.
- [ ] Mark lookup sheets in pass 0 — sheets (any size, as long as they fit in memory) without formulas, which main-sheet formulas reference.
- [ ] Multi-key lookups: user handles via concatenation at call site (`VLOOKUP(A&B, ...)`) against a helper column on the lookup sheet. Engine just sees a single text key. No composite-key logic in the classifier.

### `LookupKey`

In `xlstream-eval/src/lookup/key.rs`:

- [ ] `enum LookupKey { Number(f64), Text(Box<str>), Bool(bool) }`.
- [ ] Hash: type-aware (1 and "1" differ); text case-folded for Eq+Hash.
- [ ] `impl Ord` for binary-search needs.

### `LookupIndex` builder

- [ ] `build_index(reader, sheet, key_col, values_cols) -> LookupIndex`:
  - [ ] Stream the sheet.
  - [ ] For each row, take the cell at `key_col` as the key.
  - [ ] Insert into `exact: HashMap<LookupKey, u32>` (first-match policy matches Excel).
  - [ ] Store row values.
- [ ] Sorted variant for approx match: collect into `Vec<(LookupKey, u32)>`, sort once.

### Lookup builtins

In `xlstream-eval/src/builtins/lookup.rs` — all **stateful** (need prelude + AST args):

- [ ] `VLOOKUP(key, range, col, match?)`:
  - [ ] Exact: hash lookup.
  - [ ] Approx: binary search (largest key ≤ lookup).
- [ ] `HLOOKUP` — same but row-oriented.
- [ ] `XLOOKUP(key, lookup_arr, return_arr, not_found?, match_mode?, search_mode?)`:
  - [ ] Exact, wildcard, approx — modes respected.
- [ ] `MATCH(key, arr, match_type?)` — returns index (1-based).
- [ ] `XMATCH` — XLOOKUP-flavoured MATCH.
- [ ] `INDEX(array, row, col?)` — constant lookup into a pre-loaded range, no index build needed.
- [ ] `CHOOSE(index, val1, val2, ...)` — returns `val[index]`. No table; just arg pick.

### Error cases

- [ ] Key not found (exact) → `#N/A`.
- [ ] Column index out of range → `#REF!`.
- [ ] Invalid lookup range → `#REF!`.
- [ ] Key arg is `#VALUE!` → propagate.
- [ ] Approx match, lookup below first key → `#N/A`.

### Tests

For VLOOKUP (template for others):
- [ ] Exact hit.
- [ ] Exact miss → `#N/A`.
- [ ] Approx (sorted) hit.
- [ ] Approx miss below first key → `#N/A`.
- [ ] Case-insensitive text match.
- [ ] Numeric type match: `1` vs `"1"` — not equal (Excel semantics).
- [ ] Error in key → propagate.
- [ ] Wildcard exact match (`"ap*"`) via XLOOKUP/XMATCH — linear scan, verify correctness and log warning on large tables.

For multi-key via `&` + helper column:
- [ ] Lookup sheet has a helper column `=A&B`; main-sheet formula does `VLOOKUP(LeftCol & RightCol, helper_range, N, FALSE)`. End-to-end pass.
- [ ] Same idiom with `XLOOKUP`.
- [ ] Key construction respects Excel `&` text-coercion semantics (numbers → trimmed text, bool → `"TRUE"`/`"FALSE"`, etc.).

### Perf smoke

- [ ] Exact lookup on a 10k-row lookup table: < 1 µs per hit (hash).
- [ ] 400k rows × 4 VLOOKUP: total eval < 30 s single-threaded.

## Integration tests

- [ ] Fixture: `VLOOKUP(Region, 'Region Info'!A:C, 2, FALSE)` with a 5-row lookup sheet. Assert all 400k rows.
- [ ] Fixture: `IF(Deal Value > VLOOKUP(Region & Business, 'Thresholds'!D:E, 2, FALSE), "YES", "NO")` where `Thresholds!D` is a pre-computed `=A&B` helper column. Combines conditional + concat-key lookup.

## Done when

All listed functions work. Hash-exact lookups hit the perf target. The integration fixtures produce values matching Excel-computed golden.
