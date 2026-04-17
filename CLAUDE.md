# CLAUDE.md — xlstream

> **Read this file at the start of every session.** It is the contract between the human and every AI agent that contributes to this repo.

## What you are building

`xlstream` is a **streaming Excel formula evaluation engine** written in Rust, with Python bindings. It reads `.xlsx` files row-by-row, evaluates formulas in a single (or two-pass) streaming traversal, and writes the results to a new `.xlsx` — all in bounded memory regardless of file size.

**Design goal in one sentence:** evaluate a 400,000-row × 20-column xlsx in under 3 minutes of wall-clock time with peak RSS under 250 MB.

We are not building a general-purpose spreadsheet engine. We are building the *fastest, leanest* engine for the subset of workloads where formulas are mostly **row-local** with **shared lookup sheets that fit in memory** and **whole-column aggregates** — which is ~90% of real business workbooks. Lookup sheets can have thousands to hundreds of thousands of rows; "fits in memory" is the real constraint, not "small."

## Why this exists

The alternative, `formualizer` (Rust, graph-based), takes **5h 40m wall-clock at 3.3 GB peak RSS** to evaluate a 400k × 20 workbook (measured 2026-04-17). That's architectural: the graph holds every cell as a vertex, and umya buffers the whole workbook in memory at both load and save. By trading feature breadth (no volatile re-eval, no iterative calc, no full dynamic-array spills) for architectural simplicity (streaming, two-pass) we target **~13× less memory and ~100× faster wall-clock** on the identical workload.

Full background: [`docs/brief.md`](docs/brief.md) and [`docs/research/formualizer.md`](docs/research/formualizer.md).

## Non-negotiables

1. **Correctness first, speed second.** A fast wrong answer is useless. Every formula implementation lands with tests against known Excel outputs.
2. **No unsafe Rust in the MVP.** If performance ever demands it, justify in a design doc first, contain it behind a safe wrapper, and document the invariants.
3. **No panics in library code.** Every error path returns a typed `Result<_, XlStreamError>`. Panics are for "impossible" invariant violations only and must never be reachable from user input.
4. **Peak memory is a test, not an aspiration.** Every new feature is benchmarked on the 400k-row reference workload. Regressions > 10% RSS block merges.
5. **Every public API has rustdoc + at least one example.** Missing docs = failing CI.
6. **Tests are code.** They go through the same review bar as library code. No "quick tests" with no assertions.

## The streaming invariant

This is the single most important rule in the codebase:

> **After prelude, no evaluator function may read a cell value from a row it has not yet streamed over.**

Prelude (pass 1) is allowed — and required — to read whole columns for aggregates and to load lookup sheets fully. That's the point of prelude. Once prelude finishes and pass 2 (the row stream) begins, the row-level evaluator is bound by the invariant above. Every formula must be expressible as: *"given the current row's cells and the pre-computed scalars/lookups, produce this cell's value."* If a formula cannot fit that shape, we refuse it at classification with a clear error — we do not fall back to buffering.

Violating this invariant silently defeats the entire project. Treat attempts to relax it as architectural regressions.

## Repository layout

```
xlstream/
├── Cargo.toml              workspace root
├── rust-toolchain.toml     pinned Rust version
├── crates/
│   ├── xlstream-core/      value types, errors, traits
│   ├── xlstream-parse/     formula parser (wraps formualizer-parse)
│   ├── xlstream-eval/      streaming evaluator + builtins
│   ├── xlstream-io/        calamine reader + rust_xlsxwriter writer
│   └── xlstream-cli/       optional binary (dev tool)
├── bindings/
│   └── python/             PyO3 + maturin Python package
├── benchmarks/             criterion benches + reference workbooks
├── fixtures/               .xlsx test fixtures + expected outputs
├── tests/                  cross-crate integration tests
└── docs/                   everything you're reading
```

## Where to look for answers

| Question | File |
|---|---|
| What are we building and for whom? | [`docs/brief.md`](docs/brief.md) |
| Which functions do we support? | [`docs/functions.md`](docs/functions.md) |
| How does streaming eval actually work? | [`docs/architecture/streaming-model.md`](docs/architecture/streaming-model.md) |
| What crate does what? | [`docs/architecture/crate-layout.md`](docs/architecture/crate-layout.md) |
| How do errors flow through the system? | [`docs/architecture/errors.md`](docs/architecture/errors.md) |
| What are the code-style rules? | [`docs/standards/code-style.md`](docs/standards/code-style.md) |
| What kinds of tests do we write? | [`docs/standards/testing.md`](docs/standards/testing.md) |
| What's the current phase? | [`docs/phases/README.md`](docs/phases/README.md) |
| Why one repo, not three? | [`docs/operations/repo-structure.md`](docs/operations/repo-structure.md) |
| How is CI wired? | [`docs/operations/ci.md`](docs/operations/ci.md) |
| Why did we pick calamine / rust_xlsxwriter? | [`docs/research/`](docs/research/) |

