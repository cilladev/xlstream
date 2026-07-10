# Toolchain bump: 1.88.0 -> 1.94.1 (LTO bug dropping encoding_rs statics)

**Date:** 2026-07-10
**Status:** accepted

## Context

calamine 0.36 (RUSTSEC fix, PR #199) pulls in `encoding_rs`. rustc 1.88's
fat LTO internalizes and then drops `encoding_rs`'s `pub static` encoding
tables, leaving undefined symbols at link time. Any binary that references
`calamine::Xlsx::new` under `lto = "fat"` fails to link:

- Linux abi3 Python wheel (caught in PR #199 CI; worked around with the
  thin-LTO `[profile.wheel]`)
- macOS release CLI (caught by `make bench-report` on release/v0.4.0)

Verified matrix: 1.88 + fat = link failure; 1.88 + thin = OK;
1.94.1 + fat = OK. The bug is fixed upstream between 1.89 and 1.94.

## Decision

Pin 1.94.1 (current stable). Keep `lto = "fat"` for release and bench
profiles — the perf posture is unchanged. Keep `rust-version = "1.88"`
(the code still compiles on 1.88; only fat-LTO release builds need newer).
Keep the thin-LTO `[profile.wheel]` for now — revisit once wheels are
confirmed good on 1.94 across all release targets.

Fallout: 9 new clippy lints (8x `manual_is_multiple_of`, 1x derivable
impl), all mechanical, fixed in the same commit.

## Consequences

- Bench numbers from 1.94 are not directly comparable to 1.88-era
  baselines; the v0.4.0 bench report is the first on the new toolchain.
- CI bench-gate baseline will shift on first merge; expect one noisy
  comparison (see #201 for the gate's known cross-run weakness).
