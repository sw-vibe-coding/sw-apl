# BIRDS: combinators in each mode

BIRDS is a workspace in library 1, with a version for each mode --
`ws/lib1/BIRDS.a-70.apl.ws` and `ws/lib1/BIRDS.b-75.apl.ws` -- and
`)LOAD 1 BIRDS` takes the one for the mode the session is in:

```
      )LOAD 1 BIRDS
      DESCRIBE
      HOWBIRDS
```

It holds the combinators of Raymond Smullyan's *To Mock a
Mockingbird*, where each combinator is a bird. In (A) '70 they are the
ones APL\360 can write, with a note on the ones it cannot; in (B) '75,
which has execute, every bird in the list flies, and the note says
what is still missing. The (A) version comes first here; *In (B)
'75* below is the other. The list of birds is the core
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

## In (B) '75

(B) has execute, `⍎`, which runs characters as a line. That is what
(A) lacked: a function's name held as characters can now be called.
So in (B) a bird is given a function by its name or its glyph, in
quotes, and calls it with `⍎` -- any function, primitive or defined,
with no table in between:

```
∇R←X T F
R←⍎F,' X'
∇
```

`⍎` runs the line inside `T`, where `X` is `T`'s own argument, so
the function it names is applied to the value `T` was given.

| Bird | λ-term | (B) '75 | Example | Result |
|---|---|---|---|---|
| I, Idiot | `λx.x` | `I X` | `I 42` | `42` |
| K, Kestrel | `λx y.x` | `X K Y` | `7 K 9` | `7` |
| KI, Kite | `λx y.y` | `X KI Y` | `7 KI 9` | `9` |
| T, Thrush | `λx y.y x` | `X T F` | `6 T 'FACT'` | `720` |
| B, Bluebird | `λx y z.x (y z)` | `FG B X` | `'FACT ⌈' B 3.2` | `24` |
| C, Cardinal | `λx y z.x z y` | `F C Y Z` | `'-' C 10 3` | `¯7` |
| W, Warbler | `λx y.x y y` | `F W X` | `'HYP' W 3` | `4.2426` |
| S, Starling | `λx y z.x z (y z)` | `FG S X` | `'+ ⌽' S ⍳5` | `6 6 6 6 6` |
| M, Mockingbird | `λx.x x` | `M F` | `M 'SHOUT'` | `SHOUT!` |
| Y, Sage | `λf.(λx.f (x x)) (λx.f (x x))` | `F Y X` | `'FSTEP' Y 6` | `720` |

Two functions for the Bluebird or the Starling are written with a
space between them, `'FACT ⌈'`, because a name is more than one
character; the line `⍎` runs is then `FACT ⌈ X`, which is the
Bluebird.

**The Kite** is `K I` as Smullyan makes it: `'I' K X` is the name
`'I'`, whatever `X` is, and that name applied to `Y` is `Y`.

**The Mockingbird** applies a function to itself -- to its own name,
since that is how a function is given here. `M 'SHOUT'` is `SHOUT`
applied to `'SHOUT'`, and `SHOUT` puts an exclamation mark after its
argument.

**The Sage** gives recursion without a name. `FSTEP` is one step of a
factorial, and never names itself: its left argument is how to call
it again, as characters, and the Sage supplies it:

```
∇R←F Y X;SELF
SELF←'''',F,''' Y'
R←⍎'SELF ',F,' X'
∇
```

`'FSTEP' Y 6` is 720. APL evaluates an argument before it calls, so
the Sage it can write is the strict one, Z; here the two are the same
function.

What is still missing, `NOTHERE` says: a function as a value. A bird
gives back a value, never a function -- the Bluebird returns `F G X`,
not a new function `F G` -- and a defined function cannot be the
operand of `/` or `∘.`, which take primitives only, in (B) as in (A).

## Reference

`combinators.md` is the reference to every function in the workspace:
how to call it, what it takes and gives back, the glyphs `APPLY` and
`DYAD` reach, and the errors each can raise.

`samples/72-birds.apl` loads (A)'s workspace, flies every bird,
shows the table's limit, and prints `NOTHERE`.
`samples/80-birds-75.apl` does the same for (B)'s.
