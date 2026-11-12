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
the owner's, and a 1980 edition of the manual confirms it. The owner
used APL80 in 1980.

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

## The keys

From the manual's Character Set page (page 4 of the undated edition).
APL80 types every APL character it has in one of three ways. A
shifted letter is typed with SHIFT and shows as a graphics dot and the
letter; here it is written `.P`, as the manual's own text writes it.

**The same in APL and APL80:** `! ' ( ) * : ; = ↑ ↓ ← → + , . ? / > <`.
The four arrows are typed with SHIFT and the arrow key.

**Substitutes:**

| APL | APL80 | Note |
|---|---|---|
| `÷` | `%` | |
| `⍟` | `@` | the scan prints a filled circle; logarithm is the one APL\360 character left to match it |
| `[` `]` | `(` `)` | indexing, which therefore looks like a call |
| `≠` | `$` | |
| `∧` | `&` | |
| `∘` | `"` | so an outer product is `".+` |
| `¯` | `-` | the ordinary minus is the negative sign |
| `-` | `_` | subtract and negate, typed with the CLEAR key |
| `⍋` `⍒` | `↑` `↓` | monadic; dyadic they are take and drop |

**Shifted letters:**

| APL | APL80 | APL | APL80 | APL | APL80 |
|---|---|---|---|---|---|
| `×` | `.X` | `○` | `.O` | `\` | `.W` |
| `≥` | `.Z` | `\|` | `.J` | `⌶` | `.S` |
| `≤` | `.K` | `⎕` | `.Q` | `⌈` | `.H` |
| `~` | `.N` | `⍞` | `.M` | `⌊` | `.L` |
| `∨` | `.V` | `⍳` | `.I` | `⊥` | `.B` |
| `⍲` | `.A` | `⍴` | `.P` | `⊤` | `.T` |
| `⍱` | `.Y` | `∊` | `.E` | `⍝` | `.C` |
| | | `⌽` | `.R` | `⍉` | `.G` |
| | | | | `⍕` | `.F` |

Read from the scan, which is sharp for most of the row. Three are
read with help from the rest of the manual: `⌶` as `.S`, which is
APL80's status operator in place of the I-beams; `⍝` as `.C`, which
the function table lists as comment; and `⍕` as `.F`, which is the
format statement. Not in the table, and so not in APL80: `∆`, `⍎`,
`⌹`, `⊖`, `⌿` and `⍀` (the first-axis forms are the `d` axis
operator). `#` is not a substitute: it is APL80's own character and
number conversion, and with `)RAM`, Call.

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
  is the lowered minus. There is no `∆`: the manual's character
  table, which lists every APL character APL80 can type, has none, so
  a name is letters, digits, `.` and `-`.
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

## The owner's decisions (2026-11-10)

"Be compatible. Make it look like it did originally, do not change
appearance. Emulate a subset of peek, poke, )DOS within reason, give
errors for some."

1. **Typing and display are APL80's.** (C) types and shows APL80's own
   characters, the substitutes and the dotted shifted letters, upper
   case, on a 64-by-16 screen. The glyphs-only rule is (A)'s and (B)'s;
   (C) does not show a glyph APL80 could not.
2. **Peek, poke and call, within reason, after `)RAM`.** A modeled
   memory, with no TRS-80 ROM in it (the ROM is not ours to ship):
   - the screen, 15360 to 16383, is the 64-by-16 display: a peek reads
     the character there, a poke puts one there (the manual's own
     example puts an A on the screen with 65 at 16000);
   - the RAM above 16383 is plain bytes, peeked and poked as stored;
   - the ROM below 12288 and the keyboard and devices between are not
     modeled: a peek or poke there is an error;
   - call does the two routines the manual shows, printing a character
     (51) and reading a key as INKEY$ does (43); any other address is
     an error.
   Without `)RAM` all three are errors, as on the machine.
3. **`)DOS`, within reason.** It shows DOS READY and takes `DIR` (the
   saved workspaces and files), `KILL` and `RETURN`, which goes back
   with the workspace intact. Any other DOS command is an error.

