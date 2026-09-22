# APL\360 parity checklist

This is the definition of done for the sw-apl CLI: one row per
APL\360 feature, with its status and what pins it. "done" means
implemented and locked by a unit test or a reg-rs transcript
(`tests/reg-rs`, seeded from `samples/`). Update this file in the
same step that changes a row.

Status: `done`, `part` (partly there; the note says what is
missing), `todo`. A row that is neither -- something sw-apl does
not do and will not -- is not a row at all: it is under
**Restrictions** at the foot of this file, with the decision it
follows from.

At the time of writing there are **no** `todo` rows: everything
here is `done`, and everything sw-apl will not do is a restriction
with a reason. The check, matching table rows and not this
sentence: `grep -c '^|.*| todo |' docs/parity.md` prints 0.

## How we will know we have parity

We have no running APL\360 to compare against, so the oracle is
the IBM APL\360 User's Manual (1968, and its 1970 edition), with the
APL\360-OS/DOS manual of December 1970 for domino,
and the criterion is:

1. Every row below is `done`.
2. The worked examples in the manual, transcribed into
   `samples/manual-*.apl`, reproduce the printed output character
   for character (indent, spacing, high minus, error lines).
3. Every sample in `samples/` runs without an unimplemented reply; the error kind no longer exists.
4. Everything outside the APL\360 character set is CHARACTER
   ERROR, and the quad-named system variables of later APLs are
   SYNTAX ERROR (they are not in the language).
5. GNU APL, where the two languages agree, gives the same values
   for the shared subset (secondary oracle, not installed here yet).

Deliberately out of scope and never counted against parity: the
multi-user features (`)MSG`, `)OPR`, `)PORTS`, sign-on numbers,
`)CONTINUE HOLD`). They answer INCORRECT COMMAND, like any other
name the session does not know; `commands-reference.md` lists them
and says why each is absent.

## Syntax and the session

