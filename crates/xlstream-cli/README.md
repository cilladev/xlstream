# xlstream-cli

Development CLI for the xlstream streaming Excel evaluator. Not published -- internal tool for smoke tests and ad-hoc evaluation.

## Commands

```
xlstream evaluate <INPUT> --output <OUTPUT> [--workers N] [--verbose]
xlstream classify <FORMULA> [--sheet SHEET] [--row ROW] [--col COL] [--lookup-sheet SHEET]...
```

**evaluate** -- run the full pipeline (read, parse, classify, prelude, evaluate, write).

**classify** -- parse and classify a single formula. Shows streaming verdict and any rejection reasons.

## Dependencies

`xlstream-core`, `xlstream-parse`, `xlstream-eval`, `xlstream-io`, `clap`, `tracing`, `tracing-subscriber`, `memory-stats`.
