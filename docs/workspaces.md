# Workspaces

A workspace is everything you have: the names you have defined, the
settings you are working under, and where the random link stands. It
is what `)SAVE` writes and `)LOAD` reads back. The terminal you are
sitting at is not part of it, which is why a workspace someone else
saved does not bring their console, their clock, or their sign-on
time along with it.

`session.md` has the commands as a reference table and `parity.md`
says which of them sw-apl answers yet. This is the guide.

## What is in one

| In the workspace | Not in the workspace |
|---|---|
| Variables and defined functions | The console it reads and writes through |
| Groups, which gather names under a name | The size `--ws-size` gives it |
| The index origin, print precision, print width | The clock the I-beams read |
| The random link | The sign-on time `⌶24` reports |
| The name `)WSID` reports | Whatever the current line has displayed |

The state indicator -- suspended and pendent functions -- belongs to
the workspace too, and `)CLEAR` clears it with everything else. But
it is not written to a file: see **What a saved file cannot hold**.

An unnamed workspace is called `CLEAR WS`, which is what `)WSID`
reports until you give it a name.

## Naming one

A workspace name is an APL name -- a letter, `∆` or `⍙`, then
letters, those two, and digits -- because the name becomes a
filename and a name that is not one could name something else
entirely. `)SAVE A/B` and `)LOAD ../OTHER` are `INCORRECT COMMAND`.


**There is no default name.** `CLEAR WS` is not one -- it is the
absence of one, printed where a name would go. A workspace gets its
name from you, in one of two ways:

```apl
      )WSID CLASS          ⍝ name it now, save it later
      )SAVE CLASS          ⍝ name it and save it in one command
```

`)SAVE` with no name uses the name the workspace already answers to,
so `)WSID CLASS` then `)SAVE` and `)SAVE CLASS` come to the same
thing. `)WSID` on its own reports the name without changing it, and
`)WSID NEWNAME` renames the workspace you are in -- the file you
saved under the old name is untouched, so renaming and saving leaves
you with two.

Saving a workspace that has never been named has nothing to write
under, and says so rather than inventing something:

```
      )CLEAR
CLEAR WS
      )SAVE
NOT SAVED, THIS WS IS CLEAR WS
```

## Libraries

Libraries are numbered, as in APL\360. Library 0 is yours; the
numbered ones are public.

| Library | Directory | |
|---|---|---|
| 0 | `work/` | Yours. `)SAVE` writes here and `)LOAD NAME` reads here. Not tracked by git; the first `)SAVE` creates it |
| 1 | `ws/lib1/` | The workspaces sw-apl ships, each with a DESCRIBE function |

`)LIB` lists library 0, `)LIB 1` lists library 1 -- one at a time, as
APL\360 did. A workspace is the file `NAME.apl.ws`. `--library DIR`
sets the directory they are all under, so a script or a test can work
somewhere other than your own `work/`.

Library 1 holds `LIFE` (Conway's Life on a torus), `RACE` (a horse
race written to be read) and `EDIT` (a workspace to practise the del
editor on, whose `FACT` is wrong by one on purpose). Each is a plain
text file you can open in an editor.

### Tracked, or not

`ws/` is tracked and `work/` is not, and the line between them is
who wrote the workspace rather than what is in it. Every workspace
under `ws/` carries a provenance directive:

```apl
⍝!SOURCE sw-apl -- written for this repository. MIT, (c) 2026 Michael A Wright.
```

`scripts/check-provenance.sh` fails the build when a tracked
workspace does not carry it, or when an untracked one appears under
`ws/` on its way to being added. The point is not the line but what
committing without it would mean: historical APL workspaces are IBM
material of unclear copyright, converting one is easy, and this makes
putting one in `ws/` a deliberate false claim rather than an
oversight. Anything from elsewhere belongs in `work/`. See
`aplcourse-how-to.md`.

`)SAVE` does not write the directive. A mark a program stamps on
everything asserts nothing.

## The file

A saved workspace is APL you could have typed:

```apl
⍝ sw-apl workspace. Re-executable APL: loading it runs it.
⍝!MODES (A)(B)
⍝!SAVED 20.00.00 09/17/26
⍝!LINK 282475249
⍝!ORIGIN 0
⍝!DIGITS 3
⍝!WIDTH 80
)WSID CLASS
M←2 3⍴0 1 2 3 4 5
∇R←A HYP B
R←((A*2)+B*2)*0.5
∇
```

Loading it is running it, so the same file works as a program:

```sh
sw-apl -f work/CLASS.apl.ws
```

A workspace file is a sample and a sample is a workspace. That is
deliberate: it keeps the format honest, and it means you can read,
diff and edit a saved workspace with ordinary tools.

Groups are written after the functions, as the `)GROUP` commands
that would gather them again. A group is only names, so it does not
matter whether what it names has been written yet.

Lines beginning `⍝!` are directives: comments to APL and
instructions to sw-apl.

