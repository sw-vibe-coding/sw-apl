# numerics

Phase 5 of docs/plan.md: the last unimplemented primitive, and the
numeric edges everything else has been avoiding.

`⌹` is the only `todo` left in the primitives table of
docs/parity.md. Monadic it is matrix inverse; dyadic it is matrix
divide, which for a non-square left argument is the least-squares
solution -- that is the whole point of the glyph, and a version that
only inverts square matrices would be a stub wearing its name.

The edges are the other half. Floating point, a comparison
tolerance, `)DIGITS` from 1 to 16, exponent notation on the way in
and on the way out: each of those has a boundary, and none of them
has a test that sits on it. APL\360 was specific about several --
integer overflow becoming floating point, the fuzz used for
comparison and for `⌊`, what `⍳` does with a number too large --
and where the manual is specific, follow it and say where it says
so.

Pure APL\360 throughout: no `⎕CT`, no `⎕FC`, no system variables of
any kind. Every step: format first, then tests, clippy, and gates
(see /mw-cp); TDD; reg-rs for anything run through the binary, Rust
tests for the libraries; update docs/parity.md rows in the same
commit; commit, push, report.

## Steps

1. domino -- `⌹B` matrix inverse and `A⌹B` matrix divide, including
   the least-squares case where B has more rows than columns.
   Householder QR rather than a normal-equations shortcut, because
   the normal equations square the condition number and the whole
   reason to have this glyph is the overdetermined fit. Vectors and
   scalars are the rank cases to get right; a singular matrix is
   DOMAIN ERROR.
2. numeric-edge-cases -- overflow from integer to floating point,
   the comparison tolerance and what it does to `=`, `⌊`, `⌈` and
   `⍳` of a computed length, large `⍳`, exponent notation round
   trips, and `)DIGITS` at 1 and at 16. Find the boundaries, pin
   them, and record in parity.md which are APL\360's and which are
   ours.
