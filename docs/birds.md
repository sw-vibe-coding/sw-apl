# BIRDS: the combinators APL\360 can write

BIRDS is a workspace in library 1:

```
      )LOAD 1 BIRDS
      DESCRIBE
      HOWBIRDS
```

It holds the combinators of Raymond Smullyan's *To Mock a
Mockingbird*, where each combinator is a bird -- the ones APL\360 can
write, and a note on the ones it cannot. The list of birds is the core
aviary from Software Wrighter's post [The Sage Bird: Y Combinators in
an Eager Array
Language](https://blog.softwarewrighter.com/2026/09/21/rabbit-hole-sage-y-combinator/),
and the same birds fly in another language at
[mlpl.softwarewrighter.com](https://mlpl.softwarewrighter.com/).

## Why only some of them

A combinator is a function of functions. The Bluebird takes two
functions and gives back their composition; the Mockingbird takes a
function and applies it to itself.

APL\360 cannot hand one defined function to another. A function's
arguments are arrays, never functions. There is no execute, so a name
held as characters cannot be called. There are no defined operators, so
the only functions that take functions are the primitive operators --
reduction, scan, the outer and inner products -- and they take only
primitives.

So the birds fall into three kinds, and the workspace says which is
which rather than faking any of them.

## The birds

| Bird | λ-term | APL\360 | Example | Result |
|---|---|---|---|---|
| I, Idiot | `λx.x` | `I X` | `I 42` | `42` |
| K, Kestrel | `λx y.x` | `X K Y` | `7 K 9` | `7` |
| KI, Kite | `λx y.y` | `X KI Y` | `7 KI 9` | `9` |
| T, Thrush | `λx y.y x` | `X T F` | `5 T '⍳'` | `1 2 3 4 5` |
| B, Bluebird | `λx y z.x (y z)` | `FG B X` | `'⌽⍳' B 5` | `5 4 3 2 1` |
| C, Cardinal | `λx y z.x z y` | `F C Y Z` | `'-' C 10 3` | `¯7` |
| W, Warbler | `λx y.x y y` | `F W X` | `'×' W 5` | `25` |
| S, Starling | `λx y z.x z (y z)` | `FG S X` | `'+⌽' S ⍳5` | `6 6 6 6 6` |

`HOWBIRDS` flies every one of them.

### On values: I, K and KI

These never apply anything, so nothing stops APL\360 writing them in
full generality. `K` keeps its left argument whatever the right one
is, a character vector as happily as a number:

```
      'KEEP' K 1 2 3
KEEP
```

Smullyan makes the Kite from two other birds: it is `K I`, the
Kestrel given the Idiot. Here the Kestrel cannot be given a function,
so `KI` is written out directly instead.

### Over a table of primitives: T, B, C, W and S

These apply a function to something, and the function has to reach
them somehow. Here it comes as the glyph that names it, in quotes. The
glyph is only a character; what turns it into the function is `APPLY`
for a monadic primitive and `DYAD` for a dyadic one. Each branches on
the glyph to a line that applies that primitive:

```
→(NEG,REC,MAG,SGN,CEIL,FLR,IOTA,SHAPE,REV,RAV)['-÷|×⌈⌊⍳⍴⌽,'⍳F]
```

That table is the limit, and the limit is real. `APPLY` reaches the
monadic primitives `- ÷ | × ⌈ ⌊ ⍳ ⍴ ⌽ ,`, and `DYAD` the dyadic
`+ - × ÷ ⌈ ⌊ * | ⍴ , ⌽ ↑ ↓`. A glyph outside them is an INDEX ERROR,
at the lookup:

```
      '⍉⍳' B 5
INDEX ERROR
APPLY[4]  →(NEG,REC,MAG,SGN,CEIL,FLR,IOTA,SHAPE,REV,RAV)['-÷|×⌈⌊⍳⍴⌽,'⍳F]
                                                        ^
```

A defined function cannot be named to these birds at all: without an
execute, its name is only characters.

A bird that wants two functions -- the Bluebird and the Starling --
takes their glyphs as one vector of two, `'⌽⍳'`, because an APL\360
function has at most two arguments. The Cardinal takes the two values
it flips the same way, as a pair.

### Passing a function by dynamic scope

The Warbler, the Cardinal and the Starling use a dyadic primitive, and
`DYAD` already has both of its arguments taken by the values. The
function has nowhere to go -- except the one place APL\360 does let a
function see something it was not given.

APL\360 scopes names dynamically. A function's locals are visible to
every function it calls, for as long as it runs. So each of these
birds makes `FN` one of its locals, sets it to the glyph, and calls
`DYAD`, and `DYAD` reads `FN` as its caller left it:

```
∇R←F W X;FN
FN←F
R←X DYAD X
∇
```

`FN` is local to `W`, so it is gone when `W` returns and nothing
outside ever sees it. This is how APL\360 passed what it had no syntax
to pass, and it is the one idea in BIRDS that belongs to APL rather
than to the lambda calculus.

## The birds that are not here

`NOTHERE` names them and says why.

**M, the Mockingbird**, is `λx.x x`: a function applied to itself. The
functions in BIRDS are glyphs, and no primitive takes a glyph as its
argument, so there is no `x x` to form.

**The Sage**, `Y`, takes a function and gives back its fixed point --
a value `x` for which `f x` is `x` -- which is recursion without a
name. APL is applicative: it evaluates an argument before the function
that takes it, and the Sage built that way never stops building
itself. Strict languages use **Z** instead, which delays the
self-application by one step and so terminates. BIRDS puts Z in the
Sage's place, and then cannot write Z either: it takes a function and
gives one back, and APL\360 can do neither.

Neither is needed. An APL\360 function calls itself by name:

```
∇R←FACT N
→(N>1)/MORE
R←1
→0
MORE:R←N×FACT N-1
∇
```

`FACT 6` is 720. Recursion by name is how APL\360 recurses, and it
leaves nothing for a fixed-point combinator to do.

## Reference

`combinators.md` is the reference to every function in the workspace:
how to call it, what it takes and gives back, the glyphs `APPLY` and
`DYAD` reach, and the errors each can raise.

`samples/72-birds.apl` loads the workspace, flies every bird, shows
the table's limit, and prints `NOTHERE`.
