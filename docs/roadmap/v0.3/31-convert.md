# Feature: CONVERT

**Branch:** `feat/convert`
**Effort:** ~1 day
**Crates:** xlstream-eval

## What

Unit conversion — converts a number from one measurement unit to another.

- `CONVERT(number, from_unit, to_unit)` — converts `number` from `from_unit` to `to_unit`

```
=CONVERT(1, "lbm", "kg")       → 0.45359237
=CONVERT(68, "F", "C")          → 20
=CONVERT(1, "gal", "l")         → 3.78541178
=CONVERT(100, "km", "mi")       → 62.13711922
=CONVERT(1, "day", "hr")        → 24
=CONVERT(1024, "byte", "kbyte") → 1          (SI prefix on "byte")
=CONVERT(1, "m", "km")          → 0.001      (SI prefix on "m")
```

**Domain:** Supports ~100 base unit abbreviations across 13 categories. Units within the same category can convert; cross-category conversions return `#N/A`. Unit strings are case-sensitive. Unknown unit → `#N/A`.

**SI prefixes** (yotta through yocto) and **binary prefixes** (kibi through yobi) apply to most metric units, multiplying the base unit by the prefix factor. Temperature units do NOT accept SI prefixes.

Current behavior: no dispatch entry — returns `#VALUE!` from the fallback.

## What already exists

- `crates/xlstream-eval/src/builtins/engineering.rs` — module home. Specs 23-30 (base conversion, BASE, COMPLEX/IMREAL/IMAGINARY, DELTA/GESTEP, ERF/ERFC) will have landed here. CONVERT is standalone but follows the same dispatch pattern.
- `crates/xlstream-eval/src/builtins/mod.rs` — `mod engineering;` declared (line 12). Engineering dispatch arms already present from specs 23-30.
- `crates/xlstream-eval/src/builtins/mod.rs:30-36` — `eval_args` helper for pure eager-eval builtins
- `crates/xlstream-eval/src/builtins/math.rs:27-29` — `num_arg_ce` helper
- `crates/xlstream-eval/src/builtins/string.rs:19-25` — `text_arg` helper (extracts string from args)
- `xlstream_core::coerce::to_text` — value-to-text coercion
- Not in `UNSUPPORTED_FUNCTIONS` or `RANGE_EXPANDING_FUNCTIONS`
- `docs/functions.md` lists CONVERT as `.` (planned) for v0.3

## Where to look

- `crates/xlstream-eval/src/builtins/engineering.rs` — implementation home (base conversion + comparison + erf functions from specs 23-30 already present)
- `crates/xlstream-eval/src/builtins/mod.rs:12` — `mod engineering;` declaration
- `crates/xlstream-eval/src/builtins/mod.rs:140-157` — string builtins dispatch pattern (text-handling builtins)
- `crates/xlstream-eval/src/builtins/mod.rs:30-36` — `eval_args` helper
- `crates/xlstream-eval/src/builtins/math.rs:27-34` — `num_arg_ce`, `bool_arg_ce` helpers
- `crates/xlstream-eval/src/builtins/string.rs:19-25` — `text_arg` helper
- `crates/xlstream-eval/tests/conformance/mod.rs` — conformance test runner

## Resolution / Evaluation behavior

CONVERT is a pure scalar function — row-local, no streaming concerns, no prelude dependency.

**Classification:** RowLocal (default — not in any special set).

**Prelude:** Nothing needed.

**Row eval:** Eager-eval dispatch. All args evaluated via `eval_args`, passed as `&[Value]` to the builtin function. Arg 0 is numeric; args 1 and 2 are text.

**CONVERT(number, from_unit, to_unit):**
1. Arity check: exactly 3 args. Otherwise `#VALUE!`
2. Extract arg 0 as f64 via `num_arg_ce`. Error → propagate.
3. Extract args 1 and 2 as text strings (coerce via `to_text`). Error → propagate.
4. Parse each unit string: try exact match against base unit table. If no exact match, try stripping the longest matching SI/binary prefix from the front, then match the remainder against base units. If still no match → `#N/A`.
5. Verify both units belong to the same category. Different categories → `#N/A`.
6. Convert: for non-temperature units, `result = number * (from_base_factor / to_base_factor)`. For temperature, use dedicated conversion formulas with offsets.
7. Return `Value::Number(result)`.

