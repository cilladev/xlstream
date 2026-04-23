# xlstream-cli

Development CLI for the [xlstream](https://github.com/cilladev/xlstream) streaming Excel evaluator. Not published to crates.io — internal tool for debugging and ad-hoc evaluation.

## Commands

```
xlstream evaluate <INPUT> --output <OUTPUT> [--workers N] [--verbose]
xlstream classify <FORMULA> [--sheet SHEET] [--row ROW] [--col COL] [--lookup-sheet SHEET]...
```

**evaluate** -- run the full pipeline (read, parse, classify, prelude, evaluate, write). `--verbose` prints per-sheet stats and peak RSS.

**classify** -- parse and classify a single formula. Shows streaming verdict, classification, and any rejection reasons.

## Dependencies

`xlstream-core`, `xlstream-parse`, `xlstream-eval`, `xlstream-io`, `clap`, `tracing`, `tracing-subscriber`, `memory-stats`.

## License

Dual-licensed under Apache-2.0 or MIT, at your option.
