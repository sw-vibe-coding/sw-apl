# The (C) '80 mode

The mode is called (C) '80, and only that: APL80, the APL Phelps Gates
wrote for the TRS-80 Model I, published by Ramware. Product names
appear here to say what the mode is modeled on, never as its name.

A planning document, as `mode-b.md` was for (B). It records, from the
APL80 manual, how APL80 differs from the APL sw-apl already runs, so
that (C) is built from the source and not from memory. Everything
below is paraphrased from that manual (`citations.md`); a guess or a
decision still to be made says so.

## What (C) is

The owner's direction (2026-11-10): a live demo of APL80 for the
TRS-80, a variation of the existing demos, with the workspaces that
can run on it, linked from the timeline's C'80 callout. The year is
the owner's; the manual is undated. The owner used APL80 in 1980.

APL80 describes itself as a large subset of APL. Its manual lists only
how it differs and what it leaves out; the rest is APL as the reader
already knows it, which sw-apl takes to be APL\360 with APLSV's
residue convention, the one convention the manual names.

There were two versions, for tape and for disk. (C) follows the disk
version, which has format, scan, the axis operator, inner product and
files; the tape version is noted, not built.

## The machine

- A 64-column, 16-line screen, upper case only. A shifted letter
  shows with a graphics dot at its left, to tell it from the plain one.
- No APL keyboard. APL80 types the APL characters as ordinary keys,
  substitutes and shifted letters: `%` for divide, `@` and `&` among
  the substitutes, round parentheses for indexing, the arrows for the
  grades (monadic) beside take and drop (dyadic), and a minus on the
  CLEAR key for negation, the ordinary minus being the negative sign.
- 15-digit arithmetic, 6-digit transcendental functions, and printing
  in 6 digits (`)PS`) or 15 (`)PD`).

## What (C) changes from (A)

From the manual's "Limitations":

- **Not implemented:** matrix inverse and divide (domino), lamination,
  the diagonal case of dyadic transpose, the hyperbolic circle
  functions.
- **Limited:** encode and decode take vectors only; factorial and
  binomial take integers only.
- **Two assignments in one statement** must be two statements.
- **A quad cannot be typed in reply to a quad;** quote-quad can.
- **At most 32 functions** in a workspace, and 255 lines a function.
- **Format is a statement, not a function:** `f` with pairs of field
  width and decimals governs numeric output until the next `f`, an
  error, a command or character output.
- **An axis is the `d` operator,** not a bracket: reduce, compress,
  expand, scan, reverse and rotate take it.
- **Names** may contain `.` and `-`; `M-N` is a name, and subtraction
  is the lowered minus. Whether `∆` is allowed is not stated.
- **Indexing** floors a non-integer index without comment, and the
  semicolon between indexes is not a strong delimiter.
- **Comparison** takes characters, both arguments of one type; `?0`
  gives a number between 0 and 1.

What it adds:

- **Functions are defined by command,** not the del: `)DEF header`,
  `)EDIT name`, `)?` to display, `)P n` to page, `)n` to replace line n,
  `)E n` to edit a line in place, `)H` to print; BREAK leaves.
- **Commands:** `)OFF )CLEAR )FNS )VARS )SI )ERASE )RESET`, and on
  disk `)SAVE )LOAD )COPY` (whole workspaces only), `)AUTO` (a latent
  expression) and `)EXEC`, `)TRON )TROFF` (trace), `)PS )PD`, `)KILL`,
  `)DOS`, `)RAM`, and files: `)WRITE )READ )RESTORE )UPDATE )QUERY`.
- **`#`** converts numbers to characters and back, and sends the
  screen's control codes; **`s`** reports the clock, the file pointers,
  the line counter and the free space; peek, poke and call reach memory
  after `)RAM`.

## Decisions for the owner

1. **Typing and display.** Faithful (C) types and shows APL80's own
   characters, the substitutes and dotted shifted letters, on a
   64-by-16 screen. That departs from sw-apl's rule of glyphs only,
   which was written for (A). The recommendation is to be faithful in
   (C), and to say so in the docs.
2. **What of peek, poke, call and `)DOS`:** they reach a TRS-80 that is
   not there. The recommendation is to report them as not available,
   as (B) does for the 5110's hardware.

## The workspaces

From a scan of every workspace sw-apl and sw-apl-workspaces ship,
against the limits above:

| Verdict | Workspaces |
|---|---|
| Run as they are | LIFE, RACE, RUBIK, BIRDS (the (A) one) |
| Run with small changes | CALC and POLY (split statements that assign twice), PLOT, DRILL (the (A) one) |
| Need rework | MATH and EIGENCUBE (an encode with a matrix result), LEARN (its lessons are about the (A) and (B) keyboards) |
| Need a (C) version of their own | COURSE (55 functions, and it teaches the del), EDIT (teaches the del) |
| Cannot run | MATRIX (domino, the diagonal transpose), STATS (45 functions, domino), and the (B)-only BIRDS, TTTML and DRILL |

PLOT, POLY, DRILL, LEARN and COURSE use `∆` in names, which waits on
the question above.

## Naming the workspaces for four modes

A workspace's file is `NAME.<modes>.apl.ws`, the letters of the modes
it runs in, in order: `LIFE.abc.apl.ws`, `COURSE.ab.apl.ws`,
`BIRDS.a.apl.ws`, `COURSE.c.apl.ws`. The letters are always there; a
variant for some modes shares the name with disjoint letters, and in
any one mode a name is unique. The `⍝!MODES` line inside stays the
authority, and a gate in each repository checks that the two agree.
It replaces today's `NAME.apl.ws`, `NAME.a-70.apl.ws` and
`NAME.b-75.apl.ws`, which cannot say three modes or four.
