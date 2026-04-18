# Bump Rust toolchain 1.85 → 1.88 to integrate formualizer-parse 1.1.2

**Status:** Accepted

## Context

Phase 1 pinned `formualizer-parse = "0.5"` per [`docs/architecture/parse-reuse.md`](../architecture/parse-reuse.md). That version does not exist on crates.io. The only published versions of the upstream parser and its common-types crate are:

- `formualizer-parse 1.1.2`
- `formualizer-common 1.1.2`

Both were confirmed via `cargo search formualizer-common` and `cargo search formualizer-parse` in the Phase 2 investigation session (2026-04-18). No earlier releases exist.

Both crates require Rust 1.88 to compile:

- `formualizer-common 1.1.2` uses `let` expressions inside `if` conditions — the `let_chains` feature, rust-lang issue #53667 — at three sites: `crates/formualizer-common/src/address.rs:396`, `crates/formualizer-common/src/address.rs:401`, `crates/formualizer-common/src/error.rs:257`. Stabilised in Rust 1.88.
- Both crates declare `edition = "2024"`, which itself requires Rust 1.85+.

xlstream currently pins Rust 1.85 via `rust-toolchain.toml`, `Cargo.toml` `rust-version = "1.85"`, and `clippy.toml` `msrv = "1.85"`. Phase 1 stubbed `xlstream_parse::Ast` as a unit struct and explicitly deferred the integration decision to Phase 2 (see comment block in the root `Cargo.toml`).

## Decision

Bump the xlstream toolchain and MSRV 1.85 → 1.88. Integrate upstream directly:

- Declare both crates in `[workspace.dependencies]` with **exact** pins:
  ```toml
  formualizer-parse = "=1.1.2"
  formualizer-common = "=1.1.2"
  ```
  We declare `formualizer-common` even though it would otherwise come transitively, so we control the pin ourselves.
- Use `std::sync::OnceLock` (stable since Rust 1.70) in **our own** code wherever a process-global lazy-initialised cell is needed. Do **not** import `once_cell` directly. `once_cell` enters the workspace solely as an upstream transitive dep.
- Mechanical toolchain bump lands in a separate follow-up PR (one-line diffs across three files) that links back to this ADR. This ADR is docs-only.

## Consequences

- **MSRV advances 1.85 → 1.88.** Acceptable. Rust 1.88 has been stable since 2025-06-26. xlstream's primary downstream consumer is the Python binding (Phase 11), distributed as a maturin-built `abi3-py39` wheel; Rust MSRV only affects Rust consumers of the library crates, a small population.
- **Two new transitive runtime deps appear:** `chrono` (via `formualizer-common`) and `once_cell` (via `formualizer-parse`). Both are in the standard-Rust-ecosystem baseline; neither requires a dedicated design note beyond a one-line mention in the toolchain-bump PR description.
- **Upstream public API maps cleanly to the planned `Ast` wrapper.** From `refs/formualizer/crates/formualizer-parse/src/`:
  - Entry point: `pub fn parse<T: AsRef<str>>(formula: T) -> Result<ASTNode, ParserError>`.
  - Node type: `ASTNode { node_type: ASTNodeType, source_token: Option<Token>, contains_volatile: bool }`.
  - Variants: `ASTNodeType::{Literal, Reference, UnaryOp, BinaryOp, Function, Array}`.
  - Reference enum: `ReferenceType`.
  - Re-exported from `formualizer-common`: `LiteralValue`, `ExcelError`, `ExcelErrorKind`, `ArgKind`.
- **Exact pins** per `docs/architecture/parse-reuse.md` guidance — upstream stability policy is unknown, so `=1.1.2` prevents silent minor-version drift. Revisit if upstream publishes stability guarantees.
- **Empirically verified during the investigation session (2026-04-18):**
  - `cargo +1.88.0 check` on `refs/formualizer/crates/formualizer-parse` — clean.
  - `cargo +1.88.0 check --workspace --all-features` on xlstream — clean.
  - `cargo +1.88.0 clippy --workspace --all-targets --all-features -- -D warnings` on xlstream — clean.
- **Six ratified Phase 1 clippy divergences still apply.** None are MSRV-sensitive.

## Alternatives considered

### Option B — Pin an older compatible pair

Rejected. Only `1.1.2` is published on crates.io for both `formualizer-parse` and `formualizer-common`; there is no earlier pair to pin. Confirmed via `cargo search`.

### Option C — Vendor `formualizer-parse` and `formualizer-common` into the workspace

Rejected. Vendoring **does not** avoid the toolchain bump: both upstream crates declare `edition = "2024"`, and Rust 2024 edition requires Rust 1.85 at minimum. (Combined with the `let_chains` usage, a vendoring path that wanted to keep today's `1.85` pin would additionally have to *rewrite* the let-chain expressions, which is even more fragile than a toolchain bump.) Vendoring therefore buys no toolchain flexibility while adding ongoing maintenance burden — bug-fix merges, API-drift tracking, two extra crates in our CI matrix. Hold Option C in reserve for a future emergency (upstream goes unmaintained, API diverges destructively, etc.) rather than starting there.

### Option A — Bump toolchain 1.85 → 1.88 (chosen)

Single-commit change across three files (`rust-toolchain.toml`, `Cargo.toml` `rust-version`, `clippy.toml` `msrv`). Same mechanical pattern as PR #5 (1.82 → 1.85). No new maintenance burden. Unblocks the current, maintained upstream parser.
