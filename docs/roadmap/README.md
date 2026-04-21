# Roadmap

Each version gets its own folder with a checklist. When a version ships, its folder moves to `archive/`.

## Current version

**v0.2** — See [`v0.2/README.md`](v0.2/README.md).

## Archive

- [v0.1](archive/v0.1/README.md) — shipped 2026-04-20

## Rules for agents

1. Read the current version's README before starting work.
2. One PR per checklist item.
3. Tick the box in the same PR that lands the work.
4. When all boxes are ticked, ship and archive.

---

## Big picture

### Where we are

Excel has ~516 functions. xlstream implements 113 — but covers all 15 functions that appear in 76% of real business spreadsheets (Enron Corpus, Hermans et al., IEEE ICSE 2015). See [FUNCTIONS.md](../../FUNCTIONS.md) for the full cross-reference.

### Where we're going

The goal is to support **every formula that fits the streaming model** — ~465 of Excel's ~516 functions. The ~51 we'll never support are incompatible with streaming (OFFSET, INDIRECT, dynamic arrays, LAMBDA family, network/OLAP).

### Version plan

```
v0.1  ✓  Core engine (113 functions, streaming, Python bindings)
v0.2     Coverage + fidelity (named ranges, table refs, keep-formulas)
v0.3     Statistical + engineering (STDEV, VAR, NORM.DIST, CONVERT, HEX2DEC)
v0.4     LET + advanced financial (XNPV, XIRR, NPER, SLN, variable binding)
v0.5     Compatibility layer (39 legacy function aliases) + database functions
v1.0     API stability commitment — no breaking changes after this
```

### v0.2 — Coverage + fidelity

Named ranges, table references, SUMPRODUCT, MINIFS/MAXIFS, ROWS/COLUMNS. Keep-formulas output mode. See [`v0.2/README.md`](v0.2/README.md).

### v0.3 — Statistical + engineering

Excel has 109 statistical and 78 engineering functions. Most are pure math — no streaming concerns.

**Statistical (~30 high-value):** STDEV.S, STDEV.P, VAR.S, VAR.P, PERCENTILE.INC, PERCENTILE.EXC, QUARTILE, RANK.EQ, RANK.AVG, LARGE, SMALL, MODE.SNGL, NORM.DIST, NORM.INV, T.DIST, T.INV, CORREL, COVARIANCE.P, FORECAST.LINEAR, TREND, GROWTH, SLOPE, INTERCEPT, RSQ, PERMUT, COMBIN, FACT

**Engineering (~15 high-value):** CONVERT, HEX2DEC, DEC2HEX, HEX2BIN, BIN2DEC, BIN2HEX, OCT2DEC, DEC2OCT, COMPLEX, IMREAL, IMAGINARY, DELTA, GESTEP, ERF, ERFC

All row-local (pure functions of their args). No streaming concerns. Mostly math formulas from reference implementations.

### v0.4 — LET + advanced financial

**LET** is variable binding inside formulas: `=LET(x, A2*1.1, y, B2*0.9, IF(x>y, x, y))`. No spill, no closures, no recursion — just scoped name substitution. Compatible with streaming.

**Advanced financial (~20):** XNPV, XIRR, NPER, SLN, DDB, DB, EFFECT, NOMINAL, CUMIPMT, CUMPRINC, PPMT, IPMT, DISC, PRICE, YIELD, DURATION, MDURATION, ACCRINT, TBILLEQ, TBILLPRICE

All row-local. Some iterative (XIRR uses Newton's method like IRR).

### v0.5 — Compatibility + database

**Compatibility (39 functions):** Old names that Excel keeps for backward compat. STDEV → STDEV.S, VAR → VAR.P, MODE → MODE.SNGL, PERCENTILE → PERCENTILE.INC, RANK → RANK.EQ, CONCATENATE → CONCAT, etc. Thin wrappers over v0.3/v0.4 implementations.

**Database (12 functions):** DSUM, DAVERAGE, DCOUNT, DCOUNTA, DGET, DMAX, DMIN, DPRODUCT, DSTDEV, DSTDEVP, DVAR, DVARP. Range-based with criteria ranges — can be implemented as prelude aggregates with structured criteria parsing.

### v1.0 — API stability

No new functions. Focus:
- API freeze — `evaluate()` signature, `EvalOptions`, Python bindings locked
- Full documentation (mdBook site, per-crate READMEs)
- Performance hardening (memory < 250 MB target for 700k rows)
- 1-year semver stability commitment

### What we'll never support

These are architecturally incompatible with streaming. Users get a clear `ClassificationError` with a doc link explaining why.

| Category | Functions | Why |
|---|---|---|
| **Runtime address resolution** | OFFSET, INDIRECT | Reference computed at eval time — can't pre-classify |
| **Dynamic arrays (spill)** | FILTER, UNIQUE, SORT, SORTBY, SEQUENCE, RANDARRAY | Output size unknown until full scan, spill writes to arbitrary cells |
| **Higher-order + spill** | MAP, REDUCE, SCAN, BYROW, BYCOL, MAKEARRAY | LAMBDA bodies + array output |
| **LAMBDA (full)** | LAMBDA (recursive, Name Manager-stored) | Closures, recursion, user-defined dispatch |
| **Network/side-effects** | WEBSERVICE, ENCODEURL, FILTERXML | Not pure computation |
| **OLAP** | CUBE* family (7 functions) | Requires external OLAP connection |
| **External refs** | `[Book.xlsx]Sheet1!A1` | Violates single-file model |

**~51 functions permanently excluded.** ~465 are streaming-compatible. See [FUNCTIONS.md](../../FUNCTIONS.md) for the complete breakdown.

### Function count trajectory

| Version | Functions | Operators | Total surfaces |
|---|---|---|---|
| v0.1 | 113 | 13 | 126 |
| v0.2 | ~120 | 13 | ~133 |
| v0.3 | ~265 | 13 | ~278 |
| v0.4 | ~310 | 13 | ~323 |
| v0.5 | ~360 | 13 | ~373 |
| v1.0 | ~360 | 13 | ~373 (API frozen) |
