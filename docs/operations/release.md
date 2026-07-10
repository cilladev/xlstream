# Release

Every release produces two artefact families:
- **Rust crates** on crates.io (`xlstream-core`, `xlstream-parse`, `xlstream-io`, `xlstream-eval`, `xlstream`).
- **Python wheels + sdist** on PyPI (`xlstream`).

Versions are locked in step. v0.4.0 is v0.4.0 across everything.

## Versioning

Semver.

- **0.x.y**: pre-1.0. Minor bumps allowed for breaking changes.
- **x.y.0**: minor release, may include new features.
- **x.y.z**: patch, no new features.
- Tag: `v0.4.0` (prefix `v`).

Version is defined once in `Cargo.toml` workspace `[workspace.package]` and `bindings/python/pyproject.toml` (via `dynamic = ["version"]` reading from Cargo). They MUST match. A pre-push hook or CI check enforces this.

## Pre-release checklist

Before merging the version bump to main:

1. **All roadmap checkboxes** for this release's scope are ticked.
2. **CHANGELOG.md** updated — `[Unreleased]` promoted to `[0.4.0] - YYYY-MM-DD`.
3. **Docs reviewed** — no TODOs, no broken links.
4. **Benchmarks pass** — `make bench-report VERSION=0.4.0`, reference workload within budget.
5. **`make release-dry`** works (`cargo publish --dry-run` for each crate + `maturin build`).
6. **Version bumped** in `Cargo.toml` workspace and `pyproject.toml`.
7. **Commit**: `chore: bump to 0.4.0`.
8. **Merge to main** — CI auto-tags `v0.4.0` and triggers the release pipeline.

## Release pipeline (automated)

Push to `main` triggers `release.yml`:

1. **Auto-tag** — reads version from `Cargo.toml`, creates `v0.4.0` tag if missing.
2. Tag push triggers the build + publish jobs:
   - **Build wheels** — Linux x86_64/aarch64, macOS x86_64/arm64, Windows x64 (abi3-py39).
   - **Build sdist** — source distribution for platforms without prebuilt wheels.
3. **Publish to PyPI** — gated on manual approval (`pypi` environment). Uses Trusted Publishing (OIDC, no token).
4. **Publish to crates.io** — gated on manual approval (`crates-io` environment). Uses `CARGO_REGISTRY_TOKEN`.
5. **GitHub Release** — extracts changelog section, attaches wheels + sdist.

PyPI and crates.io publish independently (both depend on the build step, not each other). GitHub Release also publishes independently.

## Crate publish order

```
xlstream-core       (no deps on siblings)
xlstream-parse      (deps: xlstream-core)
xlstream-io         (deps: xlstream-core)
xlstream-eval       (deps: xlstream-core, xlstream-parse, xlstream-io)
xlstream            (facade crate, deps: all above)
```

crates.io requires published dependencies before publishing a dependent. `release.yml` runs them sequentially.

`xlstream-cli` is not published (dev-only). `bindings/python` is published to PyPI, not crates.io.

## Post-release

1. Verify `pip install xlstream` works on a clean machine for each major OS.
2. Verify `cargo add xlstream-eval` pulls the new version.
3. Announcement post (GitHub Discussion, optionally social).

## Hotfix releases

Patch versions (0.4.1, 0.4.2) are cherry-picks from main onto a release branch (`release/0.4.x`), then the same workflow: bump version, merge, auto-tag.

If the bug is on main but hasn't shipped, just fix on main and bump.

## Yanking

If a published release has a serious bug:

- `cargo yank --vers 0.4.0 xlstream-core` (and each crate).
- Yank from PyPI via the PyPI project page.
- Publish a patch with the fix.
- Document in `CHANGELOG.md` that the version was yanked.

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

## Pre-publish `cargo publish --dry-run`

Always dry-run first. Catches edge cases like missing `description` fields, files that shouldn't be packaged, etc.

```bash
make release-dry
```
