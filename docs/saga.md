# sw-apl Saga Log

Long-form notes per saga. `CHANGES.md` is the per-commit log;
`.agentrail/` holds the live saga; `.agentrail-archive/` holds
finished ones.

## bootstrap (2026-09-16)

Repository scaffold, planning documents, and the `sw-apl` CLI
skeleton. Established the process: agentrail sagas derived from
`docs/plan.md`, TDD per step, reg-rs transcripts per sample,
sw-checklist gates as design constraints, ASCII-only markdown
with glyph material in `.txt`/`.apl`/config files.

## core-mvp (2026-09-16, in progress)

Thin vertical slice first: seven crates in six new component
workspaces (value, lex, parse, prims, eval, display, session),
each sized to the sw-checklist gates, wired into the CLI. Step
002 delivered scalar arithmetic with APL\360 error display; step
003 added iota, rho, ravel, catenate, reduce, quad output, and
the index origin (two more prims crates, an eval dispatch module): the MVP
REPL. Steps 004 to 008 complete each layer.
