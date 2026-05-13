# Feature: COMPLEX / IMREAL / IMAGINARY

**Branch:** `feat/complex`
**Effort:** ~0.5 day
**Crates:** xlstream-eval

## What

Complex number creation and extraction for Excel's Analysis ToolPak. Excel represents complex numbers as text strings in the format `"a+bi"` or `"a+bj"`. These three functions form the foundation — later IM* functions (IMABS, IMSUM, IMDIV, etc.) will build on the parser established here.

- `COMPLEX(real_num, i_num, [suffix])` — creates a complex number text string from real and imaginary parts
- `IMREAL(inumber)` — extracts the real coefficient from a complex number text string
- `IMAGINARY(inumber)` — extracts the imaginary coefficient from a complex number text string

```
=COMPLEX(3, 4)          → "3+4i"
=COMPLEX(3, 4, "j")     → "3+4j"
=COMPLEX(3, -4)          → "3-4i"
=COMPLEX(3, 0)           → "3"           (no imaginary part shown)
=COMPLEX(0, 4)           → "4i"          (no real part shown)
=COMPLEX(0, 1)           → "i"           (coefficient 1 omitted)
=COMPLEX(0, -1)          → "-i"          (coefficient -1 shown as just "-")
=COMPLEX(0, 0)           → "0"           (pure zero)
=COMPLEX(1, 0)           → "1"           (pure real)
=IMREAL("3+4i")          → 3
=IMREAL("3")             → 3
=IMREAL("4i")            → 0
=IMAGINARY("3+4i")       → 4
=IMAGINARY("3-4i")       → -4
=IMAGINARY("3")          → 0
=IMAGINARY("4i")         → 4
=IMAGINARY("i")          → 1
=IMAGINARY("-i")         → -1
```

**Complex number text format (Excel canonical):**
- `"a+bi"` or `"a-bi"` — full form (a, b are decimal numbers, possibly with decimals)
- `"a"` — pure real (imaginary = 0)
- `"bi"` — pure imaginary (real = 0)
- `"i"` — shorthand for `"0+1i"` (real = 0, imaginary = 1)
- `"-i"` — shorthand for `"0-1i"` (real = 0, imaginary = -1)
- `"a+bj"` / `"a-bj"` / `"bj"` / `"j"` / `"-j"` — same with "j" suffix
- Suffix must be consistently "i" or "j" — cannot mix within one string
- Numbers can be integers or decimals (e.g., `"3.5+2.7i"`)
- Scientific notation allowed in parts (e.g., `"1E+2+3i"` = `"100+3i"`)

Current behavior: no dispatch entries — all return `#VALUE!` from the fallback.

## What already exists

- `crates/xlstream-eval/src/builtins/engineering.rs` — module home. Base conversion functions from specs 23-27 will be here. Complex number functions introduce a new concern: parsing complex number strings. This parser should be a reusable internal helper since all IM* functions need it.
- `crates/xlstream-eval/src/builtins/mod.rs` — `mod engineering;` declared (line 12). Base conversion dispatch arms already present.
- `crates/xlstream-eval/src/builtins/mod.rs:30-36` — `eval_args` helper for pure eager-eval builtins
- `crates/xlstream-eval/src/builtins/math.rs:27-29` — `num_arg_ce` helper (extracts f64 from args)
- `crates/xlstream-eval/src/builtins/string.rs:19-25` — `text_arg` helper (extracts string from args)
- `xlstream_core::coerce::to_number` and `xlstream_core::coerce::to_text` — value coercion functions
- Not in `UNSUPPORTED_FUNCTIONS` or `RANGE_EXPANDING_FUNCTIONS`
- `docs/functions.md` lists all three as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/engineering.rs` — implementation home (base conversion functions from specs 23-27 already present)
- `crates/xlstream-eval/src/builtins/mod.rs:12` — `mod engineering;` declaration
- `crates/xlstream-eval/src/builtins/mod.rs:140-170` — string builtins dispatch pattern
- `crates/xlstream-eval/src/builtins/mod.rs:30-36` — `eval_args` helper
- `crates/xlstream-eval/src/builtins/math.rs:27-34` — `num_arg_ce`, `bool_arg_ce` helpers
- `crates/xlstream-eval/src/builtins/string.rs:19-25` — `text_arg` helper
- `crates/xlstream-eval/tests/conformance/mod.rs` — conformance test runner

## Resolution / Evaluation behavior

All three functions are pure scalar functions — row-local, no streaming concerns, no prelude dependency.

**Classification:** RowLocal (default — not in any special set).

**Prelude:** Nothing needed.

**Row eval:** Eager-eval dispatch. Args evaluated via `eval_args`, passed as `&[Value]` to the builtin function.

### Internal complex number parser

Introduce a reusable struct/function inside `engineering.rs` (not public API — `pub(crate)` at most):

```
struct Complex { real: f64, imag: f64, suffix: char }  // suffix is 'i' or 'j'
fn parse_complex(s: &str) -> Result<Complex, CellError>
fn format_complex(real: f64, imag: f64, suffix: char) -> String
```

The parser handles all the formats listed above. Invalid format → `#NUM!`. This parser will be reused by all future IM* functions.

