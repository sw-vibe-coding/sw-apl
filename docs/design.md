# sw-apl Design Decisions

Each entry records a decision, the reason, and what it rules out.
Newest at the bottom.

## D1. Clean room, from scratch

The interpreter is written from the APL\360 language description
(the 1968 User's Manual and the 1970 additions) and from observed
terminal behaviour, not by
translating `sw-cor24-apl` (C) or GNU APL. Those projects are
consulted for test expectations only. Rules out: copying parser
or evaluator structure from either.

## D2. Glyphs only

Input is Unicode. There are no Latin keyword aliases (`rho`,
`iota`) and no translation table between glyph, Greek letter
name, and English function name. Rules out: the cor24 keyword
surface syntax and any "prettify" layer in the interpreter.
Typing help belongs in `glyph-entry.md`, not in the lexer.

Canonical code points are listed in `glyphs.txt`. Lookalike Greek
letters (rho, iota, alpha, omega) are rejected with CHARACTER
ERROR by default; see the open question in `plan.md`.

## D3. Pure APL\360, I-beams for the system interface

Primitives and session behaviour follow APL\360 (flat arrays, the
1968 to 1970 primitive set including take, drop, and domino).
The system interface is APL\360's too: I-beam functions for time,
date, workspace space, and the state indicator; `)ORIGIN`,
`)DIGITS`, `)WIDTH` for settings (replying `WAS n`); `)ERASE`,
`)FNS`, `)VARS` for name management. Quad and quote-quad are the
I/O forms. Rules out: every APLSV quad system variable and
function (quad-IO, quad-CT, quad-EX, quad-NL, quad-FX, ...),
execute, format, and APL2 or Dyalog extensions. (Owner decision
2026-09-16, replacing the earlier APLSV-interface choice.)

## D4. Floating point first-class, one semantic number type

Numbers are one type to the user. Internally `Int(i64)` and
`Float(f64)` with promotion on overflow or fractional results and
tolerant comparison with the fixed APL\360 fuzz (1E-13 relative).
Integers display without a decimal point; floats display with up
to `)DIGITS`
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
humans, diffable in git, and re-executable: loading a workspace is
running it. A workspace file is therefore also a sample, and a
sample is also a workspace. Rules out: binary snapshots.

Two things APL cannot say about itself travel as `⍝!` directives,
which are comments to the interpreter and instructions to `)LOAD`:
`⍝!SAVED` is when the workspace was written, so `)LOAD` can print
the APL\360 `SAVED` line, and `⍝!LINK` is where the random link
stands, so a loaded workspace carries on its sequence.

Names are written sorted, so the same workspace writes the same
bytes every time and a saved file is worth keeping in git.

What a text file cannot hold is a suspended function: the state
indicator is a stack of half-run calls, and re-executing a file
cannot put execution back in the middle of one. `)SAVE` leaves it
out and `)LOAD` gives a workspace with none, which APL\360 -- whose
workspaces were binary -- did not have to do. `parity.md` carries
the row.

Numbered libraries map to directories through a small config
file so `)LOAD 1 CLASS` works as in APL\360. `)LIB` lists a
library directory.

## D8. DESCRIBE convention

Every workspace shipped with sw-apl defines `DESCRIBE`, which
prints what the workspace contains and how to start. A niladic
function is the usual form, and the one the IBM library
workspaces used, but a character matrix of the same name reads
identically to whoever loads it: typing a name runs a niladic
function or displays a variable, and the convention is the name.
Per-function help may follow the `HOWNAME` convention. Rules out:
a separate help system.

`)LOAD` prints the line saying when the workspace was saved and
nothing else, as APL\360 did. The greeting is the user typing
DESCRIBE. sw-apl does not add a hint, friendlier though that
would be: it would be output APL\360 never produced, and a
transcript from sw-apl should be able to sit beside a 1968 one
unchanged. Rules out: a loader that helps.

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

## D13. A workspace is a modelled size, not measured memory

A workspace holds a fixed number of bytes, set by `--ws-size` and
reported free by `⌶22`. What a value costs is charged the way
APL\360 charged it -- a descriptor, an entry per axis, eight bytes
a number and one a character -- and never measured from Rust's
allocator, so a workspace is the same size on every machine, in
every build, and in a test that states a number. The total is
recomputed from the symbol table rather than carried along, so a
name going out of scope cannot leak space by being forgotten in
one place.

The size belongs to the session and is not written by `)SAVE`,
exactly as an APL\360 quota belonged to the account rather than
the workspace. Rules out: `⎕WA`, a saved quota, and an accounting
that reports how much memory the process is using.

A command that fills a workspace by running APL -- `)LOAD`,
`)COPY` -- is all or nothing: the old workspace is put aside first
and given back if any line fails. Rules out: a half-loaded
workspace, which cannot be told from a whole one.

## D14. A locked function obscures the whole workspace file

APL\360 workspaces were binary, so a locked function was opaque to
whoever you sent it to. A sw-apl workspace is text and the writer
must put a locked body into it in full, which would hand it to
anyone who opened the file. So `)SAVE` writes a workspace holding at
least one locked function as a three-line preamble and the rest
rot-13; `)LOAD` and `)COPY` read both forms, telling them apart by
the first line. A workspace with nothing locked stays plain text,
because readable, diffable and re-executable is the format's whole
virtue.

This is obscuring and not encryption, and the docs say so rather
than implying otherwise. It stops a locked body being read by
accident or by curiosity, which is all APL\360's binary format
stopped. Rules out: claiming a text file can keep a secret, and
obscuring workspaces that hold nothing locked.

An obscured file is not a program. A plain workspace file can be run
with `-f`, and an obscured one says so in its preamble and signs
off, rather than printing a screen of CHARACTER ERRORs. Rules out:
leaving it to be discovered.
