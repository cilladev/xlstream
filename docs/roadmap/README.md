# Roadmap

Each version gets its own folder with a checklist. When a version ships, its folder moves to `archive/`.

## Current version

**v0.3** — See [`v0.3/README.md`](v0.3/README.md).

## Archive

- [v0.2](archive/v0.2/README.md) — shipped 2026-05-11
- [v0.1](archive/v0.1/README.md) — shipped 2026-04-20

## Rules for agents

1. Read the current version's README before starting work.
2. One PR per checklist item.
3. Tick the box in the same PR that lands the work.
4. When all boxes are ticked, ship and archive.

---

## Big picture

### Where we are

Excel has ~516 functions. xlstream implements 106 functions — covering all 15 functions that appear in 76% of real business spreadsheets (Enron Corpus, Hermans et al., IEEE ICSE 2015). See [functions.md](../functions.md) for the full cross-reference.

### Where we're going

The goal is to support **every formula that fits the streaming model** — ~465 of Excel's ~516 functions. The ~51 we'll never support are incompatible with streaming (OFFSET, INDIRECT, dynamic arrays, LAMBDA family, network/OLAP).

### Version plan

```
v0.1  ✓  Core engine (103 functions, streaming, Python bindings)
v0.2     Coverage + fidelity (named ranges, table refs, keep-formulas)
v0.3     Statistical + engineering (STDEV, VAR, NORM.DIST, CONVERT, HEX2DEC)
v0.4     LET + financial (38 functions) + multi-format I/O (xlsm, xlsb, csv)
v0.5     Compatibility aliases (39) + database functions (12)
v1.0     API stability — no breaking changes after this
```

### v0.2 — Coverage + fidelity

Named ranges, table references, SUMPRODUCT, MINIFS/MAXIFS. Keep-formulas output mode. Self-referential formula support. See [`v0.2/README.md`](v0.2/README.md).

### v0.3 — Statistical + engineering

Excel has 109 statistical and 78 engineering functions. Most are pure math — no streaming concerns.

**Statistical (~30 high-value):** STDEV.S, STDEV.P, VAR.S, VAR.P, PERCENTILE.INC, PERCENTILE.EXC, QUARTILE, RANK.EQ, RANK.AVG, LARGE, SMALL, MODE.SNGL, NORM.DIST, NORM.INV, T.DIST, T.INV, CORREL, COVARIANCE.P, FORECAST.LINEAR, TREND, GROWTH, SLOPE, INTERCEPT, RSQ, PERMUT, COMBIN, FACT

**Engineering (~15 high-value):** CONVERT, HEX2DEC, DEC2HEX, HEX2BIN, BIN2DEC, BIN2HEX, OCT2DEC, DEC2OCT, COMPLEX, IMREAL, IMAGINARY, DELTA, GESTEP, ERF, ERFC

All row-local (pure functions of their args). No streaming concerns. Mostly math formulas from reference implementations.

### v0.4 — LET + financial + multi-format I/O

**LET** is scoped variable binding: `=LET(x, A2*1.1, y, B2*0.9, IF(x>y, x, y))`. No spill, no closures, no recursion — just name substitution. Compatible with streaming.

**Financial (37 functions):** XNPV, XIRR, NPER, SLN, DDB, DB, EFFECT, NOMINAL, CUMIPMT, CUMPRINC, PPMT, IPMT, DISC, PRICE, YIELD, DURATION, MDURATION, ACCRINT, TBILLEQ, TBILLPRICE, VDB, MIRR, FVSCHEDULE, DOLLARDE, DOLLARFR, PDURATION, RRI, and more. All row-local. Some iterative (XIRR uses Newton's method).

**Input formats:** .xlsm (free — same reader as xlsx), .xltx/.xltm/.xlam (free), .xlsb (calamine has streaming reader). **Output formats:** .csv (data extraction), .xlsm (macro passthrough). See [`v0.4/README.md`](v0.4/README.md).

### v0.5 — Compatibility + database

**Compatibility (39 aliases):** Old function names Excel keeps for backward compat. STDEV → STDEV.S, VAR → VAR.P, MODE → MODE.SNGL, PERCENTILE → PERCENTILE.INC, RANK → RANK.EQ, etc. Thin dispatch aliases to v0.3 implementations. No new logic.

**Database (12 functions):** DSUM, DAVERAGE, DCOUNT, DCOUNTA, DGET, DMAX, DMIN, DPRODUCT, DSTDEV, DSTDEVP, DVAR, DVARP. Prelude aggregates with structured criteria parsing. See [`v0.5/README.md`](v0.5/README.md).

### v1.0 — API stability

No new functions. Focus:
- API freeze — `evaluate()` signature, `EvaluateOptions`, `OutputMode`, Python bindings, CLI locked
- Performance hardening — memory < 250 MB, wall-clock < 15s for 100k rows
- Documentation — mdBook site, migration guide, per-crate README audit
- Quality — 100% conformance coverage, fuzz testing, property-based testing
- 1-year semver stability commitment

See [`v1.0/README.md`](v1.0/README.md).

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

**~51 functions permanently excluded.** ~434 are streaming-compatible and planned through v0.5. See [functions.md](../functions.md) for the complete breakdown.

### Function count trajectory

| Version | New | Cumulative | Operators | Total surfaces |
|---|---|---|---|---|
| v0.1 | 103 | 103 | 13 | 116 |
| v0.2 | +3 | 106 | 13 | 119 |
| v0.3 | +239 | 345 | 13 | 358 |
| v0.4 | +38 | 383 | 13 | 396 |
| v0.5 | +51 | 434 | 13 | 447 |
| v1.0 | 0 | 434 | 13 | 447 (API frozen) |