**COMPLEX(real_num, i_num, [suffix]):**
1. Arity check: 2-3 args. Otherwise → `#VALUE!`
2. Extract arg 0 as number (`real`). Check error propagation.
3. Extract arg 1 as number (`imag`). Check error propagation.
4. If arg 2 present: extract as text. Must be exactly `"i"` or `"j"` (case sensitive). Other values → `#VALUE!`
5. If arg 2 absent: suffix defaults to `"i"`
6. Format result using `format_complex`:
   - If imag == 0 and real == 0: return `"0"`
   - If imag == 0: return formatted real (e.g., `"3"`, `"-2.5"`)
   - If real == 0: return formatted imag with suffix (e.g., `"4i"`, `"-4i"`, `"i"`, `"-i"`)
   - Otherwise: return `"real+imagsuffix"` or `"real-imagsuffix"` (e.g., `"3+4i"`, `"3-4i"`)
   - Coefficient 1 is omitted: `"i"` not `"1i"`, `"3+i"` not `"3+1i"`
   - Coefficient -1 shown as `"-"`: `"-i"` not `"-1i"`, `"3-i"` not `"3-1i"`
7. Return `Value::Text(result.into())`

**IMREAL(inumber):**
1. Arity check: exactly 1 arg. Otherwise → `#VALUE!`
2. Extract arg 0 as text (numbers coerced to text).
3. Parse with `parse_complex`. Invalid format → `#NUM!`
4. Return `Value::Number(complex.real)`

**IMAGINARY(inumber):**
1. Arity check: exactly 1 arg. Otherwise → `#VALUE!`
2. Extract arg 0 as text (numbers coerced to text).
3. Parse with `parse_complex`. Invalid format → `#NUM!`
4. Return `Value::Number(complex.imag)`

**IMREAL/IMAGINARY input coercion:** A plain number like `5` is treated as `"5"` which parses as `5+0i`. So `IMREAL(5)` → 5, `IMAGINARY(5)` → 0. This matches Excel.

**Error conditions:**

COMPLEX:
- Wrong arity (0-1 args, 4+ args): `#VALUE!`
- suffix not "i" or "j" (case sensitive): `#VALUE!`
- suffix is "I" or "J" (uppercase): `#VALUE!`
- Error in any arg: propagate

IMREAL / IMAGINARY:
- Wrong arity (0 args, 2+ args): `#VALUE!`
- Invalid complex number format: `#NUM!`
- Error in arg: propagate

## Tests

### Unit tests (in `engineering.rs`)

**COMPLEX happy path:**
- `complex(3, 4)` → "3+4i"
- `complex(3, -4)` → "3-4i"
- `complex(1, 1)` → "1+i"
- `complex(1, -1)` → "1-i"
- `complex(-3, 4)` → "-3+4i"
- `complex(-3, -4)` → "-3-4i"

**COMPLEX zero handling:**
- `complex(0, 0)` → "0"
- `complex(3, 0)` → "3"
- `complex(-3, 0)` → "-3"
- `complex(0, 4)` → "4i"
- `complex(0, -4)` → "-4i"
- `complex(0, 1)` → "i"
- `complex(0, -1)` → "-i"

