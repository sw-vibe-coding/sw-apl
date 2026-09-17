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
| Quad input `⎕` | todo | |
| Quote-quad `⍞` input and output | todo | |
| Character literals `'...'`, doubled quote, any Unicode inside | done | lex tests, samples 19, 20 |
| Bracket indexing `A[I;J]`, elided axes | part | parsed (parse tests); evaluation in Phase 2 |
| Indexed assignment | part | parsed; evaluation in Phase 2 |
| Branch `→`, labels, `→0`, empty branch | part | `→` parsed (statement start only); semantics in Phase 3 |
| Del editor: headers, locals, `[n]`, `[⎕]`, `[∆n]`, fractional insert, `⍫` | todo | |
| Dynamic scoping, recursion, state indicator | todo | |
| Six-space prompt, batch echo, `)OFF` | done | cli tests |
| Error display: name, statement, caret | done | session tests |
| Error display inside functions (`FN[n]`), `)SI` | todo | |
| `)WIDTH` wrapping with six-space continuation | done | display tests, sample 45 |
| `)DIGITS` precision, `)ORIGIN`, `WAS n` reply | done | session tests, sample 22 |
| Strict Unicode acceptance, CHARACTER ERROR with lookalike hint | done | lex, value, session tests |
| Lexer: brackets, semicolon, colon, branch arrow, del, del-tilde, quote-quad, system command lines, strands, bracket balance | done | lex tests |
| Parser: axis brackets `f[k]`, compress vs reduce by context, SYNTAX ERROR carets, multiple assignments | done | parse tests |
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
| `?` | done | todo | roll via the random link (starts at 16807); deal in Phase 2 |
| Scalar extension, RANK and LENGTH agreement | done | done | prims tests |
| Exact integers, float promotion, fuzz | done | | value tests |

## Mixed functions

| Glyph | Monadic | Dyadic | Notes |
|---|---|---|---|
| `⍳` | done | todo | index of |
| `⍴` | done | done | |
| `,` | done | done | any rank, axis bracket, laminate, scalar and rank-1 conformance; sample 49 |
| `⌽ ⊖` | done | done | axis bracket; vector shifts; samples 08, 34 |
| `⍉` | done | done | dyadic permutes and takes diagonals; sample 35 |
| `↑ ↓` | | done | per axis, negatives, overtake fill; samples 07, 10, 44 |
| `/ ⌿` compress | | todo | |
| `\ ⍀` expand | | todo | |
| `⊥ ⊤` | | todo | |
| `∊` | | todo | |
| `⍋ ⍒` | todo | | |
| `⌹` | todo | todo | Phase 5 |
| `⌶` I-beams 20 to 27 | todo | | Phase 3 |

## Operators

| Form | Status | Notes |
|---|---|---|
| Reduce `f/` last axis | done | scalar dyadic f only |
| Reduce first axis `f⌿`, axis `f/[k]` | part | parsed; evaluation in Phase 2 |
| Scan `f\`, `f⍀` | part | parsed; evaluation in Phase 2 |
| Inner product `f.g` | part | parsed (dyadic only); evaluation in Phase 2 |
| Outer product `∘.f` | part | parsed (dyadic only); evaluation in Phase 2 |

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

## System commands

| Command | Status |
|---|---|
| `)OFF` | done |
| `)ORIGIN` `)DIGITS` `)WIDTH` | done |
| `)CLEAR` | todo |
| `)WSID` `)SAVE` `)LOAD` `)DROP` `)LIB` `)COPY` `)PCOPY` `)CONTINUE` | todo |
| `)FNS` `)VARS` `)GRPS` `)GRP` `)GROUP` `)ERASE` | todo |
| `)SI` `)SIV` | todo |
| `)SYMBOLS` | todo |
| Library form `)LOAD 1 NAME`, DESCRIBE convention | todo |
| `)MSG` `)OPR` `)PORTS` | stub |

## Errors

| Error | Raised where needed | Text pinned |
|---|---|---|
| SYNTAX, VALUE, DOMAIN, RANK, LENGTH | done | done |
| CHARACTER (with code point) | done | done |
| INDEX, DEFN, DEPTH, WS FULL, INTERRUPT | todo | done |
| NOT IMPLEMENTED (temporary, must reach zero) | in use | done |
