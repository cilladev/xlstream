# Feature: BASE

**Branch:** `feat/base`
**Effort:** ~2 hours
**Crates:** xlstream-eval

## What

General-purpose base conversion. Unlike the Analysis ToolPak functions (DEC2HEX, DEC2BIN, DEC2OCT), BASE handles arbitrary radixes 2-36 and does NOT use two's complement for negatives — negative input is simply an error.

- `BASE(number, radix, [min_length])` — converts a non-negative integer to text in the given base

```
=BASE(255, 16)        → "FF"
=BASE(255, 16, 4)     → "00FF"        (zero-padded to min_length)
=BASE(10, 2)          → "1010"
=BASE(100, 8)         → "144"
=BASE(0, 16)          → "0"
=BASE(255, 36)        → "73"           (uses digits 0-9 then A-Z)
=BASE(35, 36)         → "Z"
=BASE(1295, 36)       → "ZZ"
=BASE(7, 2, 8)        → "00000111"
```

**Domain:**
- `number`: non-negative, < 2^53 (9007199254740992). Truncated to integer. Negative → `#NUM!`
- `radix`: integer 2-36. Out of range → `#NUM!`
- `min_length`: optional, integer 1-255. Zero-pads result to this length. If result is already longer, no truncation. Omitted = no padding.
- Returns text. Uses uppercase digits 0-9, A-Z for bases > 10.

BASE was introduced in Excel 2013 and requires `_xlfn.` prefix in xlsx formula storage. However, our dispatcher already strips `_xlfn.` prefixes (see `mod.rs:95`), so no special handling is needed — just match on `"BASE"`.

Current behavior: no dispatch entry — returns `#VALUE!` from the fallback.

## What already exists

