# Getting APLCOURSE into your work directory

`APLCOURSE` is the self-teaching workspace IBM shipped in APL\360's
public library 1. Loading it and typing `TEACH` gives you the drill
that APL was taught with in 1968. It is a good informal test of this
interpreter, because it is real APL\360 written by the people who
designed the language, and it exercises quad input, defined
functions, branching and the random link all at once.

This is a guide to getting it for yourself. sw-apl does not ship it
and never will: it is IBM material from 1968 whose copyright status
is unclear. Keep what you obtain in `work/`, which is not tracked,
and do not redistribute it.

## What the workspace holds

Its own `DESCRIBE` names two functions to run and the rest as
subfunctions:

- `TEACH` -- an exercise in APL functions on scalars and vectors,
  which prints the choices and options available.
- `EASYDRILL` -- the same, with simpler problems, leaving out those
  on vectors of length zero or one.
- Subfunctions: `B1X CHECK DIM DRILL DYAD1 DYAD2 EASY FORM FUNDRILL
  GET INPUT QUES RANDOM REDSCAPATCH REPP`.

## Three routes, and which one to take

### Run it live, and copy the source out

The most practical route. Try MTS runs APL\MTS, which is close to
APL\360 and probably derived from its source, in a browser. Its own
notes walk through `)LIB 1` and `)LOAD 1 APLCOURSE`.

Once loaded, `)FNS` lists the function names and `∇NAME[⎕]∇` displays
one, in the same del form sw-apl uses. Capture the terminal text for
each function and you have the workspace as source.

Take `)VARS` too, and display each variable: the drill questions are
data, not code, and a workspace without them will not run.

### The APL\360 workspace collection

A Hercules/MVT restoration effort assembled every library 1 workspace
named in the APL\360 User's Manual as an emulator tape. It is a tape
of binary workspace images: loading them needs a working APL\360
under emulation, and reading them directly means decoding IBM's
workspace layout and its APL character encoding. Only worth it if you
already run that stack.

### The transliterated exports

The tape was made from exports of the APL\MTS libraries in a
transliterated text format -- which is the form we want. Whether it
is published anywhere is unclear; the people who made it said
publication depended on copyright.

## Converting a listing to a sw-apl workspace

A sw-apl workspace file is plain UTF-8 that APL can re-execute: del
definitions for functions, assignment for variables, settings
commands for the environment. So a listing needs transliterating and
nothing more.

Two things to fix as you go:

1. **Characters.** A listing from a 2741 or an emulator uses the
   APL\360 character set, often transliterated to ASCII digraphs.
   `docs/glyphs.txt` is the table: every glyph with its Unicode code
   point. Overstruck characters are the ones to watch -- quad-quote
   `⍞`, del-tilde `⍫`, and the I-beam `⌶`.
2. **What sw-apl does not have.** Check `docs/parity.md` before
   blaming the conversion. A function that fails may be using
   something not implemented yet rather than something mistyped.

Put the result in `work/` as `NAME.apl.ws` and load it:

```
      )LOAD APLCOURSE
      DESCRIBE
      TEACH
```

## Where workspaces live

`)SAVE` writes into `work/` and `)LOAD` reads from it, creating the
directory on first use. Nothing there is tracked, so a workspace you
save is yours alone -- which is also why converted historical
material belongs there and not in `ws/`.

## Sources

- Try MTS, APL language features: <https://try-mts.com/apl-language-features/>
- Resurrecting APL\360 library workspaces:
  <https://hercules-390.yahoogroups.narkive.com/SakfTQxp/running-apl-360-on-os-360-mvt-21-8f-resurrecting-library-workspaces>
- APL History Collection, Software Preservation Group:
  <https://softwarepreservation.computerhistory.org/apl/>
- APL\360 on the APL Wiki: <https://aplwiki.com/wiki/APL%5C360>