## Decisions the manual leaves open

The manual lists what APL80 leaves out without naming the error each
gives. (C) answers:

- **NONCE ERROR** for what is "not implemented" or "limited to" less:
  domino, lamination, the diagonal transpose, the hyperbolic functions
  (`5○` to `7○` and `¯5○` to `¯7○`), and encode or decode with an
  argument above a vector. It is the error sw-apl gives for what one
  mode leaves out and another has, as (B) does for an I-beam.
- **DOMAIN ERROR** for factorial or binomial of a number that is not
  whole: the manual says the arguments "must be" integers, an
  argument of the wrong kind. The check is in the scalar functions
  themselves, so it holds through reduction, scan and the products.
- **SYNTAX ERROR**, at the second arrow, for two specifications in a
  statement or one whose value is used (`X←Y←9`, `÷X←4`).
- **DEFN ERROR** for a 33rd function or a 256th line, when the
  function is defined. A workspace that already holds more is loaded
  as it is.

A clear workspace prints six digits, the `)PS` the manual describes,
in a line of 64. The manual does not say how a long whole number
prints, so it prints as (A)'s does.

A workspace runs in (C) only when it was saved there (owner,
2026-11-11): a save in (C) is `⍝!MODES (C)`, `NAME.c.apl.ws`, and no
workspace saved in (A) or (B) is ever taken to run in (C). A (C)
workspace is written as APL80 would type it -- each function as
`)DEF`, its header, its lines and the empty line for BREAK; the latent
expression as `)AUTO` -- so it holds no del and no `⎕LX`, and loading
it types it back in.

## Defining functions

There is no del in (C): `∇` typed there is a CHARACTER ERROR, as any
character APL80 has not got. A function is defined and changed as the
manual's Function Definition describes:

- `)DEF header` opens a new function; a name in use is a DEFN ERROR.
  `)EDIT name` opens one that exists; a name that holds no function is
  a DEFN ERROR. Each asks for the first unused line, with the prompt
  `[n]` sw-apl's del editor uses -- the manual does not show APL80's.
- A line typed is the next line; BREAK leaves. A terminal sends a line
  and not a key, so sw-apl takes an **empty line** as BREAK in a
  definition, and in a saved file.
- `)?` shows the function, the header as line 0, each line behind its
  number. `)P n` shows 14 lines from line n.
- `)n` deletes line n at once and asks for its replacement; leaving, or
  `)?`, instead keeps it deleted. `)0` asks for a new header, which must
  name the same function. After a replacement the prompt returns to
  the first unused line. Every change renumbers at once.
- `)E n` offers line n, already typed, to be typed over, where the
  terminal can offer a line (the CLI does); APL80's pointer keys are
  the screen's (`c80-screen`). The header cannot be offered.
- Any other system command typed in definition mode runs, as on
  APL80; `)SAVE`, `)COPY` and the others the del refuses are refused
  the same way.
- APL80 finds some errors as a line is typed; sw-apl, as with the
  del, finds them when the line runs.

## Commands and operators of its own

- `)TRON` names each line before it runs, as `F[2]`; `)TROFF` stops.
- `)PS` and `)PD` print six digits or fifteen. A clear workspace
  prints six.
- `)AUTO expression` sets the latent expression, `)AUTO` alone clears
  it, and `)EXEC` runs it. A workspace loaded in (C) runs it.
- **The format statement**, `f` in APL80, is typed as the format glyph
  it stands for: a line that begins `⍕6 2` sets the format, pairs of
  width and decimals (an odd count is a LENGTH ERROR). Numbers shown
  after it are laid out by (B)'s dyadic format, until a new format, an
  error, a system command, or a character array is shown.
- **Status**, `s` in APL80, is typed as the I-beam it stands for:
  `⌶0` the time since sign-on in 25-millisecond ticks, `⌶1` to `⌶3`
  the file pointers (0: (C) has no files yet), `⌶4` the line running,
  `⌶5` the bytes free.
