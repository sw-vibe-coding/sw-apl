# When each APL feature arrived

Which of seven historical APLs had which feature, as their manuals
describe them. Every cell comes from the source named for its column,
paraphrased; the manuals are listed in `citations.md`.

## The columns

| Column | What it is | Source read |
|---|---|---|
| **APL\1130** | APL for the IBM 1130, 1968 | *APL\1130 Primer*, C20-1697-0, 1968, chiefly its command and operator tables (Appendices B and C) |
| **'68** | APL\360, released outside IBM | *APL\360 User's Manual*, August 1968 |
| **'70** | APL\360 as a Program Product | *APL\360-OS and APL\360-DOS User's Manual*, SH20-0906-0, December 1970 |
| **'75** | APLSV | *APL Language*, GC26-3847-0, and the *APLSV User's Guide*, SH20-1460-1, both March 1975 |
| **'80** | APL80 for the TRS-80 (disk version unless noted) | The *APL80* manual, Phelps Gates and Ramware, undated |
| **'83** | IBM Personal Computer APL | *APL Reference*, 1502219, May 1983 |
| **'85** | APL2 Release 2 | *An Introduction to APL2*, SH20-9229-1, December 1985, with its Summary of Amendments |

The year heads a column for the edition read, not always for the
product's first appearance. APLSV dates from 1973 and the 1975
manuals describe it as it then stood. APL2 first shipped in 1984, and
the 1985 edition says which things Release 2 added. The APL80 manual
carries no date, and 1980 is the owner's.

sw-apl's (A) '70 follows the '70 column. Its (B) '75 follows the APL
of the IBM 5100 family, which IBM derived from APLSV; `mode-b.md`
records how the two differ. APL\1130 is not modelled in sw-apl.

## How to read a cell

- **yes**: the source describes it.
- **part**: the source describes a restricted form, given in the notes.
- **no**: the source says it is absent, or a complete table or
  reference in that source leaves it out.
- **?**: the source does not settle it.
- **'94**: the 1985 edition does not show it, and IBM's 1994 APL2
  manuals do. It arrived by then; which release brought it is not
  established here.

A cell records what one edition documents. The Primer is a tutorial,
so only its tables count as complete. The APL80 manual describes only
how APL80 differs from APL, with a table of its functions.

## Data

| Feature | APL\1130 | '68 | '70 | '75 | '80 | '83 | '85 |
|---|---|---|---|---|---|---|---|
| Numeric and character arrays | yes | yes | yes | yes | yes | yes | yes |
| Arrays of more than two dimensions | yes | yes | yes | yes | yes | yes | yes |
| Mixed arrays, characters with numbers | no | no | no | no | no | no | yes |
| Nested arrays | no | no | no | no | no | no | yes |
| Complex numbers | no | no | no | no | no | no | yes |

## Primitive functions

| Feature | APL\1130 | '68 | '70 | '75 | '80 | '83 | '85 |
|---|---|---|---|---|---|---|---|
| Arithmetic, relations, logic, floor, ceiling, power, logarithm | yes | yes | yes | yes | yes | yes | yes |
| Factorial and binomial | yes | yes | yes | yes | part | yes | yes |
| Circular and hyperbolic functions | no | yes | yes | yes | part | yes | yes |
| Roll and deal | yes | yes | yes | yes | yes | yes | yes |
| Shape, reshape, ravel, catenate, index | yes | yes | yes | yes | yes | yes | yes |
| Index generator, index of, membership | yes | yes | yes | yes | yes | yes | yes |
| Compress and expand | yes | yes | yes | yes | yes | yes | yes |
| Reverse and rotate | yes | yes | yes | yes | yes | yes | yes |
| Transpose, monadic | yes | yes | yes | yes | yes | yes | yes |
| Transpose, dyadic | no | yes | yes | yes | part | yes | yes |
| Take and drop | no | yes | yes | yes | yes | yes | yes |
| Grade up and down | no | yes | yes | yes | yes | yes | yes |
| Grade with a collating sequence | no | no | no | no | no | yes | yes |
| Encode and decode | yes | yes | yes | yes | part | yes | yes |
| Matrix inverse and divide (domino) | no | no | yes | yes | no | yes | yes |
| Lamination | no | no | yes | yes | no | yes | '94 |
| Execute | no | no | no | yes | no | yes | yes |
| Format | no | no | no | yes | part | yes | yes |
| Picture format | no | no | no | no | no | yes | yes |
| Depth, enclose, disclose, first, pick, match, without, find | no | no | no | no | no | no | yes |
| Enlist, partition | no | no | no | no | no | no | '94 |

## Operators and syntax

| Feature | APL\1130 | '68 | '70 | '75 | '80 | '83 | '85 |
|---|---|---|---|---|---|---|---|
| Reduction | yes | yes | yes | yes | yes | yes | yes |
| Inner and outer product | yes | yes | yes | yes | yes | yes | yes |
| Axis in brackets | part | yes | yes | yes | part | yes | yes |
| Scan | no | no | no | yes | part | yes | yes |
| Each, and defined operators | no | no | no | no | no | no | yes |
| Replicate | no | no | no | no | no | no | '94 |
| Vector specification, `(A B)←` | no | no | no | no | no | no | yes |
| Comments, the lamp | ? | yes | yes | yes | yes | yes | yes |
| Several statements on a line, the diamond | no | no | no | no | no | no | '94 |

