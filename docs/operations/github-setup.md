# GitHub repository setup

One-time tasks that have to be done in the GitHub UI (not in-repo). Do these before the first tagged release.

## 1. PyPI trusted publishing (no API tokens)

Modern PyPI uses **OIDC trusted publishing** — GitHub Actions authenticates to PyPI directly, no secrets to rotate or leak. This is what the `publish-pypi` job in `release.yml` uses.

### One-time PyPI setup

1. Sign in to https://pypi.org/.
2. **Option A (project already exists):** go to the `xlstream` project → Manage → Publishing → *Add a new publisher*.
3. **Option B (first-ever release, project doesn't exist yet):** Account → Publishing → *Add a pending publisher* (PyPI's pre-authorise flow).
4. Fill the form:

   | Field | Value |
   |---|---|
   | PyPI Project Name | `xlstream` |
   | Owner | `cilladev` |
   | Repository name | `xlstream` |
   | Workflow name | `release.yml` |
   | Environment name | `pypi` |

5. Save. PyPI now trusts pushes from your `release.yml` running under the `pypi` environment.

### One-time GitHub environment

1. Repo → Settings → Environments → *New environment*: `pypi`.
2. *Required reviewers*: add yourself (so publishes gate on a click).
3. *Deployment branches*: restrict to tags matching `v*`.
4. No secrets needed — OIDC handles auth.

## 2. TestPyPI rehearsal (optional)

Same flow against https://test.pypi.org. Create a separate environment `testpypi` and add a trusted publisher with the same owner/repo/workflow but pointed at a rehearsal workflow (or use a version suffix like `v0.1.0-rc.1` gated on a `testpypi` environment). Covered in the `release.md` operations doc.

## 3. crates.io publishing (still token-based)

crates.io does **not** support OIDC as of 2026, so the Rust publish path still needs a token.

1. crates.io → Account Settings → *New token*.
2. Scope: limit to the `xlstream-*` crates.
3. Copy the token.
4. Repo → Settings → Environments → *New environment*: `crates-io`.
5. *Required reviewers*: yourself.
6. *Environment secrets* → add `CARGO_REGISTRY_TOKEN` with the value from step 3.
7. *Deployment branches*: restrict to tags matching `v*`.

## 4. Branch protection on `main`

Repo → Settings → Branches → *Add rule* for `main`:

- ✅ Require a pull request before merging.
  - ✅ Require 1 approval.
  - ✅ Dismiss stale approvals when new commits are pushed.
- ✅ Require status checks to pass before merging.
  - Required checks (add as they appear green on the first real PR):
    - `pre-commit`
    - `ci / test (ubuntu-latest)` (and the macos / windows variants)
    - `ci / python (ubuntu-latest)` (and variants)
    - `ci / audit`
- ✅ Require branches to be up to date before merging.
- ❌ Allow force pushes — off.
- ❌ Allow deletions — off.

## 5. Dependabot alerts + security

Repo → Settings → Security & analysis:

- ✅ Dependabot alerts.
- ✅ Dependabot security updates.
- ✅ Secret scanning.
- ✅ Push protection (blocks commits containing secrets).

## 6. Actions permissions

Repo → Settings → Actions → *General*:

- *Workflow permissions*: "Read and write permissions" — needed for the nightly benchmark workflow to push to `gh-pages`.
- *Allow GitHub Actions to create and approve pull requests*: leave **off** unless a specific workflow needs it.

## 7. Issue / PR templates (optional, v0.1 polish)

Create `.github/ISSUE_TEMPLATE/bug_report.md`, `feature_request.md`, and `.github/pull_request_template.md`. Draft content lives in `docs/standards/commits.md` for the PR template. Not blocking for first release.

## Verification

After all steps:

```bash
# Push a pre-release tag.
git tag -s v0.1.0-rc.1 -m "v0.1.0-rc.1"
git push origin v0.1.0-rc.1
```

The `release.yml` workflow runs. `publish-pypi` and `publish-crates` pause at the environment approval gate. Approve in the GitHub UI. Wheels land on PyPI. Crates land on crates.io. If anything fails, the tag can be deleted and retried — it's a release *candidate*.

Then:

```bash
pip install xlstream==0.1.0rc1   # from a clean venv, each OS
```

If that works on Linux + macOS + Windows, the release-gate is functional. Proceed to v0.1.0.
