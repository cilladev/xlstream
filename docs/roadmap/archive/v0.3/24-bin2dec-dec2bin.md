# Feature: BIN2DEC / DEC2BIN

**Branch:** `feat/bin2dec-dec2bin`
**Effort:** ~2 hours
**Crates:** xlstream-eval

## What

Two-way binary conversion for Excel's Analysis ToolPak.

- `BIN2DEC(number)` — converts a binary string to a decimal number
- `DEC2BIN(number, [places])` — converts a decimal number to a binary string

```
=BIN2DEC("1100100")      → 100
=BIN2DEC("1111111111")   → -1           (10-digit two's complement)
=DEC2BIN(100)             → "1100100"
=DEC2BIN(100, 10)         → "0001100100" (zero-padded to 10 places)
=DEC2BIN(-1)              → "1111111111" (10-digit two's complement)
```

**Excel's binary domain:** 10-digit binary strings representing 10-bit two's complement integers. Range: `-512` to `511` (`1000000000` to `0111111111`). BIN2DEC returns a number; DEC2BIN returns a text string.

Current behavior: no dispatch entry — returns `#VALUE!` from the fallback.

## What already exists

- `crates/xlstream-eval/src/builtins/engineering.rs` — module home. By the time this spec is implemented, spec 23 (HEX2DEC/DEC2HEX) will have landed here, establishing the engineering dispatch pattern and shared base-conversion helpers.
- `crates/xlstream-eval/src/builtins/mod.rs` — `mod engineering;` declared (line 12). HEX2DEC/DEC2HEX dispatch arms will already exist.
- `crates/xlstream-eval/src/builtins/mod.rs:30-36` — `eval_args` helper for pure eager-eval builtins
- `crates/xlstream-eval/src/builtins/math.rs:27-29` — `num_arg_ce` helper (extracts f64 from args)
- `crates/xlstream-eval/src/builtins/string.rs:19-25` — `text_arg` helper (extracts string from args)
- `xlstream_core::coerce::to_number` and `xlstream_core::coerce::to_text` — value coercion functions
- Shared helpers from spec 23: two's complement conversion logic, `places` validation, text-or-number coercion for the input arg
- Not in `UNSUPPORTED_FUNCTIONS` or `RANGE_EXPANDING_FUNCTIONS`
- `docs/functions.md` lists both as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/engineering.rs` — implementation home (HEX2DEC/DEC2HEX already present from spec 23)
- `crates/xlstream-eval/src/builtins/mod.rs:12` — `mod engineering;` declaration
- `crates/xlstream-eval/src/builtins/mod.rs:140-170` — string builtins dispatch pattern (pure, eager eval, returns `Value` directly)
- `crates/xlstream-eval/src/builtins/mod.rs:30-36` — `eval_args` helper
- `crates/xlstream-eval/src/builtins/math.rs:27-34` — `num_arg_ce`, `bool_arg_ce` helpers
- `crates/xlstream-eval/src/builtins/string.rs:19-25` — `text_arg` helper
- `crates/xlstream-eval/tests/conformance/mod.rs` — conformance test runner

## Resolution / Evaluation behavior

Both functions are pure scalar functions — row-local, no streaming concerns, no prelude dependency.

**Classification:** RowLocal (default — not in any special set).

**Prelude:** Nothing needed.

**Row eval:** Eager-eval dispatch. Both args evaluated via `eval_args`, passed as `&[Value]` to the builtin function. Same pattern as HEX2DEC/DEC2HEX.

**BIN2DEC(number):**
1. Extract arg 0 as text (coerce number to text if needed — Excel treats `BIN2DEC(101)` same as `BIN2DEC("101")`)
2. Validate: 1-10 binary characters `[01]`. Longer or invalid chars → `#NUM!`
3. Parse as 10-bit two's complement: if 10 digits and first digit is 1, it's negative
4. Return `Value::Number(result as f64)`