| Feature | Status | Pinned by |
|---|---|---|
| Numeric literals, high minus, E notation | done | lex tests, samples 01, 03 |
| Strands, names, lamp comments, parentheses | done | lex/parse tests, sample 21 |
| Right-to-left evaluation, long right scope | done | parse/eval tests, sample 01 |
| Assignment, variables | done | sample 02 |
| Quad output `⎕←` | done | session tests, sample 20 |
| Quad input `⎕`: prompts `⎕:`, evaluates the reply in the current environment, prompts again for a reply with no value | done | eval/cli tests, sample 57 |
| Quote-quad `⍞`: reads characters without evaluating, and on the left writes with no line ending so a prompt and its answer share a line | done | eval/cli tests, sample 57 |
| A line `⍞←` left open carries across a statement boundary | done | `Reply` says its last line is unfinished and both shells honour it: the batch runner writes it without a newline and the reader prompts with it, as a terminal's carriage would sit there. Open-line tests, `tests/scripts/open-line.apl` |
| Character literals `'...'`, doubled quote, any Unicode inside | done | lex tests, samples 19, 20 |
| Bracket indexing `A[I;J]`, elided axes, index arrays of any rank | done | index tests, sample 48 |
| Indexed assignment with scalar extension | done | index tests, sample 48 |
| Branch `→`, labels as local constants, `→0`, branch off the end, empty branch, the `→LABEL×⍳COND` idiom | done | scan/eval/session tests, samples 50, 54 |
| Defined functions: del headers (`NAME`, `NAME B`, `A NAME B`, each with or without `R←`), locals, calls, the result at exit | done | eval/session tests, sample 53 |
| Definition mode: an opening `∇` collects body lines behind the `[n]` prompt until a closing `∇` | done | session/cli tests, sample 53 |
| Del editor: `[n]`, `[⎕]`, `[n⎕]`, `[∆n]`, fractional insert and renumber on close, header edit and rename `[0]`, reopen `∇NAME`, lock `⍫` to open or to close, either being enough, and `⍫NAME` reopening an unlocked function to lock it | done | session tests, locked tests, sample 55 |
| Dynamic scoping, recursion | done | eval tests, samples 53, 54. A recursion with no branch to stop it reaches DEPTH ERROR |
| State indicator: suspension, pendent callers, resumption with `→`n, clearing with a bare `→` | done | call/session tests, sample 56 |
| Six-space prompt, batch echo | done | cli tests |
| `aplterm`, a 2741 terminal: key translation, overstrikes formed at the keyboard, circled display of underscored letters, composed Unicode lines on the wire | done | `components/term`: keyboard tests, display tests, and a PTY test that drives the real binary and checks what it shows and what it sends |
| The 2741 keyboard in a browser: the same keymap, the same overstrikes, the same cells, from the same crate compiled to WebAssembly | done | `components/web/crates/apl-board`: what a keystroke means is tested natively; the page itself is checked by driving a browser, where `"wsid` types `)WSID` and `○` Ctrl-`]` `*` composes `⍟` |
| A session with no service at all: the interpreter compiled to WebAssembly, running on a Web Worker, with `⎕`, `⍞` and the del editor reading through a shared channel | done | `components/web/crates/apl-wasm`: the channel layout is tested natively; the session itself is checked by driving the built page in a browser, since a worker and a `SharedArrayBuffer` have no native equivalent. `)SAVE` and `)LOAD` have no filesystem under them and report so |
| A local service: one session per connection, each with its own workspace, over a socket for `aplterm` and over a WebSocket for a browser. `⎕`, `⍞` and the del editor read from the terminal mid-statement | done | `components/web`: link, socket and browser tests, the last driving the real binary over HTTP and a WebSocket. Remote interruption (ATTN while a statement runs) is not implemented |
| Executable `.apl` file: a leading `#!` line is the shell's and is not read as APL | done | reg-rs CLI tests, `tests/scripts/` |
| `)OFF` sign-off: time and date, connect time, processor time | done | session tests, sample 59 |
| Line editing with history (up arrow), Ctrl-C cancels a line, Ctrl-D signs off | done | manual (rustyline); tape |
| Interrupt a running statement: INTERRUPT, suspended, state indicator kept | done | call tests; a body is read between its lines, so one long line cannot yet be stopped |
| Error display: name, statement, caret | done | session tests |
| Error display inside functions: `FN[n]` heading the statement, caret aligned under it | done | session tests, sample 56 |
| `)WIDTH` wrapping with six-space continuation | done | display tests, sample 45 |
| `)DIGITS` precision, `)ORIGIN`, `WAS n` reply | done | session tests, sample 22 |
| Strict Unicode acceptance, CHARACTER ERROR with lookalike hint | done | lex, value, session tests |
| Lexer: brackets, semicolon, colon, branch arrow, del, del-tilde, quote-quad, system command lines, strands, bracket balance | done | lex tests |
| Parser: axis brackets `f[k]`, compress vs reduce by context, SYNTAX ERROR carets, multiple assignments | done | parse tests |
| Axis brackets accepted only where APL\360 allows them (the seven forms); anywhere else a SYNTAX ERROR | done | session audit test; the set is generated from `data/glyphs.toml` |
| The axis in brackets may be a one-element array: the notes to the manual's Table 3.8 let one replace any scalar | done | join tests, sample 74 |
| Structural functions on scalars and empty arrays | done | session audit test |
| Mixed output `'TEXT';X;'MORE'` (semicolon list) | done | session tests, sample 46 |
| Invalid UTF-8 reported with byte offset, run continues | done | cli tests |

## Scalar functions

