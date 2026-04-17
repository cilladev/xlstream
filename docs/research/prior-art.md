# Prior art — Excel formula engines

Surveyed April 2026.

## Competitive table

| Name | Language | Approach | Scale tested | Funcs | Licence | Active |
|---|---|---|---|---|---|---|
| **formualizer** | Rust + PyO3/WASM | Arrow columnar + dep graph + rayon | 10k rows fast; no public 100k+ bench | 320+ | MIT/Apache | Yes (v0.5.6 Apr 2026) |
| **IronCalc** | Rust + Py/JS/WASM | Full spreadsheet engine, xlsx I/O | Not public; NLnet-funded | 100s | MIT | Yes (v0.7.1 Jan 2026) |
| **HyperFormula** | TypeScript | Headless for web grids | Grid-scale, not batch | ~400 | **GPL-3.0** ($ for commercial) | Yes |
| **pycel** | Python | AST + dep graph, lazy eval | Small workbooks | ~150 | GPL-3.0 | Low |
| **xlcalculator** | Python | AST interp, modernised koala2 | Small-medium | ~100 | MIT | Moderate |
| **formulas** (vinci1it2000) | Python | Schedula graph | Medium, correctness focus | 200+ | EUPL | Moderate |
| **koala2** | Python | Graph compiler (predecessor to xlcalculator) | Superseded | ~80 | GPL-3.0 | Dead |
| **xlformula_engine** (crate) | Rust | Small utility | Toy | <50 | MIT | Stale |
| **Excel** (MS) | C++ | Smart recalc, formula groups, parallel | Industry bar | 505+ | Proprietary | — |
| **LibreOffice Calc** | C++ | FormulaGroup batching, threaded | Industrial | ~500 | MPL-2.0 | Yes |

## Where each fits

- **formualizer** — Rust, high function coverage, graph-based, 11 GB RSS on 400k × 20. We reuse the parser; we replace the engine.
- **IronCalc** — new, promising, full spreadsheet including UI. pip-installable as `ironcalc`. Focused on being a LibreOffice/Excel replacement, not specifically a batch evaluator. Different niche from us.
- **HyperFormula** — GPL; unusable for commercial embedding without a paid licence. Also TS, not Python-reachable without subprocess.
- **pycel / xlcalculator / formulas / koala2** — pure Python; correct but 10–100× slower than Rust-backed engines. Fine for < 10k-row workloads.
- **Excel / LibreOffice** — full engines with decades of maturity; overkill and shell-out-heavy.

## Where xlstream fits

Nobody publishes a "100k rows × 20 formula columns batch evaluator" benchmark. That's the gap we fill.

- Streaming architecture → 50–100× less memory than graph-based.
- Hash lookups + row parallelism → 5–10× faster than formualizer on row-independent workloads.
- Narrower feature set than Excel, but covers ~90% of real business-workbook shapes.
- MIT/Apache dual, pip-installable.

## Accuracy pitfalls to watch

Every engine gets these wrong eventually. We test them early.

1. **1900 leap year bug** — Excel treats Feb 29 1900 as valid (serial 60). Preserved from Lotus 1-2-3 for compatibility. openpyxl special-cases; pandas assumes 1899-12-30 origin. Mismatch drifts pre-March-1900 dates by one day.
2. **1904 date system** — Mac-origin workbooks sometimes use 1904 epoch. Workbook-level flag.
3. **Text comparison** — Excel `"a"="A"` is TRUE (case-insensitive) except via `EXACT()`. Python `==` is case-sensitive.
4. **Boolean coercion** — `TRUE + 1` → 2 in Excel. Some engines `TypeError`.
5. **Empty vs 0 vs ""** — Excel distinguishes; `ISBLANK`, `ISNUMBER`, `COUNTA` differ subtly.
6. **Error propagation** — `#N/A` propagates through arithmetic; `IFERROR` / `IFNA` short-circuit.
7. **IEEE 754 vs 15-digit display** — Excel rounds at display layer; `=0.1+0.2=0.3` is TRUE in Excel, FALSE in raw IEEE. Spec'd; we match.
8. **Operator precedence** — unary minus before `^` is a trap. `-2^2 = 4` in Excel, not `-4`.
9. **Whole-column refs** (`A:A`) — naïve engines allocate a million rows. We don't.
10. **Dynamic arrays / spills** — `FILTER`, `UNIQUE`, `SORT`. Not in MVP.
11. **VLOOKUP approx match on unsorted data** — Excel returns unpredictable results. We match Excel (don't try to be "helpful").
12. **Locale** — decimal comma, semicolon separator. IronCalc 0.7.1 added i18n; most engines assume en-US. We assume en-US for v0.1.

## Function coverage

The definitive, tiered list lives in [`../functions.md`](../functions.md).

Summary at a glance:

- **v0.1 ship gate**: 81 functions + 13 operators = **94 surfaces**. Covers the core of real business workbooks.
- **v0.2 stretch**: +27 functions = **121 surfaces** total. Adds trig, logs, extra stats, extra date functions.
- **Refused**: OFFSET, INDIRECT, dynamic arrays (FILTER/UNIQUE/SORT), LAMBDA/LET, network functions. See [`../architecture/streaming-model.md`](../architecture/streaming-model.md) for why.

If you're an agent implementing a phase, `functions.md` is what you tick boxes against — not this page.

## Explicitly out of MVP

- Engineering functions (BESSEL, HEX2BIN, etc.).
- Cube functions.
- Web functions (HYPERLINK, WEBSERVICE).
- Dynamic array functions (FILTER, UNIQUE, SORT — requires spill semantics).
- LAMBDA / LET (user-defined functions).
- Array formulas in general (we support function calls that *return* arrays only if they're reducible by the enclosing aggregate).

## Benchmark gap

Nobody publishes standardised "evaluate N-row xlsx" benchmarks. We produce one as part of v0.1:
- Reference workload: `benchmark_large_400k.xlsx` (400k × 20, 10 formula columns, 2 lookup sheets).
- Published results in `docs/research/benchmarks.md`.
- Plot over time on `gh-pages` via criterion + github-action-benchmark.

## References

- formualizer: private repo
- IronCalc: https://github.com/ironcalc/IronCalc
- HyperFormula: https://github.com/handsontable/hyperformula
- pycel: https://github.com/dgorissen/pycel
- xlcalculator: https://github.com/bradbase/xlcalculator
- formulas: https://github.com/vinci1it2000/formulas
- Excel 1900 leap year: https://learn.microsoft.com/en-us/troubleshoot/microsoft-365-apps/excel/wrongly-assumes-1900-is-leap-year
- LibreOffice threading: https://www.phoronix.com/news/LibreOffice-Calc-Threading
- bradbase benchmark repo: https://github.com/bradbase/Perfomance_testing_Python_Excel_calculators
