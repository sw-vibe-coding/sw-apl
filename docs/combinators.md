# Combinators: the BIRDS reference

Every function in the BIRDS workspace -- how to call it, what it takes,
what it gives back, and what goes wrong. For why the workspace is built
the way it is, and which birds are missing, see `birds.md`.

```
      )LOAD 1 BIRDS
```

Names are as in *To Mock a Mockingbird*. A bird that applies a function
takes the function as the glyph that names it, in quotes.

## Summary

| Call | Bird | Result |
|---|---|---|
| `I X` | Idiot | `X` |
| `X K Y` | Kestrel | `X` |
| `X KI Y` | Kite | `Y` |
| `X T F` | Thrush | `F X` |
| `FG B X` | Bluebird | `F G X`, for `FG` the two glyphs `F` then `G` |
| `F C Y Z` | Cardinal | `Z F Y` |
| `F W X` | Warbler | `X F X` |
| `FG S X` | Starling | `X F G X`, for `FG` the two glyphs `F` then `G` |
| `F APPLY X` | -- | the monadic primitive `F`, on `X` |
| `X DYAD Y` | -- | `X FN Y`, for the glyph in the caller's `FN` |
| `FACT N` | -- | `N` factorial, by recursion |
| `DESCRIBE` | -- | what the workspace holds |
| `HOWBIRDS` | -- | every bird, run |
| `NOTHERE` | -- | the birds that are missing, and why |

## The birds on values

These apply nothing, so they take any arrays at all.

### `I X` -- Idiot

The identity: `X`, unchanged.

```
      I 42
42
```

### `X K Y` -- Kestrel

`X`, whatever `Y` is. The two need not agree in type or shape.

```
      7 K 9
7
      'KEEP' K 1 2 3
KEEP
```

### `X KI Y` -- Kite

`Y`, whatever `X` is. Smullyan's `K I`, written out.

```
      7 KI 9
9
```

## The birds over the table

Each takes its function as a glyph, and reaches only the primitives in
the tables below. The Thrush and the Bluebird use monadic primitives,
through `APPLY`. The Cardinal, the Warbler and the Starling use a
dyadic one, through `DYAD`; the Starling uses a monadic one as well.

### `X T F` -- Thrush

`F X`: the value first, the function after, read as a pipeline. `F` is
one glyph from the monadic table.

```
      5 T '⍳'
1 2 3 4 5
      (⍳5) T '⌽'
5 4 3 2 1
```

### `FG B X` -- Bluebird

`F G X`: composition, `G` first. `FG` is a vector of two glyphs from
the monadic table, `F` then `G`.

```
      '⌽⍳' B 5
5 4 3 2 1
```

### `F C Y Z` -- Cardinal

`Z F Y`: the dyadic function `F` with its arguments flipped. `F` is one
glyph from the dyadic table; the right argument is the pair `Y Z`.

```
      '-' C 10 3
¯7
      '÷' C 2 10
5
```

### `F W X` -- Warbler

`X F X`: the dyadic function `F` with both arguments `X`. `F` is one
glyph from the dyadic table.

```
      '×' W 5
25
      '+' W 2 3 4
4 6 8
      '⍴' W 2 3
2 3 2
3 2 3
```

### `FG S X` -- Starling

`X F G X`: `X` given to the dyadic `F` on the left, and to the monadic
`G` whose result is `F`'s right. `FG` is two glyphs, `F` from the
dyadic table then `G` from the monadic.

```
      '+⌽' S ⍳5
6 6 6 6 6
      '×⍳' S 4
4 8 12 16
```

## Reaching the primitives

### `F APPLY X`

The monadic primitive whose glyph is `F`, applied to `X`.

| Glyph | Function |
|---|---|
| `-` | negative |
| `÷` | reciprocal |
| `\|` | magnitude |
| `×` | signum |
| `⌈` | ceiling |
| `⌊` | floor |
| `⍳` | index generator |
| `⍴` | shape |
| `⌽` | reverse |
| `,` | ravel |

```
      '⌈' APPLY 2.3 ¯1.5
3 ¯1
```

### `X DYAD Y`

`X FN Y`, where `FN` is the glyph of a dyadic primitive. `FN` is not an
argument: it is a local of whichever function called `DYAD`, which the
Cardinal, the Warbler and the Starling each set before calling it, and
`DYAD` sees it by dynamic scope. `birds.md` says why.

| Glyph | Function |
|---|---|
| `+` | add |
| `-` | subtract |
| `×` | multiply |
| `÷` | divide |
| `⌈` | maximum |
| `⌊` | minimum |
| `*` | power |
| `\|` | residue |
| `⍴` | reshape |
| `,` | catenate |
| `⌽` | rotate |
| `↑` | take |
| `↓` | drop |

## The rest

### `FACT N`

`N` factorial, for a whole number `N`, by a function that calls itself
by name. It stands in the Sage's place: recursion needs no fixed-point
combinator when a function can name itself. `FACT 0` and `FACT 1` are
1.

```
      FACT 6
720
      FACT 10
3628800
```

It is a factorial only of whole numbers: `FACT 2.5` is `2.5×1.5×1`,
which is 3.75.

### `DESCRIBE`, `HOWBIRDS`, `NOTHERE`

Niladic. `DESCRIBE` lists the birds and how to call them, as every
library 1 workspace's does. `HOWBIRDS` prints each bird's call and runs
it. `NOTHERE` names the birds APL\360 cannot write and says why.

## Errors

A bird does no checking of its own. What goes wrong is reported where it
goes wrong, with the caret on the cause.

| Error | When | Example |
|---|---|---|
| INDEX ERROR | a glyph that is not in the table the bird uses | `'⍉⍳' B 5`, `3 T '+'` |
| RANK ERROR | one glyph given to the Bluebird or the Starling, which want two, or one value to the Cardinal, which wants a pair | `'⍳' B 5`, `'-' C 5` |
| VALUE ERROR | `DYAD` called with no `FN` in scope | `3 DYAD 4` |
| DOMAIN ERROR, LENGTH ERROR, ... | whatever the primitive itself refuses | `'×' W 'AB'` |

The monadic table has no `+`, so the Thrush, the Bluebird and the
Starling's second glyph cannot use it.

The report names the line that failed, with the caret on the cause. The
function left suspended is the bird you called, not `APPLY` or `DYAD`
within it: a function called inside an expression unwinds on the way
out. A bare `→` clears it:

```
      '⍉⍳' B 5
INDEX ERROR
APPLY[4]  →(NEG,REC,MAG,SGN,CEIL,FLR,IOTA,SHAPE,REV,RAV)['-÷|×⌈⌊⍳⍴⌽,'⍳F]
                                                        ^
      →
```
