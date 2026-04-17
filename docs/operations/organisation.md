# GitHub organisation strategy

## Recommendation

**Personal account for now.** Move to an organisation when real need arises.

## When to create an org

Create a GitHub organisation when ALL of the following are true:

1. You have, or expect within 3 months, **2+ active contributors** who need roles distinct from "repo owner."
2. You want a **branded identity** for xlstream that's separate from your personal handle on business cards, talks, blog posts.
3. You plan to host **more than one related repo** — e.g. `xlstream`, `xlstream-site`, `xlstream-examples`.

None of these is true on day one. Don't pre-optimise.

## Why not create an org immediately

- **Admin overhead**: teams, permissions, billing, 2FA enforcement, merge queue rules. All of it wastes time until you have more than one admin to amortise it over.
- **SEO/branding switches** are cheap: transferring a repo to an org is a single click on GitHub, preserves stars/forks/issues/PRs/git history/CI.
- **Vanity**: an org with a single repo looks emptier than a personal account with a focused project.

## Migration plan (when the time comes)

1. Create org `xlstream` or similar on github.com.
2. In the repo: Settings > Danger Zone > Transfer > enter new owner.
3. Members: invite contributors with appropriate roles.
4. Update `package.repository` and `project.urls.homepage` in `Cargo.toml` / `pyproject.toml` — old URLs redirect, but new metadata is cleaner.
5. Update any external references: PyPI project page, crates.io profile links, blog posts, README badges.
6. Local clones: `git remote set-url origin git@github.com:xlstream/xlstream.git`.

Total elapsed time: 10 minutes. Plan it for a Saturday morning, not a release day.

## Name availability

If you pick an org name, check:
- GitHub: `github.com/<name>` — 404 means free.
- crates.io: `cargo search <name>` or `crates.io/crates/<name>`.
- PyPI: `pypi.org/project/<name>` — 404 means free.

xlstream appears unused on all three as of April 2026, but verify immediately before claiming.

## Public vs private

Public from day one. There's no business reason to keep it private, and public catches more eyes (including friendly ones who spot bugs).

## Related repos (future)

If we grow beyond one repo, each has a clear purpose:

- `xlstream` — the main library (current).
- `xlstream-site` — docs website source (maybe mdBook built from `docs/`, maybe a custom site). Optional.
- `xlstream-examples` — end-to-end example projects. Optional.
- `xlstream-benchmarks-public` — benchmark results over time, possibly hosted data. Probably not worth a separate repo; a `gh-pages` branch on the main repo covers it.

Don't split prematurely. A directory in the main repo beats a new repo 9 times out of 10.

## Monorepo, not polyrepo

We've already decided this — see [`repo-structure.md`](repo-structure.md). Org strategy above is about GitHub ownership, not code organisation.
