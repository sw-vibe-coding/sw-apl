# IBM 2741 APL keyboard

`APL-keybd2.svg` is the layout of the IBM 2741's APL keyboard: which
glyph each key carries, shifted above and unshifted below.

It is not ours. It is redistributed here under its own licence, kept
in `images/redistributed/` so that what sw-apl wrote and what sw-apl
borrowed are never in the same directory.

## Credit

- **Title:** APL-keybd2.svg
- **Author:** Wikimedia Commons user
  [Rursus](https://commons.wikimedia.org/wiki/User:Rursus)
- **Source:** <https://commons.wikimedia.org/wiki/File:APL-keybd2.svg>
- **First published:** 25 July 2007, derived by its author from
  `APL-keybd.svg`
- **Licence:** [Creative Commons Attribution-ShareAlike 3.0
  Unported](https://creativecommons.org/licenses/by-sa/3.0/), whose
  full text is in `LICENSE` beside this file. The author also offers
  it under the GNU Free Documentation License 1.2 or later; sw-apl
  takes it under CC BY-SA 3.0.
- **Changes:** `APL-keybd2.svg` is byte-for-byte as published.
  `APL-keybd2-board.svg` beside it is an adaptation; see below.

## The adaptation: `APL-keybd2-board.svg`

`APL-keybd2-board.svg` is an adaptation of `APL-keybd2.svg`, made so
that the on-screen keyboard in sw-apl's browser demo can emphasise
one face of every key and dim the other. It is written by
`scripts/board-keys.py --adapt` from the original, never edited by
hand, so it can be made again from the original at any time.

What was changed:

1. A `class` attribute was added to each glyph: `face-shifted` on the
   glyph a key carries above, which is its shifted face, and
   `face-normal` on the one below. There are 44 of each.
2. A `class` attribute was added to the three parts of the Caps Lock
   key -- its edge, its face and its label -- so its state can be
   shown.
3. Writing the file back re-serialised it: attribute order and
   whitespace differ from the original. Nothing drawn was moved,
   recoloured, resized or removed. Rendered at 150 dpi, the
   adaptation and the original differ in no pixel.

The adaptation is offered under **CC BY-SA 3.0**, the licence of the
work it adapts, and not under sw-apl's MIT licence. The credit above
applies to it in full; the adaptation's own additions are too slight
to claim.

## Why it is here

The English Wikipedia article on the IBM 2741 carries no table of
which key sends which glyph -- the layout exists only in this
picture. So the picture is sw-apl's source for it, and
`components/term/crates/apl-keyboard/keymap.json` is read out of it.
Keeping the two together is what lets a reader check one against the
other.

## What share-alike asks of us

CC BY-SA is not a licence to copy and forget. If this file is ever
changed -- keys given identifiers so they can be clicked, colours
altered, pieces cut out -- then:

1. the change must be stated here, saying what was changed;
2. the result must be distributed under CC BY-SA 3.0 or a compatible
   licence, not under sw-apl's MIT licence;
3. the changed file belongs in **this directory**, beside the
   original and this licence. A modified SVG does not become sw-apl's
   by being useful to sw-apl, and moving it into `pages/` or
   `components/` would put a share-alike work under an MIT tree; and
4. the credit above must survive wherever the result is shown,
   including in a browser page that displays it.

A page that shows the keyboard is a redistribution like any other.
Whoever can see it must be able to reach the author's name and the
licence.
