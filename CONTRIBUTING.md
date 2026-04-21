# Contributing

Thanks for the interest. xlstream is early — the fastest way to help is to read the docs, pick an unticked checkbox in the current phase, and open a PR.

## Where to start

1. Read [`CLAUDE.md`](CLAUDE.md) — the rules everyone follows.
2. Read [`docs/brief.md`](docs/brief.md) — what we're building.
3. Read the architecture doc for the area you're touching.

## One-time setup

```bash
make install
```

That single command:

- Checks prereqs (`git`, `python3`, `rustup`) are on PATH.
- Creates `.venv/` with Python dev deps (maturin, pytest, ruff, pre-commit).
- Installs the Rust toolchain + `rustfmt`, `clippy`, `rust-src` from `rust-toolchain.toml`.
- Installs git hooks: `pre-commit`, `commit-msg`, `pre-push`.

Then activate the venv before working:

```bash
source .venv/bin/activate
```

**Do not skip `make install`.** CI re-runs the same hooks and will block PRs from anyone who pushed without them.

### What runs when

| Stage | Checks |
|---|---|
| pre-commit | whitespace, EOF, YAML/TOML/JSON syntax, `cargo fmt --check`, `cargo clippy -D warnings`, `ruff` on Python |
| commit-msg | commit message matches `<prefix>: <imperative, lowercase>` format and contains no forbidden trailers |
| pre-push | `cargo test --all-features`, `cargo test --doc` |

Manual:

```bash
make check              # fmt + clippy + tests + doctests (full local gate)
make pre-commit         # all pre-commit hooks on every file
make pre-commit-all     # pre-commit + pre-push hooks on every file
make help               # everything else
```

If a hook is broken or you genuinely need to bypass (rare; usually fix the root cause): `git commit --no-verify`. But don't push without having run the checks first — CI will reject the PR.

## Workflow

1. Branch from `main`: `feature/<short-description>`.
2. Read the phase doc; pick one unticked checkbox.
3. Implement it, with tests and docs, in one commit.
4. Tick the box in the phase doc in the same PR.
5. Open a PR; fill in the template.
6. Address review comments; merge on approval.

## Commit messages

See [`docs/standards/commits.md`](docs/standards/commits.md).

Short version: `<crate>: <imperative, lowercase, no period>`.

Examples:
- `xlstream-eval: add VLOOKUP wildcard support`
- `docs: clarify whole-column aggregate handling`

**Do not** include `Co-Authored-By: Claude` or `Generated with Claude Code` trailers.

## Testing

See [`docs/standards/testing.md`](docs/standards/testing.md).

At minimum:
- `cargo test --workspace --all-features` green.
- `cargo test --doc` green.
- `cargo clippy --all-targets --all-features --workspace -- -D warnings` clean.
- `cargo fmt --check` clean.
- For Python: `pytest` in `bindings/python/tests/` green.

## Adding a new builtin function

1. Check [`docs/functions.md`](docs/functions.md) -- pick an unticked function.
2. Read the architecture doc for its category (e.g., `docs/architecture/lookups.md` for VLOOKUP).
3. Write tests first in the appropriate test module under `crates/xlstream-eval/src/builtins/`.
   - At minimum: happy path, empty input, error propagation, type coercion, edge case.
4. Implement in the matching `builtins/*.rs` module (e.g., `text.rs`, `math.rs`, `lookup.rs`).
5. Add a match arm in `crates/xlstream-eval/src/builtins/mod.rs` `dispatch()`.
6. If the function needs prelude data (aggregates, lookups), use the lazy dispatch path instead.
7. Add rustdoc with `# Examples` block.
8. Tick the box in `docs/functions.md` in the same PR.
9. Run `make check`.

## Code style

See [`docs/standards/code-style.md`](docs/standards/code-style.md).

## Reporting bugs

Open a GitHub Issue with:
- Minimal reproducing xlsx file (or a script that generates one).
- Expected behaviour.
- Actual behaviour.
- Rust toolchain + OS version.

## Security

Do NOT open public issues for security bugs. See [`SECURITY.md`](SECURITY.md).

## Licence

By contributing, you agree that your contributions will be dual-licensed under Apache-2.0 and MIT.
