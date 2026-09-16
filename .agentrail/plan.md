# core-mvp

Phase 1 of docs/plan.md. Steps 1 and 2 are a thin vertical slice
so the owner can evaluate simple APL at the six-space prompt as
early as possible (the MVP REPL); steps 3 to 7 widen each layer to
the full APL\360 set. Every step: TDD (unit tests first, then a
samples/*.apl transcript where visible), sw-checklist gates as
design constraints (<= 25 LOC/fn, <= 4 fns/module, <= 4
modules/crate; split before adding), commit, push, report.

## Steps

1. mvp-scalar-arithmetic -- new component workspaces value, lex,
   parse, eval, display; wire sw-apl. 2+2, strands, + - x / max
   min | * on scalars and vectors with scalar extension,
   assignment and display, APL\360 error display with caret.
   reg-rs baselines for samples 01-03.
2. mvp-iota-rho-reduce -- iota, rho (shape/reshape), ravel and
   catenate, reduce on last axis, matrix display, quad-IO. The
   MVP: the owner can play. reg-rs baselines for samples 04-06.
3. value-model-complete -- quad-CT tolerant equality,
   promotion/demotion, empties, rank > 2, full error enum.
4. display-complete -- exponential form, quad-PW wrapping,
   character arrays, empty output, mixed columns.
5. lexer-complete -- full glyph set, strings, quad names, delta
   letters, system command lines, del sentinel, CHARACTER ERROR
   for lookalikes.
6. parser-complete -- operators with axis, bracket indexing,
   indexed assignment, branch, quad/quote-quad both sides.
7. scalar-functions-complete -- every scalar primitive with
   rank/length checks, tolerance, circular, factorial/binomial,
   roll.