**COMPLEX with suffix:**
- `complex(3, 4, "j")` → "3+4j"
- `complex(0, 1, "j")` → "j"
- `complex(0, -1, "j")` → "-j"
- `complex(3, 4, "i")` → "3+4i" (explicit default)

**COMPLEX decimal parts:**
- `complex(3.5, 2.7)` → "3.5+2.7i"
- `complex(0.5, 0.5)` → "0.5+0.5i"

**COMPLEX errors:**
- `complex(3, 4, "k")` → `#VALUE!` (invalid suffix)
- `complex(3, 4, "I")` → `#VALUE!` (uppercase not allowed)
- `complex(3, 4, "J")` → `#VALUE!` (uppercase not allowed)
- `complex(3, 4, "")` → `#VALUE!` (empty suffix)
- `complex(#N/A, 4)` → `#N/A` (error propagation)
- Wrong arity (0 args, 1 arg, 4 args) → `#VALUE!`

**IMREAL happy path:**
- `imreal("3+4i")` → 3.0
- `imreal("3-4i")` → 3.0
- `imreal("3")` → 3.0 (pure real)
- `imreal("4i")` → 0.0 (pure imaginary)
- `imreal("i")` → 0.0
- `imreal("-i")` → 0.0
- `imreal("0")` → 0.0
- `imreal("-3.5+2.7i")` → -3.5
- `imreal("3+4j")` → 3.0 (j suffix)

**IMREAL coercion:**
- `imreal(5)` → 5.0 (number coerced to text "5", parsed as 5+0i)
- `imreal(0)` → 0.0

**IMREAL errors:**
- `imreal("abc")` → `#NUM!` (invalid format)
- `imreal("")` → `#NUM!` (empty string)
- `imreal("3+4i+5")` → `#NUM!` (invalid format)
- `imreal("3+4ij")` → `#NUM!` (mixed suffix)
- `imreal(#N/A)` → `#N/A` (error propagation)
- Wrong arity (0 args, 2 args) → `#VALUE!`

**IMAGINARY happy path:**
- `imaginary("3+4i")` → 4.0
- `imaginary("3-4i")` → -4.0
- `imaginary("3")` → 0.0 (pure real)
- `imaginary("4i")` → 4.0 (pure imaginary)
- `imaginary("-4i")` → -4.0
- `imaginary("i")` → 1.0
- `imaginary("-i")` → -1.0
- `imaginary("0")` → 0.0
- `imaginary("3.5+2.7j")` → 2.7 (j suffix)

**IMAGINARY coercion:**
- `imaginary(5)` → 0.0 (number coerced to text "5", parsed as 5+0i)
- `imaginary(0)` → 0.0

**IMAGINARY errors:**
- `imaginary("abc")` → `#NUM!` (invalid format)
- `imaginary("")` → `#NUM!` (empty string)
- `imaginary(#N/A)` → `#N/A` (error propagation)
- Wrong arity (0 args, 2 args) → `#VALUE!`

**Complex parser edge cases (tested via IMREAL/IMAGINARY):**
- `"1E2+3i"` → real=100, imag=3 (scientific notation)
- `"1.5E-2+3i"` → real=0.015, imag=3
- `"+3+4i"` → real=3, imag=4 (leading plus)
- `"3+0i"` → real=3, imag=0
- `"0+0i"` → real=0, imag=0
- `"3i"` → real=0, imag=3 (no real part, coefficient before suffix)
- `"-3i"` → real=0, imag=-3

### Conformance fixture

Create `tests/fixtures/engineering/complex.xlsx`.

**Sheet1 data:**
- A: "Real" header, rows 2-8: `3, -3, 0, 0, 1, 3.5, 0`
- B: "Imag" header, rows 2-8: `4, -4, 4, -1, 1, 2.7, 0`
- C: "Complex" header, rows 2-10: `3+4i, 3-4i, 3, 4i, i, -i, 0, -3.5+2.7i, 3+4j`
- D: "Error" header, row 2: `=NA()`
- E: "Suffix" header, rows 2-3: `i, j`

**Sheet2 data:**
- A: "XComplex" header, rows 2-3: `5+3i, 2-7i`

