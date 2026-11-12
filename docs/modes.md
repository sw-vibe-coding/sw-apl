# The modes compared

sw-apl runs in one mode per session, chosen when the session starts:
`--mode 70`, `75` or `80` at the CLI and the service, the tab in the
browser. This page shows, area by area, what each mode has.

- **(A) '70**, modeled on APL\360, typed on an IBM 2741.
- **(B) '75**, modeled on the APL of the IBM 5100 family (`mode-b.md`).
- **(C) '80**, modeled on APL80 for the TRS-80 (`mode-c.md`).

In the tables, **yes** means the mode has it, **no** that it has not,
and **part** that it has some of it, as the note says. The language
itself is in `language.md`; the session, its commands and its
display, in `session.md`.

## Data and names

| What | (A) '70 | (B) '75 | (C) '80 | Note |
|---|---|---|---|---|
| Numbers: integers and floating point | yes | yes | yes | |
| Characters | yes | yes | yes | |
| Flat arrays of any rank | yes | yes | yes | |
| Nested arrays | no | no | no | Not APL2 |
| Names of letters and digits, with underscored letters | yes | yes | part | Capitals and digits in (C), and `.` and `-` |
| `∆` in names | yes | yes | no | APL80 has none to type: a CHARACTER ERROR |
| `.` and `-` in names, as `M-N` | no | no | yes | |
| Comparison tolerance | fixed | `⎕CT` | fixed | |
| Index origin | `)ORIGIN` | `⎕IO` | 1 | |
| Random link, saved with the workspace | yes | yes | yes | |

## Scalar functions

| What | (A) '70 | (B) '75 | (C) '80 | Note |
|---|---|---|---|---|
| Arithmetic, comparison, logic, `⌈ ⌊ \| * ⍟ ? ~` | yes | yes | yes | |
| Circular `○`, circular and pythagorean | yes | yes | yes | |
| Hyperbolic `5○` to `7○`, `¯5○` to `¯7○` | yes | yes | no | NONCE ERROR in (C) |
| Factorial and binomial `!` | yes | yes | part | Whole numbers only in (C), through every operator |

## Mixed functions

| What | (A) '70 | (B) '75 | (C) '80 | Note |
|---|---|---|---|---|
| `⍴ ⍳ , ↑ ↓ ⍋ ⍒ ∊ ? /` and indexing | yes | yes | yes | |
| Reverse and rotate, `⌽ ⊖` | yes | yes | yes | |
| Compress and expand, `/ \ ⌿ ⍀` | yes | yes | yes | |
| Catenate | yes | yes | yes | |
| Laminate, `,[0.5]` | yes | yes | no | NONCE ERROR in (C) |
| Transpose `⍉`, monadic and dyadic | yes | yes | part | No diagonal case in (C) |
| Decode `⊥` and encode `⊤` | yes | yes | part | Scalars and vectors only in (C) |
| Domino `⌹`, inverse and divide | yes | yes | no | APL80 has no character for it |
| Execute `⍎` | no | yes | no | |
| Format `⍕`, monadic and dyadic | no | yes | no | (C) has the format statement instead |
| `#`, characters to APL80's codes and back | no | no | part | The screen codes `#` sends are not there |

## Operators

