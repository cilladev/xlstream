# v0.3 Roadmap

**Status:** in progress
**Target:** 2026 Q3
**Theme:** statistical + engineering functions, infrastructure improvements

## Carried from v0.2

These didn't ship with v0.2:

- [ ] **MID empty string workaround** — `rust_xlsxwriter` drops empty string writes. Patch upstream or work around. ~1 hour.
- [ ] **Memory optimization** — investigate calamine shared-strings buffering and rust_xlsxwriter string table. Target: < 250 MB for 100k-row workbook (currently 643 MB).

## Infrastructure

- [ ] **Auto-detect iterative calc settings** — parse `calcPr` from `xl/workbook.xml` in xlstream-io (#77). ~2 hours.
- [ ] **Cross-column same-row circular references** — SCC detection in topo sort, iterate groups (#80). ~1 day.
- [x] **ROW / COLUMN / ROWS / COLUMNS** — ROW/COLUMN return row/col number of a cell; ROWS/COLUMNS return row/col count of a range. ROWS/COLUMNS carried from v0.2 (implemented on branch but never merged). ~1 day.

## Statistical functions (~30)

All row-local. Pure math — no streaming concerns. Implement in `builtins/statistical.rs`.

### Descriptive statistics
- [x] **STDEV.S / STDEV.P** — sample and population standard deviation. ~0.5 day.
- [x] **VAR.S / VAR.P** — sample and population variance. ~0.5 day.
- [x] **AVEDEV** — average absolute deviation. ~2 hours.
- [x] **SKEW / SKEW.P** — skewness. ~2 hours.
- [x] **KURT** — kurtosis. ~2 hours.

### Ranking / percentiles
- [x] **PERCENTILE.INC / PERCENTILE.EXC** — percentile (inclusive/exclusive). ~0.5 day.
- [x] **QUARTILE.INC / QUARTILE.EXC** — quartile. ~2 hours.
- [x] **RANK.EQ / RANK.AVG** — rank in a list. ~0.5 day.
- [x] **LARGE / SMALL** — k-th largest/smallest. ~2 hours.
- [x] **MODE.SNGL** — most frequent value. ~2 hours.

### Distributions
- [x] **NORM.DIST / NORM.INV** — normal distribution CDF and inverse. ~0.5 day.
- [x] **NORM.S.DIST / NORM.S.INV** — standard normal. ~2 hours.
- [x] **T.DIST / T.INV / T.DIST.2T / T.DIST.RT / T.INV.2T** — Student's t-distribution. ~0.5 day.
- [x] **BINOM.DIST / BINOM.INV** — binomial distribution. ~0.5 day.
- [x] **POISSON.DIST** — Poisson distribution. ~2 hours.
- [x] **EXPON.DIST** — exponential distribution. ~2 hours.

### Regression / correlation
- [x] **CORREL** — Pearson correlation coefficient. ~2 hours.
- [ ] **COVARIANCE.P / COVARIANCE.S** — covariance. ~2 hours.
- [ ] **SLOPE / INTERCEPT / RSQ** — linear regression stats. ~0.5 day.
- [ ] **FORECAST.LINEAR** — predict Y from X via linear regression. ~2 hours.

### Combinatorics
- [ ] **PERMUT / PERMUTA** — permutations. ~2 hours.
- [ ] **COMBIN / COMBINA** — combinations. ~2 hours.
- [ ] **FACT / FACTDOUBLE** — factorial. ~2 hours.

## Engineering functions (~15)

All row-local. Implement in `builtins/engineering.rs`.

### Base conversion
- [ ] **HEX2DEC / DEC2HEX** — hex to decimal and back. ~2 hours.
- [ ] **BIN2DEC / DEC2BIN** — binary to decimal and back. ~2 hours.
- [ ] **OCT2DEC / DEC2OCT** — octal to decimal and back. ~2 hours.
- [ ] **HEX2BIN / BIN2HEX / HEX2OCT / OCT2HEX** — cross-base conversions. ~2 hours.
- [ ] **BASE** — convert number to text in given base. ~2 hours.

### Complex numbers
- [ ] **COMPLEX / IMREAL / IMAGINARY** — complex number create/extract. ~0.5 day.

### Comparison / special
- [ ] **DELTA / GESTEP** — Kronecker delta, unit step. ~1 hour.
- [ ] **ERF / ERFC** — error function and complement. ~2 hours.
- [ ] **CONVERT** — unit conversion (length, weight, temperature, etc.). ~1 day.
- [ ] **BITAND / BITOR / BITXOR / BITLSHIFT / BITRSHIFT** — bitwise operations. ~2 hours.

## Math extras (~15)

Row-local additions to `builtins/math.rs`.

- [ ] **ACOSH / ASINH / ATANH** — inverse hyperbolic trig. ~1 hour.
- [ ] **COSH / SINH / TANH** — hyperbolic trig. ~1 hour.
- [ ] **COT / CSC / SEC / COTH / CSCH / SECH** — reciprocal trig. ~2 hours.
- [ ] **DEGREES / RADIANS** — angle conversion. ~1 hour.
- [ ] **EVEN / ODD** — round to even/odd integer. ~1 hour.
- [ ] **TRUNC** — truncate to integer or decimal places. ~1 hour.
- [ ] **MROUND** — round to nearest multiple. ~1 hour.
- [ ] **GCD / LCM** — greatest common divisor / least common multiple. ~1 hour.
- [ ] **CEILING.MATH / FLOOR.MATH / CEILING.PRECISE / FLOOR.PRECISE / ISO.CEILING** — rounding variants. ~0.5 day.
- [ ] **ROMAN / ARABIC** — number to Roman numeral and back. ~2 hours.
- [ ] **SUBTOTAL / AGGREGATE** — multi-mode aggregate (complex — may defer to v0.4). ~1-2 days.

## Out of scope (v0.4+)

- LET (variable binding inside formulas)
- XNPV, XIRR, NPER, SLN, DDB and advanced financial (~20 functions)
- Compatibility aliases (STDEV -> STDEV.S, VAR -> VAR.P, etc.)
- Database functions (DSUM, DAVERAGE, etc.)
- RAND/RANDBETWEEN with deterministic seeding
- `.xlsb` input support
- mdBook documentation site

## Done when

- All boxes ticked
- `make check` passes
- Conformance tests pass for all new functions
- Benchmark report generated (`make bench-report VERSION=0.3.0`)
- CHANGELOG promoted to `[0.3.0]`
- Tagged and released
