# v0.5 Roadmap

**Status:** planning
**Target:** 2027 Q1
**Theme:** compatibility aliases + database functions + financial + multi-format I/O

## Compatibility aliases (~39)

Old function names that Excel keeps for backward compatibility. Each is a thin dispatch alias to the modern equivalent implemented in v0.3. No new logic — just match arms in `dispatch()`.

### Statistical aliases (~39)

- [ ] **STDEV** → STDEV.S
- [ ] **STDEVP** → STDEV.P
- [ ] **VAR** → VAR.S
- [ ] **VARP** → VAR.P
- [ ] **MODE** → MODE.SNGL
- [ ] **PERCENTILE** → PERCENTILE.INC
- [ ] **PERCENTRANK** → PERCENTRANK.INC
- [ ] **QUARTILE** → QUARTILE.INC
- [ ] **RANK** → RANK.EQ
- [ ] **BETADIST** → BETA.DIST
- [ ] **BETAINV** → BETA.INV
- [ ] **BINOMDIST** → BINOM.DIST
- [ ] **CRITBINOM** → BINOM.INV
- [ ] **CHIDIST** → CHISQ.DIST.RT
- [ ] **CHIINV** → CHISQ.INV.RT
- [ ] **CHITEST** → CHISQ.TEST
- [ ] **CONFIDENCE** → CONFIDENCE.NORM
- [ ] **COVAR** → COVARIANCE.P
- [ ] **EXPONDIST** → EXPON.DIST
- [ ] **FDIST** → F.DIST.RT
- [ ] **FINV** → F.INV.RT
- [ ] **FORECAST** → FORECAST.LINEAR
- [ ] **FTEST** → F.TEST
- [ ] **GAMMADIST** → GAMMA.DIST
- [ ] **GAMMAINV** → GAMMA.INV
- [ ] **HYPGEOMDIST** → HYPGEOM.DIST
- [ ] **LOGINV** → LOGNORM.INV
- [ ] **LOGNORMDIST** → LOGNORM.DIST
- [ ] **NEGBINOMDIST** → NEGBINOM.DIST
- [ ] **NORMDIST** → NORM.DIST
- [ ] **NORMINV** → NORM.INV
- [ ] **NORMSDIST** → NORM.S.DIST
- [ ] **NORMSINV** → NORM.S.INV
- [ ] **POISSON** → POISSON.DIST
- [ ] **TDIST** → T.DIST.2T
- [ ] **TINV** → T.INV.2T
- [ ] **TTEST** → T.TEST
- [ ] **WEIBULL** → WEIBULL.DIST
- [ ] **ZTEST** → Z.TEST

## Database functions (~12)

Range-based aggregates with structured criteria ranges. Implemented as prelude aggregates with criteria parsing.

All take `(database, field, criteria)` where:
- `database` is a range with headers in the first row
- `field` is a column name or index
- `criteria` is a range with headers matching database columns and criteria values below

- [ ] **DSUM** — sum of values matching criteria. ~0.5 day.
- [ ] **DAVERAGE** — average of values matching criteria. ~2 hours.
- [ ] **DCOUNT** — count of numeric values matching criteria. ~2 hours.
- [ ] **DCOUNTA** — count of non-blank values matching criteria. ~2 hours.
- [ ] **DGET** — single value matching criteria (error if multiple). ~2 hours.
- [ ] **DMAX** — max of values matching criteria. ~2 hours.
- [ ] **DMIN** — min of values matching criteria. ~2 hours.
- [ ] **DPRODUCT** — product of values matching criteria. ~2 hours.
- [ ] **DSTDEV** — sample standard deviation matching criteria. ~2 hours.
- [ ] **DSTDEVP** — population standard deviation matching criteria. ~2 hours.
- [ ] **DVAR** — sample variance matching criteria. ~2 hours.
- [ ] **DVARP** — population variance matching criteria. ~2 hours.

### Streaming model for database functions

Database functions reference a "database" range (prelude-loaded) and a "criteria" range (also prelude-loaded). The field and criteria are resolved during prelude. At row-eval time, the result is a pre-computed scalar — same pattern as SUMIF/COUNTIF. No streaming invariant concerns.

Implementation plan:
1. Parse the criteria range structure (headers + values)
2. Build criteria predicates from the values
3. Scan the database range during prelude, matching rows against predicates
4. Store the aggregate result as a prelude scalar
5. At row-eval time, return the pre-computed value

## Out of scope (v1.0)

- New function implementations (all shipped by v0.5)
- API stability commitment
- Performance hardening
- mdBook documentation site