**Formulas (column F, starting row 2) — 33 formulas:**

COMPLEX happy path (6):
1. `=COMPLEX(A2, B2)` → "3+4i"
2. `=COMPLEX(A3, B3)` → "-3-4i"
3. `=COMPLEX(A4, B4)` → "4i" (real=0)
4. `=COMPLEX(A5, B5)` → "-i" (real=0, imag=-1)
5. `=COMPLEX(A6, B6)` → "1+i" (imag=1)
6. `=COMPLEX(A8, B8)` → "0" (both zero)

COMPLEX with suffix (2):
7. `=COMPLEX(A2, B2, E2)` → "3+4i" (explicit "i")
8. `=COMPLEX(A2, B2, E3)` → "3+4j" (suffix "j")

COMPLEX zero parts (2):
9. `=COMPLEX(A2, 0)` → "3" (imag=0)
10. `=COMPLEX(0, B2)` → "4i" (real=0)

COMPLEX decimals (1):
11. `=COMPLEX(A7, B7)` → "3.5+2.7i"

IMREAL happy path (5):
12. `=IMREAL(C2)` → 3
13. `=IMREAL(C3)` → 3
14. `=IMREAL(C4)` → 3 (pure real)
15. `=IMREAL(C5)` → 0 (pure imaginary)
16. `=IMREAL(C6)` → 0 ("i" → real=0)

IMREAL edge cases (2):
17. `=IMREAL(C7)` → 0 ("-i" → real=0)
18. `=IMREAL(C9)` → -3.5

IMAGINARY happy path (5):
19. `=IMAGINARY(C2)` → 4
20. `=IMAGINARY(C3)` → -4
21. `=IMAGINARY(C4)` → 0 (pure real)
22. `=IMAGINARY(C5)` → 4 (pure imaginary)
23. `=IMAGINARY(C6)` → 1 ("i" → imag=1)

IMAGINARY edge cases (2):
24. `=IMAGINARY(C7)` → -1 ("-i" → imag=-1)
25. `=IMAGINARY(C8)` → 0 ("0" → imag=0)

j suffix (1):
26. `=IMAGINARY(C10)` → 4 ("3+4j")

Error conditions (3):
27. `=COMPLEX(3, 4, "k")` → `#VALUE!` (invalid suffix)
28. `=IMREAL("abc")` → `#NUM!` (invalid format)
29. `=IMAGINARY("")` → `#NUM!` (empty string)

Error propagation (2):
30. `=IMREAL(D2)` → `#N/A`
31. `=COMPLEX(D2, 4)` → `#N/A`

Round-trip (1):
32. `=IMREAL(COMPLEX(3, 4))` → 3

Cross-sheet (1):
33. `=IMREAL(Sheet2!A2)` → 5

Nested (1):
34. `=IF(IMAGINARY("3+4i")>3, "big imag", "small imag")` → "big imag"

COMPLEX, IMREAL, and IMAGINARY do NOT need `_xlfn.` prefix.

**Fixture workflow:**
1. Generate with openpyxl
2. Recalculate with LibreOffice headless
3. Add `#[test] fn complex()` in `conformance/engineering.rs`

## Design note: complex parser reuse

The complex number parser introduced here will be the foundation for all future IM* functions (IMABS, IMSUM, IMDIV, IMPOWER, IMCOS, etc.). Design it as a clean internal helper:

- `parse_complex(s: &str) -> Result<(f64, f64, char), CellError>` — returns (real, imag, suffix)
- `format_complex(real: f64, imag: f64, suffix: char) -> String` — canonical formatting

Keep it `pub(crate)` inside `engineering.rs` so later IM* functions can call it without reimplementation. Do not expose it as a public API — it's an implementation detail.

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "COMPLEX, IMREAL, IMAGINARY complex number functions" under `[Unreleased]` → `### Added` |
| `docs/functions.md` | Change COMPLEX, IMREAL, IMAGINARY from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the COMPLEX / IMREAL / IMAGINARY checkbox |

## Streaming invariant

Does not violate. All three functions are pure scalar functions of their arguments — no cross-row reads, no prelude dependency.
