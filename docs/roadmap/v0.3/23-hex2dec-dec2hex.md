# Feature: HEX2DEC / DEC2HEX

**Branch:** `feat/hex2dec-dec2hex`
**Effort:** ~2 hours
**Crates:** xlstream-eval

## What

Two-way hexadecimal conversion for Excel's Analysis ToolPak.

- `HEX2DEC(number)` — converts a hexadecimal string to a decimal number
- `DEC2HEX(number, [places])` — converts a decimal number to a hexadecimal string

```
=HEX2DEC("FF")          → 255
=HEX2DEC("A5")          → 165
=HEX2DEC("FFFFFFFFFF")  → -1          (10-digit two's complement)
=DEC2HEX(255)            → "FF"
=DEC2HEX(255, 4)         → "00FF"      (zero-padded to 4 places)
=DEC2HEX(-1)             → "FFFFFFFFFF" (10-digit two's complement)
```

**Excel's hex domain:** 10-digit hex strings representing 40-bit two's complement integers. Range: `-549755813888` to `549755813887` (`8000000000` to `7FFFFFFFFF`). HEX2DEC returns a number; DEC2HEX returns a text string.

Current behavior: no dispatch entry — returns `#VALUE!` from the fallback.

## What already exists

- `crates/xlstream-eval/src/builtins/engineering.rs` — empty module (module doc only, lines 1-5). First engineering functions to land here.
- `crates/xlstream-eval/src/builtins/mod.rs` — `mod engineering;` declared (line 12). No dispatch arms yet for engineering functions.
- `crates/xlstream-eval/src/builtins/mod.rs:30-36` — `eval_args` helper for pure eager-eval builtins
- `crates/xlstream-eval/src/builtins/math.rs:27-29` — `num_arg_ce` helper (extracts f64 from args)
- `crates/xlstream-eval/src/builtins/string.rs:19-25` — `text_arg` helper (extracts string from args)
- `xlstream_core::coerce::to_number` and `xlstream_core::coerce::to_text` — value coercion functions
- Not in `UNSUPPORTED_FUNCTIONS` or `RANGE_EXPANDING_FUNCTIONS`
- Not in dispatch
- `docs/functions.md` lists both as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/engineering.rs` — implementation home (currently empty)
- `crates/xlstream-eval/src/builtins/mod.rs:12` — `mod engineering;` declaration
- `crates/xlstream-eval/src/builtins/mod.rs:140-170` — string builtins dispatch pattern (pure, eager eval, returns `Value` directly)
- `crates/xlstream-eval/src/builtins/mod.rs:30-36` — `eval_args` helper
- `crates/xlstream-eval/src/builtins/math.rs:27-34` — `num_arg_ce`, `bool_arg_ce` helpers
- `crates/xlstream-eval/src/builtins/string.rs:19-25` — `text_arg` helper
- `crates/xlstream-eval/src/builtins/string.rs:59-73` — `builtin_left` as a pattern for `(args: &[Value]) -> Value` builtins
- `crates/xlstream-eval/tests/conformance/mod.rs` — conformance test runner

## Resolution / Evaluation behavior

Both functions are pure scalar functions — row-local, no streaming concerns, no prelude dependency.

**Classification:** RowLocal (default — not in any special set).

**Prelude:** Nothing needed.

**Row eval:** Eager-eval dispatch. Both args evaluated via `eval_args`, passed as `&[Value]` to the builtin function. Same pattern as math and string builtins.

**HEX2DEC(number):**
1. Extract arg 0 as text (coerce number to text if needed — Excel treats `HEX2DEC(1A)` same as `HEX2DEC("1A")`)
2. Validate: 1-10 hex characters `[0-9A-Fa-f]`. Longer or invalid chars → `#NUM!`
3. Parse as 40-bit two's complement: if 10 digits and first digit >= 8, it's negative
4. Return `Value::Number(result as f64)`

