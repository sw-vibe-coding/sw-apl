# The (U) '74 mode

The mode is called (U) '74: University of Maryland APL as a batch job
from punched cards, on a UNIVAC 1110 under EXEC 8. "U '74" is the
owner's label for University of Maryland APL (1974), not a name used
at the time; Sperry Univac later sold the same interpreter as APL 1100.

A planning document, as `mode-b.md` and `mode-c.md` were for (B) and
(C). Unlike them it has no manual to paraphrase: no UMD APL or APL
1100 manual is online (`citations.md`). What it rests on, and what is
ours, is said for each part.

## The sources

- **Jack Minker's history of computing at Maryland** (2004, page 38):
  APL was implemented for the UNIVAC 1108 by the Computer Science
  Center's systems group under Patrick E. Hagerty, who left Maryland
  in 1974. Workspaces could grow from 16K to 256K bytes; Maryland
  developed domino and a shared variable facility; and Hagerty
  designed an APL character set for a print bar on the high-speed
  printers, which UNIVAC made and offered to all its customers. The
  one primary source here.
- **The owner's memory.** Cards punched on an 029 at SUNY Albany's
  UNIVAC 1110 in 1976-77; two-letter `$` names for the APL symbols;
  a University of Maryland banner on the printout.
- **The owner's notes** (a chat with an LLM, `docs/research3.txt`,
  not in git). The history agrees with Minker. The rest is a
  reconstruction, labeled as one: the card layout, the run stream
  and a `$` table. It is a starting point; where it is wrong on its
  face this document says so and decides.
