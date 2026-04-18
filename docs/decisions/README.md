# Architecture Decision Records (ADRs)

Long-lived decisions that explain *why* a path was chosen — context beyond what fits in a phase doc, commit message, or architecture page.

## Conventions

- **Filename:** `YYYY-MM-DD-<kebab-case-topic>.md`. The date is the date the ADR was accepted (or first drafted if still pending), not the date the underlying change ships.
- **Sections, in this order:** `## Context` → `## Decision` → `## Consequences` → `## Alternatives considered`.
- **Status line** as the first line after the title:

  ```
  **Status:** Accepted
  ```

  Allowed values: `Accepted`, `Proposed`, `Superseded by [<filename>](./<filename>)`, `Rejected`.
- One ADR per decision. Do **not** edit an accepted ADR. If the decision is reversed, write a new ADR that references and supersedes the old one; update the old ADR's status to `Superseded by ...`.

## Index

| Date | ADR | Status |
|---|---|---|
| 2026-04-18 | [Bump Rust toolchain 1.85 → 1.88 to integrate formualizer-parse 1.1.2](2026-04-18-phase-02-toolchain-bump.md) | Accepted |
