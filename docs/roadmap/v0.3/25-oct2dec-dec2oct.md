# Feature: OCT2DEC / DEC2OCT

**Branch:** `feat/oct2dec-dec2oct`
**Effort:** ~2 hours
**Crates:** xlstream-eval

## What

Two-way octal conversion for Excel's Analysis ToolPak.

- `OCT2DEC(number)` — converts an octal string to a decimal number
- `DEC2OCT(number, [places])` — converts a decimal number to an octal string

```
=OCT2DEC("144")           → 100
=OCT2DEC("52")            → 42
=OCT2DEC("7777777777")    → -1           (10-digit two's complement)
=DEC2OCT(100)              → "144"
=DEC2OCT(100, 6)           → "000144"    (zero-padded to 6 places)
=DEC2OCT(-1)               → "7777777777" (10-digit two's complement)
```

**Excel's octal domain:** 10-digit octal strings representing 30-bit two's complement integers. Range: `-536870912` to `536870911` (`4000000000` to `3777777777`). OCT2DEC returns a number; DEC2OCT returns a text string.

Current behavior: no dispatch entry — returns `#VALUE!` from the fallback.

## What already exists

- `crates/xlstream-eval/src/builtins/engineering.rs` — module home. By the time this spec is implemented, specs 23-24 (HEX2DEC/DEC2HEX, BIN2DEC/DEC2BIN) will have landed here, establishing shared base-conversion helpers for two's complement, places validation, and input coercion.
- `crates/xlstream-eval/src/builtins/mod.rs` — `mod engineering;` declared (line 12). HEX2DEC/DEC2HEX and BIN2DEC/DEC2BIN dispatch arms will already exist.
- `crates/xlstream-eval/src/builtins/mod.rs:30-36` — `eval_args` helper for pure eager-eval builtins
- `crates/xlstream-eval/src/builtins/math.rs:27-29` — `num_arg_ce` helper (extracts f64 from args)
- `crates/xlstream-eval/src/builtins/string.rs:19-25` — `text_arg` helper (extracts string from args)
- `xlstream_core::coerce::to_number` and `xlstream_core::coerce::to_text` — value coercion functions
- Shared helpers from specs 23-24: two's complement conversion logic (parameterized by bit width), `places` validation, text-or-number coercion for the input arg
- Not in `UNSUPPORTED_FUNCTIONS` or `RANGE_EXPANDING_FUNCTIONS`
- `docs/functions.md` lists both as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/engineering.rs` — implementation home (HEX2DEC/DEC2HEX and BIN2DEC/DEC2BIN already present from specs 23-24)
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

**Row eval:** Eager-eval dispatch. Both args evaluated via `eval_args`, passed as `&[Value]` to the builtin function. Same pattern as HEX2DEC/DEC2HEX and BIN2DEC/DEC2BIN.

**OCT2DEC(number):**
1. Extract arg 0 as text (coerce number to text if needed — Excel treats `OCT2DEC(77)` same as `OCT2DEC("77")`)
2. Validate: 1-10 octal characters `[0-7]`. Longer or invalid chars → `#NUM!`
3. Parse as 30-bit two's complement: if 10 digits and first digit >= 4, it's negative
4. Return `Value::Number(result as f64)`

**DEC2OCT(number, [places]):**
1. Extract arg 0 as number via `coerce::to_number`. Truncate to integer.
2. Validate range: `-536870912` to `536870911`. Out of range → `#NUM!`
3. If negative, compute 30-bit two's complement (add 2^30 = 1073741824)
4. Format as octal string
5. If `places` provided: must be integer 1-10. If result needs more digits than `places` → `#NUM!`. Otherwise zero-pad.
6. Return `Value::Text(result.into())`

**Error conditions:**
- Wrong arity: `#VALUE!` (OCT2DEC requires exactly 1, DEC2OCT requires 1-2)
- Non-octal characters in OCT2DEC input (8, 9, A-F, etc.): `#NUM!`
- Octal string > 10 characters: `#NUM!`
- Empty string for OCT2DEC: `#NUM!`
- DEC2OCT number out of [-536870912, 536870911]: `#NUM!`
- DEC2OCT places < 1 or > 10 or non-integer: `#NUM!`
- DEC2OCT result longer than places: `#NUM!`
- Error in any arg: propagate

## Tests

### Unit tests (in `engineering.rs`)

**OCT2DEC happy path:**
- `oct2dec("144")` → 100.0
- `oct2dec("52")` → 42.0
- `oct2dec("0")` → 0.0
- `oct2dec("1")` → 1.0
- `oct2dec("7")` → 7.0
- `oct2dec("3777777777")` → 536870911.0 (max positive)
- `oct2dec("10")` → 8.0