**DEC2BIN(number, [places]):**
1. Extract arg 0 as number via `coerce::to_number`. Truncate to integer.
2. Validate range: `-512` to `511`. Out of range → `#NUM!`
3. If negative, compute 10-bit two's complement (add 2^10 = 1024)
4. Format as binary string
5. If `places` provided: must be integer 1-10. If result needs more digits than `places` → `#NUM!`. Otherwise zero-pad.
6. Return `Value::Text(result.into())`

**Error conditions:**
- Wrong arity: `#VALUE!` (BIN2DEC requires exactly 1, DEC2BIN requires 1-2)
- Non-binary characters in BIN2DEC input: `#NUM!`
- Binary string > 10 characters: `#NUM!`
- Empty string for BIN2DEC: `#NUM!`
- DEC2BIN number out of [-512, 511]: `#NUM!`
- DEC2BIN places < 1 or > 10 or non-integer: `#NUM!`
- DEC2BIN result longer than places: `#NUM!`
- Error in any arg: propagate

## Tests

### Unit tests (in `engineering.rs`)

**BIN2DEC happy path:**
- `bin2dec("1100100")` → 100.0
- `bin2dec("1010")` → 10.0
- `bin2dec("0")` → 0.0
- `bin2dec("1")` → 1.0
- `bin2dec("0111111111")` → 511.0 (max positive)
- `bin2dec("11")` → 3.0