### Unit catalog

Each category stores units as a multiplier to a canonical base unit (except temperature, which uses formulas).

**Weight/Mass** (base: kilogram):
| Unit | Abbreviation | Factor (to kg) |
|---|---|---|
| Gram | `"g"` | 0.001 |
| Slug | `"sg"` | 14.59390294 |
| Pound mass | `"lbm"` | 0.45359237 |
| U (atomic mass) | `"u"` | 1.660538921e-27 |
| Ounce mass | `"ozm"` | 0.028349523125 |
| Grain | `"grain"` | 0.00006479891 |
| US hundredweight | `"cwt"` / `"shweight"` | 45.359237 |
| UK hundredweight | `"uk_cwt"` / `"lcwt"` / `"hweight"` | 50.80234544 |
| Stone | `"stone"` | 6.35029318 |
| Ton | `"ton"` | 907.18474 |
| UK ton | `"uk_ton"` / `"LTON"` / `"brton"` | 1016.0469088 |

**Distance** (base: meter):
| Unit | Abbreviation | Factor (to m) |
|---|---|---|
| Meter | `"m"` | 1 |
| Statute mile | `"mi"` | 1609.344 |
| Nautical mile | `"Nmi"` | 1852 |
| Inch | `"in"` | 0.0254 |
| Foot | `"ft"` | 0.3048 |
| Yard | `"yd"` | 0.9144 |
| Angstrom | `"ang"` | 1e-10 |
| Ell | `"ell"` | 1.143 |
| Light-year | `"ly"` | 9.46073047258e15 |
| Parsec | `"parsec"` / `"pc"` | 3.08567758149e16 |
| Pica (1/72 in) | `"Picapt"` / `"Pica"` | 0.00035277778 |
| Pica (1/6 in) | `"pica"` | 0.00423333333 |
| US survey mile | `"survey_mi"` | 1609.347219 |

**Time** (base: second):
| Unit | Abbreviation | Factor (to s) |
|---|---|---|
| Year | `"yr"` | 31557600 (365.25 days) |
| Day | `"day"` | 86400 |
| Hour | `"hr"` | 3600 |
| Minute | `"mn"` | 60 |
| Second | `"sec"` | 1 |

**Pressure** (base: Pascal):
| Unit | Abbreviation | Factor (to Pa) |
|---|---|---|
| Pascal | `"Pa"` / `"p"` | 1 |
| Atmosphere | `"atm"` / `"at"` | 101325 |
| mmHg | `"mmHg"` | 133.322 |
| PSI | `"psi"` | 6894.757293168 |
| Torr | `"Torr"` | 133.3223684211 |

**Force** (base: Newton):
| Unit | Abbreviation | Factor (to N) |
|---|---|---|
| Newton | `"N"` | 1 |
| Dyne | `"dyn"` / `"dy"` | 1e-5 |
| Pound force | `"lbf"` | 4.4482216152605 |
| Pond | `"pond"` | 0.00980665 |

**Energy** (base: Joule):
| Unit | Abbreviation | Factor (to J) |
|---|---|---|
| Joule | `"J"` | 1 |
| Electron volt | `"eV"` / `"ev"` | 1.602176634e-19 |
| Calorie (IT) | `"cal"` | 4.1868 |
| Calorie (thermo) | `"c"` | 4.184 |
| BTU | `"BTU"` / `"btu"` | 1055.05585262 |
| Horsepower-hour | `"HPh"` / `"hh"` | 2684519.5368856 |
| Watt-hour | `"Wh"` / `"wh"` | 3600 |
| Foot-pound | `"flb"` | 1.3558179483314 |