**DEC2HEX(number, [places]):**
1. Extract arg 0 as number via `coerce::to_number`. Truncate to integer.
2. Validate range: `-549755813888` to `549755813887`. Out of range → `#NUM!`
3. If negative, compute 40-bit two's complement (add 2^40)
4. Format as uppercase hex string
5. If `places` provided: must be integer 1-10. If result needs more digits than `places` → `#NUM!`. Otherwise zero-pad.
6. Return `Value::Text(result.into())`

**Error conditions:**
- Wrong arity: `#VALUE!` (HEX2DEC requires exactly 1, DEC2HEX requires 1-2)
- Non-hex characters in HEX2DEC input: `#NUM!`
- Hex string > 10 characters: `#NUM!`
- Empty string for HEX2DEC: `#NUM!`
- DEC2HEX number out of [-549755813888, 549755813887]: `#NUM!`
- DEC2HEX places < 1 or > 10 or non-integer: `#NUM!`
- DEC2HEX result longer than places: `#NUM!`
- Error in any arg: propagate

## Tests

### Unit tests (in `engineering.rs`)

**HEX2DEC happy path:**
- `hex2dec("FF")` → 255.0
- `hex2dec("A5")` → 165.0
- `hex2dec("0")` → 0.0
- `hex2dec("1")` → 1.0
- `hex2dec("7FFFFFFFFF")` → 549755813887.0 (max positive)