## Done when

- All boxes ticked
- `make check` passes
- Conformance tests pass for all new functions
- Benchmark report generated (`make bench-report VERSION=0.5.0`)
- CHANGELOG promoted to `[0.5.0]`
- Tagged and released

## Financial — Core (~10)

High-value functions that appear in real business workbooks.

- [ ] **NPER** — number of periods for an investment. ~2 hours.
- [ ] **IPMT** — interest portion of a payment. ~2 hours.
- [ ] **PPMT** — principal portion of a payment. ~2 hours.
- [ ] **CUMIPMT** — cumulative interest between two periods. ~2 hours.
- [ ] **CUMPRINC** — cumulative principal between two periods. ~2 hours.
- [ ] **EFFECT** — effective annual interest rate from nominal. ~1 hour.
- [ ] **NOMINAL** — nominal rate from effective annual rate. ~1 hour.
- [ ] **XNPV** — net present value with irregular dates. ~0.5 day.
- [ ] **XIRR** — internal rate of return with irregular dates. Iterative (Newton's method, same pattern as IRR). ~0.5 day.
- [ ] **MIRR** — modified internal rate of return. ~2 hours.

## Financial — Depreciation (~5)

- [ ] **SLN** — straight-line depreciation. ~1 hour.
- [ ] **SYD** — sum-of-years-digits depreciation. ~1 hour.
- [ ] **DB** — fixed-declining balance depreciation. ~2 hours.
- [ ] **DDB** — double-declining balance depreciation. ~2 hours.
- [ ] **VDB** — variable declining balance (flexible DDB). ~0.5 day.

## Financial — Bonds & Securities (~15)

- [ ] **PRICE** — price of a security paying periodic interest. ~0.5 day.
- [ ] **YIELD** — yield of a security paying periodic interest. Iterative. ~0.5 day.
- [ ] **DURATION** — Macaulay duration. ~0.5 day.
- [ ] **MDURATION** — modified duration. ~2 hours.
- [ ] **DISC** — discount rate for a security. ~2 hours.
- [ ] **PRICEDISC** — price of a discounted security. ~2 hours.
- [ ] **PRICEMAT** — price of a security paying interest at maturity. ~2 hours.
- [ ] **YIELDDISC** — yield of a discounted security. ~2 hours.
- [ ] **YIELDMAT** — yield of a security paying interest at maturity. ~2 hours.
- [ ] **RECEIVED** — amount received at maturity for a fully invested security. ~2 hours.
- [ ] **INTRATE** — interest rate for a fully invested security. ~2 hours.
- [ ] **ACCRINT** — accrued interest for a periodic-coupon security. ~0.5 day.
- [ ] **ACCRINTM** — accrued interest for a maturity-paying security. ~2 hours.
- [ ] **TBILLEQ** — bond-equivalent yield for a T-bill. ~1 hour.
- [ ] **TBILLPRICE** — price per $100 face value of a T-bill. ~1 hour.
- [ ] **TBILLYIELD** — yield for a T-bill. ~1 hour.

## Financial — Other (~6)

- [ ] **DOLLARDE** — dollar price as decimal from fraction. ~1 hour.
- [ ] **DOLLARFR** — dollar price as fraction from decimal. ~1 hour.
- [ ] **FVSCHEDULE** — future value with variable rates. ~2 hours.
- [ ] **ISPMT** — interest for a specific period (straight-line). ~1 hour.
- [ ] **PDURATION** — periods required for investment to reach a value. ~1 hour.
- [ ] **RRI** — equivalent interest rate for growth of an investment. ~1 hour.

## Input format support

- [ ] **Accept .xlsm** — macro-enabled workbooks. Calamine already reads via the xlsx code path. Accept the extension, ignore VBA macros. ~10 lines.
- [ ] **Accept .xltx / .xltm / .xlam** — templates and add-ins. Same as xlsm — calamine reads them as xlsx. Accept extensions. ~10 lines.
- [ ] **Accept .xlsb** — binary xlsx. Calamine has a streaming reader (`XlsbCellsReader`) with `next_cell()` and `next_formula()`. Wire into `xlstream-io::Reader`. Note: no `load_tables()` for xlsb — table references will error. Output is always xlsx (format conversion). ~1-2 days.

## Output format support

- [ ] **XLSM output** — when input is .xlsm, copy `vbaProject.bin` from input zip to output via `rust_xlsxwriter::add_vba_project()`. Preserves macros alongside recalculated formulas. Depends on .xlsm input acceptance above. ~0.5 day.