**BIN2DEC negative (two's complement):**
- `bin2dec("1111111111")` → -1.0
- `bin2dec("1000000000")` → -512.0 (min negative)
- `bin2dec("1111111110")` → -2.0
- `bin2dec("1111111100")` → -4.0
- `bin2dec("1110000000")` → -128.0

**BIN2DEC edge cases:**
- `bin2dec("0000000001")` → 1.0 (leading zeros, 10 digits)
- `bin2dec("0000000000")` → 0.0 (all zeros, 10 digits)
- Numeric input coercion: `bin2dec(101)` as `Value::Number(101.0)` → coerced to text "101" → 5.0

**BIN2DEC errors:**
- Empty string → `#NUM!`
- `bin2dec("2")` → `#NUM!` (invalid binary char)
- `bin2dec("10201")` → `#NUM!` (invalid binary char)
- `bin2dec("1A")` → `#NUM!` (hex chars not valid)
- `bin2dec("10000000001")` → `#NUM!` (11 digits, too long)
- `bin2dec(#N/A)` → `#N/A` (error propagation)
- Wrong arity (0 args, 2 args) → `#VALUE!`

**DEC2BIN happy path:**
- `dec2bin(100)` → "1100100"
- `dec2bin(0)` → "0"
- `dec2bin(10)` → "1010"
- `dec2bin(511)` → "111111111" (max positive)
- `dec2bin(1)` → "1"

**DEC2BIN with places:**
- `dec2bin(100, 10)` → "0001100100"
- `dec2bin(100, 7)` → "1100100" (exact fit)
- `dec2bin(0, 4)` → "0000"
- `dec2bin(1, 10)` → "0000000001"

**DEC2BIN negative (two's complement):**
- `dec2bin(-1)` → "1111111111"
- `dec2bin(-512)` → "1000000000" (min negative)
- `dec2bin(-2)` → "1111111110"
- `dec2bin(-128)` → "1110000000"

**DEC2BIN errors:**
- `dec2bin(512)` → `#NUM!` (out of range, too large)
- `dec2bin(-513)` → `#NUM!` (out of range, too small)
- `dec2bin(100, 6)` → `#NUM!` (result "1100100" needs 7 places, only 6 given)
- `dec2bin(100, 0)` → `#NUM!` (places < 1)
- `dec2bin(100, 11)` → `#NUM!` (places > 10)
- `dec2bin(-1, 2)` → `#NUM!` (negative always outputs 10 digits, 2 < 10)
- `dec2bin(#N/A)` → `#N/A` (error propagation)
- Wrong arity (0 args, 3 args) → `#VALUE!`

**DEC2BIN type coercion:**
- `dec2bin(100.7)` → "1100100" (truncated to 100)
- `dec2bin("100")` → "1100100" (text coerced to number)
- `dec2bin(TRUE)` → "1" (bool coerced to 1)

### Conformance fixture

Create `tests/fixtures/engineering/bin2dec_dec2bin.xlsx`.

**Sheet1 data:**
- A: "Bin" header, rows 2-8: `1100100, 1010, 0, 0111111111, 1111111111, 1000000000, 11`
- B: "Dec" header, rows 2-8: `100, 10, 0, 511, -1, -512, -128`
- C: "Places" header, rows 2-5: `10, 7, 4, 6`
- D: "Error" header, row 2: `=NA()`
- E: "Mixed" header, rows 2-4: `"101"`, `TRUE`, `""` (empty string)

**Sheet2 data:**
- A: "XBin" header, rows 2-4: `1101, 10, 111`

**Formulas (column F, starting row 2) — 29 formulas:**

BIN2DEC happy path (5):
1. `=BIN2DEC(A2)` → 100
2. `=BIN2DEC(A3)` → 10
3. `=BIN2DEC(A4)` → 0
4. `=BIN2DEC(A5)` → 511
5. `=BIN2DEC(A8)` → 3 ("11")

BIN2DEC negative / two's complement (2):
6. `=BIN2DEC(A6)` → -1
7. `=BIN2DEC(A7)` → -512

BIN2DEC leading zeros (1):
8. `=BIN2DEC("0000000001")` → 1

DEC2BIN happy path (4):
9. `=DEC2BIN(B2)` → "1100100"
10. `=DEC2BIN(B3)` → "1010"
11. `=DEC2BIN(B4)` → "0"
12. `=DEC2BIN(B5)` → "111111111"

DEC2BIN with places (3):
13. `=DEC2BIN(B2, C2)` → "0001100100" (places=10)
14. `=DEC2BIN(B2, C3)` → "1100100" (places=7, exact fit)
15. `=DEC2BIN(B4, C4)` → "0000" (0 padded to 4)

DEC2BIN negative (2):
16. `=DEC2BIN(B6)` → "1111111111"
17. `=DEC2BIN(B7)` → "1000000000"

Boundary / error (5):
18. `=DEC2BIN(B2, C5)` → `#NUM!` (100 needs 7 binary digits, places=6)
19. `=BIN2DEC("2")` → `#NUM!` (invalid binary char)
20. `=BIN2DEC("10000000001")` → `#NUM!` (11 digits, too long)
21. `=DEC2BIN(512)` → `#NUM!` (out of range)
22. `=DEC2BIN(-513)` → `#NUM!` (out of range)

Type coercion (3):
23. `=DEC2BIN(100.7)` → "1100100" (truncated)
24. `=DEC2BIN(TRUE)` → "1" (bool → 1)
25. `=BIN2DEC("0000000000")` → 0 (all-zero 10-digit)

Error propagation (2):
26. `=BIN2DEC(D2)` → `#N/A`
27. `=DEC2BIN(D2)` → `#N/A`

Nested (1):
28. `=IFERROR(BIN2DEC("222"), "bad bin")` → "bad bin"

Round-trip (1):
29. `=DEC2BIN(BIN2DEC("1100100"))` → "1100100"

Cross-sheet (1):
30. `=BIN2DEC(Sheet2!A2)` → 13

BIN2DEC and DEC2BIN do NOT need `_xlfn.` prefix.

**Fixture workflow:**
1. Generate with openpyxl
2. Recalculate with LibreOffice headless
3. Add `#[test] fn bin2dec_dec2bin()` in `conformance/engineering.rs` (created by spec 23)

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "BIN2DEC, DEC2BIN binary conversion" under `[Unreleased]` → `### Added` |
| `docs/functions.md` | Change BIN2DEC and DEC2BIN from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the BIN2DEC / DEC2BIN checkbox |

## Streaming invariant

Does not violate. Both functions are pure scalar functions of their arguments — no cross-row reads, no prelude dependency.