**HEX2DEC negative (two's complement):**
- `hex2dec("FFFFFFFFFF")` → -1.0
- `hex2dec("8000000000")` → -549755813888.0 (min negative)
- `hex2dec("FFFFFFFF00")` → -256.0

**HEX2DEC edge cases:**
- `hex2dec("ff")` → 255.0 (case insensitive)
- `hex2dec("0000000001")` → 1.0 (leading zeros, 10 digits)
- Numeric input coercion: `hex2dec(1A)` as `Value::Number(1.0)` → coerced to text "1" → 1.0

**HEX2DEC errors:**
- Empty string → `#NUM!`
- `hex2dec("G1")` → `#NUM!` (invalid hex char)
- `hex2dec("1FFFFFFFFFF")` → `#NUM!` (11 digits, too long)
- `hex2dec(#N/A)` → `#N/A` (error propagation)
- Wrong arity (0 args, 2 args) → `#VALUE!`

**DEC2HEX happy path:**
- `dec2hex(255)` → "FF"
- `dec2hex(0)` → "0"
- `dec2hex(165)` → "A5"
- `dec2hex(549755813887)` → "7FFFFFFFFF" (max positive)

**DEC2HEX with places:**
- `dec2hex(255, 4)` → "00FF"
- `dec2hex(255, 2)` → "FF" (exact fit)
- `dec2hex(0, 4)` → "0000"
- `dec2hex(1, 10)` → "0000000001"

**DEC2HEX negative (two's complement):**
- `dec2hex(-1)` → "FFFFFFFFFF"
- `dec2hex(-549755813888)` → "8000000000" (min negative)
- `dec2hex(-256)` → "FFFFFFFF00"

**DEC2HEX errors:**
- `dec2hex(549755813888)` → `#NUM!` (out of range, too large)
- `dec2hex(-549755813889)` → `#NUM!` (out of range, too small)
- `dec2hex(255, 1)` → `#NUM!` (result "FF" needs 2 places, only 1 given)
- `dec2hex(255, 0)` → `#NUM!` (places < 1)
- `dec2hex(255, 11)` → `#NUM!` (places > 10)
- `dec2hex(-1, 2)` → `#NUM!` (negative always outputs 10 digits, 2 < 10)
- `dec2hex(#N/A)` → `#N/A` (error propagation)
- Wrong arity (0 args, 3 args) → `#VALUE!`

**DEC2HEX type coercion:**
- `dec2hex(255.7)` → "FF" (truncated to 255)
- `dec2hex("255")` → "FF" (text coerced to number)
- `dec2hex(TRUE)` → "1" (bool coerced to 1)

### Conformance fixture

Create `tests/fixtures/engineering/hex2dec_dec2hex.xlsx`.

**Sheet1 data:**
- A: "Hex" header, rows 2-8: `FF, A5, 0, 7FFFFFFFFF, FFFFFFFFFF, 8000000000, ff`
- B: "Dec" header, rows 2-8: `255, 165, 0, 549755813887, -1, -549755813888, -256`
- C: "Places" header, rows 2-5: `4, 2, 10, 1`
- D: "Error" header, row 2: `=NA()`
- E: "Mixed" header, rows 2-4: `"1A"`, `TRUE`, `""` (empty string)

**Sheet2 data:**
- A: "XHex" header, rows 2-4: `1F, 2A, FF`

**Formulas (column F, starting row 2) — 25+ formulas:**

HEX2DEC happy path (5):
1. `=HEX2DEC(A2)` → 255
2. `=HEX2DEC(A3)` → 165
3. `=HEX2DEC(A4)` → 0
4. `=HEX2DEC(A5)` → 549755813887
5. `=HEX2DEC(A8)` → 255 (lowercase "ff")

HEX2DEC negative / two's complement (2):
6. `=HEX2DEC(A6)` → -1
7. `=HEX2DEC(A7)` → -549755813888

DEC2HEX happy path (4):
8. `=DEC2HEX(B2)` → "FF"
9. `=DEC2HEX(B3)` → "A5"
10. `=DEC2HEX(B4)` → "0"
11. `=DEC2HEX(B5)` → "7FFFFFFFFF"

DEC2HEX with places (3):
12. `=DEC2HEX(B2, C2)` → "00FF" (places=4)
13. `=DEC2HEX(B2, C3)` → "FF" (places=2, exact fit)
14. `=DEC2HEX(B4, C4)` → "0000000000" (0 padded to 10)

DEC2HEX negative (2):
15. `=DEC2HEX(B6)` → "FFFFFFFFFF"
16. `=DEC2HEX(B7)` → "8000000000"

Boundary / error (4):
17. `=DEC2HEX(B2, C5)` → `#NUM!` (255 needs 2 hex digits, places=1)
18. `=HEX2DEC("G1")` → `#NUM!` (invalid hex)
19. `=DEC2HEX(549755813888)` → `#NUM!` (out of range)
20. `=DEC2HEX(-549755813889)` → `#NUM!` (out of range)

Type coercion (3):
21. `=DEC2HEX(255.7)` → "FF" (truncated)
22. `=DEC2HEX(TRUE)` → "1" (bool → 1)
23. `=HEX2DEC("0000000001")` → 1 (leading zeros)

Error propagation (2):
24. `=HEX2DEC(D2)` → `#N/A`
25. `=DEC2HEX(D2)` → `#N/A`

Nested (2):
26. `=IF(HEX2DEC("FF")>200, "big", "small")` → "big"
27. `=IFERROR(HEX2DEC("ZZ"), "bad hex")` → "bad hex"

Cross-sheet (1):
28. `=HEX2DEC(Sheet2!A2)` → 31

Combined (1):
29. `=DEC2HEX(HEX2DEC("FF"))` → "FF" (round-trip)

HEX2DEC and DEC2HEX do NOT need `_xlfn.` prefix.

**Fixture workflow:**
1. Generate with openpyxl
2. Recalculate with LibreOffice headless
3. Add `#[test] fn hex2dec_dec2hex()` in `conformance/engineering.rs` (new file — first engineering conformance test)

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "HEX2DEC, DEC2HEX hexadecimal conversion" under `[Unreleased]` → `### Added` |
| `docs/functions.md` | Change HEX2DEC and DEC2HEX from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the HEX2DEC / DEC2HEX checkbox |

## Streaming invariant

Does not violate. Both functions are pure scalar functions of their arguments — no cross-row reads, no prelude dependency.
