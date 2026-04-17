# Contributing

Thanks for the interest. xlstream is early — the fastest way to help is to read the docs, pick an unticked checkbox in the current phase, and open a PR.

## Where to start

1. Read [`CLAUDE.md`](CLAUDE.md) — the rules everyone follows.
2. Read [`docs/brief.md`](docs/brief.md) — what we're building.
3. Read [`docs/phases/README.md`](docs/phases/README.md) — find the current phase.
4. Read the architecture doc for the area you're touching.

## One-time setup

Install `pre-commit` so local commits, commit messages, and pushes are checked automatically. This is **required** — CI re-runs the same checks and will block PRs that skip them.

```bash
pip install pre-commit
pre-commit install --install-hooks
pre-commit install --hook-type commit-msg
pre-commit install --hook-type pre-push
```

What runs when:

| Stage | Checks |
|---|---|
| pre-commit | whitespace, EOF, YAML/TOML/JSON syntax, typos, `cargo fmt --check`, `cargo check`, `ruff` on Python |
| commit-msg | commit message matches `<prefix>: <imperative, lowercase>` format and contains no forbidden trailers |
| pre-push | `cargo clippy -D warnings`, `cargo test`, `cargo test --doc` |

Manual run on everything:

```bash
pre-commit run --all-files                             # pre-commit stage only
pre-commit run --all-files --hook-stage pre-push       # everything, including clippy + tests
```

If a hook is broken or you need to bypass (rare; you should usually fix the issue): `git commit --no-verify` — but don't push without having run the checks first, or CI will reject the PR.

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
