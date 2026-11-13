# (D) '83: IBM PC APL -- the plan

The plan for the fourth mode, `D'83`, modeled on IBM Personal Computer
APL (June 1983, IBM Madrid Scientific Center). Owner direction
(2026-11-13): "start a new docs/ibm-pc-apl-plan.md and corresponding
saga/step(s), to emulate that APL as D'83 using the dual monitor image
for the live-demo. Text in the green monochrome monitor and graphical
output in the CGA monitor. Find examples of code that can draw
graphics, like a wireframe cube in color." And: "be sure to plan
../sw-apl-workspaces for mode=d case, probably cannot reuse a, b, or c
workspaces; plan to copy workspaces to a mode=d compatible format for
saving/loading."

`docs/plan.md` holds the direction; this holds the design; the steps
are in saga `c80`, after `c80-lessons`, each named `d83-...`.

## Sources

Read for this plan (`docs/citations.md` lists them):

- **APL Reference**, 1502219, IBM, May 1983 -- the product's manual,
  379 pages. Chapter 1 (the displays, the libraries), chapter 2 (the
  supplied workspaces), chapter 3 (the auxiliary processors: AP205 on
  pages 3-10 to 3-20, AP210 from 3-21), chapters 7 and 8 (picture
  format, the system functions).
- **An APL System for the IBM Personal Computer**, M. L. Tavera and
  M. Alfonseca, IBM Systems Journal 24, no. 1 -- how the two displays
  and the APL character set were decided.

Not read: the 1984 paper by Tavera, Alfonseca and Rojas (its host's
certificate is refused), which by its abstract describes AP206, a
graphics processor written after the product was announced. (D)
models the 1983 product, so AP205 is its graphics.

## What the 1983 product was

From the manual and the paper, in brief:

- **(B) and more.** APLSV's language and system functions, plus
  ambivalent defined functions (a dyadic function may be called
  monadically; `⎕NC` on the left argument tells which), `⎕EA`
  (execute alternate, a trap), `⎕TF` and `)IN`/`)OUT` (transfer form,
  `.AIO` files), `⎕PK` (peek, poke and machine code), picture format,
  dyadic grades with a collating sequence, `)RESET`, and BIOS and DOS
  interrupts reachable from APL.
- **A small machine.** A main workspace of at most 64K bytes, with an
  elastic workspace in the rest of memory that unreferenced objects
  are moved to. Four internal types: 8-byte float, 2-byte integer,
  character, boolean. It required the 8087.
- **Libraries are drives.** Library 1 is drive A:, 2 is B:, 3 is C:,
  4 is D:. Without a number, the DOS default drive.
- **Auxiliary processors, over shared variables,** loaded by naming
  them on the DOS command line (`APL AP205 AP210`): AP80 the printer,
  AP205 the full screen, AP210 DOS files, AP232 the line to a host.
  A program offers a pair of variables (`205 ⎕SVO 'CNAME'`, a control
  variable whose name starts with C and a data variable starting with
  D), checks the coupling is 2, then sets commands in the control
  variable and data in the data variable, reading a return code back.

### Two monitors

A PC could have both adapters, the Monochrome Display and Printer
Adapter and the Color/Graphics Adapter, and PC APL used both, one at
a time: Alt-F1 switches to the monochrome display, Alt-F4 to color
with 40 characters a line, Alt-F8 to color with 80. Switching clears
the screen switched to and leaves the other as it was, so (the manual
says, in effect) graphics can stay up on one monitor while work goes
on at the other. A program switches with AP205's `0,n` command.

The product put the APL characters on the color display only, drawn
in software at 8 by 8; the monochrome display showed its own
character set, so APL characters appeared there as PC ASCII
characters. The Madrid team had also built the other answer -- an
EPROM with the APL characters, in place of the monochrome adapter's
socketed character ROM, at 14 by 9 -- and published it, and people
asked them for EPROMs of other character sets.

**The owner ran it that way:** "I used a custom eeprom on my MDA." So
(D) is the owner's machine: APL text on the green monochrome screen,
with an APL character set, and graphics on the color monitor.

### Graphics in 1983: AP205

AP205 manages either display as up to 50 rectangular **fields**,
defined together as an integer matrix of six columns, one row a
field: start row, start column, height, width (in character cells),
type (0 read-write, 2 read-only) and attribute. Attributes are the
monochrome adapter's (normal, underline, bright, reverse, blinking)
or, in 40-column color, the residue of 4: 0 invisible, 1 cyan, 2 red,
3 white -- the CGA's palette 1. The commands in the control variable
include:

