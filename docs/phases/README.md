# Phases — roadmap

> **Current phase: 7 — Aggregates.** See [`phase-07-aggregates.md`](phase-07-aggregates.md).

## How this works

Every phase is a markdown file with a checklist. Agents pick up one checkbox at a time. When you finish a checkbox:

1. Tick it (`[x]`) in the phase doc.
2. If the checkbox ships a builtin function, also tick it in [`../functions.md`](../functions.md).
3. Commit: `<crate>: <what you did>`.
4. PR.
5. Next agent picks up the next unticked box.

Do NOT work multiple phases simultaneously. Do NOT skip checkboxes out of order unless the phase doc explicitly says they're independent.

## Phase dependency graph

```
 0 (foundation) ──► 1 (scaffolding) ──► 2 (parser) ──► 3 (I/O) ──┐
                                                                  │
                                                                  ▼
                                        ┌─────────────► 4 (streaming core)
                                        │                         │
                                        │                         ▼
                                        │         5 (arithmetic) ─┤
                                        │                         ▼
                                        │         6 (conditional) ┤
                                        │                         ▼
                                        │         7 (aggregates)  ┤
                                        │                         ▼
                                        │         8 (lookups)     ┤
                                        │                         ▼
                                        │         9 (strings/dates/math) ─┐
                                        │                                 │
                                        ▼                                 │
                                    10 (parallelism) ◄───────────────────┤
                                        │                                 │
                                        ▼                                 │
                                    11 (python binding)                   │
                                        │                                 │
                                        ▼                                 │
                                    12 (benchmarks)                       │
                                        │                                 │
                                        ▼                                 │
                                    13 (docs) ◄───────────────────────────┘
                                        │
                                        ▼
                                    14 (release v0.1.0)
```

5 / 6 / 7 / 8 / 9 can overlap — they're different builtins. Within each, follow that phase's internal order.

## Phase list

| # | Phase | File | Status |
|---|---|---|---|
| 0 | Foundation | [`phase-00-foundation.md`](phase-00-foundation.md) | ✓ complete |
| 1 | Scaffolding | [`phase-01-scaffolding.md`](phase-01-scaffolding.md) | ✓ complete |
| 2 | Parser integration | [`phase-02-parser.md`](phase-02-parser.md) | ✓ complete |
| 3 | I/O layer | [`phase-03-io.md`](phase-03-io.md) | ✓ complete |
| 4 | Streaming core | [`phase-04-streaming-core.md`](phase-04-streaming-core.md) | ✓ complete |
| 5 | Arithmetic, comparison, concat | [`phase-05-arithmetic.md`](phase-05-arithmetic.md) | ✓ complete |
| 6 | Conditionals, logical | [`phase-06-conditional.md`](phase-06-conditional.md) | complete |
| 7 | Aggregates | [`phase-07-aggregates.md`](phase-07-aggregates.md) | in progress |
| 8 | Lookups | [`phase-08-lookups.md`](phase-08-lookups.md) | in progress |
| 9 | Strings, dates, math | [`phase-09-strings-dates-math.md`](phase-09-strings-dates-math.md) | in progress |
| 10 | Parallelism | [`phase-10-parallelism.md`](phase-10-parallelism.md) | not started |
| 11 | Python binding | [`phase-11-python-binding.md`](phase-11-python-binding.md) | not started |
| 12 | Benchmarks | [`phase-12-benchmarks.md`](phase-12-benchmarks.md) | not started |
| 13 | Docs polish | [`phase-13-docs.md`](phase-13-docs.md) | not started |
| 14 | Release v0.1.0 | [`phase-14-release.md`](phase-14-release.md) | not started |

## Estimated effort

Cumulative estimate for all phases: **6–10 weeks** of focused work by an AI agent + human review.

Individual phase estimates are in each phase doc.

## Rules for agents

1. Read the phase doc top-to-bottom before starting.
2. Read the architecture docs referenced in the phase.
3. Pick one unticked checkbox.
4. Do one checkbox per PR (unless the phase doc marks checkboxes as bundled).
5. Tick the box in the same PR that lands the work.
6. Never tick a box you didn't actually finish.
7. If blocked, ask — don't guess. Add a note to the phase doc: `> BLOCKED: <reason>`.

## When a phase is done

All checkboxes ticked. Run the full local check. Update the status in this README. Move to the next phase.

## Changing the plan

If a phase needs to change (new insight, requirements shift), open a PR to this README + the affected phase doc. Don't change phases silently.
