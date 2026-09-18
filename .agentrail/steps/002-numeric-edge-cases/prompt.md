Phase 5 step 2 (docs/plan.md). The numeric edges.

Everything so far has worked in the middle of the range. This step
goes to the boundaries and pins them, so that a later change cannot
move one without a test noticing.

At least these, and add what you find:

- Integer overflow. `Number::exact_int` uses checked arithmetic and
  falls back to the floating path; find the exact point where
  `2*63` style arithmetic stops being an `Int` and pin it, in both
  directions, and for `+`, `-` and `×`.
- The comparison tolerance. `FUZZ` is 1e-13 and relative. Pin what
  it does to `=`, `≠`, `<` and friends, and to `⌊` and `⌈` -- a
  value a hair under an integer floors to that integer in APL, and
  the boundary of "a hair" is a number this repo has never stated.
- `⍳` of a computed length. `⍳2.0000000000001` and `⍳` of a number
  that is not whole within the fuzz are different answers; find
  which is which.
- Large `⍳`. There is a workspace quota now, so `⍳` of something
  enormous is WS FULL when it is assigned but not when it is only
  counted. Pin both.
- Exponent notation, in and out. `1E10`, `1E¯10`, `1.5E300`,
  numbers that need more digits than `)DIGITS` allows, and the
  round trip through a saved workspace, which writes literals.
- `)DIGITS 1` and `)DIGITS 16`, against values that expose the
  difference.

APL\360 was specific about several of these. Where the manual
settles one, follow it and say where it says so; where it does not,
choose, and record in parity.md that the choice is ours. That
distinction is the deliverable as much as the tests are.

Some of these belong in a sample and some do not: a transcript that
pins a float's last digit is a baseline that will break for the
wrong reason. Put the exact ones in a sample and the inexact ones in
Rust tests where a tolerance can be written down.

TDD; reg-rs for anything run through the binary; update
docs/parity.md rows in the same commit; commit, push, report.
