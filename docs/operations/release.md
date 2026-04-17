# Release

Every release produces two artefact families:
- **Rust crates** on crates.io (`xlstream-core`, `xlstream-parse`, `xlstream-io`, `xlstream-eval`).
- **Python wheels + sdist** on PyPI (`xlstream`).

Versions are locked in step. v0.1.0 is v0.1.0 across everything.

## Versioning

Semver.

- **0.x.y**: pre-1.0. Minor bumps allowed for breaking changes.
- **x.y.0**: minor release, may include new features.
- **x.y.z**: patch, no new features.
- Tag: `v0.1.0` (prefix `v`).

Version is defined once in `Cargo.toml` workspace `[workspace.package]` and `bindings/python/pyproject.toml` (via `dynamic = ["version"]` reading from Cargo). They MUST match. A pre-push hook or CI check enforces this.

## Pre-release checklist

Before tagging:

1. **CHANGELOG.md** updated — `[Unreleased]` promoted to `[0.1.0] - YYYY-MM-DD`.
2. **All phase checklists** for this release's scope are ticked.
3. **Docs reviewed** — no TODOs, no broken links.
4. **Benchmarks pass** — reference workload within budget.
5. **`cargo publish --dry-run`** works for each crate.
6. **`maturin build --release`** works locally.
7. **Rust version bumps** in `Cargo.toml` and `pyproject.toml`.
8. **Commit the bump**: `chore: bump to 0.1.0`.
9. **Tag**: `git tag -s v0.1.0 -m "v0.1.0"` then `git push origin v0.1.0`.

## Release workflow (automated)

Tag push triggers `.github/workflows/release.yml`:

1. Build wheels on Linux x86_64 / aarch64, macOS x86_64 / arm64, Windows x64.
2. Build sdist.
3. Upload artefacts.
4. **Manual approval gate** (`environment: release-pypi`).
5. `maturin upload` to PyPI.
6. **Manual approval gate** (`environment: release-crates`).
7. `cargo publish` each crate in dependency order.
8. GitHub Release created with auto-generated notes.

Manual approval gates ensure we don't accidentally publish a bad release.

## Post-release

1. Verify `pip install xlstream` works on a clean machine for each major OS.
2. Verify `cargo add xlstream-eval` pulls the new version.
3. Announcement post (GitHub Discussion, optionally social).
4. Move README `Status: pre-alpha` to `Status: v0.1.0` or remove.

## Hotfix releases

Patch versions (0.1.1, 0.1.2) are cherry-picks from main onto a release branch, then the same workflow. Release branches are `release/0.1.x`.

If the bug is on main but hasn't shipped, just fix on main and bump.

## Yanking

If a published release has a serious bug:

- `cargo yank --vers 0.1.0 xlstream-core` (and each crate).
- `twine yank xlstream==0.1.0` (via maturin or PyPI UI).
- Publish a patch with the fix.
- Document in `CHANGELOG.md` that 0.1.0 was yanked.

Do NOT delete the git tag.

## Publishing credentials

### PyPI — trusted publishing (OIDC, no token)

We use PyPI's Trusted Publishing flow. GitHub Actions authenticates to PyPI directly via OIDC — no API token is stored. One-time setup in the GitHub + PyPI UI is documented in [`github-setup.md`](github-setup.md#1-pypi-trusted-publishing-no-api-tokens).

In practice this means:
- No `PYPI_API_TOKEN` secret.
- Our `release.yml` uses `pypa/gh-action-pypi-publish@release/v1` with `permissions: id-token: write` under the `pypi` environment.
- To revoke access: remove the trusted publisher on the PyPI project page. No secret rotation needed.

### crates.io — API token (no OIDC support yet)

crates.io does not yet support OIDC, so this one stays on tokens.

| Credential | Location | Rotation |
|---|---|---|
| `CARGO_REGISTRY_TOKEN` | GitHub environment `crates-io`, secret | Rotate annually or on suspicion |

Never expose outside CI. Token is scoped to just the `xlstream-*` crates.

## First release (v0.1.0) specifics

- Announce on Reddit `/r/rust` and `/r/Python`, plus relevant Discord channels.
- PyPI description is the `README.md`.
- crates.io description is the first line of each crate's `README.md`.
- Open discussion posts on any prior-art projects (formualizer) cross-linking for users who hit memory issues.

## Crate publish order

```
xlstream-core       (no deps on siblings)
xlstream-parse      (deps: xlstream-core)
xlstream-io         (deps: xlstream-core)
xlstream-eval       (deps: xlstream-core, xlstream-parse, xlstream-io)
```

crates.io requires published dependencies before publishing a dependent. `release.yml` runs them sequentially.

`xlstream-cli` is not published (dev-only). `bindings/python` is published to PyPI, not crates.io.

## Pre-publish `cargo publish --dry-run`

Always dry-run first. Catches unbilletable edge cases like missing `description` fields, files that shouldn't be packaged, etc.

```bash
for c in xlstream-core xlstream-parse xlstream-io xlstream-eval; do
  cargo publish -p $c --dry-run
done
```
