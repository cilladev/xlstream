## Summary

<!-- What changed and why. 1-3 bullet points. -->

## Test plan

- [ ] `make check` passes (fmt + clippy + tests + doctests)
- [ ] New tests added for changed behavior
- [ ] Existing tests still pass

<!-- Test location guide:
  - Unit tests: in-module `#[cfg(test)] mod tests`
  - Per-category integration: `crates/xlstream-eval/tests/<category>.rs`
  - End-to-end pipeline: `crates/xlstream-eval/tests/end_to_end/<feature>.rs`
  - See docs/standards/testing.md for the full test structure.
-->

## Checklist

- [ ] Read `CONTRIBUTING.md` and `docs/standards/` before starting
- [ ] No `unwrap`/`expect`/`panic` in library code
- [ ] Public items have rustdoc + doctest
- [ ] Commit messages follow `<prefix>: <imperative, lowercase>` format
- [ ] Roadmap checkboxes updated (if applicable)
- [ ] `docs/functions.md` updated (if new function added)
- [ ] Criterion benchmark added (if novel code path introduced)
