# Feature: HEX2BIN / BIN2HEX / HEX2OCT / OCT2HEX / BIN2OCT / OCT2BIN

**Branch:** `feat/cross-base`
**Effort:** ~2 hours
**Crates:** xlstream-eval

## What

Cross-base conversions for Excel's Analysis ToolPak. Six functions that convert between hex, binary, and octal without going through decimal in the user-facing API (though internally they convert to decimal as an intermediate step).

- `HEX2BIN(number, [places])` — hex string to binary string
- `BIN2HEX(number, [places])` — binary string to hex string
- `HEX2OCT(number, [places])` — hex string to octal string
- `OCT2HEX(number, [places])` — octal string to hex string
- `BIN2OCT(number, [places])` — binary string to octal string
- `OCT2BIN(number, [places])` — octal string to binary string

```
=HEX2BIN("F")              → "1111"
=HEX2BIN("F", 8)           → "00001111"   (zero-padded)
=BIN2HEX("1111")            → "F"
=BIN2HEX("1111", 4)         → "000F"
=HEX2OCT("F")               → "17"
=OCT2HEX("17")              → "F"
=BIN2OCT("1111")             → "17"
=OCT2BIN("17")               → "1111"
=HEX2BIN("FFFFFFFFFE")      → "1111111110" (negative: -2)
=OCT2BIN("7777777776")       → "1111111110" (negative: -2)
```

All six return text. All accept an optional `places` argument for zero-padding (positive results only).

