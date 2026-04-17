# Security

## Threat model

xlstream reads `.xlsx` files from untrusted sources and evaluates their formulas. Our threat surface:

1. **Malformed xlsx files** that could panic the parser, OOM the allocator, or exploit a zip/XML bug.
2. **Formula injection** — formulas that reference system paths, network resources, or exploit parser edge cases to execute something.
3. **Denial of service** — formulas or file shapes that consume unbounded memory or time.
4. **Dependency vulnerabilities** — our Rust dependencies may have CVEs.

## Mitigations

### Input validation

- **File size limit**: reject xlsx files > 1 GB by default. Configurable.
- **Row count limit**: reject sheets with > 10M rows. Configurable.
- **Column count limit**: reject sheets with > 20k columns.
- **Formula nesting limit**: reject formulas nested > 64 deep. This mirrors Excel's own limit.
- **Lookup sheet size limit**: lookup sheets must be < 10M cells; otherwise refuse.

All limits are `const` in `xlstream-core::limits` and overridable via `EvaluateOptions`.

### Formula injection

We do **not** implement:
- `INDIRECT` — resolves cell addresses at runtime. Refused.
- `OFFSET` — same. Refused.
- `HYPERLINK` — evaluates to a URL; we return the display-name value, not execute.
- Any network or file-system function (`WEBSERVICE`, `ENCODEURL`, etc.). Refused.

No formula can read a file, open a URL, or execute a shell command. The evaluator is a pure function from inputs to outputs.

### DoS

- Streaming architecture bounds memory by input shape, not by row count.
- Per-evaluation timeout (not implemented in v0.1; tracked for v0.2).
- Circular references detected at classification; refused with `CircularReferenceError`.

### Dependency hygiene

- `cargo audit` in CI, run on every PR and weekly.
- Dependencies reviewed when added. Every addition requires a design-note line in the PR: why this crate, what it gives us, alternatives considered.
- Dependabot enabled for security-only updates; non-security updates are opt-in.
- Avoid crates with < 6 months of maintenance activity or < 100 stars, unless there's no alternative.

### Panic policy

No panics in library code. A panic from a malformed file is a vulnerability — it would make xlstream unusable from a long-lived process.

Fuzz tests exist explicitly to flush out panics from malformed input.

## No unsafe in v0.1

Zero `unsafe` blocks in library crates. If v0.2 needs some, the design doc must cover safety argument + mitigation.

## Secrets

The evaluator never logs cell values at the default log level. `tracing::debug!`/`trace!` may log them, but these are opt-in and off by default.

`info!` logs file paths and row counts, not cell contents.

## Supply chain

- Pin Rust toolchain version in `rust-toolchain.toml`. Updates are PRs.
- Pin dependency versions with `Cargo.lock` committed (it's a binary crate for `xlstream-cli` and `bindings/python`).
- Workspace `Cargo.toml` uses `=` version pins for dev-dep tools (clippy, rustfmt) where needed.

## CI secrets

- `CARGO_REGISTRY_TOKEN` — only accessible from the `release` workflow on tag push.
- `PYPI_API_TOKEN` — same.
- No secrets in PR CI. Fork PRs run against a restricted workflow.

## Reporting vulnerabilities

Private disclosure via GitHub's "Report a vulnerability" button on the repo. `SECURITY.md` at repo root documents the process (to be added before v0.1).
