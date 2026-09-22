# sw-apl Language Reference

This describes the language sw-apl implements, in its two modes:

- **(A) '70**, modelled on APL\360. Behaviour follows the IBM
  APL\360 User's Manual (1968, and its 1970 edition), with domino
  from the APL\360-OS/DOS manual of December 1970.
- **(B) '75**, modelled on the APL of the IBM 5100 family, following
  the IBM 5110 APL Reference Manual. `mode-b.md` records the sources
  and every decision where sw-apl departs from them.

Most of the language is one shared core, and the sections from
*Accepted Unicode* to *Defined functions* describe it: they hold in
both modes except where they say otherwise. Then come what (B) adds
-- execute and the quad system variables and functions -- and what
(A) has that (B) has not -- the I-beam functions and the settings
commands. `--mode 70` or `--mode 75` chooses the mode at the command
line, and the tab does in the browser.

Glyphs are mostly named in prose; `glyphs.txt` is the
machine-readable table of characters and code points, and the
"Accepted Unicode" section below shows them.

## Accepted Unicode

Source is UTF-8. Outside quoted literals and lamp comments,
exactly these characters are valid:

- Space (U+0020) and printable ASCII U+0021 to U+007E. Of these,
  letters, digits, and the ASCII glyphs in the table below have
  meaning; the rest (for example `#`, `$`, `&`, backtick, `{`,
  `}`) are CHARACTER ERROR.
- The APL glyphs: × ÷ ⌈ ⌊ ⍟ ○ ∧ ∨ ⍲ ⍱ ≤ ≥ ≠ ⍳ ⍴ ⌽ ⊖ ⍉ ↑ ↓ ⌿ ⍀
  ⊥ ⊤ ∊ ⍋ ⍒ ⌹ ⌶ ∘ ← → ∇ ⍫ ⍝ ¯ ⎕ ⍞ ∆ ⍙ (code points in
  `glyphs.txt`). In (B), also execute ⍎ and format ⍕. Glyphs from
  later APLs (⍺ ⍵ ⊂ ⊃ ¨ ⋄ ..., and ⍎ ⍕ in (A)) are CHARACTER ERROR.
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
- The underscored letters A̲ through Z̲ are letters too, each
  distinct from the plain one: `X` and `X̲` are two names. Each is
  written as its letter followed by U+0332 COMBINING LOW LINE,
  which continues a name but cannot start one and must follow a
  letter; elsewhere it is a CHARACTER ERROR. It prints in one
  position and is counted as one column. As character data each
  underscored letter is two elements, so `⍴'X̲'` is 2.
- In (A), a quad followed by letters is quad input followed by a
  name; there are no quad-named system variables in APL\360.
  System information comes from the I-beam functions and settings
  from `)ORIGIN`, `)DIGITS`, `)WIDTH`. In (B), a quad directly
  before a letter is one name, a system variable or function such as
  `⎕IO` or `⎕FX`.
- A lamp starts a comment that runs to end of line.

## Numbers

- Literals: digits, optional decimal point, optional exponent
  with `E`, negative with the high minus prefix. `1E6`, `2.5`,
  high-minus `3`, `1E` high-minus `13` are literals. A leading
  ASCII minus is the subtract function, not part of a literal.
- Semantically one numeric type, and it is a double: precision is
  about sixteen decimal digits, so the last integer held exactly is
  `2*53`. Beyond that, consecutive integers are the same number.
  Plus, minus and times take an exact path while both arguments are
  integers and the answer fits; past that they fall to the floating
  path rather than wrapping round.
- Booleans are the numbers 0 and 1.
- Comparison tolerance (fuzz) is fixed at `1E` high-minus `13`
  relative and applies to equal, not-equal, less-or-equal, greater-or-equal,
  floor, ceiling, residue, membership, index-of, and to an argument
  that has to be a count: `⍳(0.1+0.2)×10` is `1 2 3`, because that
  value prints as 3, floors to 3 and compares equal to 3. Zero has
  no slack: nothing but zero equals zero.
