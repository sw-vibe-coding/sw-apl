# sw-apl Language Reference (APL\360 subset)

This describes the language sw-apl implements. Glyphs are mostly
named in prose; `glyphs.txt` is the machine-readable table of
characters and code points, and the "Accepted Unicode" section
below shows them. Behaviour follows
the IBM APL\360 User's Manual and the APLSV additions named
below. Where the two references differ, APL\360 wins for
primitives and APLSV wins for the quad system interface.

## Accepted Unicode

Source is UTF-8. Outside quoted literals and lamp comments,
exactly these characters are valid:

- Space (U+0020) and printable ASCII U+0021 to U+007E. Of these,
  letters, digits, and the ASCII glyphs in the table below have
  meaning; the rest (for example `#`, `$`, `&`, backtick, `{`,
  `}`) are CHARACTER ERROR.
- The APL glyphs: × ÷ ⌈ ⌊ ⍟ ○ ∧ ∨ ⍲ ⍱ ≤ ≥ ≠ ⍳ ⍴ ⌽ ⊖ ⍉ ↑ ↓ ⌿ ⍀
  ⊥ ⊤ ∊ ⍋ ⍒ ⌹ ⍎ ⍕ ∘ ← → ∇ ⍫ ⍝ ¯ ⎕ ⍞ ∆ ⍙ ⍺ ⍵ (code points in
  `glyphs.txt`).
- Newline ends a line; carriage return before a newline is
  ignored so CRLF files load.

Everything else is CHARACTER ERROR, and the message names the
code point, for example `CHARACTER ERROR: U+03C1 (use ⍴ U+2374)`
for the Greek rho lookalike. Tabs, other control characters, and
non-APL Unicode are errors outside quotes. Inside quoted
literals and after a lamp, any Unicode scalar value is accepted
as data or comment text. Input that is not valid UTF-8 is
reported as CHARACTER ERROR with the byte offset of the first
bad sequence; the session continues with the next line.

## Characters and names

- Names start with a letter, delta, or delta-underbar and
  continue with letters, digits, delta, delta-underbar. Case is
  significant. Traditional programs use upper case.
- Quad names (quad followed by letters) are system variables or
  system functions and cannot be assigned unless documented as
  variables.
- A lamp starts a comment that runs to end of line.

## Numbers

- Literals: digits, optional decimal point, optional exponent
  with `E`, negative with the high minus prefix. `1E6`, `2.5`,
  high-minus `3`, `1E` high-minus `13` are literals. A leading
  ASCII minus is the subtract function, not part of a literal.
- Semantically one numeric type. Integers are exact within i64;
  results that are not integral, or that overflow, are floats.
- Booleans are the numbers 0 and 1.
- Comparison tolerance: quad-CT, default `1E` high-minus `13`,
  applies to equal, not-equal, less-or-equal, greater-or-equal,
  floor, ceiling, residue, membership, and index-of.
- Display: up to quad-PP significant digits (default 10),
  exponential form when the magnitude needs it, high minus for
  negatives, no trailing zeros.

## Characters

- Literals are enclosed in single quotes; a quote inside is
  doubled. A one-character literal is a scalar; longer literals
  are vectors. `''` is the empty character vector.
- Character arrays display without quotes.

## Arrays

- Flat arrays of any rank. Rank 0 is a scalar, rank 1 a vector,
  rank 2 a matrix.
- Numeric literals written side by side form a vector (strand).
- Empty arrays: `iota 0`, `0 rho X`, `''`. An empty numeric
  vector displays as a blank line.
- Index origin quad-IO is 1 by default and applies to iota,
  indexing, grade, index-of, deal, roll, and axis specification.

## Scalar functions

Each scalar function applies element by element with scalar
extension: a scalar argument pairs with every element of the
other argument; otherwise shapes must match (LENGTH ERROR on the
same rank, RANK ERROR otherwise). Monadic and dyadic meanings are
listed in `glyphs.txt`: plus, minus, times, divide, upstile,
downstile, star, log, stile, shriek, circle, tilde, and, or,
nand, nor, the six comparisons, and query.

Notes:

- Divide by zero: `0 divide 0` is 1; anything else DOMAIN ERROR.
- Residue is the APL definition: `A stile B` is B minus A times
  floor of B divide A, with tolerance, and `0 stile B` is B.