| Command | Does |
|---|---|
| `0` | clear the screen |
| `0,n` | switch to display n -- 1 monochrome, 4 color 40, 8 color 80 -- and clear it |
| `1` | define the fields from the matrix in the data variable |
| `2,f` / `4,f` | write the character vector in the data variable to field f |
| `2,f,1` | write the boolean vector in the data variable to field f, one element a picture element: 1 lit, 0 dark |
| `2,f,2` `2,f,3` `2,f,4` | combine the boolean vector with the field's pixels by and, or, exclusive or |
| `3` / `8` | let the user type in the fields, until Esc, Enter, PgUp, PgDn or an F key |
| `5,f` / `5,f,1` | read field f as characters / as pixels |
| `9`, `9,f` | read the field definitions |
| `12` | set the cursor |

So 1983's graphics is **bit-mapped fields**: a program computes a
boolean picture in APL -- which APL is good at -- and writes it to a
field, in the field's color. A field of h by w cells in color 40 is
8h by 8w pixels; the whole screen, 25 by 40 cells, is the CGA's 320
by 200. That the boolean vector is the field's pixels row by row is
this plan's reading of the manual, which does not draw it: step
`d83-ap205` confirms it against the manual's examples before building.

## Decisions

1. **The language is (B)'s, plus PC APL's extensions as steps.** (D)
   is a profile on (B)'s crates (architecture.md, "Adding a mode"):
   no I-beams, `⎕` system variables and functions, execute and format,
   the del editor. Its extensions are their own crates under `d83/`.
2. **Text on the green screen, with APL characters** -- the owner's
   EPROM. 80 by 25, the monochrome adapter's green phosphor. (D)
   types and shows glyphs, as (A) and (B) do: the identity surface.
3. **Graphics on the color monitor, through AP205 as 1983 had it.**
   The CGA is modeled as a 320 by 200, four-color frame (and 640 by
   200, two colors, for color 80). A field's lit pixels take its
   color.
4. **Both monitors are always shown.** The demo draws the owner's
   dual-monitor picture with the session in the left monitor's glass
   and the CGA frame in the right's. Alt-F1, F4 and F8 and AP205's
   `0,n` choose where the session's text goes, as on the PC; the
   picture shows both at once, as the owner's desk did.
5. **Overlapping fields are drawn in order.** The manual calls the
   effect of overlapping fields unpredictable; a colored cube wants
   its edges in different fields over the same area. (D) draws fields
   in the order defined, a later field's lit pixels over an earlier
   one's, and says so (emulation-policy.md: **extension**).
6. **Only AP205, then AP210.** `⎕SVO` with any other processor number
   gives 0, as a processor not loaded did. AP80 (printer) and AP232
   (host line) are unavailable. AP210 (files) maps a drive's files to
   the browser's library 0 and the CLI's directory, after AP205.
