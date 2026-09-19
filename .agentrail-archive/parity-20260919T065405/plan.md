# parity

Phase 7 of docs/plan.md, owner direction 2026-09-18: clear the last
rows of the checklist before starting a new component.

Five rows are left. Two are real gaps, and three are restrictions --
things this implementation does not do and will not, which is a
different statement from work outstanding. Carrying a restriction as
`todo` says the wrong thing to anyone reading the file, including
the next agent.

The two real ones share a shape: sw-apl answers NOT IMPLEMENTED or
nothing at all where APL\360 had a definite answer. NOT IMPLEMENTED
is a placeholder this project put in deliberately and undertook to
remove; the row saying so has been there since Phase 1.

Read the manual rather than recalling it, and quote it where it
settles a question. The text is at
https://archive.org/stream/bitsavers_ibmaplAPL3_8068299/APL_360_Users_Manual_Aug68_djvu.txt

Every step: format first, then tests, clippy, and gates (see
/mw-cp); TDD; reg-rs for anything run through the binary; update
docs/parity.md rows in the same commit; commit, push, report.

## Steps

1. valence-syntax-error -- a glyph used where it has no meaning is
   a SYNTAX ERROR, and `ErrorKind::NotImplemented` goes.
2. open-definition-guard -- NOT WITH OPEN DEFINITION for the
   commands the manual gives it to.
3. documented-restrictions -- say what this implementation does not
   do, in a Restrictions section, and stop calling it todo.
