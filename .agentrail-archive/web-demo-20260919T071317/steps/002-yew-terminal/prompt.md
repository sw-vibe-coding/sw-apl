Phase 8 step 2 (docs/plan.md). The terminal.

A Yew component that looks and behaves like the printing terminal
the CLI imitates: output starting in column one, input indented six
spaces, the bracketed `[n]` prompt in definition mode, and a line
that `⍞←` left open carrying into what is typed next -- the CLI
learned that last one in Phase 7 and the browser is the third caller
of the same API, so it should need no new decisions.

The glyphs are the point of the demo. A reader who cannot type `⍴`
cannot try anything, so:

- A clickable glyph keyboard, laid out the way `docs/glyphs.txt`
  groups them, inserting at the cursor.
- Espanso-style expansions for typing without the mouse. The
  mappings already exist in `docs/espanso/` and `docs/emacs/`; use
  the same ones rather than inventing a third set, and say in the
  docs that all three agree.
- The APL\360 character set only. A glyph from a later APL must
  behave exactly as it does in the CLI: CHARACTER ERROR naming the
  code point.

Show something on arrival. An empty prompt asks the reader to know
APL already; a first screen that has run something invites them to
change it. `ws/lib1/` has LIFE and RACE for exactly this.

Keep the transcript honest. It is the same `Session`, so the
transcript should be character for character what `sw-apl` prints
for the same input. A sample run in the browser and the same sample
run through the CLI should agree; that is worth a test rather than
an intention.

Mind the size of what is shipped. The wasm blob is the page's cost
and a reader on a phone pays it; measure it, put the number in the
commit, and say what it is mostly made of.

Accessibility is not optional: the terminal is a text surface, so it
should be selectable, copyable and readable by a screen reader, and
the glyph keyboard's buttons need names that say what they insert.
