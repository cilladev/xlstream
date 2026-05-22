# v0.4 Roadmap

**Status:** planning
**Target:** 2026 Q4
**Theme:** LET variable binding, advanced financial functions, multi-format I/O

## Cleanup
- [] Formula registry clean up

## LET

- [ ] **LET** — scoped variable binding inside formulas. `=LET(x, A2*1.1, y, B2*0.9, IF(x>y, x, y))`. No spill, no closures, no recursion — just name substitution evaluated left-to-right. ~2 days.

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

- [ ] **CSV output** — `--output-format csv` flag. Bypass rust_xlsxwriter, write computed values row-by-row via `csv::Writer`. No formulas, no formatting — pure data extraction. Add `csv` crate dependency. ~0.5 day.
- [ ] **XLSM output** — when input is .xlsm, copy `vbaProject.bin` from input zip to output via `rust_xlsxwriter::add_vba_project()`. Preserves macros alongside recalculated formulas. ~0.5 day.

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