## System variables and functions

| Feature | APL\1130 | '68 | '70 | '75 | '80 | '83 | '85 |
|---|---|---|---|---|---|---|---|
| I-beam functions | no | yes | yes | ? | no | no | no |
| Quad-named system variables | no | no | no | yes | no | yes | yes |
| `⎕CR`, `⎕FX`, `⎕EX`, `⎕NL`, `⎕NC` | no | no | no | yes | no | yes | yes |
| Shared variables | no | no | no | yes | no | yes | yes |
| `⎕EA`, execute alternate | no | no | no | no | no | yes | yes |
| `⎕EC`, execute controlled | no | no | no | no | no | no | yes |
| `⎕TF`, transfer form, with `)IN` and `)OUT` | no | no | no | no | no | yes | yes |
| Peek, poke, machine-code calls | no | no | no | no | yes | yes | ? |

## Defining and editing functions

| Feature | APL\1130 | '68 | '70 | '75 | '80 | '83 | '85 |
|---|---|---|---|---|---|---|---|
| Del editor | yes | yes | yes | yes | no | yes | yes |
| `[N⎕M]` line editing | ? | yes | yes | ? | no | ? | ? |
| A line editor of another kind | ? | no | no | ? | yes | yes | yes |
| Full-screen function editor | no | no | no | no | no | yes | yes |
| Trace | yes | yes | yes | yes | yes | yes | yes |
| Locked functions | yes | yes | yes | yes | ? | yes | yes |
| Ambivalent defined functions | no | no | no | no | no | yes | ? |

## Workspaces and files

| Feature | APL\1130 | '68 | '70 | '75 | '80 | '83 | '85 |
|---|---|---|---|---|---|---|---|
| `)SAVE`, `)LOAD`, `)CLEAR`, `)FNS`, `)VARS` | yes | yes | yes | yes | yes | yes | yes |
| `)COPY` | no | yes | yes | yes | part | yes | yes |
| `)PCOPY` | no | yes | yes | yes | no | no | yes |
| `)ORIGIN`, `)DIGITS`, `)WIDTH` | no | yes | yes | no | no | no | no |
| Latent expression run on load | no | no | no | yes | yes | yes | yes |
| Files from within APL, without a shared variable | no | no | no | no | yes | no | no |
| Files through a shared variable | no | no | no | yes | no | yes | yes |

## Notes

**APL\1130.** Its Primer limits an array to 255 elements along any
one dimension, and says that labels, common libraries and passwords
were not in the program first distributed in February 1968. Axis
choice is the last or first dimension only, written with the two
forms of each symbol. Its operator table has no circular function,
take, drop, grade or domino.

**1968 to 1970.** The December 1970 manual adds matrix inverse and
divide, and lamination. Both manuals describe `[N⎕M]` line editing and
the three commands `)ORIGIN`, `)DIGITS` and `)WIDTH`.

**Scan.** Neither APL\360 manual has scan, in its text or its index.
The APLSV User's Guide has a section on it and the 1975 language
manual counts it among the operators. A later APL\360 edition, the
January 1973 GH20-0906-1, was not read.

**APLSV.** Execute, format, the quad names and shared variables are
the additions. Its guide says the older workspace functions built on
I-beams, such as `ORIGIN` and `SETFUZZ`, were redefined in terms of
the system variables. Whether APLSV kept any I-beams is left open.

**APL80.** The manual calls it a large subset of APL, and most of what
it says is limitation. Matrix inverse, lamination, hyperbolic
functions and the diagonal case of dyadic transpose are not
implemented. Encode and decode take vectors only, and factorial
integers only. Format is a statement of its own and not a function.
A `d` operator chooses the axis. Functions are written with `)DEF`
and `)EDIT`, with a line editor in the disk version, and not with the
del. The tape version also lacks scan, transpose, format, inner
product and the axis choice, and calculates to 6 digits. Its files,
through `)READ`, `)WRITE` and `)UPDATE`, need no auxiliary processor.
`)COPY` takes a whole workspace only.

**IBM PC APL.** Developed at the IBM Madrid Scientific Center and
announced in June 1983. Its extensions include ambivalent defined
functions, `⎕EA`, `⎕TF`, `⎕PK` for peek and poke, picture format and
dyadic grades. The main workspace is limited to 64K bytes, with an
elastic workspace beyond it. Files are reached through auxiliary
processors, over shared variables. Its i-beam character prefixes a
line sent to the local system while connected to a host, and is not
a function.

**APL2.** Vector specification, `⎕EC` and lowercase in names are what
the December 1985 edition lists as new in Release 2. Its other
entries here were in Release 1, or the edition does not distinguish.
It says the I-beam functions have been removed. IBM's 1994 migration
guide lists what APL2 added over VS APL, which is where the `'94`
cells come from: it was revised for later releases, so a feature it
lists may be later than 1985.

## Not covered

VS APL (1976), the APL\360 editions other than the two read, and
APL2 Release 1 have no column. Limits of workspace size and
precision are noted only where they shaped the language.
