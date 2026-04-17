# xlstream docs

Navigation for everything written about the project.

## Start here

- [`brief.md`](brief.md) — what we're building, for whom, and why.
- [`functions.md`](functions.md) — **canonical list of every supported function** with tick-box status.
- [`../CLAUDE.md`](../CLAUDE.md) — rules every AI agent follows.
- [`phases/README.md`](phases/README.md) — the phased roadmap, current progress, checklists.

## Architecture

- [`architecture/overview.md`](architecture/overview.md) — high-level system shape.
- [`architecture/streaming-model.md`](architecture/streaming-model.md) — the core design: how streaming eval actually works.
- [`architecture/crate-layout.md`](architecture/crate-layout.md) — what each crate owns.
- [`architecture/parse-reuse.md`](architecture/parse-reuse.md) — how we use `formualizer-parse`.
- [`architecture/io.md`](architecture/io.md) — calamine reader + rust_xlsxwriter writer.
- [`architecture/evaluator.md`](architecture/evaluator.md) — per-cell eval engine.
- [`architecture/lookups.md`](architecture/lookups.md) — hash indexes for VLOOKUP / XLOOKUP / MATCH.
- [`architecture/aggregates.md`](architecture/aggregates.md) — whole-column pre-pass.
- [`architecture/parallelism.md`](architecture/parallelism.md) — row sharding strategy.
- [`architecture/python-bindings.md`](architecture/python-bindings.md) — PyO3 + maturin.
- [`architecture/errors.md`](architecture/errors.md) — error taxonomy and propagation.

## Standards

- [`standards/code-style.md`](standards/code-style.md) — rustfmt, clippy, naming, module layout.
- [`standards/testing.md`](standards/testing.md) — unit / integration / property / fuzz / benchmark.
- [`standards/documentation.md`](standards/documentation.md) — rustdoc, examples, docs-as-code.
- [`standards/commits.md`](standards/commits.md) — commit messages, branches, PRs.
- [`standards/security.md`](standards/security.md) — input validation, panic policy, dependency vetting.

## Operations

- [`operations/repo-structure.md`](operations/repo-structure.md) — why one monorepo.
- [`operations/organisation.md`](operations/organisation.md) — GitHub org strategy.
- [`operations/ci.md`](operations/ci.md) — GitHub Actions pipelines.
- [`operations/release.md`](operations/release.md) — crates.io + PyPI release flow.
- [`operations/github-setup.md`](operations/github-setup.md) — GitHub UI-only setup (trusted publishing, environments, branch protection).

## Research

- [`research/formualizer.md`](research/formualizer.md) — deep review of the Rust engine we're replacing.
- [`research/calamine.md`](research/calamine.md) — streaming xlsx reader.
- [`research/rust_xlsxwriter.md`](research/rust_xlsxwriter.md) — streaming xlsx writer.
- [`research/pyo3-maturin.md`](research/pyo3-maturin.md) — Python binding tooling in 2026.
- [`research/prior-art.md`](research/prior-art.md) — competitive landscape.
- [`research/benchmarks.md`](research/benchmarks.md) — reference workloads and target numbers.

## Phases

- [`phases/README.md`](phases/README.md) — overview and current phase.
- [`phases/phase-00-foundation.md`](phases/phase-00-foundation.md) — repo, CI, tooling.
- [`phases/phase-01-scaffolding.md`](phases/phase-01-scaffolding.md) — workspace crates.
- [`phases/phase-02-parser.md`](phases/phase-02-parser.md) — formualizer-parse integration + classification.
- [`phases/phase-03-io.md`](phases/phase-03-io.md) — calamine + rust_xlsxwriter.
- [`phases/phase-04-streaming-core.md`](phases/phase-04-streaming-core.md) — driver, row scope, topo order.
- [`phases/phase-05-arithmetic.md`](phases/phase-05-arithmetic.md) — arithmetic, comparison, concat.
- [`phases/phase-06-conditional.md`](phases/phase-06-conditional.md) — IF, IFS, AND, OR, IFERROR.
- [`phases/phase-07-aggregates.md`](phases/phase-07-aggregates.md) — whole-column pre-pass.
- [`phases/phase-08-lookups.md`](phases/phase-08-lookups.md) — VLOOKUP, HLOOKUP, XLOOKUP, MATCH, XMATCH, INDEX, CHOOSE.
- [`phases/phase-09-strings-dates-math.md`](phases/phase-09-strings-dates-math.md) — builtin coverage.
- [`phases/phase-10-parallelism.md`](phases/phase-10-parallelism.md) — row sharding.
- [`phases/phase-11-python-binding.md`](phases/phase-11-python-binding.md) — PyO3 + maturin.
- [`phases/phase-12-benchmarks.md`](phases/phase-12-benchmarks.md) — criterion + reference corpus.
- [`phases/phase-13-docs.md`](phases/phase-13-docs.md) — rustdoc, mdBook, site.
- [`phases/phase-14-release.md`](phases/phase-14-release.md) — v0.1.0 to crates.io + PyPI.