- **`#`** turns characters into APL80's codes and back: the TRS-80's
  ASCII, with the substitutes on their own codes (`÷` is 37, `∘` 34)
  and each shifted letter on its lower-case code (`⍴`, the shifted P,
  is 112). A code that controls the screen rather than printing, and
  Call, the dyadic form, are the screen's (`c80-screen`), a NONCE
  ERROR until then.
- **The `d` axis operator** is a spelling of the bracket axis --
  `+/1dM` is `+/[1]M` -- and is the screen's to translate.

## Names with `∆`

`∆` is not an operator or a function: in APL it is a letter, allowed in
names, and these workspaces use it by convention to mark their own
helpers (`∆TRIM`, `∆JOIN`, `∆L1`) so that they sort together and do
not clash with the user's names. APL80 has no `∆` to type, so in (C)
those names are renamed: `∆` becomes a `D.` prefix (`∆TRIM` is
`D.TRIM`), a legal APL80 name that keeps both properties. The rename is
mechanical, made by the script that makes a workspace's (C) variant,
so the (A) and (B) workspaces are untouched.

## The workspaces

From a scan of every workspace sw-apl and sw-apl-workspaces ship,
against the limits above. Each that ports gets a `.c` file of its own,
written for APL80 (owner, 2026-11-11); the verdict says how much the
code changes on the way:

| Verdict | Workspaces |
|---|---|
| Port with the code as it is | LIFE, RACE, RUBIK, BIRDS (the (A) one) |
| Port with small changes | CALC and POLY (split statements that assign twice), PLOT, DRILL (the (A) one) |
| Need rework | MATH and EIGENCUBE (an encode with a matrix result), LEARN (its lessons are about the (A) and (B) keyboards) |
| Need a (C) version of their own | COURSE (55 functions, and it teaches the del), EDIT (teaches the del) |
| Cannot run | MATRIX (domino, the diagonal transpose), STATS (45 functions, domino), and the (B)-only BIRDS, TTTML and DRILL |

PLOT (2 names) and POLY (4) port with the `∆` rename above. DRILL
(about 50) renames as easily, but it drills the APL keyboard, so its
(C) version needs APL80's keys as well. LEARN is about the (A) and (B)
keyboards and is not ported: the (C) lessons below take its place.
COURSE gets its own (C) version in any case. STATS cannot run.

## The lessons

APL80's disk came with five workspaces, LESSON1/APL to LESSON5/APL,
loaded with `)LOAD LESSON1/APL`. The cassette version had none. The
manual names them and says no more, and no copy of the disk is known
to survive, so their text is lost.

(C) gets five lessons of its own, LESSON1 to LESSON5: new text, in
the spirit of the originals, that follows the order of the manual's
Using APL chapter:

1. arithmetic from the keyboard, and vectors;
2. variables, the index generator, and defining functions with
   `)DEF` and `)EDIT`;
3. arrays and the products;
4. shape, grading, membership and indexing;
5. branching, quad input, and files.

Nothing of APL80 itself goes into either repository: no program, no
disk image, no manual text. The lessons, like the rest of (C), are
written from a reading of the manual and cite it in
[citations.md](citations.md).

## Naming the workspaces for four modes

A workspace's file is `NAME.<modes>.apl.ws`, the letters of the modes
it runs in, in order: `LIFE.ab.apl.ws`, `LIFE.c.apl.ws`,
`BIRDS.a.apl.ws`. The letters are always there; a
variant for some modes shares the name with disjoint letters, and in
any one mode a name is unique. The `⍝!MODES` line inside stays the
authority, and a gate in each repository checks that the two agree.
It replaced `NAME.apl.ws`, `NAME.a-70.apl.ws` and
`NAME.b-75.apl.ws`, which could not say three modes or four; files
named those ways are still read.
A (C) workspace has the letter c alone: (C) shares no file with (A)
or (B), so a ported workspace is a `.c` file of its own beside theirs.
