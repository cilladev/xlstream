# Commits, branches, PRs

## Local enforcement: pre-commit

Contributors run `pre-commit` locally so nonsense never reaches CI. See [`CONTRIBUTING.md`](../../CONTRIBUTING.md#one-time-setup) for the install steps. The hooks enforce:

- Generic hygiene: trailing whitespace, EOF newline, merge markers, YAML / TOML / JSON syntax, large-file detection, line-ending normalisation.
- Typos (via `crate-ci/typos`).
- Rust: `cargo fmt --check` + `cargo check` on every commit. `cargo clippy -D warnings` + `cargo test` + `cargo test --doc` on every push.
- Python (`bindings/python/` only): `ruff check --fix` + `ruff-format`.
- Commit message format (see below).

The same checks run in CI via `.github/workflows/pre-commit.yml`. A PR from someone who skipped local hooks will hit the same wall at CI.

## Branches

```
feature/<short-dash-case>       # new feature
fix/<short-dash-case>           # bug fix
refactor/<short-dash-case>      # internal reshape, no behaviour change
docs/<short-dash-case>          # docs only
perf/<short-dash-case>          # perf work (with benchmark evidence)
chore/<short-dash-case>         # tooling, CI, repo housekeeping
```

Keep branch names short and descriptive. `feature/vlookup-wildcard` — good. `feature/my-new-work` — bad.

Main branch: `main`. Protected. No direct pushes.

## Commit messages

**Format (enforced by `commit-msg` pre-commit hook):**

```
<prefix>: <imperative, lowercase, no trailing period>

<optional body — wrap at 72 chars>

<optional footer: Closes #123 / See #456>
```

Allowed `<prefix>` values:
- A crate name: `xlstream-core`, `xlstream-parse`, `xlstream-eval`, `xlstream-io`, `xlstream-cli`, `xlstream-python`.
- Or a non-crate kind: `docs`, `chore`, `ci`, `refactor`, `fix`, `feat`, `perf`, `test`, `build`, `release`.

**Examples:**

```
xlstream-eval: add VLOOKUP wildcard support
xlstream-io: fix panic on empty shared-strings table
xlstream-parse: update formualizer-parse to 0.5.1
docs: clarify whole-column aggregate handling
chore: bump calamine to 0.35
ci: add pre-commit workflow
```

Why this format: `git log --oneline` becomes grep-friendly, each commit orients the reviewer immediately, and the hook prevents drift.

The validator script lives at [`scripts/check-commit-msg.sh`](../../scripts/check-commit-msg.sh). It also refuses any commit message containing `Co-Authored-By: Claude` or `Generated with Claude Code` — those are forbidden by [`CLAUDE.md`](../../CLAUDE.md).

## Body rules

- Only when needed. A one-line change doesn't need a body.
- Wraps at 72 chars.
- Explains **why**, not what. The diff shows what.
- Names the motivating case when relevant: "introduced because XLOOKUP wildcard lookups were failing silently on large lookup tables."

## Don'ts

- No `Generated with Claude Code` footer.
- No `Co-Authored-By: Claude` trailer.
- No emojis.
- No trailing period on the subject line.
- No capitalised subject line (except proper nouns).
- No present-tense summary ("adds X"); use imperative ("add X").
- No future-tense ("will add X").

## Commit frequency

One logical change per commit. If a PR contains 3 unrelated changes, it's 3 commits (and probably 3 PRs).

A logical change includes:
- The code.
- The tests.
- The docs.

All in one commit. No "add feature" → "add tests for feature" → "add docs for feature" sequences. Those are hard to review and hard to revert.

## Squash vs rebase

We use **squash merge** to main. Your branch can have as many WIP commits as you want; GitHub squashes into one clean commit on merge.

Squash commit message is authored by the PR title. Write PR titles that make good commit messages.

## PRs

### Title

Same rules as commit messages. `<crate>: <imperative description>`.

### Description

Template:

```markdown
## What

<one paragraph>

## Why

<link to the phase, issue, or motivating case>

## How

<optional — for non-obvious architectural changes>

## Testing

- [ ] Added unit tests in <file>
- [ ] Added integration test in <file>
- [ ] Benchmark still within budget (link to benchmark run)

## Docs

- [ ] Rustdoc updated
- [ ] Relevant docs/ page updated (link)

## Checklist

- [ ] `cargo fmt --check` passes
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` clean
- [ ] `cargo test --all-features` passes
- [ ] `cargo test --doc` passes
- [ ] Python tests pass (if binding touched)
- [ ] CHANGELOG.md updated
```

### Size

Aim for < 400 lines changed per PR. Large PRs are hard to review, hard to revert.

If a change is genuinely large and atomic (refactor, rename, framework swap), call it out in the description: "this PR is 2000 lines because X; it cannot be split without breaking Y."

### Reviews

Every PR requires approval from at least one reviewer before merge. Self-merges are allowed only for trivial docs typos, explicitly marked `chore/` PRs.

### CI must be green

No merging with failing CI. If CI is broken due to infra, fix it before merging.

## Tagging releases

Semver. Start at `0.1.0`.

```
v0.1.0
v0.1.1
v0.2.0
```

Tag on `main` after merge. CI picks up the tag and builds/publishes artefacts.

## `CHANGELOG.md`

Updated in the same PR as the feature. Release tags move `[Unreleased]` items to a new versioned section.