7. **The machine stays out.** `⎕PK`'s machine code and the BIOS and
   DOS interrupts are not emulated (DOMAIN, or NONCE where a subset
   makes sense, like (C)'s `)RAM`); peek and poke of the screen
   buffers may be, as (C) did for the TRS-80's screen.
8. **The 64K main workspace is not imposed** (as (B)'s 64K is not);
   `⎕WA` reports what is left of sw-apl's size.

## Workspaces: `NAME.d.apl.ws`

The owner: (D) "probably cannot reuse a, b, or c workspaces; plan to
copy workspaces to a mode=d compatible format for saving/loading."

- **A letter of its own.** (D) lists and loads only workspaces whose
  `⍝!MODES` line names (D): `NAME.d.apl.ws`, `⍝!MODES (D)`, in this
  repository's library 1 and in sw-apl-workspaces' library 2. No
  existing file gains a `d`; nothing (A), (B) or (C) loads moves.
- **Copies, by script, then reviewed.** A script in sw-apl-workspaces
  copies each (B) workspace to `NAME.d.apl.ws`: the modes line
  rewritten, `⍝!SOURCE` kept, the `⍝!` settings (B) reads kept, and a
  `⍝!DERIVED NAME.b.apl.ws` line naming its source. Each copy then
  runs its own tests in (D) before it is committed; one that needs a
  (D) change gets it in the copy only. `scripts/check-provenance.sh`
  checks the letters as it does now.
- **Saved as (D).** `)SAVE` in (D) writes `⍝!MODES (D)` and (D)'s
  settings. Library numbers keep sw-apl's meaning (0 the user's, 1
  shipped, 2 the companion repository); the PC's drive letters are
  accepted as names for them -- `A:` for 1, `B:` for 2, `C:` for 0 --
  so `)LOAD A:LIFE` and `)LOAD 1 LIFE` are one command. `)LIB` with
  no number lists library 0, the "default drive".
- **Transfer form later.** `)IN`, `)OUT` and `⎕TF` read and write
  `.AIO` transfer form, which is how PC APL exchanged workspaces with
  other systems; a step of its own, after the demo is live.
- **New (D) workspaces:** `GRAPHICS` (below), and a `SCREEN`
  workspace of AP205 cover functions, as the product's own EDIT,
  PRINT and FILE workspaces were covers -- written new, not copied.

## Graphics examples, to write

None of these is copied from anywhere; the math is the ordinary
math of each figure. Each is a (D) function in `GRAPHICS.d.apl.ws`
over a cover `SHOW` that shares the AP205 variables, defines the
fields and writes a 200 by 320 boolean matrix.

- **CUBE** -- the wireframe cube in color, turning. The corners are
  the eight rows of `¯1+2×⍉(3⍴2)⊤¯1+⍳8`; an edge joins two corners
  that differ in one coordinate (`1=` the sum of the three `∘.≠`
  tables, upper triangle). Each frame multiplies the corners by a
  rotation matrix built from `1○` and `2○` of the angle, projects
  them (dropping z, or dividing by distance for perspective), and
  draws each edge as the rounded points along it into a 64000-element
  boolean by indexed assignment. Edges along x go to a red field,
  along y cyan and along z white (decision 5).
- **PLOT** -- a function's graph with axes: `Y←F X` sampled at 320
  points, scaled to rows, one `1` a column.
- **LISSAJOUS** -- `1○` of two frequencies against each other, many
  points, one field.
- **SIERPINSKI** -- Pascal's triangle mod 2 as a boolean picture,
  `2|` of binomials by `!`, the APL classic.
- **MANDELBROT** -- the escape count over a grid of complex points,
  carried as two real matrices; points that stay inside lit.
- **LIFE** -- the generation step as a boolean matrix, shown on the
  color screen one cell a 4 by 4 block.

## The demo

- **The picture** is the owner's dual-monitor image (the monochrome
  monitor on the system unit, the color monitor beside it), converted
  as the others were and placed as `pages/machines/83-desk.webp`.
  The timeline's `machines/83.webp` and its D'83 callout link to
  `demo.html?mode=D` when the demo is live.
- **The monochrome glass** holds the session, 80 by 25, green on
  black, APL385's glyphs (the EPROM), fitted as (C)'s monitor is.
- **The color glass** holds a canvas of 320 by 200 (or 640 by 200),
  scaled without smoothing, black until a program draws; the four
  colors are the palette's.
- **Keys.** The 2741 board (B) uses, until a PC APL keyboard picture
  is chosen; Alt-F1, F4 and F8 as on the PC.
- **Help** for (D) only, as (C)'s is.

## Steps

In saga `c80`, after `c80-lessons`:

1. `d83-mode` -- add (D) to apl-modes and the CLI as `--mode 83`:
   (B)'s profile and the letter D; a workspace's `(D)`; `)HELP` names
   it. reg-rs proves (A), (B) and (C) do not move.
2. `d83-workspaces` -- in sw-apl-workspaces: the copying script, the
   `(D)` copies of (B)'s workspaces that pass their tests in (D), the
   provenance check, the drive-letter library names in sw-apl.
3. `d83-shared-variables` -- `⎕SVO`, `⎕SVR`, `⎕SVC`, `⎕SVQ` and the
   shared-variable protocol, with a processor interface and none
   loaded but AP205.
4. `d83-ap205` -- AP205: fields, the commands above, the monochrome
   text screen and the CGA frame as data, return codes as the manual
   gives them; CLI tests over a text rendering of the frame.
5. `d83-graphics` -- `GRAPHICS.d.apl.ws` with CUBE and the others,
   each with a test of its pixels.
6. `d83-demo` -- the dual-monitor page, both glasses, Help, the
   timeline link live.
7. `d83-extensions` -- ambivalent functions, `⎕EA`, picture format,
   dyadic grades, `)RESET`; then `)IN`/`)OUT`/`⎕TF`, AP210.
