# I-beam Reference

`⌶` is a monadic function whose argument selects a system value.
There are eight, numbered 20 to 27. They are how APL\360 reported
the time, the clock and the state of the workspace, and they are how
sw-apl reports them: the quad-named system variables and system
functions that replaced them in later APLs arrived with APL.SV, and
sw-apl has none of them.

`language.md` has the one-line table. This is the long version:
what each reports, in what units, and how to turn it into something
a person would want to read.

Four of the eight read the clock, so the numbers shown below are
from one run and yours will differ. The shapes of the answers are
what to read.

## The eight

| | Reports | Units |
|---|---|---|
| `⌶20` | Time of day | Sixtieths of a second since midnight |
| `⌶21` | Processor time used this session | Sixtieths of a second |
| `⌶22` | Workspace space still free | Bytes |
| `⌶23` | Terminals connected | A count, always 1 |
| `⌶24` | Time this session signed on | Sixtieths of a second since midnight |
| `⌶25` | Today's date | The integer MMDDYY |
| `⌶26` | The line now executing | A line number, 0 for none |
| `⌶27` | The state indicator | A vector of line numbers, innermost first |

Each answers a scalar except `⌶27`, which answers a vector.

## Reading the clock

Times are in sixtieths of a second, which is the unit a 360 counted
in. Divide by 60 for seconds:

```apl
      ⌶20
3129818
```

That is a number of sixtieths, not a time. Encode it to get one:

```apl
      60 60 60⊤⌊(⌶20)÷60
14 29 23
```

Hours, minutes, seconds -- twenty-nine minutes and twenty-three
seconds past two in the afternoon. The hour alone is:

```apl
      ⌊(⌶20)÷60×60×60
14
```

`⌶24` is the same unit, so the two subtract to how long this session
has been connected:

```apl
      60 60 60⊤⌊((⌶20)-⌶24)÷60
```

`⌶21` is processor time in the same unit, so seconds of it are:

```apl
      (⌶21)÷60
```

On a machine this fast that is usually 0: a session has to work for
a while before it has used a sixtieth of a second.

## Reading the date

`⌶25` is one integer holding month, day and year, two digits each:

```apl
      ⌶25
91826
```

The leading zero of a month before October is not there to see,
which is what the integer form costs. Take it apart with `⊤`:

```apl
      100 100 100⊤⌶25
9 18 26
```

## Reading the space

`⌶22` is the bytes still free, of a workspace whose size is set by
`--ws-size`. In kilobytes:

```apl
      ⌊(⌶22)÷1024
1024
```

It falls as the workspace fills and rises when names go away.
`workspaces.md` has what a value costs and what happens when one
will not fit.

## The state indicator

`⌶26` and `⌶27` read the state indicator, which is the stack of
calls that have started and not finished. Inside a function, `⌶26`
is the line running now:

```apl
      ∇R←WHERE
[1]   R←0
[2]   R←⌶26
[3]   ∇
      WHERE
2
```

`⌶27` is every line on the stack, innermost first, so a function
called from another reports both:

```apl
      ∇R←OUTER
[1]   R←0
[2]   R←INNER
[3]   ∇
      ∇R←INNER
[1]   R←⌶27
[2]   ∇
      OUTER
1 2
```

INNER is on its line 1 and OUTER on its line 2.

They read the state indicator rather than the call: a function that
suspends stays on the stack, so after an error they report it from
immediate execution too.

```apl
      ∇FAILS
[1]   NOSUCH
[2]   ∇
      FAILS
VALUE ERROR
FAILS[1]  NOSUCH
          ^
      ⌶26
1
```

With nothing on the stack, `⌶26` is 0 and `⌶27` is empty:

```apl
      →
      ⌶26
0
      ⍴⌶27
0
```

`)SI` shows the same stack with the function names, which is what
you want when reading; `⌶27` is what you want when computing.

## What is not there

`⌶23` is always 1. APL\360 ran on a shared machine and counted the
terminals signed on to it; sw-apl runs on yours, so there is one and
it is yours.

There is nothing here for accounts, ports or other users. sw-apl has
no accounts, and the commands APL\360 had for talking to them --
`)MSG`, `)OPR`, `)PORTS` -- are not implemented: they answer
INCORRECT COMMAND like any other name the session does not know.

## Errors

`⌶` is monadic. A left argument is DOMAIN ERROR, because there is
nothing for one to mean:

```apl
      1⌶20
DOMAIN ERROR
      1⌶20
       ^
```

So is an argument that is not one whole number between 20 and 27:

```apl
      ⌶19
DOMAIN ERROR
      ⌶28
DOMAIN ERROR
      ⌶20.5
DOMAIN ERROR
      ⌶20 21
DOMAIN ERROR
```

An axis bracket is a SYNTAX ERROR, since `⌶` takes no axis:

```apl
      ⌶[1]20
SYNTAX ERROR
      ⌶[1]20
      ^
```

A bracket *after* the argument is not an axis but an index, and
indexing a scalar is a RANK ERROR:

```apl
      ⌶20[1]
RANK ERROR
```

## Reproducing a transcript

Four of these read the clock, so what they print differs every run.
A sample that shows one labels it, and the regression filter masks
what follows the label; `samples/58-ibeams.apl` is the worked
example and `testing.md` explains the convention.

`⌶20` is also the usual way to make a program that deals
differently every run, by advancing the random link a
clock-dependent number of times:

```apl
      ∇STIR;N
[1]   N←?(1+60|⌶20)⍴2
[2]   ∇
```

The rolls are thrown away; only their effect on the link is wanted.