- `crates/xlstream-eval/src/builtins/engineering.rs` — module home. Specs 23-26 will have landed the base-conversion family here. BASE is standalone (not two's complement, not Analysis ToolPak) but belongs in the same module as it's a base-conversion function.
- `crates/xlstream-eval/src/builtins/mod.rs:30-36` — `eval_args` helper for pure eager-eval builtins
- `crates/xlstream-eval/src/builtins/math.rs:27-29` — `num_arg_ce` helper (extracts f64 from args)
- `xlstream_core::coerce::to_number` — value coercion
- Not in `UNSUPPORTED_FUNCTIONS` or `RANGE_EXPANDING_FUNCTIONS`
- `docs/functions.md` lists BASE as `.` (planned) for v0.3 in the Math & Trigonometry section

## Where to look

- `crates/xlstream-eval/src/builtins/engineering.rs` — implementation home (base-conversion functions from specs 23-26 already present)
- `crates/xlstream-eval/src/builtins/mod.rs:12` — `mod engineering;` declaration
- `crates/xlstream-eval/src/builtins/mod.rs:95` — `_xlfn.` prefix stripping
- `crates/xlstream-eval/src/builtins/mod.rs:140-170` — string builtins dispatch pattern
- `crates/xlstream-eval/src/builtins/mod.rs:30-36` — `eval_args` helper
- `crates/xlstream-eval/src/builtins/math.rs:27-34` — `num_arg_ce`, `bool_arg_ce` helpers
- `crates/xlstream-eval/tests/conformance/mod.rs` — conformance test runner

## Resolution / Evaluation behavior

BASE is a pure scalar function — row-local, no streaming concerns, no prelude dependency.

**Classification:** RowLocal (default — not in any special set).

**Prelude:** Nothing needed.

**Row eval:** Eager-eval dispatch. Args evaluated via `eval_args`, passed as `&[Value]` to the builtin function.

**BASE(number, radix, [min_length]):**
1. Arity check: 2-3 args. Otherwise → `#VALUE!`
2. Extract arg 0 as number via `coerce::to_number`. Check for error propagation.
3. Truncate to integer (floor toward zero, like Excel's INT). If negative → `#NUM!`. If >= 2^53 → `#NUM!`
4. Extract arg 1 as number via `coerce::to_number`. Truncate to integer. Must be 2-36. Out of range → `#NUM!`
5. If arg 2 present: extract as number, truncate to integer. Must be 1-255 (0 is also valid in some Excel versions — match Excel behavior). Out of range → `#NUM!`
6. Convert integer to string in given radix using digits `0-9A-Z`
7. If min_length given and result is shorter, left-pad with zeros
8. Return `Value::Text(result.into())`

**Key difference from DEC2HEX/DEC2BIN/DEC2OCT:** BASE does NOT handle negatives via two's complement. Negative input → `#NUM!`. This is a completely separate code path — do not reuse the two's complement machinery from specs 23-26.

**Error conditions:**
- Wrong arity (0-1 args, 4+ args): `#VALUE!`
- number < 0: `#NUM!`
- number >= 2^53 (9007199254740992): `#NUM!`
- radix < 2 or > 36: `#NUM!`
- min_length < 0 or > 255: `#NUM!`
- Error in any arg: propagate

## Tests

### Unit tests (in `engineering.rs`)

**BASE happy path:**
- `base(255, 16)` → "FF"
- `base(10, 2)` → "1010"
- `base(100, 8)` → "144"
- `base(0, 16)` → "0"
- `base(0, 2)` → "0"
- `base(1, 10)` → "1"
- `base(255, 10)` → "255"
- `base(35, 36)` → "Z"
- `base(36, 36)` → "10"
- `base(1295, 36)` → "ZZ"

**BASE with min_length:**
- `base(255, 16, 4)` → "00FF"
- `base(7, 2, 8)` → "00000111"
- `base(255, 16, 2)` → "FF" (result already 2 digits, no padding needed)
- `base(255, 16, 1)` → "FF" (result longer than min_length, no truncation)
- `base(0, 2, 4)` → "0000"
- `base(1, 10, 5)` → "00001"

**BASE large values:**
- `base(9007199254740991, 16)` → "1FFFFFFFFFFFFF" (2^53 - 1, max valid)
- `base(4294967295, 16)` → "FFFFFFFF" (2^32 - 1)

**BASE various radixes:**
- `base(10, 2)` → "1010"
- `base(10, 3)` → "101"
- `base(10, 8)` → "12"
- `base(10, 10)` → "10"
- `base(10, 16)` → "A"
- `base(10, 36)` → "A"

**BASE type coercion:**
- `base(255.9, 16)` → "FF" (truncated to 255)
- `base("255", 16)` → "FF" (text coerced to number)
- `base(TRUE, 2)` → "1" (bool coerced to 1)
- `base("10", "2")` → "1010" (both args coerced)

**BASE errors:**
- `base(-1, 16)` → `#NUM!` (negative)
- `base(9007199254740992, 16)` → `#NUM!` (>= 2^53)
- `base(10, 1)` → `#NUM!` (radix < 2)
- `base(10, 37)` → `#NUM!` (radix > 36)
- `base(10, 0)` → `#NUM!` (radix < 2)
- `base(10, 2, 0)` → `#NUM!` (min_length < 1; Excel varies — verify)
- `base(10, 2, 256)` → `#NUM!` (min_length > 255)
- `base(10, 2, -1)` → `#NUM!` (negative min_length)
- `base(#N/A, 16)` → `#N/A` (error propagation)
- `base(10, #N/A)` → `#N/A` (error propagation)
- Wrong arity (0 args, 1 arg, 4 args) → `#VALUE!`

### Conformance fixture

Create `tests/fixtures/engineering/base.xlsx`.

**Sheet1 data:**
- A: "Number" header, rows 2-8: `255, 10, 100, 0, 35, 1295, 7`
- B: "Radix" header, rows 2-8: `16, 2, 8, 16, 36, 36, 2`
- C: "MinLen" header, rows 2-5: `4, 8, 6, 1`
- D: "Error" header, row 2: `=NA()`
- E: "Edge" header, rows 2-4: `-1`, `9007199254740991`, `9007199254740992`

**Sheet2 data:**
- A: "XNum" header, rows 2-3: `42, 100`

**Formulas (column F, starting row 2) — 28 formulas:**

BASE happy path (7):
1. `=BASE(A2, B2)` → "FF" (255 base 16)
2. `=BASE(A3, B3)` → "1010" (10 base 2)
3. `=BASE(A4, B4)` → "144" (100 base 8)
4. `=BASE(A5, B5)` → "0" (0 base 16)
5. `=BASE(A6, B6)` → "Z" (35 base 36)
6. `=BASE(A7, B7)` → "ZZ" (1295 base 36)
7. `=BASE(A8, B8)` → "111" (7 base 2)

BASE with min_length (4):
8. `=BASE(A2, B2, C2)` → "00FF" (min_length=4)
9. `=BASE(A8, B8, C3)` → "00000111" (min_length=8)
10. `=BASE(A4, B4, C4)` → "000144" (min_length=6)
11. `=BASE(A2, B2, C5)` → "FF" (min_length=1, result already longer)

Various radixes (5):
12. `=BASE(255, 2)` → "11111111"
13. `=BASE(255, 8)` → "377"
14. `=BASE(255, 10)` → "255"
15. `=BASE(255, 36)` → "73"
16. `=BASE(36, 36)` → "10"

Large values (2):
17. `=BASE(4294967295, 16)` → "FFFFFFFF"
18. `=BASE(E3, 16)` → "1FFFFFFFFFFFFF" (2^53 - 1)

Error conditions (5):
19. `=BASE(E2, 16)` → `#NUM!` (-1, negative)
20. `=BASE(E4, 16)` → `#NUM!` (2^53, too large)
21. `=BASE(10, 1)` → `#NUM!` (radix < 2)
22. `=BASE(10, 37)` → `#NUM!` (radix > 36)
23. `=BASE(10, 2, 256)` → `#NUM!` (min_length > 255)

Type coercion (2):
24. `=BASE(255.9, 16)` → "FF" (truncated)
25. `=BASE(TRUE, 2)` → "1" (bool → 1)

Error propagation (1):
26. `=BASE(D2, 16)` → `#N/A`

Nested (1):
27. `=IFERROR(BASE(-1, 16), "no negatives")` → "no negatives"

Cross-sheet (1):
28. `=BASE(Sheet2!A2, 16)` → "2A" (42 base 16)

BASE needs `_xlfn.` prefix in xlsx storage. The fixture generator (openpyxl) should store formulas as `=_xlfn.BASE(...)`. Our dispatcher strips the prefix automatically.

**Fixture workflow:**
1. Generate with openpyxl (use `_xlfn.BASE(...)` in formula strings)
2. Recalculate with LibreOffice headless
3. Add `#[test] fn base()` in `conformance/engineering.rs`

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "BASE general base conversion" under `[Unreleased]` → `### Added` |
| `docs/functions.md` | Change BASE from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the BASE checkbox |

## Streaming invariant

Does not violate. BASE is a pure scalar function of its arguments — no cross-row reads, no prelude dependency.
