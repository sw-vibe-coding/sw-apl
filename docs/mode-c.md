# The (C) '80 mode

The mode is called (C) '80, and only that: APL80, the APL Phelps Gates
wrote for the TRS-80 Model I, published by Ramware. Product names
appear here to say what the mode is modeled on, never as its name.

`modes.md` compares (C) with (A) and (B), area by area.

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

## Typing and showing APL80's characters

(C) is typed in APL80's characters and shows them, as the owner
decided. The table above is the whole keyboard:

- **A shifted letter is its lower-case code.** The TRS-80 keeps a
  shifted letter as the lower-case code and draws it with a dot, so
  `p` is ⍴ and `i` is ⍳. A terminal shows the lower-case letter; the
  demo's screen draws it dotted. A name is in capitals.
- **The arrows.** `←` and `→`, `↑` and `↓` as Unicode, or as the codes
  the TRS-80 gives them, which an ASCII keyboard types as `]`, `^`, `[`
  and `\`. An up or down arrow with nothing on its left grades: `↑V` is
  ⍋V, `2↑V` is take.
- **The minus signs.** `-` is the negative sign, `-3`, and `_`
  subtracts, `5_2`. In a name a `-` is part of it: `M-N` is one name.
- **Round brackets index** a variable or a string -- `A(2)`,
  `' *'(1+G)` -- and group anything else, so `F(2)` calls `F`. Which a
  name is, variable or function, is asked when the line runs, so a
  function line written before the function it calls still calls it.
- **`1d`** before an argument chooses the axis: `+/1dM` is `+/[1]M`.
- **`f`** begins the format statement, **`s`** is status, **`c`**
  begins a comment, **`u`** is peek and poke.
- **Anything else** -- an APL glyph, `{` -- is a CHARACTER ERROR, the
  caret under it. A line that begins `⍝` is a workspace file's
  directive or a sample's note, and is taken as a comment.

How it is done: a line is kept as it was typed -- a function's lines
too, which is what `)?` shows and a saved file holds -- and translated
into glyphs when it runs (`apl-c80-keys`). An error's caret is put
back under the character typed. What is shown -- values, quad's prompt
`q:`, names in `)FNS` and the like -- is translated back on its way
out. Inside, a name's `.` and `-` are `⍙` and `∆`, which APL80 cannot
type, and character data holds the glyph each character stands for,
so `#` gives every character its TRS-80 code. A (C) workspace file is
written in these characters, its first line the comment
`c SW-APL WORKSPACE, AS APL80 TYPES IT: LOADING IT RUNS IT.`

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
- **The format statement**, `f`: a line that begins `f6 2` sets the
  format, pairs of width and decimals (an odd count is a LENGTH
  ERROR). Numbers shown after it are laid out by (B)'s dyadic format,
  until a new format, an error, a system command, or a character array
  is shown.
- **Status**, `s`: `s0` the time since sign-on in 25-millisecond
  ticks, `s1` to `s3` the file pointers (0: (C) has no files), `s4`
  the line running, `s5` the bytes free.
- **`#`** turns characters into APL80's codes and back: the TRS-80's
  ASCII, with the substitutes on their own codes (`%` is 37, `"` 34)
  and each shifted letter on its lower-case code (`p` is 112). The
  codes that are not printed are below, under The screen. Call, the
  dyadic form, is under The memory.
- **The `d` axis operator**, `+/1dM`, plus-reduce on the first axis.

## The screen

The TRS-80 shows 16 lines of 64 characters, upper case, a shifted
letter with a dot at its left. The demo's (C) draws that inside the
glass of a render of the owner's model of the TRS-80's monitor, which
fills the window between its header and its bar, the text sized so
the 64 by 16 screen fills the glass: green on black, the line being typed at the cursor, and a board of APL80's
characters, each key the character to type with the symbol it stands
for under it. A tap, like a key, types the character: an unshifted
letter the capital, a shifted one its lower-case code.

`#` of a code that is not printed gives a character that does what
the code does when it is shown (`apl-c80-screen`):

- **The Level II control codes**: 8 backspace, 10 and 13 a new line,
  24 to 27 the cursor left, right, down and up, 28 home, 29 the start
  of the line, 30 clear to the end of the line, 31 to the end of the
  screen, 14 and 15 the cursor shown and hidden. The others do
  nothing.
- **128 to 191, the graphics blocks**, each held as the Unicode sextant
  that draws the same 2-by-3 cells.
- **192 to 255, space compression**, 192 + n for n spaces.
- **-1 to -1023, PRINT @**: the next character goes at that place, 64
  to a line.