- Display: up to the print precision in significant digits,
  exponential form when the magnitude needs it, high minus for
  negatives, no trailing zeros. The precision bounds what is shown
  and never what is held: `3×1÷3` is 1 at a precision of 1. In (A)
  it is `)DIGITS` (default 10), and it caps every number and not
  only the fractions, so an exact integer wider than the setting is
  shown in exponential form. In (B) it is `⎕PP` (default 5), and a
  whole number of up to ten digits is shown in full whatever it
  says.

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
- The index origin (`)ORIGIN` in (A), `⎕IO` in (B), default 1 in
  both) applies to iota,
  indexing, grade, index-of, deal, roll, and axis specification.

## Scalar functions

Each scalar function applies element by element with scalar
extension: a scalar or a one-element array of any rank pairs with
every element of the other argument, and the result has the other
argument's shape; otherwise shapes must match (LENGTH ERROR on the
same rank, RANK ERROR otherwise). Two one-element arrays of
different shapes give the shape of the one of higher rank. Monadic and dyadic meanings are
listed in `glyphs.txt`: plus, minus, times, divide, upstile,
downstile, star, log, stile, shriek, circle, tilde, and, or,
nand, nor, the six comparisons, and query.

Notes:

- Equal and not-equal compare characters as well as numbers --
  element by element, and in reduction and inner and outer products,
  so `+/W='S'` counts a letter and `'CAT' and-dot-equal 'CAT'` matches
  a word. A character is never equal to a number. The other scalar
  functions take numbers only. A scan of characters is DOMAIN ERROR,
  since its result would mix a character with numbers.
- Divide by zero: `0 divide 0` is 1; anything else DOMAIN ERROR.
- Residue is the APL definition: `A stile B` is B minus A times
  floor of B divide A, with tolerance, and `0 stile B` is B.
- Circular: `K circle X` selects sine, cosine, tangent, their
  inverses, hyperbolic forms, and the pythagorean forms by K in
  the APL\360 table (K from high-minus 7 to 7).
- Shriek on non-integers is the gamma function shifted by one.
- Roll: `query N` is a random integer in the index range of N.
  Deal: `M query N` (each a scalar or a one-element vector, so a
  shape will do) is M distinct random integers from the index
  range of N. Both advance the workspace random link, a Lehmer
  generator (multiplier 16807, modulus 2^31 - 1) that starts at
  16807 in a clear workspace and is saved with the workspace, so a
  loaded workspace continues its sequence and transcripts that use
  `?` reproduce.

  A link is mapped onto the range as `⌊N×(link-1)÷(P-1)`, with P the
  modulus. Roger Hui's "Roll" gives J's as `⌊N×link÷P`. The two agree
  almost always for small N and differ by one at large N -- `?1E9`
  from a clear workspace is 131537788 here and 131537789 there.
  Nothing establishes that J's formula was APL\360's, so the sequence
  a transcript records is this one.

  There are only 2^31-2 distinct links, so `?N` for N larger than
  that cannot produce every value in its range. It is not refused:
  each roll is still computed exactly and is always in range. Deal
  takes memory in proportion to M, not N, so `1?1E9` is as cheap as
  `1?10`.

## Mixed functions

- iota: index generator (monadic), index of (dyadic; not found
  yields one past the last index). The index generator takes a
  non-negative integer that is a scalar or a one-element vector, so
  iota of the shape of a vector counts its elements.
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
  the first. A scalar or one-element left argument to compress
  applies to every element of the right one; a scalar right
  argument is one element, so `1 0 1/7` is LENGTH ERROR. A scalar
  left argument to expand does not extend.
- up-tack and down-tack: decode and encode in mixed radix. Either
  argument of decode may be a scalar or a one-element vector.
- epsilon: membership.
- grade-up and grade-down: permutation vectors, stable.
- domino: matrix inverse, and matrix divide, which is the least
  squares solution when the right argument has more rows than
  columns. Both arguments are rank 2 or less: a vector is one
  column and a scalar is one by one, so two vectors divide to the
  one coefficient that fits them. A singular right argument, one
  with more columns than rows, and character data are all DOMAIN
  ERROR; rank 3 is RANK ERROR.

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
  expand. The axis is a scalar or a one-element array.

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
  prompts with `⎕:` and reads a line, which is evaluated as a
  whole statement in the current environment, so it sees the
  locals of whatever is running. A reply with no value -- blank,
  a comment, an assignment -- prompts again; a branch abandons
  the read. Quote-quad on the right reads the characters as they
  were typed, with no prompt and no evaluation; on the left it
  writes them with no line ending, so a prompt and the answer
  typed after it share a line. A read with no more input to take
  is INTERRUPT.