**OCT2DEC negative (two's complement):**
- `oct2dec("7777777777")` → -1.0
- `oct2dec("4000000000")` → -536870912.0 (min negative)
- `oct2dec("7777777770")` → -8.0
- `oct2dec("7777777600")` → -128.0

**OCT2DEC edge cases:**
- `oct2dec("0000000001")` → 1.0 (leading zeros, 10 digits)
- `oct2dec("0000000000")` → 0.0 (all zeros, 10 digits)
- Numeric input coercion: `oct2dec(77)` as `Value::Number(77.0)` → coerced to text "77" → 63.0

**OCT2DEC errors:**
- Empty string → `#NUM!`
- `oct2dec("8")` → `#NUM!` (invalid octal char — 8 not allowed)
- `oct2dec("9")` → `#NUM!` (invalid octal char — 9 not allowed)
- `oct2dec("1A")` → `#NUM!` (hex chars not valid in octal)
- `oct2dec("10000000001")` → `#NUM!` (11 digits, too long)
- `oct2dec(#N/A)` → `#N/A` (error propagation)
- Wrong arity (0 args, 2 args) → `#VALUE!`

**DEC2OCT happy path:**
- `dec2oct(100)` → "144"
- `dec2oct(0)` → "0"
- `dec2oct(42)` → "52"
- `dec2oct(8)` → "10"
- `dec2oct(536870911)` → "3777777777" (max positive)

**DEC2OCT with places:**
- `dec2oct(100, 6)` → "000144"
- `dec2oct(100, 3)` → "144" (exact fit)
- `dec2oct(0, 4)` → "0000"
- `dec2oct(1, 10)` → "0000000001"

**DEC2OCT negative (two's complement):**
- `dec2oct(-1)` → "7777777777"
- `dec2oct(-536870912)` → "4000000000" (min negative)
- `dec2oct(-8)` → "7777777770"
- `dec2oct(-128)` → "7777777600"

**DEC2OCT errors:**
- `dec2oct(536870912)` → `#NUM!` (out of range, too large)
- `dec2oct(-536870913)` → `#NUM!` (out of range, too small)
- `dec2oct(100, 2)` → `#NUM!` (result "144" needs 3 places, only 2 given)
- `dec2oct(100, 0)` → `#NUM!` (places < 1)
- `dec2oct(100, 11)` → `#NUM!` (places > 10)
- `dec2oct(-1, 2)` → `#NUM!` (negative always outputs 10 digits, 2 < 10)
- `dec2oct(#N/A)` → `#N/A` (error propagation)
- Wrong arity (0 args, 3 args) → `#VALUE!`

**DEC2OCT type coercion:**
- `dec2oct(100.7)` → "144" (truncated to 100)
- `dec2oct("100")` → "144" (text coerced to number)
- `dec2oct(TRUE)` → "1" (bool coerced to 1)

### Conformance fixture

Create `tests/fixtures/engineering/oct2dec_dec2oct.xlsx`.

**Sheet1 data:**
- A: "Oct" header, rows 2-8: `144, 52, 0, 3777777777, 7777777777, 4000000000, 10`
- B: "Dec" header, rows 2-8: `100, 42, 0, 536870911, -1, -536870912, -128`
- C: "Places" header, rows 2-5: `6, 3, 4, 2`
- D: "Error" header, row 2: `=NA()`
- E: "Mixed" header, rows 2-4: `"77"`, `TRUE`, `""` (empty string)

**Sheet2 data:**
- A: "XOct" header, rows 2-4: `15, 77, 144`

**Formulas (column F, starting row 2) — 30 formulas:**

OCT2DEC happy path (5):
1. `=OCT2DEC(A2)` → 100
2. `=OCT2DEC(A3)` → 42
3. `=OCT2DEC(A4)` → 0
4. `=OCT2DEC(A5)` → 536870911
5. `=OCT2DEC(A8)` → 8 ("10" octal)

OCT2DEC negative / two's complement (2):
6. `=OCT2DEC(A6)` → -1
7. `=OCT2DEC(A7)` → -536870912

OCT2DEC leading zeros (1):
8. `=OCT2DEC("0000000001")` → 1

DEC2OCT happy path (4):
9. `=DEC2OCT(B2)` → "144"
10. `=DEC2OCT(B3)` → "52"
11. `=DEC2OCT(B4)` → "0"
12. `=DEC2OCT(B5)` → "3777777777"

DEC2OCT with places (3):
13. `=DEC2OCT(B2, C2)` → "000144" (places=6)
14. `=DEC2OCT(B2, C3)` → "144" (places=3, exact fit)
15. `=DEC2OCT(B4, C4)` → "0000" (0 padded to 4)

DEC2OCT negative (2):
16. `=DEC2OCT(B6)` → "7777777777"
17. `=DEC2OCT(B7)` → "4000000000"

Boundary / error (5):
18. `=DEC2OCT(B2, C5)` → `#NUM!` (100 → "144" needs 3 places, only 2 given)
19. `=OCT2DEC("89")` → `#NUM!` (invalid octal chars — 8 and 9 not allowed)
20. `=OCT2DEC("10000000001")` → `#NUM!` (11 digits, too long)
21. `=DEC2OCT(536870912)` → `#NUM!` (out of range)
22. `=DEC2OCT(-536870913)` → `#NUM!` (out of range)

Type coercion (3):
23. `=DEC2OCT(100.7)` → "144" (truncated)
24. `=DEC2OCT(TRUE)` → "1" (bool → 1)
25. `=OCT2DEC("0000000000")` → 0 (all-zero 10-digit)

Error propagation (2):
26. `=OCT2DEC(D2)` → `#N/A`
27. `=DEC2OCT(D2)` → `#N/A`

Nested (1):
28. `=IFERROR(OCT2DEC("99"), "bad oct")` → "bad oct"

Round-trip (1):
29. `=DEC2OCT(OCT2DEC("144"))` → "144"

Cross-sheet (1):
30. `=OCT2DEC(Sheet2!A2)` → 13

OCT2DEC and DEC2OCT do NOT need `_xlfn.` prefix.

**Fixture workflow:**
1. Generate with openpyxl
2. Recalculate with LibreOffice headless
3. Add `#[test] fn oct2dec_dec2oct()` in `conformance/engineering.rs` (created by spec 23)

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "OCT2DEC, DEC2OCT octal conversion" under `[Unreleased]` → `### Added` |
| `docs/functions.md` | Change OCT2DEC and DEC2OCT from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the OCT2DEC / DEC2OCT checkbox |

## Streaming invariant

Does not violate. Both functions are pure scalar functions of their arguments — no cross-row reads, no prelude dependency.
