# v0.4 Roadmap

**Status:** planning
**Target:** 2026 Q4
**Theme:** LET variable binding, v0.3 carry-over functions, CSV output

## Cleanup
- [x] Formula registry clean up
- [x] EvalMode dispatch — replace ~134 eager/aggregate handler wrappers in builtins/mod.rs with a 3-variant enum (Eager/Aggregate/Custom) in registry. Keep ~34 Custom wrappers as glue layer. ~0.5 day.

# Bug fixes
- [] Fix all outstanding issues

## LET

- [ ] **LET** — scoped variable binding inside formulas. `=LET(x, A2*1.1, y, B2*0.9, IF(x>y, x, y))`. No spill, no closures, no recursion — just name substitution evaluated left-to-right. ~2 days.


## Carried over from v0.3 (135)

Planned in [`functions.md`](../../functions.md) with a v0.3 target but never
scheduled in the v0.3 checklist, so they slipped. One PR per checkbox.

### Math & trig — carried from v0.3 (12)

- [ ] **ACOT** — inverse cotangent. ~1 hour.
- [ ] **ACOTH** — inverse hyperbolic cotangent. ~1 hour.
- [ ] **DECIMAL** — parse text in a given radix (2-36) to number. ~2 hours.
- [ ] **MDETERM** — matrix determinant of a square range (scalar result). ~0.5 day.
- [ ] **MULTINOMIAL** — multinomial coefficient of the args. ~1 hour.
- [ ] **QUOTIENT** — integer division, truncated toward zero. ~1 hour.
- [ ] **SERIESSUM** — power series sum. ~1 hour.
- [ ] **SQRTPI** — square root of x*pi. ~1 hour.
- [ ] **SUMSQ** — sum of squares; whole-column form needs a prelude fold. ~0.5 day.
- [ ] **SUMX2MY2** — sum of x^2 - y^2 over paired ranges. ~2 hours.
- [ ] **SUMX2PY2** — sum of x^2 + y^2 over paired ranges. ~2 hours.
- [ ] **SUMXMY2** — sum of (x - y)^2 over paired ranges. ~2 hours.

### Statistical — distributions, carried from v0.3 (33)

First function per family carries the special-function cost (incomplete gamma/beta); the rest reuse it.

- [ ] **BETA.DIST** — beta distribution PDF/CDF; brings incomplete beta. ~0.5 day.
- [ ] **BETA.INV** — inverse beta CDF. ~2 hours.
- [ ] **BINOM.DIST.RANGE** — binomial probability over a trial range. ~1 hour.
- [ ] **CHISQ.DIST** — chi-squared PDF/CDF; brings incomplete gamma. ~0.5 day.
- [ ] **CHISQ.DIST.RT** — right-tailed chi-squared CDF. ~1 hour.
- [ ] **CHISQ.INV** — inverse chi-squared CDF. ~2 hours.
- [ ] **CHISQ.INV.RT** — inverse right-tailed chi-squared CDF. ~1 hour.
- [ ] **CHISQ.TEST** — chi-squared independence test over two ranges. ~2 hours.
- [ ] **CONFIDENCE.NORM** — confidence interval, normal distribution. ~1 hour.
- [ ] **CONFIDENCE.T** — confidence interval, t-distribution. ~1 hour.
- [ ] **F.DIST** — F distribution PDF/CDF. ~2 hours.
- [ ] **F.DIST.RT** — right-tailed F CDF. ~1 hour.
- [ ] **F.INV** — inverse F CDF. ~2 hours.
- [ ] **F.INV.RT** — inverse right-tailed F CDF. ~1 hour.
- [ ] **F.TEST** — F-test two-sample variance p-value. ~2 hours.
- [ ] **FISHER** — Fisher transformation. ~1 hour.
- [ ] **FISHERINV** — inverse Fisher transformation. ~1 hour.
- [ ] **GAMMA** — gamma function. ~1 hour.
- [ ] **GAMMA.DIST** — gamma distribution PDF/CDF. ~2 hours.
- [ ] **GAMMA.INV** — inverse gamma CDF. ~2 hours.
- [ ] **GAMMALN** — natural log of gamma. ~1 hour.
- [ ] **GAMMALN.PRECISE** — alias-precision variant of GAMMALN. ~1 hour.
- [ ] **GAUSS** — standard normal CDF minus 0.5. ~1 hour.
- [ ] **HYPGEOM.DIST** — hypergeometric PMF/CDF. ~2 hours.
- [ ] **LOGNORM.DIST** — lognormal PDF/CDF. ~1 hour.
- [ ] **LOGNORM.INV** — inverse lognormal CDF. ~1 hour.
- [ ] **NEGBINOM.DIST** — negative binomial PMF/CDF. ~1 hour.
- [ ] **PHI** — standard normal PDF. ~1 hour.
- [ ] **PROB** — probability of range values within bounds. ~2 hours.
- [ ] **STANDARDIZE** — z-score normalization. ~1 hour.
- [ ] **T.TEST** — Student's t-test p-value over two ranges. ~2 hours.
- [ ] **WEIBULL.DIST** — Weibull PDF/CDF. ~1 hour.
- [ ] **Z.TEST** — one-sample z-test p-value. ~2 hours.