| Glyph | Monadic | Dyadic | Notes |
|---|---|---|---|
| `+` | done | done | |
| `-` | done | done | |
| `×` | done | done | |
| `÷` | done | done | 0÷0 is 1 |
| `⌈` | done | done | tolerant |
| `⌊` | done | done | tolerant |
| `\|` | done | done | tolerant residue |
| `*` | done | done | |
| `⍟` | done | done | sample 47 |
| `○` | done | done | pi times; k from ¯7 to 7 |
| `!` | done | done | exact to 20, gamma beyond |
| `~` | done | | 0 and 1 only |
| `∧ ∨ ⍲ ⍱` | | done | 0 and 1 only |
| `< ≤ = ≥ > ≠` | | done | with fuzz; `=` and `≠` compare characters too (APL\360 User's Manual p. 3.8), element by element, in reduction and in inner and outer products; a character never equals a number; the other four take numbers only; a scan of characters is DOMAIN ERROR, since its result would mix characters and numbers; samples 15, 47, 75 |
| `?` | done | done | roll and deal via the random link (starts at 16807); deal takes a scalar or a one-element vector for each argument, so `A[(⍴A)?⍴A]` shuffles; samples 41, 47 |
| Scalar extension, RANK and LENGTH agreement | done | done | a scalar or a one-element array of any rank extends (APL\360 User's Manual, 1968 and 1970, p. 3.33); two one-element arrays of different shapes take the higher rank's shape, which the manual does not settle. Prims tests, sample 74 |
| Exact integers, float promotion, fuzz | done | | value tests |

| Overstrikes: a glyph struck from two characters, as on a 2741 | done | every struck glyph is there: the table is in `data/glyphs.toml` with its provenance per pair, and the underscored alphabet A̲ to Z̲ beside it as a rule -- any letter struck with `_` is a further character of the set, a letter in its own right and distinct from the plain one. `Ctrl-]` takes the carriage back, a file may use `0x08`, either order forms the glyph, and a pair that forms none is CHARACTER ERROR -- which is the manual's own answer, "Illegitimate overstrike" being a cause of one. An underscored letter is written as its letter and U+0332 and counted as one column; as character data it is two elements, where APL\360 had one of its 256. Strike, lexer, session and display tests, sample 71. |

## Numeric boundaries

Where the manual is specific, it is followed and quoted; where it is
not, the choice is ours and is marked so. Pinned by the value tests,
the session numeric tests, and sample 67.

| Boundary | Whose | |
|---|---|---|
| The fuzz is about 1E¯13 | manual | "For operations such as floor and ceiling, and in comparisons, a 'fuzz' of about 1E¯13 is applied in order to avoid anomalous results that might otherwise be engendered by doing decimal arithmetic on a binary machine." |
| The fuzz is relative, not absolute | ours | The manual says "about 1E¯13" and not against what. Relative, so the same significant digits agree at any magnitude |
| Zero has no slack | ours | The tolerance is a fraction of the larger magnitude, so nothing but zero equals zero. Otherwise every small number would be zero |
| Two integers compare exactly | ours | They hold their value exactly, so a tolerance could only make two different ones equal |
| The fuzz applies to a count as well: `⍳(0.1+0.2)×10` is `1 2 3` | ours | The manual's list is "operations such as", and counting to a length that prints as 3, floors to 3 and compares equal to 3 is exactly the anomaly the sentence is about |
| A number is exact up to 2*53 | manual | `)DIGITS` "has no effect on the precision of internal calculations, which is approximately 16 decimal digits". Everything is a double, so the last exact integer is 2*53, not the width of a machine integer |
| Arithmetic past the end of i64 falls to the floating path rather than wrapping | ours | An implementation detail below what the manual describes; it must not produce a negative from two positives |
| `)DIGITS` caps integers too, so a wider one prints in exponential form | manual | "Subsequent output of numbers will show no greater number of significant digits than indicated." Later APLs print integers in full regardless; this follows the sentence |
| Exponential form only where it is needed | ours | The manual defines the form without saying when it is chosen |
| `⍳` of something too large to keep is WS FULL only on assignment | ours | The workspace bounds what it holds, not what an expression builds; see `workspaces.md` |

## Mixed functions

| Glyph | Monadic | Dyadic | Notes |
|---|---|---|---|
| `⍳` | done | done | index of, one past the end when absent; the index generator takes a scalar or a one-element vector, so `⍳⍴V` works; samples 30, 73 |
| `⍴` | done | done | |
| `,` | done | done | any rank, axis bracket, laminate, scalar and rank-1 conformance; sample 49. Monadic ravel with an axis is APL2, not implemented |
| `⌽ ⊖` | done | done | axis bracket; vector shifts; a one-element left argument rotates as a scalar does; samples 08, 34 |
| `⍉` | done | done | dyadic permutes and takes diagonals; sample 35 |
| `↑ ↓` | | done | per axis, negatives, overtake fill; samples 07, 10, 44 |
| `/ ⌿` compress | | done | boolean left, either axis, axis bracket; a scalar or one-element left argument extends, a scalar right argument does not (manual p. 3.41), so `1 0 1/7` is LENGTH ERROR; samples 17, 74 |
| `\ ⍀` expand | | done | boolean left, either axis, axis bracket |
| `⊥ ⊤` | | done | mixed radix, scalar extension, matrix columns; either argument of decode may be a one-element vector (manual p. 3.42); samples 36, 74 |
| `∊` | | done | with the fuzz; sample 30 |
| `⍋ ⍒` | done | | vectors, stable, origin-aware; sample 32 |
| `⌹` | done | done | inverse, left inverse, least-squares divide by Householder QR; vectors are one column and scalars one by one; singular, wider than tall, and characters are DOMAIN ERROR; rank 3 is RANK ERROR; sample 66. Not in the Aug 1968 manual: domino was added to APL\360 in 1970, and the rules followed are the APLX Language Manual's for the same lineage |
| `⌶` I-beams 20 to 27 | done | | time, processor time, space still free, terminals, sign-on, date, the line now executing, the state indicator; a left argument is DOMAIN ERROR; samples 58 and 62; `i-beam-reference.md` |

## Operators

| Form | Status | Notes |
|---|---|---|
| Reduce `f/` last axis | done | scalar dyadic f; identity table for empties |
| Reduce first axis `f⌿`, axis `f/[k]` | done | ops tests, sample 52 |
| Scan `f\`, `f⍀` | done | prefix reductions on either axis; samples 33, 52 |
| Inner product `f.g` | done | any ranks, scalar extension; sample 39 |
| Outer product `∘.f` | done | result shape is both shapes catenated; sample 38 |

## Display

| Feature | Status |
|---|---|
| Integers, high minus, single-space vectors | done |
| Numbers to `)DIGITS` significant digits, E form (integers too) | done |
| Matrix columns right-aligned | done |
| Empty vector prints a blank line | done |
| Rank 3 and higher (blank lines between planes) | done |
| Character arrays without quotes | done | display tests, samples 19 and 50 |
| `)WIDTH` wrapping (vectors between elements, matrices in column blocks) | done |
| Mixed integer and float columns (each element formatted, right-aligned) | done |

## Workspaces

| Feature | Status | Pinned by |
|---|---|---|
| The workspace as a value: what `)SAVE` writes is separate from the terminal it runs on | done | workspace tests |
| `)CLEAR` clears the state indicator with everything else | done | session tests, sample 60 |
| Workspace file: re-executable UTF-8, stable byte-for-byte, round trips through the interpreter | done | wsfile tests, `tests/scripts/saved-workspace.apl.ws` |
| Workspace file names the modes it runs in (`⍝!MODES`) and carries its settings and random link as directives both modes read | done | wsfile modes tests, session modes tests |
| Libraries by mode: `)LIB`, `)LOAD`, `)SAVE`, `)DROP`, `)COPY` see only the workspaces that run in the mode (`--mode 70` or `75`) | done | shelves tests, session modes tests |
| '70-only parts in their own crates (`components/a70/`), reached only in (A): the I-beams, `)ORIGIN` `)DIGITS` `)WIDTH`, `)GROUP` `)GRP` `)GRPS`. In (B) an I-beam is NONCE ERROR and those commands INCORRECT COMMAND | done | a70 crate tests, session modes tests |
| (B) '75: the lexer takes execute and format as primitives in (B) only, from `mode = "B"` in `data/glyphs.toml`; in (A) they are the same CHARACTER ERROR as ever | done | execute session tests |
| (B) '75: execute `⍎`, in `components/b75/`: a character scalar or vector run as a line; as a whole statement it shows what the line shows (nothing for an assignment or an empty line), inside an expression it must give a value (VALUE ERROR otherwise); an error in the line is the statement's, caret on the execute | done | execute session tests, sample 76 |
| (B) '75: execute and format struck from `⊥`/`⊤` and `∘`, the IBM 5100's pairs (`OVERSTRIKE_B`); in (A) each pair stays an illegitimate overstrike. The CLI composes for its `--mode`; `aplterm` for the mode each frame names; the browser board for the mode it is built with | done | strike, keyboard and link tests; sample 76 |
| (B) '75: format `⍕` | NONCE ERROR in (B) until implemented | |
| (B) '75: system names -- in (B) a quad directly before a letter is one name, `⎕IO`, from `[system_names]` in `data/glyphs.toml`; in (A) `⎕IO` is still quad beside a name, the same SYNTAX ERROR as ever | done | lexer tests, system variables session tests |
| (B) '75: system variables, in `components/b75/`: `⎕IO`, `⎕PP`, `⎕PW`, `⎕RL` read and set the workspace's own origin, precision, width and link, through the bounds the '70 commands and the directives use (DOMAIN ERROR outside them); `⎕CT` reads 1E¯13; `⎕LC`, `⎕WA`, `⎕AV` report and ignore an assignment; `⎕LX` is kept with the workspace and run when it is loaded; `⎕AI`, `⎕TS`, `⎕TT`, `⎕UL` hold the 5110's fixed values; any other quad name is SYNTAX ERROR | done | system variables session tests, atomic vector tests, sample 77 |
| (B) '75: a clear workspace prints five digits in a line of 64, and a whole number of up to ten digits in full | done | settings tests, display tests, system variables session tests |
| (B) '75: system functions, in `components/b75/`: `⎕CR` the function as a character matrix, flush left and without line numbers (anything else, a locked function included, gives a 0 by 0 matrix); `⎕FX` the function that a matrix spells, giving its name, or the number of the first line the editor could not have taken, changing nothing; `⎕EX` erases the active referent and says whether the name is free; `⎕NL` lists names by class and initial letter; `⎕NC` classifies names (0 free, 1 label, 2 variable, 3 function, 4 not a name); `⎕CC` checks what the 5110's console control is asked and answers, with no screen, alarm or printer to act on; `⎕DL` is the 5110's fixed value, not APLSV's delay | done | system functions session tests, console control tests, sample 78 |
| (B) '75: a function fixed under a name that is running, waiting, locked, a variable or local to a running function is refused, the header being the line at fault; local function names are not implemented | partial | system functions session tests |
| (B) '75: `⎕CT` set to another tolerance, a system variable localized in a function header, `⎕PW` 128 while a definition is open, indexed assignment into a system variable | NONCE ERROR or not implemented | |
| `)SAVE` and `)LOAD` round trip a workspace through a file | done | command tests, sample 61 |
| `)COPY` takes the definitions and leaves the settings | done | command tests, sample 61; see `index-origin-considerations.md` |

## System commands

| Command | Status |
|---|---|
| Only the first four characters of a command name are significant | done; `)CLEA`, `)CLEAR` and `)CLEAVER` are one command, and a name of four characters or fewer must be exact. Name tests, sample 68 |
| `)OFF` | done |
| `)ORIGIN` `)DIGITS` `)WIDTH` | done; `)WIDTH` accepts 30 to 254 where the manual's table says 30 to 130. Deliberate: a 360 printed on a terminal 130 columns wide at most, and a window today is wider. The lower bound and the `WAS n` reply are the manual's |
| `)CLEAR` | done |
| `)WSID` | done; a workspace name must be an APL name, which `)WSID` holds to as `)SAVE` does |
| A workspace name is an APL name | done; it becomes a filename, so `)SAVE A/B`, `)LOAD ../X` and the `NAME:LOCK` password form are INCORRECT COMMAND. Lower case is accepted, as it is for every other name here and was not in APL\360. Library name tests, sample 65 |
| `)SAVE` `)LOAD` `)DROP` `)LIB` `)COPY` `)PCOPY` `)CONTINUE` | done; `)DROP` replies with the moment alone, as APL\360's did |
| `)FNS` `)VARS` `)GRPS` `)GRP` `)GROUP` `)ERASE` | done |
| `)SI` `)SIV` | done |
| `)SYMBOLS` | reports; the number cannot be set -- see Restrictions |
| Library form `)LOAD 1 NAME`, DESCRIBE convention | done; `--library` sets the directory they are under, library 1 is `ws/lib1/`, and LIFE, RACE, EDIT and BIRDS each carry a DESCRIBE; library tests, samples 64 and 72. A library is somewhere workspaces are kept, not a directory: the browser keeps library 1 in the bundle and library 0 in its own storage, and the commands do not change. Store tests |
| `)LOAD` prints only the SAVED line, as APL\360 did, and nothing runs on load | done | command tests, sample 61 |
| Trouble reports: WS NOT FOUND, OBJECT NOT FOUND, IMPROPER LIBRARY REFERENCE, NOT SAVED THIS WS IS | done; INCORRECT COMMAND is kept for a command given an argument it does not take, as the manual's table has it; report tests, sample 65 |
| NOT WITH OPEN DEFINITION | done | done; `)SAVE`, `)COPY`, `)PCOPY` and `)CONTINUE` are refused while a definition is open, and no command is ever taken as a body line. Definition tests, sample 70 |
| `)COPY` and `)PCOPY` print the SAVED line, and `)PCOPY` a NOT COPIED list | done | the manual's WC3 and WC4: "SAVED, followed by the time of day and the date that the source workspace was last stored", and "NOT COPIED:, followed by the names of objects not copied". Command tests, samples 61 and 65 |
| `)NUMBER` `)OFF HOLD` `)CONTINUE HOLD` `)MSG` `)MSGN` `)OPR` `)OPRN` `)PORTS`, and the `[LOCK]`/`[KEY]` passwords | not implemented, and not wanted; they answer INCORRECT COMMAND. They are the multi-user surface of a shared machine -- accounts at ports, an operator, other users to message, a library someone else can read -- and sw-apl has one user. `commands-reference.md` says so per command |

## Errors

| Error | Raised where needed | Text pinned |
|---|---|---|
| SYNTAX, VALUE, DOMAIN, RANK, LENGTH | done | done |
| CHARACTER (with code point) | done | done |
| INDEX, DEFN, DEPTH | done | done |
| INTERRUPT when a read finds no more input, and from the keyboard | done | eval/cli/call tests |
| WS FULL | done | done; the workspace holds a fixed number of bytes (`--ws-size`, default 1048576) and anything that will not fit is refused without changing it; space, workspace and session tests, sample 62 |
| A glyph used where it has no such form (`1~0`, `1⍋2`, `⍳/1 2`, `,[1]M`) | done | done; SYNTAX ERROR, because APL\360 has no such function and the sentence does not parse. A glyph that *has* the form and was given a bad argument is still DOMAIN ERROR |
| NOT IMPLEMENTED (temporary, must reach zero) | reached zero; the error kind is gone from the vocabulary | |

## Restrictions

What sw-apl does not do, and will not. Each follows from a decision
already made and written down; none is work outstanding, and none
is counted against parity. They are here so that nobody -- reader
or agent -- sets about implementing one.

### A saved workspace does not keep a suspended function

APL\360's workspaces were binary images, and `)SAVE` really did
store a suspension: you could load a workspace and take up a
function in the middle of a call.

sw-apl's workspace file is re-executable APL, which is design.md
D7, and re-executing a file cannot put execution back into the
middle of a call. `)SAVE` leaves the state indicator out and
`)LOAD` gives you a workspace with an empty one. The names and the
settings all survive; only the suspension does not.

It is the price of a file you can read, diff and edit, and D7 pays
it deliberately.

### A call inside a larger expression does not suspend

When a defined function fails, APL\360 left it suspended wherever
it was called from. sw-apl does that too, unless the call was
written inside a larger expression: `2+F 3` unwinds instead of
suspending, because the evaluator is a recursive walk over the
tree and cannot take up a half-evaluated expression.

The error still names the function and the line it failed on, so
what went wrong is as clear; what is missing is being able to
resume from it.

### A locked function is obscured, not hidden

A binary workspace made a locked function genuinely unreadable to
whoever you sent it to. sw-apl's is text, and the writer must put
a locked body into it in full, so a workspace holding one is
written rot-13 instead (design.md D14).

That is obscuring and not encryption, and `workspaces.md` says so
plainly. It stops a locked body being read by accident or by
curiosity, which is all a binary format ever stopped, and it stops
nothing else.

### `)SYMBOLS` reports but cannot be set

APL\360 set a symbol table aside when you signed on, and
`)SYMBOLS n` in a clear workspace resized it. sw-apl sets nothing
aside: names are charged against the workspace like everything
else, so there is no separate table to size. `)SYMBOLS` reports
`IS n, USED m` and `)SYMBOLS n` is INCORRECT COMMAND.

### WS LOCKED and NOT SAVED, WS QUOTA USED UP cannot arise

`WS LOCKED` meant a stored workspace was behind a password. sw-apl
has one user, no accounts and no shared library, so there is
nothing to lock against and no key to be wrong.

`NOT SAVED, WS QUOTA USED UP` meant the account's library
allocation was full. sw-apl saves into a directory; if the disk is
full the operating system says so, and `)SAVE` passes that on as
`NOT SAVED, <reason>`. That is a different sentence about a
different thing.

Both belong to the multi-user surface that is out of scope at the
top of this file.