## Working rules for AI agents

### Before you touch code

1. **Read the current phase doc.** Find it via [`docs/phases/README.md`](docs/phases/README.md). You are implementing a specific checkbox in a specific phase. Don't free-lance.
2. **Read the architecture doc that covers your area.** Unclear architecture → stop and ask. Do not invent.
3. **Search for existing patterns.** If three other builtins handle errors a given way, yours does too. Consistency outranks cleverness.
4. **Always use Context7 MCP for up-to-date docs before writing code.** This is non-negotiable. Any time you're about to use a library, framework, SDK, or crate — calamine, rust_xlsxwriter, pyo3, maturin, rayon, formualizer-parse, phf, thiserror, tokio, anything — call Context7 first to verify the API shape against the **current** documentation. Your training-data knowledge may be stale; library APIs drift between versions. Context7 costs nothing; a silently-wrong import costs hours. Prefer it over general web search for library docs.

### When you write code

1. **One behaviour per commit.** Scope creep is the enemy of reviewability. If you notice something unrelated that's broken, open an issue, don't fix it here.
2. **Write the test first.** Then the implementation. Then the rustdoc. Never the other order.
3. **Every public item has rustdoc.** Every rustdoc has at least one `# Examples` block that compiles under `cargo test --doc`.
4. **Commit messages:** imperative, lowercase, prefixed with crate name. Example: `xlstream-eval: add SUMIF over pre-computed column aggregate`.
5. **No `unwrap()`, `expect()`, `panic!()`, or `todo!()` in library code.** Tests may use them. Binaries may use them sparingly with clear messages. Library code returns `Result`.
6. **No `Box<dyn Error>`.** Use our typed `XlStreamError` enum.
7. **No `println!` in library code.** Use `tracing`.

### When you finish

1. **Run `make check`.** That's `cargo fmt --check && clippy -D warnings && cargo test --all-features && cargo test --doc` in one go. Nothing ships unless this passes.
2. **Let pre-commit do the heavy lifting.** If you ran `make install` on first clone, hooks fire automatically: `pre-commit` on commit, `pre-push` on push. Don't bypass them; fix the underlying issue.
3. **Update the phase checklist.** Tick the box you completed. Do not tick boxes you didn't actually finish.
4. **Update any doc that's now stale.** If you changed a public API, the rustdoc and any mention in `docs/` must be updated in the same PR.
5. **If you landed a builtin function, tick it in [`docs/functions.md`](docs/functions.md) too.** Same PR.
6. **One PR per checkbox** unless the doc says otherwise.

### Do NOT

- Add dependencies without a design note in the PR description explaining *why this one* vs alternatives.
- Add `unsafe` blocks without a design doc approved first.
- Skip tests because "it's trivial." Trivial things break most often.
- Rewrite modules you don't own without prior agreement in an issue.
- Use emojis in code, comments, or commit messages. They go in reaction threads, not the codebase.
- Claim work is "complete" without running the full check listed above. A failing test is not complete.
- Invent function names. If Excel calls it `VLOOKUP`, so do we (case-preserving, case-insensitive match).

### When you're stuck

Prefer asking over guessing. A five-line clarifying question is cheaper than a fifty-line PR revert.

## Performance budgets (enforced by CI, eventually)

| Workload | Peak RSS | Wall-clock |
|---|---|---|
| 10,000 × 20 (10 formula cols) | < 50 MB | < 2 s |
| 100,000 × 20 (10 formula cols) | < 150 MB | < 15 s |
| 400,000 × 20 (10 formula cols) | < 250 MB | < 3 min |

These are targets for v0.1. They will tighten in later releases.

## Current state

See [`docs/phases/README.md`](docs/phases/README.md) for the phase you are in.

## Tone

Write code, comments, and docs the way a senior engineer writes to another senior engineer. Terse, declarative, no hedging, no ceremony. If a sentence can be cut without losing meaning, cut it. If a function name explains itself, don't re-explain in a comment.
