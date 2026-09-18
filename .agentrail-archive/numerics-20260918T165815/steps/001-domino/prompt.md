Phase 5 step 1 (docs/plan.md). Matrix divide and matrix inverse.

`⌹` is the only `todo` left in the primitives table of
docs/parity.md. Monadic `⌹B` is the inverse of B; dyadic `A⌹B`
solves `B⊥.×X ←→ A` for X -- and when B has more rows than columns
that is the least-squares solution, which is the whole point of the
glyph. A version that only inverts square matrices would be a stub
wearing the name.

Use a Householder QR factorisation rather than forming and solving
the normal equations. The normal equations square the condition
number, and the overdetermined fit is exactly the case that makes
this primitive worth having; a fit that quietly loses half its
digits is worse than no fit.

The rank cases are where this is usually got wrong, so decide each
one deliberately and test it:

- B a matrix, A a matrix: one solution column per column of A.
- B a matrix, A a vector: one solution vector.
- B a vector: treated as a one-column matrix, so `A⌹B` is the
  least-squares scalar, and `⌹B` of a vector is its pseudo-inverse.
- B a scalar: `⌹B` is `÷B`, and `A⌹B` is `A÷B`.
- Anything of rank 3 or more is RANK ERROR.
- B with more columns than rows is DOMAIN ERROR: it is
  underdetermined, and APL\360 did not choose a solution for you.
- A singular or rank-deficient B is DOMAIN ERROR. Decide the
  tolerance for "singular" and say in the code why that number and
  not another; do not use the comparison fuzz just because it is
  there.
- Character data is DOMAIN ERROR.
- A and B with different numbers of rows is LENGTH ERROR.

Check what the IBM APL\360 User's Manual (Aug 1968) says about `⌹`
before choosing behaviour, and quote it in the commit where it
settles a question. Where it does not, say so and say what you chose
instead. The manual text is at
https://archive.org/stream/bitsavers_ibmaplAPL3_8068299/APL_360_Users_Manual_Aug68_djvu.txt
-- note that domino may be a later addition than Aug 1968, in which
case say that plainly and name the source you did follow.

Watch the display: a solution is floating point and will print under
`)DIGITS`, so a sample's transcript pins the printing as much as the
arithmetic. Prefer examples whose answers are exact in binary
floating point (halves, quarters) for the transcript, and test the
inexact ones in Rust where a tolerance can be stated.

TDD; reg-rs for anything run through the binary; update
docs/parity.md rows in the same commit; commit, push, report.
