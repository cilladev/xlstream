# Release

Every release produces two artefact families:
- **Rust crates** on crates.io (`xlstream-core`, `xlstream-parse`, `xlstream-io`, `xlstream-eval`, `xlstream`).
- **Python wheels + sdist** on PyPI (`xlstream`).

Versions are locked in step across all crates and bindings.

## How to release

### 1. Create a release branch

```bash
git checkout main && git pull
git checkout -b release/v0.5.0
```

### 2. Complete the checklist

Work through these on the release branch. Each item is a commit.

- [ ] All roadmap checkboxes for this version are ticked
- [ ] `CHANGELOG.md` — promote `[Unreleased]` to `[0.5.0] - YYYY-MM-DD`
- [ ] Docs reviewed — no TODOs, no broken links, no stale version references
- [ ] Benchmarks pass — `make bench-report VERSION=0.5.0`, reference workload within budget
- [ ] Version bumped — `make bump VERSION=0.5.0`
- [ ] Dry run passes — `make release-dry`
- [ ] Commit: `chore: bump to 0.5.0`

### 3. Open a PR and merge to main

```bash
git push -u origin release/v0.5.0
gh pr create --base main --title "release: v0.5.0" --body "Release checklist for v0.5.0"
```

Get CI green, then merge. On merge, CI auto-tags `v0.5.0` from `Cargo.toml` and runs the build + publish pipeline.

### 4. Approve publishing

CI builds wheels and sdist automatically. Publishing is gated on manual approval:

- **PyPI** — approve in the `pypi` environment
- **crates.io** — approve in the `crates-io` environment

A GitHub Release is created automatically with changelog notes and wheel artifacts.

### 5. Verify

- [ ] `pip install xlstream==0.5.0` works on a clean machine
- [ ] `cargo add xlstream-eval@0.5.0` pulls the new version
- [ ] GitHub Release page has wheels + sdist + changelog

## Versioning

Semver. Version is defined once in `Cargo.toml` workspace `[workspace.package]`. `pyproject.toml` reads it dynamically via maturin. `make bump VERSION=x.y.z` updates all inter-crate dependency pins.

- **0.x.y**: pre-1.0. Minor bumps allowed for breaking changes.
- **x.y.0**: minor release, may include new features.
- **x.y.z**: patch, no new features.
- Tag format: `v0.5.0` (prefix `v`).

## CI pipeline (`.github/workflows/release.yml`)

On push to main, the workflow:

1. **Auto-tags** — reads version from `Cargo.toml`, creates the tag if it doesn't exist.
2. **Builds wheels** — Linux x86_64/aarch64, macOS x86_64/arm64, Windows x64 (abi3-py39).
3. **Builds sdist** — source distribution for platforms without a prebuilt wheel.
4. **Publishes to PyPI** — gated on `pypi` environment approval. Trusted Publishing (OIDC, no token).
5. **Publishes to crates.io** — gated on `crates-io` environment approval. Uses `CARGO_REGISTRY_TOKEN`.
6. **Creates GitHub Release** — extracts changelog section, attaches all artifacts.

### Crate publish order

```
xlstream-core       (no deps on siblings)
xlstream-parse      (deps: xlstream-core)
xlstream-io         (deps: xlstream-core)
xlstream-eval       (deps: xlstream-core, xlstream-parse, xlstream-io)
xlstream            (facade crate, deps: all above)
```

`xlstream-cli` is not published (dev-only). `bindings/python` is published to PyPI, not crates.io.

## Hotfix releases

```bash
git checkout main && git pull
git checkout -b release/v0.5.1
# cherry-pick the fix, bump, same checklist
```

If the bug is on main but hasn't shipped, just fix on main and release normally.

## Yanking

If a published release has a serious bug:

- `cargo yank --vers 0.5.0 xlstream-core` (and each crate)
- Yank from PyPI via the project page
- Publish a patch with the fix
- Document in `CHANGELOG.md` that the version was yanked

Do NOT delete the git tag.

## Publishing credentials

### PyPI — trusted publishing (OIDC, no token)

GitHub Actions authenticates to PyPI directly via OIDC. Setup documented in [`github-setup.md`](github-setup.md#1-pypi-trusted-publishing-no-api-tokens).

- No `PYPI_API_TOKEN` secret needed.
- `release.yml` uses `pypa/gh-action-pypi-publish@release/v1` under the `pypi` environment.
- To revoke: remove the trusted publisher on the PyPI project page.

### crates.io — API token

| Credential | Location | Rotation |
|---|---|---|
| `CARGO_REGISTRY_TOKEN` | GitHub environment `crates-io`, secret | Rotate annually or on suspicion |

Never expose outside CI. Token is scoped to the `xlstream-*` crates.