### Statistical — aggregates & ranking, carried from v0.3 (15)

Whole-column forms ride the existing prelude fold machinery; several need new fold kinds.

- [ ] **AVERAGEA** — average counting text/FALSE as 0, TRUE as 1. ~2 hours.
- [ ] **DEVSQ** — sum of squared deviations from the mean. ~2 hours.
- [ ] **GEOMEAN** — geometric mean. ~2 hours.
- [ ] **HARMEAN** — harmonic mean. ~2 hours.
- [ ] **MAXA** — max counting text/logicals. ~1 hour.
- [ ] **MINA** — min counting text/logicals. ~1 hour.
- [ ] **PEARSON** — Pearson correlation (alias-level twin of CORREL). ~1 hour.
- [ ] **PERCENTRANK.EXC** — exclusive percent rank of a value. ~2 hours.
- [ ] **PERCENTRANK.INC** — inclusive percent rank of a value. ~2 hours.
- [ ] **STDEVA** — stdev counting text/logicals. ~1 hour.
- [ ] **STDEVPA** — population stdev counting text/logicals. ~1 hour.
- [ ] **STEYX** — standard error of predicted y in regression. ~2 hours.
- [ ] **TRIMMEAN** — mean excluding top/bottom fraction. ~2 hours.
- [ ] **VARA** — variance counting text/logicals. ~1 hour.
- [ ] **VARPA** — population variance counting text/logicals. ~1 hour.

### Statistical & matrix — array-returning, carried from v0.3 (9)

These return arrays in Excel. Decide per function before implementing: scalar first-cell semantics, or refuse at classification with a doc link. No spill machinery — that stays permanently excluded.

- [ ] **FREQUENCY** — bin counts over a data range. decision + ~0.5 day.
- [ ] **GROWTH** — exponential trend prediction. decision + ~0.5 day.
- [ ] **LINEST** — linear regression coefficients. decision + ~0.5 day.
- [ ] **LOGEST** — exponential regression coefficients. decision + ~0.5 day.
- [ ] **MODE.MULT** — all modes of a range. decision + ~2 hours.
- [ ] **TREND** — linear trend prediction. decision + ~0.5 day.
- [ ] **MINVERSE** — matrix inverse. decision + ~0.5 day.
- [ ] **MMULT** — matrix product. decision + ~0.5 day.
- [ ] **MUNIT** — identity matrix. decision + ~1 hour.

### Text — carried from v0.3 (19)

B-variants use byte positions in DBCS locales; in single-byte contexts they behave as their non-B twins — implement as aliases with that documented.

- [ ] **CHAR** — character from ANSI code. ~1 hour.
- [ ] **CODE** — ANSI code of first character. ~1 hour.
- [ ] **DOLLAR** — format number as currency text. ~2 hours.
- [ ] **FINDB** — byte-variant of FIND. ~1 hour.
- [ ] **FIXED** — format number with fixed decimals as text. ~2 hours.
- [ ] **LEFTB** — byte-variant of LEFT. ~1 hour.
- [ ] **LENB** — byte-variant of LEN. ~1 hour.
- [ ] **MIDB** — byte-variant of MID. ~1 hour.
- [ ] **NUMBERVALUE** — locale-aware text to number with explicit separators. ~0.5 day.
- [ ] **REPLACEB** — byte-variant of REPLACE. ~1 hour.
- [ ] **REPT** — repeat text n times. ~1 hour.
- [ ] **RIGHTB** — byte-variant of RIGHT. ~1 hour.
- [ ] **SEARCHB** — byte-variant of SEARCH. ~1 hour.
- [ ] **T** — return text arg, empty for non-text. ~1 hour.
- [ ] **TEXTAFTER** — text after a delimiter. ~2 hours.
- [ ] **TEXTBEFORE** — text before a delimiter. ~2 hours.
- [ ] **UNICHAR** — character from Unicode code point. ~1 hour.
- [ ] **UNICODE** — code point of first character. ~1 hour.
- [ ] **VALUETOTEXT** — value to text (concise/strict mode). ~1 hour.

### Date & time — carried from v0.3 (13)