**Power** (base: Watt):
| Unit | Abbreviation | Factor (to W) |
|---|---|---|
| Watt | `"W"` / `"w"` | 1 |
| Horsepower | `"HP"` / `"h"` | 745.69987158227 |
| Pferdestärke | `"PS"` | 735.49875 |

**Magnetism** (base: Tesla):
| Unit | Abbreviation | Factor (to T) |
|---|---|---|
| Tesla | `"T"` | 1 |
| Gauss | `"ga"` | 1e-4 |

**Temperature** (special — non-multiplicative):
| Unit | Abbreviation |
|---|---|
| Celsius | `"C"` / `"cel"` |
| Fahrenheit | `"F"` / `"fah"` |
| Kelvin | `"K"` / `"kel"` |
| Rankine | `"Rank"` |
| Réaumur | `"Reau"` |

Temperature conversion formulas (convert to Celsius as intermediate):
- C → F: `C * 9/5 + 32`
- C → K: `C + 273.15`
- C → Rank: `(C + 273.15) * 9/5`
- C → Reau: `C * 4/5`
- F → C: `(F - 32) * 5/9`
- K → C: `K - 273.15`
- Rank → C: `Rank * 5/9 - 273.15`
- Reau → C: `Reau * 5/4`

Temperature units do NOT accept SI prefixes. An attempt like `"kC"` → `#N/A`.

**Volume** (base: liter — note: Excel CONVERT uses liter, not cubic meter):
| Unit | Abbreviation | Factor (to l) |
|---|---|---|
| Teaspoon | `"tsp"` | 0.00492892159375 |
| Modern teaspoon | `"tspm"` | 0.005 |
| Tablespoon | `"tbs"` | 0.01478676478125 |
| Fluid ounce | `"oz"` | 0.0295735295625 |
| Cup | `"cup"` | 0.2365882365 |
| US pint | `"pt"` / `"us_pt"` | 0.473176473 |
| UK pint | `"uk_pt"` | 0.56826125 |
| Quart | `"qt"` | 0.946352946 |
| Imperial quart | `"uk_qt"` | 1.1365225 |
| Gallon | `"gal"` | 3.785411784 |
| Imperial gallon | `"uk_gal"` | 4.54609 |
| Liter | `"l"` / `"L"` / `"lt"` | 1 |
| Cubic angstrom | `"ang3"` / `"ang^3"` | 1e-27 |
| US oil barrel | `"barrel"` | 158.987294928 |
| US bushel | `"bushel"` | 35.23907017 |
| Cubic foot | `"ft3"` / `"ft^3"` | 28.316846592 |
| Cubic inch | `"in3"` / `"in^3"` | 0.016387064 |
| Cubic light-year | `"ly3"` / `"ly^3"` | 8.46786664624e47 |
| Cubic meter | `"m3"` / `"m^3"` | 1000 |
| Cubic mile | `"mi3"` / `"mi^3"` | 4.168181825e12 |
| Cubic yard | `"yd3"` / `"yd^3"` | 764.554857984 |
| Cubic nautical mile | `"Nmi3"` / `"Nmi^3"` | 6.352182208e9 |
| Cubic Pica | `"Picapt3"` / `"Picapt^3"` | 4.39157e-11 |
| Gross registered ton | `"GRT"` / `"regton"` | 2831.6846592 |
| Measurement ton | `"MTON"` | 1132.67386368 |

**Area** (base: square meter):
| Unit | Abbreviation | Factor (to m^2) |
|---|---|---|
| International acre | `"uk_acre"` | 4046.8564224 |
| US survey acre | `"us_acre"` | 4046.872609874 |
| Square angstrom | `"ang2"` / `"ang^2"` | 1e-20 |
| Are | `"ar"` | 100 |
| Square foot | `"ft2"` / `"ft^2"` | 0.09290304 |
| Hectare | `"ha"` | 10000 |
| Square inch | `"in2"` / `"in^2"` | 0.00064516 |
| Square light-year | `"ly2"` / `"ly^2"` | 8.9505421075e31 |
| Square meter | `"m2"` / `"m^2"` | 1 |
| Morgen | `"Morgen"` | 2500 |
| Square mile | `"mi2"` / `"mi^2"` | 2589988.110336 |
| Square nautical mile | `"Nmi2"` / `"Nmi^2"` | 3429904 |
| Square Pica | `"Picapt2"` / `"Picapt^2"` | 1.24452e-7 |
| Square yard | `"yd2"` / `"yd^2"` | 0.83612736 |

