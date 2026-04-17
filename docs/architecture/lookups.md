# Lookups

Lookups are O(1) amortised via hash indexes built once during prelude.

## Supported functions

Pure Excel only. No custom extensions.

| Function | Signature | Match mode |
|---|---|---|
| `VLOOKUP` | `(lookup, table, col_index, match_mode?)` | exact (`FALSE`/`0`), approx sorted (`TRUE`/`1`) |
| `HLOOKUP` | `(lookup, table, row_index, match_mode?)` | same |
| `XLOOKUP` | `(lookup, lookup_array, return_array, not_found?, match_mode?, search_mode?)` | exact, wildcard, approx |
| `MATCH` | `(lookup, lookup_array, match_type?)` | exact / asc-sorted / desc-sorted |
| `XMATCH` | `(lookup, lookup_array, match_mode?, search_mode?)` | exact, wildcard, approx |
| `INDEX` | `(array, row, col?)` | constant lookup — no index needed |
| `CHOOSE` | `(index, val1, val2, ...)` | argument-pick, no table |

### Multi-key lookups

Excel's canonical way to do multi-key lookup: pre-compute a helper column in the lookup sheet that concatenates the keys, then do a standard `VLOOKUP` / `XLOOKUP` against it.

```
# In 'Thresholds' sheet:
A (Region)  B (Business)  C (Threshold)  D (helper: =A&B)
EMEA        Rates         50000          EMEARates
APAC        FX            30000          APACFX
...

# In main sheet formula:
=VLOOKUP(Region & Business, 'Thresholds'!D:E, 2, FALSE)
```

This is 100% pure Excel. Our hash index keys on the single text column; no composite-key logic needed in the engine.

## Classification

A formula is `LookupOnly` if:
- It contains at least one supported lookup function call.
- The lookup range is a whole-column or whole-range reference into a different sheet.
- That sheet is a **lookup sheet** (classified in pass 0) — small, no formulas that depend on the main sheet's rows.
- All other refs in the formula are row-local.

Forward-pointing cross-sheet refs from a lookup sheet back to the main sheet's rows make the sheet not-a-lookup-sheet. Refuse.

## Prelude: building indexes

For each supported lookup call, during classification we record:
- The lookup sheet.
- The **key column** (single). For `VLOOKUP(K, Sheet!A:C, 2, FALSE)`, key is column A.
- The **value column(s)** being returned.
- The **match mode**.

We deduplicate across formulas: if two formulas both do `VLOOKUP(_, Lookup!A:B, 2, FALSE)`, one index serves both.

### Index structure

```rust
pub struct LookupIndex {
    /// Sheet this index belongs to.
    pub sheet: SheetId,

    /// Key column in the source sheet.
    pub key_col: u16,

    /// For exact match: hash from key -> first row index (1-based, matching Excel's).
    pub exact: HashMap<LookupKey, u32>,

    /// For approx match: sorted list of (key, row_index).
    pub sorted: Option<Vec<(LookupKey, u32)>>,

    /// All row values, for fast INDEX-style retrieval.
    pub rows: Vec<Vec<Value>>,
}
```

`LookupKey` is a type-aware wrapper over `Value` that hashes case-insensitively for text (matching Excel default).

### Build cost

A lookup sheet of N rows, K key columns: O(N × K) read time + O(N) hash inserts. For typical lookup sheets (25–10,000 rows), this is sub-millisecond.

### Memory

`rows` carries the full sheet data once. For a 10k-row × 10-col lookup sheet with average 20-char strings: ~2 MB. Negligible.

## Evaluation

### `VLOOKUP(key, Lookup!A:C, 2, FALSE)`

At row time:
1. Evaluate `key` from the current row.
2. Wrap as `LookupKey`.
3. `index.exact.get(&key)` → `row_index`.
4. Return `index.rows[row_index][col_offset]` where `col_offset = 2 - 1 = 1` (Excel 1-based column).

