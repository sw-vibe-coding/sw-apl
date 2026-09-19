Phase 8 step 3 (docs/plan.md). The keyboard, and arriving.

Typing APL is the barrier. A reader who cannot produce `⍴` cannot
try anything, so this step is about what a first-time visitor can
actually do.

- The IBM 2741 APL layout. `docs/input-methods.md` describes it
  already -- rho on R, iota on I -- so the terminal should follow
  the same layout rather than a new one, and the doc should say
  the two agree.
- A clickable glyph board for readers without the keyboard, laid
  out the way `docs/glyphs.txt` groups the glyphs, inserting at
  the cursor. Buttons need names that say what they insert, for a
  screen reader and for a hover.
- The expansions in `docs/espanso/` and `docs/emacs/`. Use those
  mappings, not a third set, and say in the docs that all three
  agree. If they cannot be shared without duplicating the table,
  that is the same argument as the overstrikes: put it in
  `data/glyphs.toml` and generate.
- A first screen worth arriving at. An empty prompt asks the
  reader to already know APL. `ws/lib1/` has LIFE and RACE for
  exactly this, and `)LOAD 1 LIFE` then `GLIDER` then `RUN 4` is a
  better welcome than a cursor.

Then close the loop: the README links to docs and should link to
this, with a line saying what it is, that it runs locally, and that
nothing leaves the machine. A vhs recording for readers who will
not clone -- `docs/tapes/` already holds one and `just tape`
renders it.

Check it in a browser before claiming it works, and if the browser
tools are available, drive it: type an expression, form an
overstruck glyph, run a function, and compare the transcript to
what the CLI prints for the same input.