**Speed** (base: m/s):
| Unit | Abbreviation | Factor (to m/s) |
|---|---|---|
| Admiralty knot | `"admkn"` | 0.514773333 |
| Knot | `"kn"` | 0.514444444 |
| Meters per hour | `"m/h"` / `"m/hr"` | 1/3600 |
| Meters per second | `"m/s"` / `"m/sec"` | 1 |
| Miles per hour | `"mph"` | 0.44704 |

**Information** (base: bit):
| Unit | Abbreviation | Factor (to bit) |
|---|---|---|
| Bit | `"bit"` | 1 |
| Byte | `"byte"` | 8 |

### SI prefixes (apply to metric units)

| Prefix | Symbol | Factor |
|---|---|---|
| yotta | `"Y"` | 1e24 |
| zetta | `"Z"` | 1e21 |
| exa | `"E"` | 1e18 |
| peta | `"P"` | 1e15 |
| tera | `"T"` | 1e12 |
| giga | `"G"` | 1e9 |
| mega | `"M"` | 1e6 |
| kilo | `"k"` | 1e3 |
| hecto | `"h"` | 1e2 |
| deka | `"da"` | 1e1 |
| deci | `"d"` | 1e-1 |
| centi | `"c"` | 1e-2 |
| milli | `"m"` | 1e-3 |
| micro | `"u"` | 1e-6 |
| nano | `"n"` | 1e-9 |
| pico | `"p"` | 1e-12 |
| femto | `"f"` | 1e-15 |
| atto | `"a"` | 1e-18 |
| zepto | `"z"` | 1e-21 |
| yocto | `"y"` | 1e-24 |

### Binary prefixes (apply to information units)

| Prefix | Symbol | Factor |
|---|---|---|
| kibi | `"ki"` | 1024 |
| mebi | `"Mi"` | 1048576 |
| gibi | `"Gi"` | 1073741824 |
| tebi | `"Ti"` | 1099511627776 |
| pebi | `"Pi"` | 1125899906842624 |
| exbi | `"Ei"` | 1152921504606846976 |
| zebi | `"Zi"` | 1.180591620717e21 |
| yobi | `"Yi"` | 1.208925819615e24 |

### Unit string parsing algorithm

1. Check exact match against all base unit abbreviations (including aliases). If found → return (category, factor).
2. Try each SI prefix (longest first: `"da"` before `"d"`, `"Yi"` before `"Y"`). Strip prefix, check remainder against base units. If base unit found and is in a category that accepts SI prefixes (not temperature) → return (category, factor * prefix_factor). Binary prefixes are tried for information units.
3. No match → `#N/A`.

**Prefix ambiguity:** Some unit abbreviations start with letters that are also prefix symbols. The algorithm must try the exact match first. For example, `"ft"` is foot, not femto-ton. `"da"` as a standalone is not a unit — it's only a prefix.

**Case sensitivity:** Unit strings are case-sensitive. `"m"` = meter, `"M"` = mega prefix. `"C"` = Celsius, `"c"` = thermochemical calorie. This must be preserved exactly.

**Error conditions:**
- Wrong arity: `#VALUE!` (requires exactly 3 args)
- Non-numeric first arg: `#VALUE!`
- Unknown from_unit or to_unit: `#N/A`
- Cross-category conversion (e.g., `"kg"` to `"m"`): `#N/A`
- Error in any arg: propagate
- Same unit: returns input number unchanged

**Implementation note:** The unit table is large (~100 base entries + aliases + prefix combinations) but static. A `phf` compile-time map or a series of `match` arms are both viable. Given the project already depends on `phf` (check `Cargo.toml`), a `phf_map!` keyed on abbreviation string is likely cleanest. Alternatively, a flat `match` on `&str` — the compiler optimizes this well for string matching.