**Domain constraints per function:**
- HEX2BIN: input hex must represent a value in the 10-bit binary domain [-512, 511]. Max 10 hex digits but the decimal value must fit. Negative hex values (40-bit two's complement) must also fit in 10-bit.
- BIN2HEX: input is 10-bit binary domain. Output is 40-bit hex domain (always fits).
- HEX2OCT: input hex must represent a value in the 30-bit octal domain [-536870912, 536870911]. Negative hex values must also fit.
- OCT2HEX: input is 30-bit octal domain. Output is 40-bit hex domain (always fits).
- BIN2OCT: input is 10-bit binary domain. Output is 30-bit octal domain (always fits).
- OCT2BIN: input octal must represent a value in the 10-bit binary domain [-512, 511]. Negative octal values must also fit.

Current behavior: no dispatch entries — all return `#VALUE!` from the fallback.

## What already exists

- `crates/xlstream-eval/src/builtins/engineering.rs` — module home. By the time this spec is implemented, specs 23-25 (HEX2DEC/DEC2HEX, BIN2DEC/DEC2BIN, OCT2DEC/DEC2OCT) will have landed here, providing all the base ↔ decimal conversion helpers needed.
- Shared helpers from specs 23-25: `hex_to_dec`, `dec_to_hex`, `bin_to_dec`, `dec_to_bin`, `oct_to_dec`, `dec_to_oct` (or equivalent internal functions). Two's complement logic parameterized by bit width. `places` validation.
- `crates/xlstream-eval/src/builtins/mod.rs` — dispatch arms for HEX2DEC, DEC2HEX, BIN2DEC, DEC2BIN, OCT2DEC, DEC2OCT already present.
- `crates/xlstream-eval/src/builtins/mod.rs:30-36` — `eval_args` helper for pure eager-eval builtins
- `xlstream_core::coerce::to_number` and `xlstream_core::coerce::to_text` — value coercion functions
- Not in `UNSUPPORTED_FUNCTIONS` or `RANGE_EXPANDING_FUNCTIONS`
- `docs/functions.md` lists all six as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/engineering.rs` — implementation home (hex/bin/oct ↔ dec helpers from specs 23-25)
- `crates/xlstream-eval/src/builtins/mod.rs:12` — `mod engineering;` declaration
- `crates/xlstream-eval/src/builtins/mod.rs:140-170` — string builtins dispatch pattern
- `crates/xlstream-eval/src/builtins/mod.rs:30-36` — `eval_args` helper
- `crates/xlstream-eval/src/builtins/math.rs:27-34` — `num_arg_ce`, `bool_arg_ce` helpers
- `crates/xlstream-eval/src/builtins/string.rs:19-25` — `text_arg` helper
- `crates/xlstream-eval/tests/conformance/mod.rs` — conformance test runner

## Resolution / Evaluation behavior

All six functions are pure scalar functions — row-local, no streaming concerns, no prelude dependency.

**Classification:** RowLocal (default — not in any special set).

**Prelude:** Nothing needed.

**Row eval:** Eager-eval dispatch. Args evaluated via `eval_args`, passed as `&[Value]` to the builtin function.

**Implementation strategy:** Each cross-base function is a composition of existing helpers:
- HEX2BIN(n, [p]) = dec_to_bin(hex_to_dec(n), [p]) — but validate that hex_to_dec(n) is in [-512, 511]
- BIN2HEX(n, [p]) = dec_to_hex(bin_to_dec(n), [p])
- HEX2OCT(n, [p]) = dec_to_oct(hex_to_dec(n), [p]) — but validate that hex_to_dec(n) is in [-536870912, 536870911]
- OCT2HEX(n, [p]) = dec_to_hex(oct_to_dec(n), [p])
- BIN2OCT(n, [p]) = dec_to_oct(bin_to_dec(n), [p])
- OCT2BIN(n, [p]) = dec_to_bin(oct_to_dec(n), [p]) — but validate that oct_to_dec(n) is in [-512, 511]

**HEX2BIN(number, [places]):**
1. Parse hex input to decimal (same as HEX2DEC). Validate hex chars, 1-10 digits.
2. Validate decimal result is in [-512, 511]. Out of range → `#NUM!`
3. Convert decimal to binary string (same as DEC2BIN). Apply places if given.
4. Return `Value::Text(result.into())`

**BIN2HEX(number, [places]):**
1. Parse binary input to decimal (same as BIN2DEC). Validate binary chars, 1-10 digits.
2. Convert decimal to hex string (same as DEC2HEX). Apply places if given.
3. Return `Value::Text(result.into())`

**HEX2OCT(number, [places]):**
1. Parse hex input to decimal (same as HEX2DEC). Validate hex chars, 1-10 digits.
2. Validate decimal result is in [-536870912, 536870911]. Out of range → `#NUM!`
3. Convert decimal to octal string (same as DEC2OCT). Apply places if given.
4. Return `Value::Text(result.into())`

**OCT2HEX(number, [places]):**
1. Parse octal input to decimal (same as OCT2DEC). Validate octal chars, 1-10 digits.
2. Convert decimal to hex string (same as DEC2HEX). Apply places if given.
3. Return `Value::Text(result.into())`

**BIN2OCT(number, [places]):**
1. Parse binary input to decimal (same as BIN2DEC). Validate binary chars, 1-10 digits.
2. Convert decimal to octal string (same as DEC2OCT). Apply places if given.
3. Return `Value::Text(result.into())`

**OCT2BIN(number, [places]):**
1. Parse octal input to decimal (same as OCT2DEC). Validate octal chars, 1-10 digits.
2. Validate decimal result is in [-512, 511]. Out of range → `#NUM!`
3. Convert decimal to binary string (same as DEC2BIN). Apply places if given.
4. Return `Value::Text(result.into())`

**Error conditions (all six functions):**
- Wrong arity: `#VALUE!` (each requires 1-2 args)
- Invalid characters for source base: `#NUM!`
- Source string > 10 characters: `#NUM!`
- Empty source string: `#NUM!`
- Result out of target base domain (HEX2BIN, HEX2OCT, OCT2BIN): `#NUM!`
- places < 1 or > 10 or non-integer: `#NUM!`
- Result longer than places: `#NUM!`
- Negative values ignore places (always output 10 digits); if places < 10 → `#NUM!`
- Error in any arg: propagate

## Tests

### Unit tests (in `engineering.rs`)

**HEX2BIN happy path:**
- `hex2bin("F")` → "1111"
- `hex2bin("A")` → "1010"
- `hex2bin("0")` → "0"
- `hex2bin("1FF")` → "111111111" (511 decimal, max positive)

**HEX2BIN with places:**
- `hex2bin("F", 8)` → "00001111"
- `hex2bin("F", 4)` → "1111" (exact fit)

**HEX2BIN negative:**
- `hex2bin("FFFFFFFFFE")` → "1111111110" (-2)
- `hex2bin("FFFFFFFE00")` → "1000000000" (-512, min negative)

**HEX2BIN errors:**
- `hex2bin("200")` → `#NUM!` (512 decimal, out of binary domain)
- `hex2bin("FFFFFFFDFF")` → `#NUM!` (-513, out of binary domain)

**BIN2HEX happy path:**
- `bin2hex("1111")` → "F"
- `bin2hex("0")` → "0"
- `bin2hex("0111111111")` → "1FF" (511)

**BIN2HEX with places:**
- `bin2hex("1111", 4)` → "000F"

**BIN2HEX negative:**
- `bin2hex("1111111111")` → "FFFFFFFFFF" (-1)
- `bin2hex("1000000000")` → "FFFFFFFE00" (-512)

**HEX2OCT happy path:**
- `hex2oct("F")` → "17"
- `hex2oct("0")` → "0"
- `hex2oct("1FFFFFFF")` → "3777777777" (max positive for octal)

**HEX2OCT with places:**
- `hex2oct("F", 4)` → "0017"

**HEX2OCT negative:**
- `hex2oct("FFFFFFFFFF")` → "7777777777" (-1)

**HEX2OCT errors:**
- `hex2oct("2000000000")` → `#NUM!` (out of octal domain)

**OCT2HEX happy path:**
- `oct2hex("17")` → "F"
- `oct2hex("0")` → "0"
- `oct2hex("3777777777")` → "1FFFFFFF"

**OCT2HEX with places:**
- `oct2hex("17", 4)` → "000F"

**OCT2HEX negative:**
- `oct2hex("7777777777")` → "FFFFFFFFFF" (-1)

**BIN2OCT happy path:**
- `bin2oct("1111")` → "17"
- `bin2oct("0")` → "0"
- `bin2oct("0111111111")` → "777" (511)

**BIN2OCT with places:**
- `bin2oct("1111", 4)` → "0017"

**BIN2OCT negative:**
- `bin2oct("1111111111")` → "7777777777" (-1)
- `bin2oct("1000000000")` → "7777777000" (-512)

**OCT2BIN happy path:**
- `oct2bin("17")` → "1111"
- `oct2bin("0")` → "0"
- `oct2bin("777")` → "111111111" (511)

**OCT2BIN with places:**
- `oct2bin("17", 8)` → "00001111"

**OCT2BIN negative:**
- `oct2bin("7777777776")` → "1111111110" (-2)
- `oct2bin("7777777000")` → "1000000000" (-512)

**OCT2BIN errors:**
- `oct2bin("1000")` → `#NUM!` (512 decimal, out of binary domain)

**Shared error tests (representative — test for each function):**
- Wrong arity (0 args, 3 args) → `#VALUE!`
- Error propagation: arg is `#N/A` → `#N/A`
- Empty string → `#NUM!`

### Conformance fixture

Create `tests/fixtures/engineering/cross_base.xlsx`.

**Sheet1 data:**
- A: "Hex" header, rows 2-6: `F, A, 1FF, FFFFFFFFFE, 0`
- B: "Bin" header, rows 2-6: `1111, 1010, 0111111111, 1111111110, 0`
- C: "Oct" header, rows 2-6: `17, 12, 777, 7777777776, 0`
- D: "Places" header, rows 2-4: `8, 4, 10`
- E: "Error" header, row 2: `=NA()`

**Sheet2 data:**
- A: "XHex" header, rows 2-3: `1F, A5`

**Formulas (column F, starting row 2) — 38 formulas:**

HEX2BIN happy path (3):
1. `=HEX2BIN(A2)` → "1111" (F → binary)
2. `=HEX2BIN(A3)` → "1010" (A → binary)
3. `=HEX2BIN(A6)` → "0"

HEX2BIN with places (2):
4. `=HEX2BIN(A2, D2)` → "00001111" (places=8)
5. `=HEX2BIN(A2, D3)` → "1111" (places=4, exact fit)

HEX2BIN negative (1):
6. `=HEX2BIN(A5)` → "1111111110" (FFFFFFFFFE → -2)

HEX2BIN error (1):
7. `=HEX2BIN("200")` → `#NUM!` (512, out of binary range)

BIN2HEX happy path (3):
8. `=BIN2HEX(B2)` → "F"
9. `=BIN2HEX(B4)` → "1FF" (0111111111 → 511 → hex)
10. `=BIN2HEX(B6)` → "0"

BIN2HEX with places (1):
11. `=BIN2HEX(B2, D3)` → "000F" (places=4)

BIN2HEX negative (1):
12. `=BIN2HEX(B5)` → "FFFFFFFFFE" (1111111110 → -2)

HEX2OCT happy path (2):
13. `=HEX2OCT(A2)` → "17"
14. `=HEX2OCT(A6)` → "0"

HEX2OCT with places (1):
15. `=HEX2OCT(A2, D3)` → "0017" (places=4)

HEX2OCT negative (1):
16. `=HEX2OCT("FFFFFFFFFF")` → "7777777777" (-1)

HEX2OCT error (1):
17. `=HEX2OCT("2000000000")` → `#NUM!` (out of octal range)

OCT2HEX happy path (2):
18. `=OCT2HEX(C2)` → "F"
19. `=OCT2HEX(C6)` → "0"

OCT2HEX with places (1):
20. `=OCT2HEX(C2, D3)` → "000F" (places=4)

OCT2HEX negative (1):
21. `=OCT2HEX(C5)` → "FFFFFFFFFE" (7777777776 → -2)

BIN2OCT happy path (2):
22. `=BIN2OCT(B2)` → "17"
23. `=BIN2OCT(B6)` → "0"

BIN2OCT with places (1):
24. `=BIN2OCT(B2, D3)` → "0017" (places=4)

BIN2OCT negative (1):
25. `=BIN2OCT(B5)` → "7777777776" (1111111110 → -2)

OCT2BIN happy path (2):
26. `=OCT2BIN(C2)` → "1111"
27. `=OCT2BIN(C6)` → "0"

OCT2BIN with places (1):
28. `=OCT2BIN(C2, D2)` → "00001111" (places=8)

OCT2BIN negative (1):
29. `=OCT2BIN(C5)` → "1111111110" (7777777776 → -2)

OCT2BIN error (1):
30. `=OCT2BIN("1000")` → `#NUM!` (512 decimal, out of binary range)

Error propagation (2):
31. `=HEX2BIN(E2)` → `#N/A`
32. `=BIN2HEX(E2)` → `#N/A`

Type coercion (2):
33. `=HEX2BIN("f")` → "1111" (lowercase hex)
34. `=BIN2HEX("01111", 4)` → "000F" (leading zero in binary)

Round-trip (2):
35. `=BIN2HEX(HEX2BIN("F"))` → "F"
36. `=OCT2BIN(BIN2OCT("1111"))` → "1111"

Cross-sheet (1):
37. `=HEX2BIN(Sheet2!A2)` → `#NUM!` (1F = 31, fits in binary domain → "11111")

Nested (1):
38. `=IFERROR(OCT2BIN("77777"), "too big")` → "too big"

None of these functions need `_xlfn.` prefix.

**Fixture workflow:**
1. Generate with openpyxl
2. Recalculate with LibreOffice headless
3. Add `#[test] fn cross_base()` in `conformance/engineering.rs`

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "HEX2BIN, BIN2HEX, HEX2OCT, OCT2HEX, BIN2OCT, OCT2BIN cross-base conversion" under `[Unreleased]` → `### Added` |
| `docs/functions.md` | Change HEX2BIN, BIN2HEX, HEX2OCT, OCT2HEX, BIN2OCT, OCT2BIN from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the HEX2BIN / BIN2HEX / HEX2OCT / OCT2HEX / BIN2OCT / OCT2BIN checkbox |

## Streaming invariant

Does not violate. All six functions are pure scalar functions of their arguments — no cross-row reads, no prelude dependency.
