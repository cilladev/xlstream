# Security policy

## Reporting a vulnerability

**Do NOT open a public GitHub issue.**

Use GitHub's private vulnerability reporting:
`https://github.com/cilladev/xlstream/security/advisories/new`

Or email the maintainer listed in the repository metadata. We aim to acknowledge within 48 hours and provide a fix or mitigation plan within 7 days for critical issues.

## Supported versions

v0.1.x — best-effort support. Until v1.0.0, security fixes are released as patch versions of the current minor line.

## Scope

In scope:
- Panics on malformed xlsx input.
- OOM or timeouts from crafted xlsx files.
- Dependency vulnerabilities exposed by our API surface.
- Unsafe memory access (we are safe-Rust only; any `unsafe` bug is in scope).

Out of scope:
- Bugs that require the attacker to already have write access to the machine running xlstream.
- General denial-of-service by huge legitimate inputs (we document limits; file an issue if a limit is wrong).
- Side-channel attacks.

## Design notes

See [`docs/standards/security.md`](docs/standards/security.md) for the full threat model and mitigations.