## Tests

### Unit tests (in `engineering.rs`)

**Basic conversions (one per category):**
- `convert(1.0, "lbm", "kg")` → ~0.453592 (weight)
- `convert(1.0, "mi", "km")` → ~1.609344 (distance)
- `convert(1.0, "day", "hr")` → 24.0 (time)
- `convert(1.0, "atm", "Pa")` → 101325.0 (pressure)
- `convert(1.0, "lbf", "N")` → ~4.448222 (force)
- `convert(1.0, "BTU", "J")` → ~1055.056 (energy)
- `convert(1.0, "HP", "W")` → ~745.700 (power)
- `convert(1.0, "T", "ga")` → 10000.0 (magnetism)
- `convert(1.0, "gal", "l")` → ~3.785412 (volume)
- `convert(1.0, "ha", "m2")` → 10000.0 (area)
- `convert(1.0, "kn", "m/s")` → ~0.514444 (speed)
- `convert(1.0, "byte", "bit")` → 8.0 (information)

**Temperature:**
- `convert(0.0, "C", "F")` → 32.0
- `convert(100.0, "C", "F")` → 212.0
- `convert(68.0, "F", "C")` → 20.0
- `convert(0.0, "C", "K")` → 273.15
- `convert(0.0, "K", "C")` → -273.15
- `convert(0.0, "C", "Rank")` → 491.67
- `convert(0.0, "C", "Reau")` → 0.0
- `convert(80.0, "Reau", "C")` → 100.0

**SI prefixes:**
- `convert(1.0, "m", "km")` → 0.001
- `convert(1.0, "km", "m")` → 1000.0
- `convert(1.0, "kg", "g")` → 1000.0
- `convert(1.0, "mg", "g")` → 0.001
- `convert(1.0, "MW", "W")` → 1000000.0

**Binary prefixes:**
- `convert(1024.0, "byte", "kbyte")` → 1.0
- `convert(1.0, "Mibyte", "byte")` → 1048576.0
- `convert(1.0, "Gibit", "bit")` → 1073741824.0

**Same unit:**
- `convert(42.0, "m", "m")` → 42.0

**Unit aliases:**
- `convert(1.0, "l", "lt")` → 1.0 (liter aliases)
- `convert(1.0, "L", "l")` → 1.0

**Cross-category error:**
- `convert(1.0, "kg", "m")` → `#N/A`
- `convert(1.0, "C", "m")` → `#N/A`

**Unknown unit:**
- `convert(1.0, "xyz", "m")` → `#N/A`
- `convert(1.0, "m", "xyz")` → `#N/A`

**Temperature with SI prefix rejected:**
- `convert(1.0, "kC", "F")` → `#N/A`

**Arity / type errors:**
- 2 args → `#VALUE!`
- 4 args → `#VALUE!`
- Non-numeric first arg: `convert("abc", "m", "km")` → `#VALUE!`
- Error propagation: `convert(#N/A, "m", "km")` → `#N/A`

**Coercion:**
- `convert(TRUE, "m", "km")` → 0.001 (TRUE = 1.0)
- `convert("100", "km", "mi")` → ~62.137 (text coerced to number)

### Conformance fixture

Create `tests/fixtures/engineering/convert.xlsx`.

**Sheet1 data:**
- A: "Number" header, rows 2-10: `1, 100, 68, 0, 1024, 42, 1, 0, -40`
- B: "From" header, rows 2-10: `lbm, km, F, C, byte, m, HP, K, C`
- C: "To" header, rows 2-10: `kg, mi, C, F, kbyte, m, W, C, F`
- D: "Error" header, row 2: `=NA()`
- E: "Text" header, rows 2-3: `"xyz"`, `"100"`

**Sheet2 data:**
- A: "Val" header, rows 2-3: `1, "m"`

**Formulas (column F, starting row 2) — 35 formulas:**

