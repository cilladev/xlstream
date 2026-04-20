# Repository structure: one monorepo

## Decision

One Git repository. Rust workspace + Python binding + docs + benchmarks, all together. Single versioning, single CI, single source of truth.

## Why one repo

The Rust+Python hybrid library pattern in 2026 is overwhelmingly monorepo. Reference projects:

| Project | Layout |
|---|---|
| `pola-rs/polars` | Rust workspace + `py-polars/` sibling |
| `pydantic/pydantic-core` | Rust + Python flat at root |
| `astral-sh/ruff` | Rust workspace + `python/` source |
| `huggingface/tokenizers` | Rust workspace + `bindings/python/` (our pattern) |
| `openai/tiktoken` | Flat Rust + Python (uses setuptools-rust, older) |

### Benefits

- **One source of truth for versioning.** A release is a single tag; Rust crate and Python wheel share the version.
- **Cross-cutting PRs work naturally.** A change that touches the parser AND the Python binding lands as one reviewable unit.
- **Tests exercise both sides.** A Python-level integration test can catch a Rust regression.
- **Build dependency ordering is implicit.** Cargo knows; maturin knows. No manual "publish crate first, then binding."
- **Single CI pipeline.** Less duplication. One workflow knows how to build the world.
- **Contributors see everything.** Reading the Rust code while reviewing the Python test is one click away.

### Downsides

- **Larger clone.** Negligible for this repo's size projection.
- **Breaks if you want to open-source the Rust core while keeping bindings private.** Not our situation. (And if it becomes our situation: split at that point. GitHub supports it.)
- **Release logic is slightly more complex** — two artefacts from one tag. Solved by CI templates.

## What about separate repos?

We considered: `xlstream` (Rust), `xlstream-py` (Python). Rejected.

- Two release processes to keep in sync.
- Version drift between Rust crate and Python binding.
- Cross-cutting changes require two PRs, coordinated via branch names.
- Duplicated CI infrastructure.
- Duplicated docs (where does "how does streaming work" live?).

The monorepo pattern is the default in 2026 for a reason: the above costs exceed any benefit for projects our size.

## Physical layout

```
xlstream/                         # the repo
├── Cargo.toml                    # workspace
├── rust-toolchain.toml           # pinned Rust
├── rustfmt.toml
├── clippy.toml
├── README.md
├── CLAUDE.md
├── CHANGELOG.md
├── LICENSE-APACHE
├── LICENSE-MIT
├── .gitignore
├── .editorconfig
│
├── crates/                       # Rust library crates
│   ├── xlstream-core/
│   ├── xlstream-parse/
│   ├── xlstream-eval/
│   ├── xlstream-io/
│   └── xlstream-cli/
│
├── bindings/
│   └── python/                   # PyO3 crate + Python wrapper + tests
│
├── benchmarks/                   # criterion benches
├── fixtures/                     # test xlsx files + scripts
├── tests/                        # workspace-level integration tests
│
├── docs/                         # everything written
│
├── .github/
│   ├── workflows/
│   │   ├── ci.yml                # build, test, lint
│   │   ├── release.yml           # on tag push
│   │   └── pre-commit.yml        # pre-commit hooks
│   └── dependabot.yml
│
└── fuzz/                         # cargo-fuzz targets
```

## Workspace Cargo.toml

```toml
[workspace]
resolver = "2"
members = [
    "crates/xlstream-core",
    "crates/xlstream-parse",
    "crates/xlstream-eval",
    "crates/xlstream-io",
    "crates/xlstream-cli",
    "bindings/python",
    "benchmarks",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
rust-version = "1.85"
license = "MIT OR Apache-2.0"
repository = "https://github.com/cilladev/xlstream"
authors = ["Priscilla Emasoga"]

[workspace.dependencies]
calamine = "0.34"
rust_xlsxwriter = { version = "0.94", features = ["constant_memory", "zlib", "ryu"] }
formualizer-parse = "0.5"     # pin exact version after first integration
pyo3 = { version = "0.28", features = ["extension-module", "abi3-py39"] }
rayon = "1.10"
smallvec = "1.13"
thiserror = "2.0"
tracing = "0.1"
phf = { version = "0.11", features = ["macros"] }

# Dev dependencies
proptest = "1.5"
criterion = "0.5"
tempfile = "3.14"
```

Children use `version.workspace = true` and `<dep>.workspace = true` for DRY.

## GitHub organisation

**For now**: personal account. The repo is `github.com/cilladev/xlstream`.

**When to move to an organisation**:
- You have 2+ active human collaborators and want shared ownership.
- You want a branded identity separate from your personal handle.
- You expect to host multiple related repos (e.g., `xlstream`, `xlstream-examples`, `xlstream-benchmarks-public`).

**How to move**: GitHub > Settings > Transfer ownership. Git history, issues, PRs, forks all follow. `git remote set-url origin ...` for each clone afterwards.

Delay until one of those conditions is genuinely true. Moving too early wastes time on org admin.

## Tags, releases, branches

- **`main`** is protected. Requires PR + passing CI.
- **Tags** are `v0.1.0`, `v0.1.1`, etc. Tag on main, never on a branch.
- **Releases** are created by CI on tag push.
- **Long-lived branches**: none. We merge to main; if we need to support a stable line while developing 0.2, we'll branch then — v0.1 probably won't need it.

## Why we don't use git submodules

No. Submodules are a footgun at the scale most teams operate. If we need a shared component, it's a crate in this workspace or a crates.io dependency.