| Directive | What it says |
|---|---|
| `⍝!MODES` | The modes the workspace runs in: `(A)`, `(B)` or `(A)(B)`. See *Modes* below |
| `⍝!SAVED` | When it was written, which `)LOAD` reports |
| `⍝!LINK` | Where the random link stands, so a loaded workspace carries on its sequence rather than starting over |
| `⍝!ORIGIN` | The index origin |
| `⍝!DIGITS` | The printing precision |
| `⍝!WIDTH` | The printing width |

The settings are directives rather than the `)ORIGIN`, `)DIGITS` and
`)WIDTH` commands because only (A) has those commands; every mode
reads a directive. A settings directive takes effect wherever the
line comes from -- a `)LOAD`, a file run with `-f`, or the keyboard --
and one out of range is ignored, as the command would refuse it.
Inside a function definition it is a comment line of the function.

A file written before the directives existed has `)ORIGIN`, `)DIGITS`
and `)WIDTH` lines instead and no `⍝!MODES` line. It still loads in
(A), which is the only mode it is listed in.

## Modes

sw-apl has two modes, (A) '70 and (B) '75, chosen with `--mode 70`
or `--mode 75` (the default is 70). A workspace is listed and loaded
only in the modes its `⍝!MODES` line names, and a file with no such
line is an (A) workspace.

`)SAVE` writes the line from what the workspace uses, not from the
mode it was saved in:

- an I-beam or a group makes it (A) only;
- execute, format, or a quad followed by a name (`⎕IO`, `⎕FX`) makes
  it (B) only;
- anything else -- quad and quote-quad included -- runs in both.

A glyph in a character literal or a comment is not a use.

A workspace that runs in both is kept once and listed in both.
Saving or dropping it in one mode changes it there, and in the other
mode too only where the other mode was listing the same workspace:

- A save replaces the workspace this mode lists under the name. If
  the new one no longer runs in a mode that listed the old one, that
  mode keeps the old one.
- A save is also listed in any other mode it runs in that has no
  workspace of that name. A workspace another mode keeps as its own is
  never touched, so an (A) workspace and a (B) one may share a name.
  The second is kept in the file `NAME@B.apl.ws` (or `NAME@A.apl.ws`),
  and both modes still call it `NAME`.
- `)DROP` in one mode takes the workspace out of that mode only; the
  file goes when no mode lists it.

## What a saved file cannot hold

A **suspended function**. The state indicator is a stack of half-run
calls, and re-executing a file cannot put execution back in the
middle of one. `)SAVE` leaves it out and `)LOAD` gives you a
workspace with an empty state indicator.

APL\360 did not have this problem, because its workspaces were binary
images and `)SAVE` really did preserve a suspension. It is the price
of a file you can read, and `parity.md` carries it under
**Restrictions** rather than as work outstanding.

## How much room is left

A workspace holds a fixed number of bytes. `⌶22` reports how many
are still free, and it is the only way APL\360 offered -- the `⎕WA`
of later systems is one more quad-name sw-apl does not have.

```apl
      ⌶22
1048576
      A←⍳100
      ⌶22
1047747
```

The default size is 1048576 bytes, and `--ws-size` sets a different
one:

```sh
sw-apl --ws-size 4096
```

**The size belongs to the session, not to the workspace.** Like the
console and the clock it is not written by `)SAVE`, so a workspace
saved under a large size need not load under a small one -- which is
what happened on a real APL\360, and is the reason `)COPY` of a few
names mattered as much as `)LOAD` of the lot.

### What a thing costs

The figures are a model, not a measurement of Rust's heap: a
workspace has to be the same size on every machine and in every
build, so what is charged is what APL\360 would have charged.

| | Bytes |
|---|---|
| A value | 16, plus 4 an axis, plus its elements |
| A number | 8, whether or not it is whole |
| A character | 1, whatever it takes in UTF-8 |
| A name in the symbol table | 8, plus its characters |
| A defined function | 16, plus the names its header makes local, plus 4 and its text for each body line |

A function's body is held as text and parsed when it runs, so text
is what it costs.

### WS FULL

Anything that will not fit in what is left is `WS FULL`, and nothing
is stored: an assignment, a definition, the arguments a call binds,
and a `)LOAD` or `)COPY` of a workspace too big for the size you are
working under.

```
      BIG←⍳200000
WS FULL
      BIG←⍳200000
      ^
```

A refused `)LOAD` leaves the workspace you were in exactly as it
was, down to its name and its settings. The file is APL and loading
it is running it, so a load that stopped part way would leave you
with half a workspace and no way to tell which half; instead the old
one is put aside first and given back the moment a line will not
fit.

What a workspace *holds* is bounded. What an expression builds on
the way to a result is not: `⍴⍳200000` answers 200000 in a
workspace far too small to hold that vector, because nothing keeps
it. The size is the workspace's, not the machine's.

`samples/62-workspace-space.apl` is the worked example.

## Loading, and copying

They are different, and the difference matters:

- **`)LOAD NAME`** replaces your workspace with the saved one. You get
  its names *and* its settings, including its index origin, so its
  functions work as its author intended.
- **`)COPY NAME [objects]`** brings names out of a saved workspace and
  into the one you are in, which keeps *your* settings. A copied
  function then runs under an origin it may never have been written
  for, and the result can be wrong without being an error.
