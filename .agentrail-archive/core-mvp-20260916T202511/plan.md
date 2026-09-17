# core-mvp

DIRECTION CHANGE 2026-09-16 (owner): pure APL\360. There are NO
quad-named system variables or functions (no ⎕IO, ⎕CT, ⎕PP, ⎕RL,
⎕EX ...): read docs/plan.md "Owner decisions" item 5. Wherever a
step prompt below or in its step file says quad-PP / quad-PW /
quad-IO / quad-CT / quad-RL, read )DIGITS / )WIDTH / )ORIGIN / the
fixed fuzz / the workspace random link. Execute and format are out.
System information comes from I-beam functions (Phase 3).

Phase 1 of docs/plan.md. Steps 2 and 3 were the thin vertical slice
(the MVP REPL); the remaining steps widen each layer to the full
APL\360 set. Every step: TDD, sw-checklist gates as design
constraints, commit, push, report.

## Steps

1. fold-owner-answers -- done.
2. mvp-scalar-arithmetic -- done.
3. mvp-iota-rho-reduce -- done.
4. readme-vhs-tape -- done.
5. value-model-complete -- done.
6. pure-apl360-ibeams -- this direction change: docs, keymaps,
   lexer/parser/eval without quad names, )ORIGIN and )DIGITS with
   the WAS reply, I-beam glyph accepted.
7. display-complete -- exponential form, )WIDTH wrapping with
   six-space continuation, character arrays, empty output, mixed
   columns, rank > 2.
8. lexer-complete -- full glyph set, strings, delta letters, system
   command lines, del sentinel, CHARACTER ERROR for lookalikes.
9. parser-complete -- operators with axis, bracket indexing,
   indexed assignment, branch, quad/quote-quad both sides.
10. scalar-functions-complete -- every scalar primitive with
    rank/length checks, fixed-fuzz tolerance, circular,
    factorial/binomial, roll via the workspace random link.
