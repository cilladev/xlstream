# Phase 14 — Release v0.1.0

**Goal:** ship v0.1.0 to crates.io + PyPI. Announce.

**Estimated effort:** 2–3 days.

**Prerequisites:** every earlier phase complete. CI green.

**Reading:** [`docs/operations/release.md`](../operations/release.md).

**Output:** `pip install xlstream` works for anyone. `cargo add xlstream-eval` works for anyone. Git tag `v0.1.0` exists. GitHub Release page published.

## Checklist

### Pre-flight

- [ ] All Phase 13 items done.
- [ ] `cargo test --workspace --all-features` green.
- [ ] `cargo test --doc` green.
- [ ] `cargo clippy --all-targets --all-features --workspace` clean.
- [ ] `cargo fmt --check` clean.
- [ ] `pytest` green across OS matrix.
- [ ] Benchmarks at or better than targets.
- [ ] `cargo audit` clean.
- [ ] No `TODO` / `FIXME` / `unimplemented!` in library code.

### Version bump

- [ ] `Cargo.toml` `[workspace.package] version = "0.1.0"`.
- [ ] `bindings/python/pyproject.toml` reads version dynamically from Cargo — confirm it renders as `0.1.0`.
- [ ] `py_src/xlstream/__init__.py`: `__version__ = "0.1.0"`.
- [ ] Commit: `chore: bump version to 0.1.0`.

### CHANGELOG

- [ ] Promote `[Unreleased]` to `[0.1.0] - <today>`.
- [ ] Add a new empty `[Unreleased]` section at the top.
- [ ] Include migration notes from formualizer.
- [ ] Include any known limitations.
- [ ] Include benchmark numbers.

### Dry-run publishes

- [ ] `cargo publish -p xlstream-core --dry-run`.
- [ ] `cargo publish -p xlstream-parse --dry-run`.
- [ ] `cargo publish -p xlstream-io --dry-run`.
- [ ] `cargo publish -p xlstream-eval --dry-run`.
- [ ] `maturin build --release` locally — verify wheel installs and works.
- [ ] `maturin publish --dry-run` (or `--repository testpypi`).

### TestPyPI upload (rehearsal)

- [ ] Push a `v0.1.0-rc.1` tag.
- [ ] `release.yml` builds wheels + uploads to **TestPyPI** (gated by `environment: release-pypi-test`).
- [ ] In a clean venv: `pip install --index-url https://test.pypi.org/simple/ xlstream` — works?
- [ ] Run the 400k reference workload from the installed wheel — passes.
- [ ] If any issue: fix, retag, re-release RC.

### Tag and push

- [ ] `git tag -s v0.1.0 -m "v0.1.0"` (signed tag).
- [ ] `git push origin v0.1.0`.

### Release automation

- [ ] `release.yml` triggered by tag push.
- [ ] Build wheels (Linux x86_64 / aarch64, macOS x86_64 / arm64, Windows x64).
- [ ] Build sdist.
- [ ] Manual approval in `environment: release-pypi`.
- [ ] `maturin upload` → PyPI.
- [ ] Manual approval in `environment: release-crates`.
- [ ] `cargo publish` each crate in order: core → parse, io → eval.
- [ ] GitHub Release page auto-created with notes from CHANGELOG.

### Smoke test published artefacts

- [ ] `pip install xlstream` in a clean venv on each OS — installs and imports cleanly.
- [ ] `cargo add xlstream-eval` in a fresh crate and write a 10-line Rust example — compiles and runs.

### Announcement

- [ ] GitHub Discussion post: "xlstream v0.1.0 released".
- [ ] Optional: `/r/rust` or `/r/Python` release post.
- [ ] Optional: Twitter / LinkedIn.
- [ ] Mention in any relevant downstream projects (xlformula) with a note on how to migrate from formualizer.

### Post-release

- [ ] Mark this phase's checklist fully ticked.
- [ ] Update `docs/phases/README.md` status column.
- [ ] Open next milestone issue: "v0.2.0 planning".
- [ ] Celebrate.

## If something goes wrong

### Bad PyPI wheel

- [ ] `twine yank xlstream==0.1.0` (or via PyPI UI).
- [ ] Fix, bump to 0.1.1, re-release.
- [ ] Document yank reason in CHANGELOG.

### Bad crates.io release

- [ ] `cargo yank --vers 0.1.0 xlstream-<crate>`.
- [ ] Fix, bump to 0.1.1, re-release.

### Git tag pushed by accident

- [ ] `git tag -d v0.1.0 && git push origin :refs/tags/v0.1.0`.
- [ ] This only works if nothing downstream (crates.io, PyPI) consumed it. If consumed: just bump to 0.1.1.

## Done when

- `pip install xlstream` works on all major OS.
- `cargo add xlstream-eval` works.
- GitHub Release page exists at `v0.1.0`.
- Announcement made.
- Team has celebrated.