## Defined functions

- Opened with del followed by a header: `NAME`, `NAME B`,
  `A NAME B`, or any of those with `R left-arrow` in front.
  Locals follow the header after semicolons.
- Lines are numbered from 1. The editor accepts `[n]` to
  reposition, `[quad]` and `[n quad]` to display, `[delta n]` to
  delete, fractional numbers to insert, and `[0]` to edit the
  header. A closing del ends definition and renumbers the lines
  from 1; del-tilde ends it and locks the function, which can then
  be neither reopened nor displayed. `session.md` has the full
  table.
- Labels are names followed by a colon at the start of a line;
  they are local constants holding the line number.
- Branch: right arrow followed by an expression. The first
  element selects the line; an empty vector continues with the
  next line; zero or a number outside the function exits,
  returning the result variable's value if the header has one.
  Running off the last line exits the same way. `→LABEL×⍳COND`
  is the conditional branch: when COND is 0 the product is
  empty and execution falls through. A bare right arrow in
  immediate execution clears the top of the state indicator.
- Dynamic scoping: locals shadow globals for the duration of the
  call, including in called functions. A local hides whatever the
  name holds, a global function as well as a global variable, and
  the global is back when the call returns. Under a suspension
  `)FNS` and `)VARS` list the globals, and `)SAVE` saves them, not
  the locals that hide them.
- Recursion is allowed; calls may nest 128 deep before DEPTH
  ERROR, which also bounds the state indicator.
- A line that fails suspends the function rather than unwinding
  it: the locals stay visible and the state indicator records
  where it stopped. `session.md` has the display and the
  commands.

## What (B) '75 adds

### Execute

`⍎B` runs a character scalar or vector as a line. As a whole
statement it shows what the line would show, which is nothing for an
assignment or an empty line; inside an expression the line must give
a value, or it is a VALUE ERROR. An error in the line is the
statement's, with the caret on the execute. Execute is monadic.

```apl
      ⍎'2+3'
5
      X←⍎'⍳3'
      X
1 2 3
```

On the keyboard, execute is `⊥` struck with `∘`.

### Format

Format, `⍕`, is a glyph of (B) -- struck from `⊤` and `∘` -- but not
implemented: using it is a NONCE ERROR.

### System variables

| Name | |
|---|---|
| `⎕IO` | Index origin, 0 or 1 |
| `⎕PP` | Print precision, 1 to 16 |
| `⎕PW` | Print width, 30 to 254 |
| `⎕RL` | Random link |
| `⎕CT` | Comparison tolerance, 0 to just under 1: the relations, floor, ceiling, residue, membership and index-of use it. `1E¯13` in a clear workspace, APL\360's fixed fuzz |
| `⎕LC` | Line counter: the lines being executed, innermost first |
| `⎕WA` | Workspace available, in bytes |
| `⎕AV` | The atomic vector: 256 characters, in the 5110's order |
| `⎕LX` | Latent expression, run by `)LOAD` once the workspace is in |
| `⎕AI` `⎕TS` `⎕TT` `⎕UL` `⎕DL` | Fixed values, kept for compatibility: the 5110 had one user and no clock. `⎕TS` is `1900 0 0 0 0 0 0` until assigned |

`⎕IO`, `⎕PP`, `⎕PW` and `⎕RL` are the same settings (A)'s commands
and directives set: a workspace saved with `)ORIGIN 0` in (A) has
`⎕IO` 0 in (B). A value a setting cannot take is a DOMAIN ERROR at
the assignment. An assignment to `⎕LC`, `⎕WA`, `⎕AV`, `⎕TT`, `⎕UL`
or `⎕DL` is ignored. A clear workspace in (B) has `⎕PP` 5 and `⎕PW`
64, the 5110's screen. A quad name the system has not got is a
SYNTAX ERROR.

