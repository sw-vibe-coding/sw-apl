Found 2026-09-22 building step 014 (system variables): what that step
left out, each recorded in docs/mode-b.md under "The system variables
as built" and in docs/parity.md as not implemented.

- A settable comparison tolerance. sw-apl's fuzz is a constant
  (FUZZ in apl-value), so ⎕CT reads 1E¯13 and any other value is a
  NONCE ERROR. The 5110 lets it be set (Chapter 5), and it affects the
  relations, floor, ceiling, membership and index of. Thread it through
  the primitives from the workspace, as the index origin is, with (A)
  fixed at 1E¯13 and every (A) transcript unmoved.
- System variables localized in a function header, as in
  R←F;⎕IO -- the header parser lexes with no mode glyphs today, so a
  quad name is a SYNTAX ERROR there. A localized setting is restored
  on return; the manual's IMPLICIT ERROR for one used while undefined
  needs a decision (sw-apl refuses bad values at assignment).
- ⎕PW 128 while a definition is open, returning afterwards (5110
  manual, Chapter 5, note under ⎕PW).
- Indexed assignment into a system variable, NONCE ERROR today.

TDD, (B) only, reg-rs proves (A) unmoved.