- Circular: `K circle X` selects sine, cosine, tangent, their
  inverses, hyperbolic forms, and the pythagorean forms by K in
  the APL\360 table (K from high-minus 7 to 7).
- Shriek on non-integers is the gamma function shifted by one.
- Roll: `query N` is a random integer in the index range of N.
  Deal: `M query N` is M distinct random integers from the index
  range of N. Both use quad-RL.

## Mixed functions

- iota: index generator (monadic), index of (dyadic; not found
  yields one past the last index).
- rho: shape (monadic), reshape (dyadic, cycling the data).
- comma: ravel (monadic), catenate along the last axis (dyadic),
  laminate when the axis is fractional.
- circle-stile and circle-bar: reverse and rotate along the last
  or first axis.
- circle-backslash: monadic transpose reverses the axes; dyadic
  transpose permutes and can take diagonals.
- up-arrow and down-arrow: take and drop, per axis, negative
  counts from the end, overtake pads with zero or blank.
- slash and backslash with a boolean left argument: compress and
  expand along the last axis; slash-bar and backslash-bar along
  the first.
- up-tack and down-tack: decode and encode in mixed radix.
- epsilon: membership.
- grade-up and grade-down: permutation vectors, stable.
- domino: matrix inverse and least-squares divide (Phase 5).
- execute and format: APLSV additions (Phase 5).

## Operators

- Reduce: `f slash` along the last axis, `f slash-bar` along the
  first. Reducing an empty vector yields the identity element of
  f when one exists, otherwise DOMAIN ERROR.
- Scan: `f backslash` and `f backslash-bar` produce running
  reductions.
- Inner product: `f dot g`, the classic `plus dot times` for
  matrix product.
- Outer product: `jot dot f`.
- Axis: a bracketed axis after a function or operator selects the
  axis for reduce, scan, reverse, rotate, catenate, compress,
  expand.

## Indexing and assignment

- `A[I]`, `M[I;J]`, with an elided axis selecting everything.
  Index arrays of any rank; the result shape is the catenation
  of the index shapes.
- Indexed assignment `A[I] left-arrow V` with scalar extension.
- Plain assignment `A left-arrow V` returns V but does not
  display it. Assignment can appear inside an expression.
- INDEX ERROR when an index is out of range.

## Statements and evaluation

- Right to left. A function takes the whole expression to its
  right and the single array to its left. Parentheses group.
- Operators bind before functions and take operands to their
  left.
- A statement whose value is not assigned is displayed.
- Quad on the left of assignment displays; quad on the right
  reads and evaluates a line. Quote-quad reads or writes raw
  characters without a trailing newline on output.

## Defined functions

- Opened with del followed by a header: `NAME`, `NAME B`,
  `A NAME B`, or any of those with `R left-arrow` in front.
  Locals follow the header after semicolons.
- Lines are numbered from 1. The editor accepts `[n]` to
  reposition, `[n quad]` to display, `[delta n]` to delete,
  fractional numbers to insert, and `[0]` to edit the header.
  A closing del ends definition; del-tilde locks the function.
- Labels are names followed by a colon at the start of a line;
  they are local constants holding the line number.
- Branch: right arrow followed by an expression. The first
  element selects the line; an empty vector continues; zero or
  a number outside the function exits; a bare right arrow in
  immediate execution clears the top of the state indicator.
- Dynamic scoping: locals shadow globals for the duration of the
  call, including in called functions.
- Recursion is allowed; depth is bounded by memory (DEPTH ERROR
  as a guard).

## System variables (APLSV names)

quad-IO (index origin), quad-PP (print precision), quad-PW (print
width), quad-CT (comparison tolerance), quad-RL (random link),
quad-LX (latent expression run after load), quad-TS (time stamp),
quad-AI (account information), quad-WA (workspace available),
quad-LC (line counter).

## System functions (APLSV names)

quad-EX (expunge names), quad-NL (name list by class), quad-NC
(name class), quad-FX (fix a function from a character matrix),
quad-CR (canonical representation of a function), quad-DL (delay
seconds).

## Errors

SYNTAX ERROR, VALUE ERROR, DOMAIN ERROR, RANK ERROR, LENGTH
ERROR, INDEX ERROR, WS FULL, DEFN ERROR, CHARACTER ERROR, DEPTH
ERROR, INTERRUPT. Display format is in `session.md`.
