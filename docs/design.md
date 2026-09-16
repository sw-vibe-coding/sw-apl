# sw-apl Design Decisions

Each entry records a decision, the reason, and what it rules out.
Newest at the bottom.

## D1. Clean room, from scratch

The interpreter is written from the APL\360 and APLSV language
descriptions and from observed terminal behaviour, not by
translating `sw-cor24-apl` (C) or GNU APL. Those projects are
consulted for test expectations only. Rules out: copying parser
or evaluator structure from either.

## D2. Glyphs only

Input is Unicode. There are no Latin keyword aliases (`rho`,
`iota`) and no translation table between glyph, Greek letter
name, and English function name. Rules out: the cor24 keyword
surface syntax and any "prettify" layer in the interpreter.
Typing help belongs in `input-methods.md`, not in the lexer.

Canonical code points are listed in `glyphs.txt`. Lookalike Greek
letters (rho, iota, alpha, omega) are rejected with CHARACTER
ERROR by default; see the open question in `plan.md`.

## D3. APL\360 scope, APLSV system interface

Primitives and session behaviour follow APL\360 (flat arrays, the
1968 to 1970 primitive set including take, drop, and domino).
System variables and functions follow APLSV (quad-IO, quad-EX,
...) instead of I-beams. The APL\360 settings commands )ORIGIN,
)DIGITS, )WIDTH remain available as aliases for the quad
variables. Rules out: APL2 nested arrays and operators, Dyalog
extensions.

## D4. Floating point first-class, one semantic number type

Numbers are one type to the user. Internally `Int(i64)` and
`Float(f64)` with promotion on overflow or fractional results and
tolerant comparison via quad-CT (default 1E-13). Integers display
without a decimal point; floats display with up to quad-PP
significant digits. Rules out: separate integer and float types
visible to programs, arbitrary precision.

## D5. Six-space indent prompt and printer-style transcript

The REPL prompts by printing six spaces and leaving the cursor
there. Output starts in column one. Definition mode prompts with
`[n]` followed by spaces. The transcript scrolls like a printing
terminal; there is no screen clearing. Rules out: a modern
"apl> " prompt or a full-screen TUI.

## D6. Right-to-left evaluation, long right scope

Statements are parsed right to left. A function's right argument
is everything to its right; its left argument is the single
array immediately to its left. Operators bind tighter than
functions and take their operands from the left. Strands of
numeric literals form vectors at lex time.

## D7. Workspaces are plain-text UTF-8 files

`)SAVE` writes a file that lists settings, variables as APL
expressions, and functions as del definitions. It is readable by
humans, diffable in git, and re-executable. A header line carries
the workspace id and save timestamp so `)LOAD` can print the
APL\360 `SAVED` line. Rules out: binary snapshots.

Numbered libraries map to directories through a small config
file so `)LOAD 1 CLASS` works as in APL\360. `)LIB` lists a
library directory.

## D8. DESCRIBE convention

Every workspace shipped with sw-apl defines a niladic function
`DESCRIBE` that prints what the workspace contains and how to
start. The session prints a hint to type DESCRIBE after `)LOAD`
when the function exists. Per-function help may follow the
`HOWNAME` convention. Rules out: a separate help system.

## D9. sw-checklist gates drive crate granularity

Design new code to the gates (25 LOC per function, 4 functions
per module, 4 modules per crate), split before adding, never
suppress. Rules out: one big interpreter crate.

## D10. TDD plus reg-rs transcripts

Every step writes failing unit tests first. End-to-end behaviour
is pinned with reg-rs: each `samples/*.apl` file is a test whose
baseline is the interpreter's transcript. Baselines are rebased
only with a stated reason in the commit message. See
`testing.md`.

## D11. ASCII-only README, Unicode allowed in docs/

`README.md` and other top-level markdown stay ASCII so GitHub
renders them predictably and `sw-markdown-checker` gates them.
`docs/*.md` may contain glyphs; the README links to those docs.
`glyphs.txt` remains the machine-readable glyph table. Rules out:
glyph transcripts in the README (future vhs recordings supply
images instead).

## D12. Strict Unicode acceptance

Outside quoted literals and comments, only printable ASCII, space,
and the code points in `glyphs.txt` are valid source; anything
else is CHARACTER ERROR naming the code point. Inside quotes and
comments any Unicode scalar value is data. Invalid UTF-8 input is
reported as CHARACTER ERROR with a byte offset. Rules out: silent
acceptance of Greek lookalikes and of tabs or other control
characters in source.
