# v0.5 Roadmap

**Status:** planning
**Target:** 2027 Q1
**Theme:** compatibility aliases + database functions

## Compatibility aliases (~39)

Old function names that Excel keeps for backward compatibility. Each is a thin dispatch alias to the modern equivalent implemented in v0.3. No new logic — just match arms in `dispatch()`.

### Statistical aliases (~30)

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
