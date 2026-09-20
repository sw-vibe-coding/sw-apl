Phase 8 step 3 (docs/plan.md), rewritten by owner direction
2026-09-20, fourth: four ways to enter a glyph, one of them a
touch screen.

The owner has not yet been able to enter an APL glyph through the
UI and wants that fixed before anything else. Typing APL is the
barrier; a reader who cannot produce `⍴` cannot try anything.

Four ways, and the fourth is the one nothing covers today:

1. The 2741 layout on a physical keyboard. Shift-R is `⍴`,
   Shift-I is `⍳`. This works: `apl-keyboard`'s `keymap.json` is
   the map and `apl-board` is the browser's way into it.
2. Overstrikes. `○`, Ctrl-], `*` is `⍟`. This works.
3. A prefix on a physical keyboard, for a reader whose fingers
   know `docs/espanso/apl.yml`: backtick then `r` is `⍴`. That
   table exists twice already, in `docs/espanso/` and
   `docs/emacs/`, and must not exist a third time -- put the
   positions in `data/glyphs.toml` and generate, as the
   overstrikes are.
4. Tapping a board on the screen. This is the one for a touch
   screen, which has no keyboard to intercept at all, and for a
   reader who has not relabelled their keycaps.

For the fourth: draw the board from the keymap, not from a
picture. The redistributed SVG is an Inkscape drawing whose keys
are not addressable and whose licence travels with it; it stays
what it is -- a reference picture, shown on request, with the
author and licence reachable wherever it is shown. The board a
reader taps is ours, laid out like the 2741 so that tapping also
teaches where the key is, each key carrying both faces (the ASCII
it is painted with and the glyph it sends), with a layer toggle so
a touch screen can reach letters and digits too, and space,
backspace, return and the overstrike key.

It must not take the typing focus, must be reachable with a
keyboard and named for a screen reader, and must not cost the page
its layout when it is not shown. Remember whether it was shown.

A board takes room a phone has not got, so take only as much of
the footer as the board needs: move the prose behind a help
control and leave the rest of the layout to `page-layout`.

Say in the docs that all the ways agree, and where each is
documented. `docs/glyph-entry.md` is where a reader looks.

Check it in a browser and do not claim it works otherwise: enter
`)LIB 1` and `⍳5` by tapping alone, at phone width, and compare
the transcript to what the CLI prints for the same input.
`just check-pages` is where that check lives.
