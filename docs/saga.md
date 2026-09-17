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

## core-mvp (2026-09-16, complete)

Thin vertical slice first: seven crates in six new component
workspaces (value, lex, parse, prims, eval, display, session),
each sized to the sw-checklist gates, wired into the CLI. Step
002 delivered scalar arithmetic with APL\360 error display; step
003 added iota, rho, ravel, catenate, reduce, quad output, and the
index origin: the MVP REPL. Then the direction change to pure
APL\360 (I-beams, the settings commands, no quad names), the vhs
tape, and the complete value, display, lexer, parser, and
scalar-function layers. Eleven steps.

## core-mixed (2026-09-16, complete)

Phase 2: the structural functions and operators, giving meaning to
every form the parser already accepted. Catenate and laminate with
axis; take, drop, reverse, rotate, transpose; compress, expand,
membership, index-of; grade, encode, decode, deal; reduce and scan
on any axis with the identity table; inner and outer products;
bracket indexing and indexed assignment; and a closing audit that
made an axis bracket a SYNTAX ERROR wherever APL\360 has no such
form. Two steps came from the owner mid-saga: line editing with
history in the REPL, and generating the glyph tables from
`data/glyphs.toml`. Conway's Life runs at the end of it, written
the APL\360 way with eight neighbour rotations.