Basic conversions (6):
1. `=CONVERT(A2, B2, C2)` → ~0.453592 (1 lbm → kg)
2. `=CONVERT(A3, B3, C3)` → ~62.137119 (100 km → mi)
3. `=CONVERT(A4, B4, C4)` → 20 (68 F → C)
4. `=CONVERT(A5, B5, C5)` → 32 (0 C → F)
5. `=CONVERT(A6, B6, C6)` → 1 (1024 byte → kbyte)
6. `=CONVERT(A7, B7, C7)` → 42 (42 m → m, same unit)

Temperature (5):
7. `=CONVERT(100, "C", "F")` → 212
8. `=CONVERT(0, "C", "K")` → 273.15
9. `=CONVERT(0, "K", "C")` → -273.15
10. `=CONVERT(A10, "C", "F")` → -40 (-40 C = -40 F)
11. `=CONVERT(80, "Reau", "C")` → 100

SI prefixes (5):
12. `=CONVERT(1, "m", "km")` → 0.001
13. `=CONVERT(1, "km", "m")` → 1000
14. `=CONVERT(1, "kg", "g")` → 1000
15. `=CONVERT(1, "mg", "g")` → 0.001
16. `=CONVERT(1, "MW", "W")` → 1000000

Binary prefixes (2):
17. `=CONVERT(1, "Mibyte", "byte")` → 1048576
18. `=CONVERT(1, "Gibit", "bit")` → 1073741824

Weight (2):
19. `=CONVERT(1, "stone", "lbm")` → ~14.0 (6.35029318/0.45359237)
20. `=CONVERT(1, "ozm", "g")` → ~28.349523

Distance (2):
21. `=CONVERT(1, "Nmi", "m")` → 1852
22. `=CONVERT(1, "in", "cm")` → 2.54

Time (1):
23. `=CONVERT(1, "yr", "day")` → 365.25

Pressure / force / energy (2):
24. `=CONVERT(1, "atm", "psi")` → ~14.69595 (101325/6894.757...)
25. `=CONVERT(1, "BTU", "J")` → ~1055.056

Power / magnetism (2):
26. `=CONVERT(A8, B8, C8)` → ~745.700 (1 HP → W)
27. `=CONVERT(1, "T", "ga")` → 10000

Cross-category error (2):
28. `=CONVERT(1, "kg", "m")` → `#N/A`
29. `=CONVERT(1, "C", "m")` → `#N/A`

Unknown unit (1):
30. `=CONVERT(1, E2, "m")` → `#N/A` ("xyz" unknown)

Error propagation (1):
31. `=CONVERT(D2, "m", "km")` → `#N/A`

Coercion (1):
32. `=CONVERT(E3, "km", "mi")` → ~62.137119 ("100" coerced to 100)

Volume (1):
33. `=CONVERT(1, "gal", "l")` → ~3.785412

Area / speed (2):
34. `=CONVERT(1, "ha", "ar")` → 100
35. `=CONVERT(1, "mph", "m/s")` → ~0.44704

Nested (1):
36. `=IF(CONVERT(1, "kg", "lbm")>2, "heavy", "light")` → "light"

Cross-sheet (1):
37. `=CONVERT(Sheet2!A2, "m", "ft")` → ~3.28084 (1 m → ft)

CONVERT does NOT need `_xlfn.` prefix.

**Fixture workflow:**
1. Generate with openpyxl
2. Recalculate with LibreOffice headless
3. Add `#[test] fn convert()` in `conformance/engineering.rs`

## Docs to update (same PR)

| File | Change |
|---|---|
| `CHANGELOG.md` | Add "CONVERT unit conversion (~100 units, SI/binary prefixes, 13 categories)" under `[Unreleased]` → `### Added` |
| `docs/functions.md` | Change CONVERT from `.` to `x` |
| `docs/roadmap/v0.3/README.md` | Tick the CONVERT checkbox |

## Streaming invariant

Does not violate. CONVERT is a pure scalar function of its three arguments — no cross-row reads, no prelude dependency. The unit table is static compile-time data.