- [ ] **DATEVALUE** — parse date text to serial; pin to locale-independent formats, ambiguous input returns #VALUE!. ~0.5 day.
- [ ] **TIMEVALUE** — parse time text to serial fraction; shares DATEVALUE's parser. ~2 hours.
- [ ] **HOUR** — hour component of a serial. ~1 hour.
- [ ] **MINUTE** — minute component of a serial. ~1 hour.
- [ ] **SECOND** — second component of a serial. ~1 hour.
- [ ] **TIME** — build serial fraction from h/m/s. ~1 hour.
- [ ] **DAYS** — days between two dates. ~1 hour.
- [ ] **DAYS360** — days between dates on a 360-day year (US/EU methods). ~2 hours.
- [ ] **ISOWEEKNUM** — ISO 8601 week number. ~1 hour.
- [ ] **WEEKNUM** — week number with return-type variants. ~2 hours.
- [ ] **NETWORKDAYS.INTL** — workdays between dates with weekend mask. ~0.5 day.
- [ ] **WORKDAY.INTL** — date offset by workdays with weekend mask. ~0.5 day.
- [ ] **YEARFRAC** — year fraction between dates (5 day-count bases). ~0.5 day.

### Lookup & information — carried from v0.3 (7)

- [ ] **ADDRESS** — build cell reference text from row/col numbers. ~2 hours.
- [ ] **LOOKUP** — legacy vector/array lookup. ~0.5 day.
- [ ] **ERROR.TYPE** — numeric code of an error value. ~1 hour.
- [ ] **ISERR** — TRUE for any error except #N/A. ~1 hour.
- [ ] **ISEVEN** — TRUE for even integers. ~1 hour.
- [ ] **ISODD** — TRUE for odd integers. ~1 hour.
- [ ] **N** — number passthrough; dates to serial, text to 0. ~1 hour.

### Engineering — complex & Bessel, carried from v0.3 (27)

Complex parse/format helpers shipped in v0.3 (COMPLEX, IMREAL, IMAGINARY) — the IM* batch reuses them.

- [ ] **IMABS** — modulus of a complex number. ~1 hour.
- [ ] **IMARGUMENT** — argument (angle) of a complex number. ~1 hour.
- [ ] **IMCONJUGATE** — complex conjugate. ~1 hour.
- [ ] **IMCOS** — complex cosine. ~1 hour.
- [ ] **IMCOSH** — complex hyperbolic cosine. ~1 hour.
- [ ] **IMCOT** — complex cotangent. ~1 hour.
- [ ] **IMCSC** — complex cosecant. ~1 hour.
- [ ] **IMCSCH** — complex hyperbolic cosecant. ~1 hour.
- [ ] **IMDIV** — complex division. ~1 hour.
- [ ] **IMEXP** — complex exponential. ~1 hour.
- [ ] **IMLN** — complex natural log. ~1 hour.
- [ ] **IMLOG10** — complex base-10 log. ~1 hour.
- [ ] **IMLOG2** — complex base-2 log. ~1 hour.
- [ ] **IMPOWER** — complex number to a real power. ~1 hour.
- [ ] **IMPRODUCT** — product of complex numbers. ~1 hour.
- [ ] **IMSEC** — complex secant. ~1 hour.
- [ ] **IMSECH** — complex hyperbolic secant. ~1 hour.
- [ ] **IMSIN** — complex sine. ~1 hour.
- [ ] **IMSINH** — complex hyperbolic sine. ~1 hour.
- [ ] **IMSQRT** — complex square root. ~1 hour.
- [ ] **IMSUB** — complex subtraction. ~1 hour.
- [ ] **IMSUM** — sum of complex numbers. ~1 hour.
- [ ] **IMTAN** — complex tangent. ~1 hour.
- [ ] **BESSELI** — modified Bessel function I_n(x). ~0.5 day.
- [ ] **BESSELJ** — Bessel function J_n(x). ~0.5 day.
- [ ] **BESSELK** — modified Bessel function K_n(x). ~0.5 day.
- [ ] **BESSELY** — Bessel function Y_n(x). ~0.5 day.

## Output format support

- [ ] **CSV output** — `--output-format csv` flag. Bypass rust_xlsxwriter, write computed values row-by-row via `csv::Writer`. No formulas, no formatting — pure data extraction. Add `csv` crate dependency. ~0.5 day.

## Out of scope (v0.5+)

- Compatibility aliases (STDEV -> STDEV.S, etc.)
- Database functions (DSUM, DAVERAGE, etc.)
- RAND/RANDBETWEEN with deterministic seeding
- mdBook documentation site

## Done when

- All boxes ticked
- `make check` passes
- Conformance tests pass for all new functions
- Benchmark report generated (`make bench-report VERSION=0.4.0`)
- CHANGELOG promoted to `[0.4.0]`
- Tagged and released