- **`)PCOPY`** is `)COPY` that will not overwrite a name you already
  have, and says which names it kept.

Copying a **group** by name brings the group and the objects its
members name -- that is what a group is for. `session.md` has the
group rules.

`index-origin-considerations.md` is the long version of why that
matters, and how APL\360 programmers lived with it.

## Greeting: the DESCRIBE convention

**APL\360 has no run-on-load.** There is no latent expression: the
`⎕LX` that later APLs execute when a workspace loads arrived with
APL.SV in 1973, along with the rest of the quad-named system
variables, and sw-apl has none of them.

Nor is there an automatic banner. All `)LOAD` prints is the line
saying when the workspace was saved. This is APL\360 loading its own
teaching workspace:

```
      )LOAD 1 APLCOURSE
SAVED  16.13.05 08/08/68
      DESCRIBE
THE MAIN FUNCTIONS IN THIS LIBRARY WORKSPACE ARE:
...
```

The greeting is the user typing `DESCRIBE`, and that is the whole
convention. sw-apl does the same: `)LOAD` prints the SAVED line and
nothing else. It would be friendlier to add a line pointing at
DESCRIBE, and sw-apl does not, because that is output APL\360 never
produced.

**A function or a variable, as you please.** What matters is the
name, because typing a name either runs a niladic function or
displays a variable, and the reader cannot tell which. In APLCOURSE
it is a function -- it appears in `)FNS` -- but a character matrix
called DESCRIBE reads the same to whoever loads the workspace. A
function is the more usual choice, and the more flexible, since it
can decide what to say.

Nothing registers it either way. You do not declare it before saving,
and there is no command that marks it. You simply define it, and it
is in the workspace like any other name:

```apl
      ∇DESCRIBE
[1]   'CLASS -- worked examples from the course.'
[2]   'THE FUNCTIONS ARE HYP, SUM AND MEAN.'
[3]   'TYPE HOWSUM FOR MORE ON SUM.'
[4]   ∇
      )SAVE CLASS
```

Longer help follows the same idea: one niladic function per
documented function, named `HOWNAME`. There is no mechanism here
either -- the convention *is* the mechanism, which is why it is worth
following.

## Locking, and why it cannot be undone

Opening or closing a definition with `⍫` instead of `∇` locks the
function; either one is enough, and `⍫NAME` reopening an unlocked
function locks it when it closes. A
locked function still runs, but it cannot be displayed, cannot be
reopened, and **cannot be unlocked**. There is no command for it. The
only way back is to erase it and define it again from source you kept
elsewhere:

```apl
      )ERASE SECRET
      ∇R←SECRET
[1]   R←42
[2]   ∇
```

So a lock is a one-way door, and the source you typed is the only
copy of what is behind it. In APL\360 that was the whole point:
workspaces were binary, so a locked function was genuinely opaque to
whoever you sent it to.

**In sw-apl a locked function is written into the file in full**,
closed with `⍫` so that loading it reproduces the lock. There is
nowhere else for it to go. So a workspace holding at least one
locked function is not written as plain text: `)SAVE` obscures the
whole file.

```
⍝!OBSCURED sw-apl workspace. Rot-13, not encryption: docs/workspaces.md.
'OBSCURED WORKSPACE. )LOAD IT -- IT CANNOT BE RUN AS A PROGRAM.'
)OFF
⍝!MODES (A)(B)
⍝ fj-ncy jbexfcnpr. Er-rkrphgnoyr NCY: ybnqvat vg ehaf vg.
...
```

`)LOAD` and `)COPY` read both forms and tell them apart by the first
line, so nothing changes in how you use one. A workspace with
nothing locked stays plain text, because readable, diffable and
re-executable is the format's whole virtue; one locked function is
enough to obscure the lot.

**This is obscuring, not encryption, and you should not treat it as
anything else.** Rot-13 is a letter shift. Anyone who means to read
an obscured workspace can, in one line of any language, and this
document is not going to pretend otherwise. What it stops is reading
a locked function by accident or by curiosity -- which is exactly
what APL\360's binary workspaces stopped, and all they stopped. If
you need a function nobody can read, a workspace is the wrong place
for it.

### An obscured workspace is not a program

A plain workspace file is also a sample: `sw-apl -f
work/NAME.apl.ws` rebuilds the workspace, because loading it is
running it. An obscured one cannot be, and rather than leave that
to be found out it says so:

```
      sw-apl -f work/VAULT.apl.ws
OBSCURED WORKSPACE. )LOAD IT -- IT CANNOT BE RUN AS A PROGRAM.
```

Those two lines and the `)OFF` after them are the only plain APL in
the file; the `⍝!MODES` line after them stays in the clear too, so the
workspace is listed in the right modes without being revealed. `)LOAD` is the way in.

## Writing a workspace for someone else

- Give it a `DESCRIBE`, and say in it which index origin the
  workspace needs.
- If its functions are meant to be `)COPY`ed rather than `)LOAD`ed,
  write them to work in either origin.
- Keep the source of anything you lock.
- Workspaces you did not write belong in `work/`, which git ignores,
  so that material you do not own is not committed by accident. See
  `aplcourse-how-to.md`.