They are held as characters -- a control code as itself, a block as
its sextant, the rest in the Private Use Area -- and sent as the
terminal codes that do the same: a terminal obeys them, and so does
the demo's screen. `#` of the codes APL80 gives an action does it and
gives no character: 5 and 9, a printer on and off, do nothing, with no
printer to drive; 6 randomizes the random link from the clock; 11
starts it again; 18 sets the clock `s0` reads to nothing. Zone
printing, 3, 7, 15 and 255, and stopping and starting the clock, 16
and 17, are NONCE ERRORs.

## The memory

`)RAM` opens a modeled memory, as the owner decided, and it stays open
for the session; a workspace does not carry it.

- **Peek**, `u` with an address on its right, gives the byte there, and
  takes a list of addresses as readily as one.
- **Poke**, `u` with bytes on its left, puts them there and gives them
  back. One byte or address goes with every one of the other; nothing
  is poked unless all of them can be.
- **The screen**, 15360 to 16383, is what (C) has shown: each line
  typed after its prompt, and everything printed. A peek reads the code
  of the character there, so it reads what has scrolled into place. A
  poke shows its character at once: the cursor is saved, moved there,
  and put back, with the terminal codes `ESC [ s` and `ESC [ u`. A
  code the screen does not print, below 32 or above 191, is a DOMAIN
  ERROR there.
- **The RAM**, 16384 to 65535, holds what was poked; the rest is 0.
- **Anything below 15360**, the ROM, the keyboard and the devices, is
  not modeled, and a peek or poke there is a DOMAIN ERROR, as is a
  byte that is not 0 to 255.
- **Call**, `51 # C`, prints the character with code `C` and gives `C`
  back; `43 # 0` reads a key as INKEY$ does, and since no key is ever
  waiting while a line runs, gives 0. Any other address is a DOMAIN
  ERROR.
- **Without `)RAM`** peek, poke and Call are NONCE ERRORs.

`)DOS` replies DOS READY and takes the keyboard. `DIR` lists the saved
workspaces, as `)LIB` does; `KILL NAME` drops one; `RETURN` goes back
to APL with the workspace intact. Each but RETURN ends with DOS READY;
anything else is INCORRECT COMMAND, (C)'s own answer, since the
manual does not give DOS's.

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
| Ported, library 1 | LIFE, RACE, BIRDS (from the (A) one) |
| Ported, library 2 | CALC, POLY, PLOT, DRILL (from the (A) one) |
| Need rework | MATH and EIGENCUBE (an encode with a matrix result), RUBIK (it copies functions from MATRIX and EIGENCUBE, and (C)'s `)COPY` takes whole workspaces only), LEARN (its lessons are about the (A) and (B) keyboards) |
| Need a (C) version of their own | COURSE (55 functions, and it teaches the del), EDIT (teaches the del) |
| Cannot run | MATRIX (domino, the diagonal transpose), STATS (45 functions, domino), and the (B)-only BIRDS, TTTML and DRILL |

**How a port is made.** `scripts/port-c80.sh NAME.ab.apl.ws` writes
`NAME.c.apl.ws` beside it with `apl-c80-port`: each `∇` definition as
`)DEF` and BREAK, every line in APL80's characters, indexing in round
brackets and an axis as `d`, the `∆` names as `D.`, and a branch on
an indexed list of labels, `→(A,B)[I]`, through a local, `D.B←A,B`
then `→D.B(I)`, since round brackets cannot index a parenthesized
expression. The script then rewords what APL80 would show badly in
prose: a dash (APL80 shows a minus as `_`) becomes a colon, and
APL\360 is written APL/360 (it shows `\` as `w`). The rest was done
by hand, and the port was run in (C) before it was committed:

- a statement that assigns twice is split, one specification to a
  line (CALC 2, POLY 3);
- characters APL80 cannot type as data -- `|` and the box drawing
  -- are replaced: RACE's track is `:`, `.` and `>`; PLOT's axis `:`;
- BIRDS' prose, written for 120 columns, is rewrapped to 64;
- DRILL leaves out the exercise that indexes a vector, `(5 6)(1 1)`,
  which APL80 cannot write; INDEX drills index-of and membership.

`apl-c80-port --sample` makes a sample's typed lines APL80's the same
way. Each port has a transcript: `99-ports-80` here, the `-80`
samples in sw-apl-workspaces.

Porting found that a label was lost when the rest of its line was
APL80's -- `MORE:R←NxFACT N_1` -- because the label was looked for by
reading the whole line with the glyphs' lexer. Only the line up to
its first colon is read now, in every mode.

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
