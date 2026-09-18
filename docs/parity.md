# APL\360 parity checklist

This is the definition of done for the sw-apl CLI: one row per
APL\360 feature, with its status and what pins it. "done" means
implemented and locked by a unit test or a reg-rs transcript
(`tests/reg-rs`, seeded from `samples/`). Update this file in the
same step that changes a row.

Status: `done`, `part` (partly there; the note says what is
missing), `todo`.

## How we will know we have parity

We have no running APL\360 to compare against, so the oracle is
the IBM APL\360 User's Manual (1968) with its 1970 supplement,
and the criterion is:

1. Every row below is `done`.
2. The worked examples in the manual, transcribed into
   `samples/manual-*.apl`, reproduce the printed output character
   for character (indent, spacing, high minus, error lines).
3. Every sample in `samples/` runs without `NOT IMPLEMENTED`.
4. Everything outside the APL\360 character set is CHARACTER
   ERROR, and the quad-named system variables of later APLs are
   SYNTAX ERROR (they are not in the language).
5. GNU APL, where the two languages agree, gives the same values
   for the shared subset (secondary oracle, not installed here yet).

Deliberately out of scope and never counted against parity: the
multi-user features (`)MSG`, `)OPR`, `)PORTS`, sign-on numbers,
`)CONTINUE HOLD`), which get honest stub replies.

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
| A line `⍞←` left open carries across a statement boundary | todo | it carries within a statement, so a prompt and read written on two lines of one function share a line; in immediate execution the statement ends the line first |
| Character literals `'...'`, doubled quote, any Unicode inside | done | lex tests, samples 19, 20 |
| Bracket indexing `A[I;J]`, elided axes, index arrays of any rank | done | index tests, sample 48 |
| Indexed assignment with scalar extension | done | index tests, sample 48 |
| Branch `→`, labels as local constants, `→0`, branch off the end, empty branch, the `→LABEL×⍳COND` idiom | done | scan/eval/session tests, samples 50, 54 |
| Defined functions: del headers (`NAME`, `NAME B`, `A NAME B`, each with or without `R←`), locals, calls, the result at exit | done | eval/session tests, sample 53 |
| Definition mode: an opening `∇` collects body lines behind the `[n]` prompt until a closing `∇` | done | session/cli tests, sample 53 |
| Del editor: `[n]`, `[⎕]`, `[n⎕]`, `[∆n]`, fractional insert and renumber on close, header edit and rename `[0]`, reopen `∇NAME`, lock `⍫` | done | session tests, sample 55 |
| Dynamic scoping, recursion | done | eval tests, samples 53, 54. A recursion with no branch to stop it reaches DEPTH ERROR |
| State indicator: suspension, pendent callers, resumption with `→`n, clearing with a bare `→` | done | call/session tests, sample 56 |
| Suspending a call written inside a larger expression | todo | sw-apl unwinds it instead, since it cannot take up a half-evaluated expression; the error still names the function and line |
| Six-space prompt, batch echo | done | cli tests |
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
| `< ≤ = ≥ > ≠` | | done | with fuzz; samples 15, 47 |
| `?` | done | done | roll and deal via the random link (starts at 16807); samples 41, 47 |
| Scalar extension, RANK and LENGTH agreement | done | done | prims tests |
| Exact integers, float promotion, fuzz | done | | value tests |

## Mixed functions

| Glyph | Monadic | Dyadic | Notes |
|---|---|---|---|
| `⍳` | done | done | index of, one past the end when absent; sample 30 |
| `⍴` | done | done | |
| `,` | done | done | any rank, axis bracket, laminate, scalar and rank-1 conformance; sample 49. Monadic ravel with an axis is APL2, not implemented |
| `⌽ ⊖` | done | done | axis bracket; vector shifts; samples 08, 34 |
| `⍉` | done | done | dyadic permutes and takes diagonals; sample 35 |
| `↑ ↓` | | done | per axis, negatives, overtake fill; samples 07, 10, 44 |
| `/ ⌿` compress | | done | boolean left, either axis, axis bracket; sample 17 |
| `\ ⍀` expand | | done | boolean left, either axis, axis bracket |
| `⊥ ⊤` | | done | mixed radix, scalar extension, matrix columns; sample 36 |
| `∊` | | done | with the fuzz; sample 30 |
| `⍋ ⍒` | done | | vectors, stable, origin-aware; sample 32 |
| `⌹` | todo | todo | Phase 5 |
| `⌶` I-beams 20 to 27 | done | | time, processor time, space still free, terminals, sign-on, date, the line now executing, the state indicator; a left argument is DOMAIN ERROR; samples 58 and 62 |

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
| Character arrays without quotes | done (no literals yet) |
| `)WIDTH` wrapping (vectors between elements, matrices in column blocks) | done |
| Mixed integer and float columns (each element formatted, right-aligned) | done |

## Workspaces

| Feature | Status | Pinned by |
|---|---|---|
| The workspace as a value: what `)SAVE` writes is separate from the terminal it runs on | done | workspace tests |
| `)CLEAR` clears the state indicator with everything else | done | session tests, sample 60 |
| Workspace file: re-executable UTF-8, stable byte-for-byte, round trips through the interpreter | done | wsfile tests, `tests/scripts/saved-workspace.apl.ws` |
| A saved workspace keeps a suspended function | todo | a re-executable file cannot put execution back in the middle of a call, so `)SAVE` leaves the state indicator out; APL\360's workspaces were binary and kept it |
| `)SAVE` and `)LOAD` round trip a workspace through a file | done | command tests, sample 61 |
| `)COPY` takes the definitions and leaves the settings | done | command tests, sample 61; see `index-origin-considerations.md` |

## System commands

| Command | Status |
|---|---|
| `)OFF` | done |
| `)ORIGIN` `)DIGITS` `)WIDTH` | done |
| `)CLEAR` | done |
| `)WSID` | done |
| `)SAVE` `)LOAD` `)DROP` `)LIB` `)COPY` `)PCOPY` `)CONTINUE` | done |
| `)FNS` `)VARS` `)GRPS` `)GRP` `)GROUP` `)ERASE` | todo |
| `)SI` `)SIV` | done |
| `)SYMBOLS` | todo |
| Library form `)LOAD 1 NAME`, DESCRIBE convention | todo |
| `)LOAD` prints only the SAVED line, as APL\360 did, and nothing runs on load | done | command tests, sample 61 |
| `)MSG` `)OPR` `)PORTS` | stub |

## Errors

| Error | Raised where needed | Text pinned |
|---|---|---|
| SYNTAX, VALUE, DOMAIN, RANK, LENGTH | done | done |
| CHARACTER (with code point) | done | done |
| INDEX, DEFN, DEPTH | done | done |
| INTERRUPT when a read finds no more input, and from the keyboard | done | eval/cli/call tests |
| WS FULL | done | done; the workspace holds a fixed number of bytes (`--ws-size`, default 1048576) and anything that will not fit is refused without changing it; space, workspace and session tests, sample 62 |
| A monadic-only primitive used dyadically (`1~0`, `1⍋2`) | todo | answers NOT IMPLEMENTED; APL\360 has no such form, so it should be a SYNTAX ERROR |
| NOT IMPLEMENTED (temporary, must reach zero) | in use | done |
