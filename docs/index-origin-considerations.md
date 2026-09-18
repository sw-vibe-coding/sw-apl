# Index origin: what it breaks, and what APL\360 could do about it

The index origin is either 1 or 0, it is set by `)ORIGIN`, and it
changes what several primitives mean. In APL\360 it is a property of
the workspace and there is no way to set it from inside a function.
That combination is a real hazard, and it is worth understanding
before writing a workspace anyone else will load.

## What depends on it

Per `language.md`: iota, indexing, grade, index-of, deal, roll, and
an axis in brackets. Everything else is unaffected.

The sharp edge is `⍳` on a scalar:

```
      )ORIGIN 1          )ORIGIN 0
      ⍳1                 ⍳1
1                  0
      3×⍳1               3×⍳1
3                  0
```

In both origins `⍳1` is a one-element vector; the element is 1 in
origin 1 and 0 in origin 0. `⍳0` is empty either way.

That is why the conditional branch every APL\360 program uses:

```apl
→LOOP×⍳COND
```

**works only in origin 1.** When COND is true, origin 1 gives
`LOOP×1`, the line number, and the branch is taken; origin 0 gives
`LOOP×0`, which is 0, and the function returns instead. The failure
is silent: no error, just a loop that runs once. It cost a test in
this repository an afternoon.

In origin 0 the same idea is written without the `⍳`:

```apl
→LOOP×COND
```

which works in either origin, because COND is already 0 or 1.

## Why a function cannot fix it for itself

`)ORIGIN` is a system command. System commands are typed in
immediate execution; they cannot appear in a function body. So an
APL\360 function cannot set the origin it needs, cannot restore the
one it found, and cannot even ask what it is.

This is the defect that later APLs fixed. From APLSV onwards the
origin is a system *variable*, `⎕IO`, which can be localised in a
header:

```apl
∇R←F X;⎕IO
⎕IO←0
```

Now the function sets its own origin and the old one comes back when
it returns. sw-apl has no `⎕IO`, deliberately: it is APL\360, and
`quad`-named system variables are APLSV. See `language.md`.

## What that leaves you with

The three things APL\360 programmers actually did:

1. **Say so.** A workspace that needs an origin says which, in its
   DESCRIBE function and in its documentation. This is the main
   reason the DESCRIBE convention exists.
2. **Write origin-independent code** for anything meant to be copied
   rather than loaded. Branch on the condition rather than on `⍳` of
   it. Prefer `⍴X` to an index computed from a length. Compare rather
   than subscript where you can.
3. **Set the origin first.** Before copying someone's functions,
   `)ORIGIN` to what they expect.

## Loading versus copying

This is the part that bites, and the two commands differ:

- **`)LOAD`** replaces your active workspace with a saved one, and a
  saved workspace carries its own origin. You get the author's
  origin, so the author's functions work. Loading is safe.
- **`)COPY`** brings named objects out of a saved workspace and into
  the one you are already in, which keeps *your* settings. Copied
  functions then run under an origin they were never written for.
  Copying is where silent wrong answers come from.

That asymmetry is deliberate: `)COPY` exists to bring a function into
your work, and it would be worse if it silently changed the origin
under the code you had already written.

## What sw-apl does

- The origin is part of the workspace, written into the file as
  `)ORIGIN n` along with the print settings and the random link. See
  `design.md` D7.
- `)LOAD` applies it, because loading a workspace means running the
  file, and the file sets it.
- `)COPY` must not. Since the file is re-executable APL, copying
  cannot simply run it: the loader has to take the definitions and
  leave the settings commands alone. That is a requirement on the
  step that implements `)COPY`, not something that falls out for
  free.

A caution on the last two points: the `)LOAD`/`)COPY` split above is
how APL\360 is described to behave and how sw-apl will behave, but
the exact wording is worth checking against the IBM APL\360 User's
Manual (1968) before it is treated as settled. `parity.md` is the
place any correction lands.

## Writing a workspace here

If a workspace only ever gets `)LOAD`ed, use whichever origin suits
it and let the file carry it. If its functions are meant to be
`)COPY`ed into other people's work, write them to run in either
origin, and say in DESCRIBE which one you assumed if you could not.
