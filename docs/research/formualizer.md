# formualizer — what we're displacing

Deep review conducted April 2026 on `~/Projects/formualizer/` at commit 0.5.x. This is the engine we're replacing as the default evaluator in downstream projects (starting with xlformula).

## One-paragraph summary

`formualizer` is a Rust spreadsheet evaluator with Python bindings (PyO3). It reads .xlsx via `umya-spreadsheet`, builds a full dependency graph over every cell, evaluates via a topological scheduler with rayon parallelism, and writes back through umya. Correctness is excellent; feature coverage is broad (320+ functions, cross-sheet refs, whole-column ranges, `_xlfn.` prefixes, static schedule caching). But the graph + umya + computed-overlay architecture costs ~11 GB RSS on a 56 MB / 400k-row / 20-col file, and ~30 minutes of wall-clock to evaluate it. Those costs are structural, not bugs.

## Architecture

### Crate layout

- `formualizer` — root entry point.
- `formualizer-parse` — standalone formula tokenizer + parser. **Reusable; we depend on this.**
- `formualizer-eval` — dependency graph, scheduler, builtins, arrow-backed cell storage.
- `formualizer-workbook` — umya-spreadsheet I/O, `recalculate_file()` entry point.
- `bindings/python` — PyO3 wrapper.

### Pipeline

1. **Load** — umya parses the whole xlsx into an in-memory workbook representation.
2. **Ingest** — iterate rows, populate Arrow column stores + dependency graph vertices.
3. **Parse** — formulas → ASTs via `formualizer-parse` (with per-formula token caching).
4. **Graph build** — dependency DAG (SoA vertex store, CSR edges, compressed range stripes).
5. **Evaluate** — topological scheduler, rayon-parallel within topological layers.
6. **Writeback** — computed values overlay → umya in-memory → xlsx save.

### Scale at 400k × 20

- ~8M vertices in the graph.
- ~4M edges.
- Computed overlay hashmap: ~150–300 MB.
- Arrow column stores: ~300–500 MB.
- umya in-memory workbook: ~1–2 GB.
- jemalloc fragmentation: 20–30% overhead on top.

## Where memory actually goes

Agent review identified, in order of impact:

1. **umya in-memory workbook (1–2 GB)** — full object model. Not streaming.
2. **Dependency graph (1–2 GB)** — 8M vertices + edges, unavoidable in graph-based eval.
3. **Per-cell AST storage (1–2 GB)** — every formula cell stores its own AST node.
4. **Writer buffer (500 MB – 1 GB)** — umya saves in one shot, no streaming.
5. **Arrow column stores (300–500 MB)**.
6. **Computed overlays (150–300 MB)** — `HashMap<VertexId, ValueRef>` retained until Engine drops.
7. **String interner (50–200 MB)** — grows monotonically within a process.
8. **jemalloc fragmentation (~25%)** — doesn't return to OS.

Fixes (1)(2)(3)(4)(5) require structural changes — the choice of a graph + umya architecture.

Fixes (6)(7) are ~200–500 MB of low-hanging fruit but don't meaningfully move the needle.

## Where CPU goes

1. **Graph construction** — 8M vertex allocations + edge CSR. Real cost even before evaluation.
2. **umya parse** — whole-file load, ~10–30 s on 56 MB.
3. **umya save** — full serialisation, ~30–60 s.
4. **Per-cell dispatch** — scheduler indirection on every cell.
5. **Computed-overlay writeback** — extra pass over every formula cell.
6. **Linear / binary-search lookups** — no hash index for VLOOKUP / XLOOKUP / MATCH exact match. 400k × 10 formulas × lookup scan = O(rows × lookup_table_size) comparisons.

## What's good

- `formualizer-parse` is mature. Tokenizer, `_xlfn.` handling, cross-sheet ref support, whole-column ranges, `BatchParser` with token caching. **We reuse this wholesale.**
- Function coverage: 320+ builtins. Broad and correct.
- Active maintenance: 12+ releases, commits as recent as April 2026.
- Licence: MIT / Apache dual. Compatible with us.
- Already published to crates.io and PyPI (`pip install formualizer`).

## What we learn from the review

1. **The graph is the memory.** Not a bug we can fix upstream — a consequence of the architecture.
2. **umya is not streaming.** Formualizer is stuck with the library's shape.
3. **Subprocess workaround is correct.** Memory doesn't return to OS; exiting the process is the reliable reset. We adopt the same pattern for formualizer callers, but we don't need it ourselves.
4. **Lookup indexes are the single biggest missing optimisation.** Hash-based VLOOKUP on 10k-row tables = 10–100× speedup. We do this from day one.
5. **Row parallelism beats column parallelism** for row-independent workloads. Formualizer does intra-layer rayon which helps less than it sounds.

## What we deliberately do differently

| Formualizer | xlstream |
|---|---|
| umya (full in-memory workbook) | calamine (streaming row iterator) |
| Full dependency graph | Streaming, 2-pass |
| Per-cell AST | AST cached by formula text |
| Linear / binary lookup | Hash-indexed lookup |
| Topological scheduler + rayon | Row-sharded rayon |
| umya save (full rewrite) | rust_xlsxwriter `constant_memory` |
| 320+ functions | 80 (MVP, grows) |
| Circular refs, iterative calc | Refused with a clear error |

## What we keep from formualizer

- `formualizer-parse` as a dependency. Pin exact version.
- The error taxonomy shape (separate cell-level errors from library-level errors).
- The subprocess pattern for downstream callers who want a hard memory reset (but we shouldn't need it).

## What we do NOT steal

- Its scheduler.
- Its dependency graph.
- Its umya integration.
- Its computed-overlay writeback.
- Its string interner (we don't need one — formulas are cached by text, rows stream through).

## Upstream contributions we may file

From the review, two clearly-valuable PRs for formualizer itself:

1. Clear `StringInterner` + computed overlays after each `evaluate_all()` call.
2. Hash-based exact-match path for `VLOOKUP` / `XLOOKUP` / `MATCH`.

Neither is our primary project. Nice-to-have, not on the critical path. If maintainers want them, we file. If not, our users benefit through xlstream directly.

## References

- Repo: (private)
- CHANGELOG: documents 0.5.x perf work including static schedule reuse (March 2026) and cached lowered text lanes for MATCH/XLOOKUP (March 2026). Active maintainers.
- Python bindings via PyO3 + maturin; published to PyPI.
