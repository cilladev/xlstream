# Phase 0 — Foundation

**Goal:** repo exists, tooling is configured, CI runs, nothing compiles yet.

**Estimated effort:** 1–2 days.

**Prerequisites:** none.

**Output:** a cloned repo with configured toolchain, CI green on a placeholder commit.

## Checklist

### Repo setup

- [x] Repo created at `github.com/cilladev/xlstream`
- [x] `README.md` with pre-alpha notice and links to `docs/`
- [x] `CLAUDE.md` with agent rules
- [x] `LICENSE-APACHE` + `LICENSE-MIT` (dual-licence)
- [x] `.gitignore` covering Rust target, Python builds, IDE, OS noise
- [x] `docs/` tree populated with all architecture, standards, operations, research, phase files (this commit)

### Toolchain

- [x] `rust-toolchain.toml` pins Rust 1.85 stable.
- [x] `rustfmt.toml` with project conventions (see `docs/standards/code-style.md`).
- [x] `clippy.toml` with project conventions.
- [x] `.editorconfig` for cross-editor consistency.

### Workspace

- [x] Root `Cargo.toml` declares `[workspace]` with `members = []` initially. Members added in Phase 1.
- [x] `[workspace.package]` defines shared metadata: version `0.1.0`, edition `2021`, rust-version `1.85`, licence `MIT OR Apache-2.0`, repository URL.
- [x] `[workspace.dependencies]` declares shared dependency pins (calamine, rust_xlsxwriter with features, formualizer-parse, pyo3, rayon, crossbeam-channel, smallvec, thiserror, tracing, phf, memory-stats, proptest, criterion, tempfile). See `docs/operations/repo-structure.md` for the list.

### Pre-commit + onboarding

- [x] `.pre-commit-config.yaml` at repo root: hygiene hooks, `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test` + `cargo test --doc` on pre-push, `ruff` for Python, commit-msg format validator.
- [x] `scripts/check-commit-msg.sh` validates the `<prefix>: <imperative>` format and rejects Claude trailers.
- [x] `Makefile` target `make install` — creates `.venv`, installs Rust toolchain components, installs Python dev deps (maturin, pytest, ruff, pre-commit), installs all three git hook types. Single command for new-dev onboarding.
- [x] README.md and CONTRIBUTING.md point every new contributor at `make install` as the single onboarding command.

### CI

- [x] `.github/workflows/pre-commit.yml` runs the full pre-commit suite (all stages including pre-push) on every PR and push to main.
- [x] `.github/workflows/ci.yml` — platform matrix (Linux / macOS / Windows) for `cargo test` + Python `maturin develop` + pytest + `cargo audit` + per-PR benchmark smoke.
- [x] `.github/workflows/release.yml` — tag-triggered wheel building across OS matrix + sdist + gated `publish-pypi` (via PyPI Trusted Publishing / OIDC — no token) + gated `publish-crates` (crates published in dep order: core → parse → io → eval).
- [x] ~~`.github/workflows/nightly.yml`~~ — removed. Benchmarks run per-PR via `bench-smoke` job in `ci.yml`.
- [x] `.github/dependabot.yml` — weekly updates for cargo, pip (bindings/python), and github-actions.
- [x] `Makefile` at repo root — `make help` lists every command.

### GitHub UI-only setup (cannot be done in-repo)

See [`docs/operations/github-setup.md`](../operations/github-setup.md) for the step-by-step walkthrough.

- [ ] Create PyPI project `xlstream` (or use "pending publisher" flow).
- [ ] Add trusted publisher on PyPI → Owner `cilladev`, Repo `xlstream`, Workflow `release.yml`, Environment `pypi`.
- [ ] Configure GitHub environment `pypi` (required reviewer, tag-only deployment branches, no secrets).
- [ ] Generate crates.io API token scoped to `xlstream-*` crates.
- [ ] Configure GitHub environment `crates-io` (required reviewer, tag-only), add `CARGO_REGISTRY_TOKEN` secret.
- [ ] Branch protection on `main`: require PR, 1 approval, required status checks (`pre-commit`, `ci / test`, `ci / python`, `ci / audit`).
- [ ] Enable Dependabot alerts, secret scanning, push protection in Settings → Security.

### Docs

- [x] `CHANGELOG.md` with `[Unreleased]` section only.
- [x] `SECURITY.md` at root (short; points to `docs/standards/security.md`).
- [x] `CONTRIBUTING.md` at root (short; points to `docs/standards/commits.md` + phase docs).

### First commit

- [ ] Everything above committed. PR from a `chore/foundation` branch to `main`. Merged.
- [ ] CI green on the foundation commit (all jobs will be mostly no-ops since no Rust code exists yet — OK).

## Sample files

### `rust-toolchain.toml`

```toml
[toolchain]
channel = "1.85.0"
components = ["rustfmt", "clippy", "rust-src"]
profile = "default"
```

### Minimal workspace `Cargo.toml` (Phase 0 — no members yet)

```toml
[workspace]
resolver = "2"
members = []

[workspace.package]
version = "0.1.0"
edition = "2021"
rust-version = "1.85"
license = "MIT OR Apache-2.0"
repository = "https://github.com/cilladev/xlstream"
authors = ["Priscilla Emasoga"]

[workspace.dependencies]
# see docs/operations/repo-structure.md for the full list to add here
```

### `.editorconfig`

```ini
root = true

[*]
end_of_line = lf
insert_final_newline = true
indent_style = space
indent_size = 4
trim_trailing_whitespace = true
charset = utf-8

[*.{md,yml,yaml,toml,json}]
indent_size = 2

[*.py]
indent_size = 4

[Makefile]
indent_style = tab
```

## Verification

After this phase:

```bash
cargo check --workspace      # should succeed with nothing to build
cargo fmt --check            # clean
cargo clippy --workspace     # clean (nothing to lint)
```

CI on main should be green.

## Done when

All checkboxes above are ticked, CI is green, the repo is ready for Phase 1 crate scaffolding.