```rust
fn vlookup_stateful(
    args: &[AstNode],
    interp: &Interpreter,
    scope: &RowScope,
) -> Result<Value, XlStreamError> {
    let key = interp.eval(&args[0], scope)?;
    let col_idx = interp.eval(&args[2], scope)?.as_integer()? as usize;
    let match_mode = args.get(3)
        .map(|a| interp.eval(a, scope))
        .transpose()?
        .map(|v| v.as_bool().unwrap_or(true))
        .unwrap_or(true);

    let index = interp.prelude.lookup_index_for(&args[1])?;
    let key = LookupKey::from(&key);

    if match_mode {
        // Approximate match — binary search in sorted vec.
        let sorted = index.sorted.as_ref()
            .ok_or(CellError::Na)?;
        match sorted.binary_search_by(|(k, _)| k.cmp(&key)) {
            Ok(i) | Err(i) if i > 0 => {
                let row = sorted[i.saturating_sub(if matches!(..., Err(_)) { 1 } else { 0 })].1;
                Ok(index.rows[row as usize - 1][col_idx - 1].clone())
            },
            _ => Ok(Value::Error(CellError::Na)),
        }
    } else {
        // Exact match — hash probe.
        match index.exact.get(&key) {
            Some(row) => Ok(index.rows[*row as usize - 1][col_idx - 1].clone()),
            None => Ok(Value::Error(CellError::Na)),
        }
    }
}
```

### `XLOOKUP(key, lookup_array, return_array, not_found?, match_mode?, search_mode?)`

Generalisation of VLOOKUP. Takes separate arrays for key and return value — meaning the index can be built per-column-pair.

### `MATCH(key, array, match_type?)`

Returns the row index rather than a value. Single-column lookup.

### `XMATCH(key, array, match_mode?, search_mode?)`

Modern MATCH. Supports wildcard and binary-search modes via explicit parameters.

### `INDEX(array, row, col?)`

Returns a cell from a pre-loaded range at `(row, col)`. No index build — constant-time array access via pre-loaded lookup sheet data.

### `CHOOSE(index, val1, val2, ...)`

Argument-pick: returns `val[index]`. No lookup table; evaluates only the selected argument (short-circuit).

## Approximate match

Excel's approx VLOOKUP requires the lookup column to be **sorted ascending**. Binary search with "find largest value ≤ key" semantics:

```rust
let pos = sorted.partition_point(|(k, _)| k <= &key);
if pos == 0 {
    Ok(Value::Error(CellError::Na))
} else {
    let (_, row) = &sorted[pos - 1];
    Ok(index.rows[*row as usize - 1][col_idx - 1].clone())
}
```

If the user's data isn't sorted, Excel returns unpredictable results — and so do we, matching Excel's behaviour.

## Wildcard match (XLOOKUP match_mode=2)

Wildcard patterns (`*abc*`, `a?c`) cannot use a hash index. Fall back to linear scan over the sorted array, up to the first match. Cost: O(N) per wildcard lookup. Acceptable because lookup sheets are small.

Classification marks a formula as "wildcard" at parse time; if seen on a large lookup sheet (> 100k rows), emit a warning that perf will be suboptimal.

## Cross-sheet lookups

All our supported lookups reference sheets other than the main sheet. Same-sheet lookups (formulas in the main sheet looking up values in the main sheet) are not supported — they would require the sheet's data to be built during prelude, and for forward-row refs, that's impossible in a streaming model.

Exception: same-sheet lookups where the lookup range is strictly rows above the current row are theoretically possible but not planned for v0.1.

## Error semantics

- Key not found, exact match → `#N/A`.
- Key not found, approx match, below first key → `#N/A`.
- Lookup range isn't valid → `#REF!`.
- Column index out of range → `#REF!`.
- Key evaluates to `#VALUE!` → propagate `#VALUE!`.

All of these are **cell-level** errors — they write to the output cell, they do not abort the run.

## Tests for lookups

Every lookup function has:
- Exact-match hit.
- Exact-match miss → `#N/A`.
- Approx match hit (sorted).
- Approx match miss (below first key) → `#N/A`.
- Case-insensitive text match.
- Type mismatch (number key into text column) → `#N/A`.
- Error propagation from key argument.
- Multi-key via `VLOOKUP(A&B, ...)` against a lookup sheet with a helper column — end-to-end test.
- Large lookup sheet (10k rows) performance test — budget < 1 ms per 1000 lookups.