Not implemented: a system variable localized in a function header,
`⎕PW` 128 while a definition is open, and indexed assignment into a
system variable (NONCE ERROR).

### System functions

| Call | Result |
|---|---|
| `⎕CR 'F'` | The function as a character matrix: header first, no line numbers or dels, padded with blanks. Anything that is not an unlocked function gives a 0 by 0 matrix |
| `⎕FX M` | Defines the function the rows of `M` spell and gives its name; or, changing nothing, the number of the first line the del editor would not have taken (the header is 0) |
| `⎕EX 'NAME'` | Erases what the name holds; 1 if the name is then free, 0 if it could not be freed |
| `⎕NC 'NAME'` | The name's class: 0 free, 1 label, 2 variable, 3 function, 4 not to be used as a name. A matrix of names gives a vector |
| `[L] ⎕NL K` | The names of the classes in `K`, one to a row, alphabetically; `L` restricts them to those initial letters |
| `⎕CC V` | The 5110's console control: checks the request and answers 1 or 0 as the 5110 would. sw-apl has no screen, alarm or printer for it to act on |

`⎕FX` refuses what the del editor would refuse, and a name that holds
a variable, is locked, or is running or waiting on the state
indicator. Under a name a running function has made local, it fixes
a local function: it hides a global of the same name, `⎕EX` of the
name erases the local one, and it goes when that function returns.

### Errors

(B) has one error (A) has not: NONCE ERROR, for something the
language has and sw-apl does not yet do -- format, and an I-beam,
which the 5110 manual makes a NONCE ERROR.

## What (A) '70 has that (B) '75 does not

The I-beam functions and the settings commands. The 5100 family
replaced both with system variables and functions; in (B) an I-beam
is a NONCE ERROR and the commands are INCORRECT COMMAND.

### I-beam functions

The I-beam (U+2336) is a monadic function whose integer argument
selects a system value, as in APL\360:

| Call | Result |
|---|---|
| `⌶20` | time of day, in sixtieths of a second since midnight |
| `⌶21` | CPU time used this session, in sixtieths of a second |
| `⌶22` | workspace space still free, in bytes |
| `⌶23` | number of terminals connected (always 1) |
| `⌶24` | time of sign-on, in sixtieths of a second since midnight |
| `⌶25` | today's date as the integer MMDDYY |
| `⌶26` | line number of the statement now executing (first of the state indicator) |
| `⌶27` | vector of line numbers in the state indicator |

`i-beam-reference.md` is the long version: the units spelled out,
worked examples of reading a clock and a date, what each reports
when there is nothing to report, and the errors.

Any other argument is DOMAIN ERROR, as is an argument that is not
one whole number. There is no dyadic I-beam: a left argument is
DOMAIN ERROR too, and there is no axis form.

The time of day, the processor time, and the date are read from a
clock the workspace holds. A clear workspace has one that does not
move, so a transcript made without a terminal reproduces; the
terminal installs the real one when a session starts. A program
that wants to differ from run to run stirs the random link from
the clock -- `?(1+60|⌶20)⍴2` throws away a clock-dependent number
of rolls -- which is what `samples/58-ibeams.apl` shows.

### Settings commands

`)ORIGIN n` (0 or 1), `)DIGITS n` (1 to 16), and `)WIDTH n` (30 to
254) change the index origin, print precision, and print width
and reply with the previous value as `WAS n`. They are saved with
the workspace, and read back in (B) as `⎕IO`, `⎕PP` and `⎕PW`.

## Errors

SYNTAX ERROR, VALUE ERROR, DOMAIN ERROR, RANK ERROR, LENGTH
ERROR, INDEX ERROR, WS FULL, DEFN ERROR, CHARACTER ERROR, DEPTH
ERROR, INTERRUPT, and in (B) NONCE ERROR. That is the whole
vocabulary. Display format is
in `session.md`.

A glyph used where it has no such form is a SYNTAX ERROR: `1~0`,
`1⍋2` and `⍳/1 2` name functions APL\360 has not got, so the
sentence does not parse. A glyph that does have the form and was
given an argument outside it is a DOMAIN ERROR: `÷0` and `1⍟0`.