- **ibm-1130-rs** (the owner's): the 029's characters and their card
  codes, `crates/punch-card-core/src/hollerith.rs`.

## What (U) is

The owner's direction (2026-11-22): not a UNIVAC emulator, but UMD
APL's syntax on punched cards. The user punches a deck on a virtual
029, submits it as a batch job, and gets back a green-bar listing.
And (2026-11-24):

- **No disk, no saved workspaces, no system commands.** A run starts
  with a clear workspace; the deck's cards build it, run it and are
  gone. A program is kept as a deck, not as a workspace.
- **No APL keyboard.** The 029's own keys, on screen, and the
  computer's keyboard; nothing else.

The language is (A)'s, APL\360, which already has domino, with scan,
which (A) has not got: the notes list it, and APLSV (1973) had it. Minker
credits Maryland with shared variables; (U) leaves them out, as a
batch job has no partner to share with. Execute and format are left
out: they arrived with APLSV in 1973, and nothing says Maryland had
them by 1974 (the notes list both, as `$DE` and `$FM`; see below). Nor
does (U) have quad-named system variables or functions, which are
APLSV's.

## The card

- 80 columns. Columns 1-72 are the statement; columns 73-80 are a
  sequence number, printed in the listing and otherwise ignored
  (the notes; the usual layout for decks of the time).
- A card holds one statement. There is no continuation card: a
  statement that does not fit in 72 columns is split into two (ours;
  the notes give none).
- Upper case only, the 029 having nothing else. A blank card is
  listed and does nothing.

## What an 029 punches

From the card code table: the letters, the digits, blank, and
`& - / . < ( + | ! $ * ) ; ¬ , % _ > ? : # @ ' = "`. The 029's cent
sign is not in the table and is not used. There is no `[`, `]`, `\`
or `^`, and nothing beyond these: typing any other character on the
computer's keyboard punches nothing.

What the specials mean in (U):

- `+ - * / . < > = ( ) ; : , ' | ! ?` mean what they mean in APL\360.
- `¬` is not, as `~`; the 029 has no tilde (ours).
- `$` begins a mnemonic, below.
- `& % _ # @ "` have no meaning outside quotes: CHARACTER ERROR.
  `@` in column 1 is a control card.

## The `$` mnemonics

A `$` and two letters stand for one APL character, anywhere on the
card, inside quotes too, so that a character string can hold `⍴`. A
`$` followed by anything that is not a mnemonic is a CHARACTER
ERROR at that column. `$$` is a dollar sign (ours: the notes give no
way to quote one).

From the notes, as they give them:

| Card | APL | | Card | APL | | Card | APL |
|---|---|---|---|---|---|---|---|
| `$AL` | `←` | | `$EP` | `∊` | | `$LE` | `≤` |
| `$RA` | `→` | | `$DM` | `⌹` | | `$GE` | `≥` |
| `$DL` | `∇` | | `$CE` | `⌈` | | `$NE` | `≠` |
| `$IO` | `⍳` | | `$FL` | `⌊` | | `$AN` | `∧` |
| `$RH` | `⍴` | | `$AB` | `\|` | | `$OR` | `∨` |
| `$TA` | `↑` | | `$LO` | `⍟` | | `$NT` | `~` |
| `$DR` | `↓` | | `$CI` | `○` | | `$NG` | `¯` |
| `$RV` | `⌽` | | `$FA` | `!` | | `$QD` | `⎕` |
| `$TR` | `⍉` | | `$RO` | `?` | | `$QQ` | `⍞` |
| `$MU` | `×` | | `$DI` | `÷` | | `$CM` | `⍝` |
| `$SO` | `∘` | | `$IB` | `⌶` | | | |

`$AB`, `$FA`, `$RO` and `$NT` are also on the 029 as `|`, `!`, `?`
and `¬`; either is accepted.

Corrected: the notes put NAND on `⊼`, which is not an APL character,
and NOR on `⍲`, which is NAND. (U) keeps their names on the right
glyphs: `$NA` is `⍲` (nand) and `$NO` is `⍱` (nor).

Ours, for what APL\360 has and neither the 029 nor the notes give:

| Card | APL | | Card | APL | | Card | APL |
|---|---|---|---|---|---|---|---|
| `$LB` | `[` | | `$SB` | `⌿` | | `$DC` | `⊥` |
| `$RB` | `]` | | `$BB` | `⍀` | | `$EC` | `⊤` |
| `$BS` | `\` | | `$RF` | `⊖` | | `$GU` | `⍋` |
| `$DT` | `∆` | | | | | `$GD` | `⍒` |

Left out: `$DE` and `$FM`, the notes' execute and format (and `$DE`
is also their second name for del); `⍫`, as a deck has no one to lock
a function against; `⍙`.

## The run stream

Control cards have `@` in column 1 (EXEC 8; the notes):

```text
@RUN   ALBANY,76104,CS301,1,20
@APL,I
... the APL cards ...
@FIN
```

- `@RUN` opens the run; its fields (user, account, project, time,
  pages) are listed and not checked.
- `@APL` calls the processor; its options are listed and ignored.
  Cards before it are listed and not run.
- `@FIN` ends the run; cards after it are listed and not run. A deck
  that ends without it ends the run there.
- No `)` commands, by the owner's direction, so no `)OFF` (the notes
  have one before `@FIN`). A card whose statement begins with `)` is
  INCORRECT COMMAND.
- A deck without `@RUN` or `@APL` still runs, with a line in the
  listing saying what was missing (ours: the demo should forgive).

## Functions

A `$DL` card with a header opens a definition; the cards that follow
are its lines, numbered 1, 2, ... in order; a card that is `$DL`
alone closes it. There is no editing: a wrong line is repunched and
the deck run again. A header naming a function already defined in the
run is DEFN ERROR.

## Input

`⎕` and `⍞` read the next card of the deck as data: `⍞` its columns
1-72, trailing blanks dropped, as they are punched; `⎕` the card's
statement, mnemonics and all, evaluated. With no card left the run
ends, with a line saying so (ours).

## The listing

Hagerty's print bar printed APL characters (Minker), so the listing
shows the glyphs. It also shows each card as punched, because the
user reads the two side by side to repunch.

- A banner naming University of Maryland APL, the date and time, and
  CLEAR WS (the notes; the wording is ours).
- Each card: its place in the deck, its 80 columns as punched; for an
  APL card, the statement in glyphs on the next line, indented six
  as APL echoes input; then its results.
- An error: APL\360's message, the statement in glyphs and the caret
  under the point of error, and the card's number. The run goes on
  with the next card; an error inside a function leaves it, the state
  indicator cleared (ours: there is no one to resume it).
- The page is a green-bar sheet, 132 columns; the print width is
  APL\360's default, 120.
- At the end, a line saying the run ended, and how: END OF RUN at
  `@FIN`; END OF DECK, NO @FIN CARD; or NO CARD LEFT TO READ. A
  deck that stops inside a definition says that first.
- An error names its place as `CARD 6 COLUMN 12`, or `CARD 6` when
  the caret is not on the card's own statement, as in a function it
  called.

## The deck as a file

At the CLI, `sw-apl --mode 74 -f DECK` (or a deck on stdin) runs a
text file, one card a line, and prints its listing. What lies past
column 80 is not on the card. A character an 029 cannot punch, which
a file can hold and a card cannot, is a CHARACTER ERROR at its
column. A data card is any card that is not a control card: a read
that meets a control card finds no data.
## The first deck

From the notes, a 2 by 2 system solved with domino. `$CM` makes the
first APL card a comment.

```text
@RUN   ALBANY,76104,CS301,1,20
@APL,I
$CM SOLVE A+.$MUX = B WITH DOMINO
A $AL 2 2 $RH 2 1 1 3
B $AL 5 10
X $AL B $DM A
$QD $AL X
@FIN
```

The listing shows `1 3`.