| What | (A) '70 | (B) '75 | (C) '80 | Note |
|---|---|---|---|---|
| Reduce `f/` and `f⌿` | yes | yes | yes | |
| Scan `f\` and `f⍀` | no | yes | yes | APL\360 had none |
| Inner product `f.g` | yes | yes | yes | |
| Outer product `∘.f` | yes | yes | yes | |
| An axis, `f/[1]M`; in (C), `f/1dM` | yes | yes | yes | |

## Statements

| What | (A) '70 | (B) '75 | (C) '80 | Note |
|---|---|---|---|---|
| Several specifications in one statement, `X←Y←9` | yes | yes | no | SYNTAX ERROR in (C) |
| Mixed output, `'A';X` | yes | yes | yes | |
| Quad and quote-quad input and output | yes | yes | yes | |
| The format statement, APL80's `f6 2` | no | no | yes | |

## System values

| What | (A) '70 | (B) '75 | (C) '80 | Note |
|---|---|---|---|---|
| I-beams `⌶20` to `⌶27` | yes | no | no | NONCE ERROR in (B) |
| Status, APL80's `s0` to `s5` | no | no | yes | The file pointers are 0 |
| Quad system variables: `⎕IO ⎕PP ⎕PW ⎕RL ⎕CT ⎕LC ⎕WA ⎕AV ⎕LX` and the fixed ones | no | yes | no | |
| Quad system functions: `⎕CR ⎕FX ⎕EX ⎕NC ⎕NL ⎕CC` | no | yes | no | |
| A clock | yes | part | yes | (B) has none, as the 5110 had none, unless started with `--clock hardware` |

## Defining functions

| What | (A) '70 | (B) '75 | (C) '80 | Note |
|---|---|---|---|---|
| The del editor, `∇` and `⍫` | yes | yes | no | CHARACTER ERROR in (C) |
| Line editing `[n⎕m]` | yes | part | no | (B) offers the line to type over, as the 5110 did |
| `)DEF` and `)EDIT`, with `)? )P n )n )0 )E n` | no | no | yes | An empty line ends one |
| Locked functions | yes | yes | no | |
| Limits on functions and lines | no | no | yes | 32 functions, 255 lines |

## System commands

| What | (A) '70 | (B) '75 | (C) '80 | Note |
|---|---|---|---|---|
| `)CLEAR )ERASE )SAVE )LOAD )COPY )FNS )VARS )SI )OFF` | yes | yes | yes | |
| `)WSID )PCOPY )DROP )LIB )SIV )SYMBOLS )CONTINUE` | yes | yes | yes | APL80 had none of these; (C) answers them as (A) does |
| `)ORIGIN )DIGITS )WIDTH` | yes | no | no | |
| `)GROUP )GRP )GRPS` | yes | no | no | |
| `)DEF )EDIT` | no | no | yes | Definition mode, above |
| `)TRON )TROFF` | no | no | yes | |
| `)PS )PD` | no | no | yes | |
| `)AUTO )EXEC` | no | no | yes | (B) has `⎕LX` |
| `)RESET )KILL )DOS )RAM`, the file commands, peek, poke, Call | no | no | no | APL80's, not there |
| `)DIALECT )LIBS )HELP`, sw-apl's own | yes | yes | yes | |

## Workspaces

| What | (A) '70 | (B) '75 | (C) '80 | Note |
|---|---|---|---|---|
| Lists and loads only the workspaces that run in it | yes | yes | yes | |
| A file shared with another mode | yes | yes | no | `NAME.ab.apl.ws` runs in (A) and (B); (C)'s are `NAME.c.apl.ws` alone |
| Functions written as | del | del | `)DEF` | |
| Latent expression | no | `⎕LX` | `)AUTO` | |
| A clear workspace prints | 10 digits in 120 | 5 digits in 64 | 6 digits in 64 | |

## Typing and display

| What | (A) '70 | (B) '75 | (C) '80 | Note |
|---|---|---|---|---|
| APL glyphs, typed and shown | yes | yes | no | A CHARACTER ERROR in (C) |
| Overstrikes | yes | yes | no | (B) adds the 5110's for `⍎` and `⍕`; APL80 had none |
| APL80's own characters: shifted letters as lower case, the substitutes, `-` the negative sign and `_` subtract, round-bracket indexing | no | no | yes | `language.md`, "What (C) '80 types" |
| The TRS-80's screen: 64 by 16, upper case, shifted letters drawn dotted | no | no | part | 64 columns; a terminal shows the lower-case letter |
| Errors | APL\360's | and NONCE ERROR | and NONCE ERROR | |
