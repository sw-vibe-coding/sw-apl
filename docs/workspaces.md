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
| The index origin, print precision, print width | The clock the I-beams read |
| The random link | The sign-on time `⌶24` reports |
| The name `)WSID` reports | Whatever the current line has displayed |

The state indicator -- suspended and pendent functions -- belongs to
the workspace too, and `)CLEAR` clears it with everything else. But
it is not written to a file: see **What a saved file cannot hold**.

An unnamed workspace is called `CLEAR WS`, which is what `)WSID`
reports until you give it a name.

## Naming one

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
NOT SAVED: THIS WS IS CLEAR WS
```

## Libraries

Libraries are numbered, as in APL\360. Library 0 is yours; the
numbered ones are public.

| Library | Directory | |
|---|---|---|
| 0 | `work/` | Yours. `)SAVE` writes here and `)LOAD NAME` reads here. Not tracked by git; the first `)SAVE` creates it |
| 1 | `ws/lib1/` | The workspaces sw-apl ships, each with a DESCRIBE function |

`)LIB` lists library 0, `)LIB 1` lists library 1 -- one at a time, as
APL\360 did. A workspace is the file `NAME.apl.ws`.

## The file

A saved workspace is APL you could have typed:

```apl
⍝ sw-apl workspace. Re-executable APL: loading it runs it.
⍝!SAVED 20.00.00 09/17/26
⍝!LINK 282475249
)WSID CLASS
)ORIGIN 0
)DIGITS 3
)WIDTH 80
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

Lines beginning `⍝!` are comments to APL and instructions to `)LOAD`.
There are two, for the things APL has no way of saying about itself:
`⍝!SAVED` is when it was written, which `)LOAD` reports, and `⍝!LINK`
is where the random link stands, so a loaded workspace carries on its
sequence rather than starting over.

## What a saved file cannot hold

A **suspended function**. The state indicator is a stack of half-run
calls, and re-executing a file cannot put execution back in the
middle of one. `)SAVE` leaves it out and `)LOAD` gives you a
workspace with an empty state indicator.

APL\360 did not have this problem, because its workspaces were binary
images and `)SAVE` really did preserve a suspension. It is the price
of a file you can read. `parity.md` carries the row.

## How much room is left

`⌶22` reports the space available, in bytes, and it is the only way
APL\360 offered -- the `⎕WA` of later systems is one more
quad-name sw-apl does not have.

```apl
      ⌶22
1048576
```

**In sw-apl the number is nominal.** A workspace is a Rust process's
memory, not a fixed partition carved out of a 360, so there is no
quota to report against and `⌶22` answers the same figure every time.
`WS FULL` is in `parity.md` as not implemented and is never raised:
sw-apl will not refuse a `)LOAD` for size, and a workspace grows
until the machine itself objects.

On a real APL\360 this was a live constraint. A workspace was a
fixed allocation, `WS FULL` was an error you hit routinely, and a
`)LOAD` of a large workspace could fail against a small quota -- which
is why `)COPY` of a few names mattered as much as it did, and why
`⌶22` was worth checking before starting something big.

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
  have.

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

Closing a definition with `⍫` instead of `∇` locks the function. A
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

**In sw-apl it is weaker, and you should know that.** A saved
workspace is text, and a locked function is written into it in full,
closed with `⍫` so that loading it reproduces the lock. Anyone with
the file can read the body and retype it unlocked. Locking here
guards the editor, not the file. If you need a function nobody can
read, a text workspace is the wrong place for it.

## Writing a workspace for someone else

- Give it a `DESCRIBE`, and say in it which index origin the
  workspace needs.
- If its functions are meant to be `)COPY`ed rather than `)LOAD`ed,
  write them to work in either origin.
- Keep the source of anything you lock.
- Workspaces you did not write belong in `work/`, which git ignores,
  so that material you do not own is not committed by accident. See
  `aplcourse-how-to.md`.
