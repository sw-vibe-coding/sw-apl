Owner direction 2026-09-20: the board, refactored, with four modes.

The owner has been using the board and found it wrong in ways that
come from how it was built, and proposed a shape for the fix.

**Four modes: APL, ABC, Commands, Idioms.** A touch reader today
taps `)LIB 1` one key at a time, and has no way to find out what the
system commands are without reading Help.

- *APL* and *ABC* are the two faces the board has now: the glyph a
  key sends and the letter or digit it is painted with.
- *Commands* are the system commands, one tap each. Take the list
  from what sw-apl actually implements -- `docs/commands-reference.md`
  -- and not from memory. A command that takes an argument, like
  `)LOAD`, inserts itself and a space and leaves the carriage there;
  it does not submit, because a command sent without its argument is
  an INCORRECT COMMAND the reader did not mean.
- *Idioms* are the short expressions an APL reader reaches for, one
  tap each. They must be APL\360 -- nothing from a later APL, no
  each, no nested arrays -- and every one must be pinned by a test
  that runs it, because an idiom on the board that is a SYNTAX ERROR
  teaches the wrong thing and nothing would notice.

Four modes on one cycling button is a bad control: a reader cannot
see where they are going. Make it a segmented control, one segment
per mode, the current one marked.

**The keys the owner found wrong, found by using it:**

- *Return is tiny.* `.board .key.wide` sets `font-size: 12px`, so
  the return glyph renders smaller than an ordinary key. Return is
  the most important key on the board and should look it.
- *The overstrike key reads as a glyph.* It is labelled `○*` -- the
  example of an overstrike, not what the key does -- so it looks like
  a key that inserts circle-star, and the owner could not tell what
  it was for. It needs a label that says what it does, and a name a
  screen reader can read, and Help should say how an overstrike is
  made on the board.
- *ATTN* lands on the board in the attention step. Place it where a
  reader who has started a runaway loop will find it without
  looking, which on a phone is not the corner.

The owner said earlier that history on the board belongs with this
refactoring; it is not asked for here, but leave it a place.

`just check-pages` has cases that tap keys by their data attributes
and by the classes of the control row. They will move; keep what
they prove, and add a case per mode.

---

**Owner question, 2026-09-20: why is the board not the 2741 SVG,
shown twice, normal and shifted?** It should be, and the reason it
was not was wrong.

The glyph-keyboard step dismissed the redistributed picture as "an
Inkscape drawing whose keys are not addressable". That was said
without looking. Looked at:

- It has 46 keys, each drawn as a pair of rectangles -- an outer one
  25.9 by 26.4 and a face 19.4 by 20.0 -- at known coordinates. The
  keys are addressable by geometry: sort the rectangles into rows by
  y and columns by x.
- Its glyphs are outlined paths, not text: no `<text>` element in the
  file, 112 paths. So the labels cannot be read out of it -- but they
  do not need to be. The keymap already says what each key sends;
  what the picture gives is where each key is.
- The board this repository draws instead lays glyphs on a US
  keyboard's rows and calls it a 2741. The picture is the 2741. It is
  the more faithful board and it is the one a reader should tap.

Licence: CC BY-SA. Showing the picture unmodified and laying
transparent hit regions over it is display, not adaptation, so it
does not pull the page under share-alike. Do not edit the SVG's
contents; if a highlight is wanted on a pressed key, draw it in an
overlay. The attribution link beside the board stays.

"Twice" is the owner's: an unshifted view and a shifted one, which
is also what makes the APL and ABC modes concrete -- they are the two
views of one picture rather than two layouts. Commands and Idioms are
not on a 2741 and are not views of the picture; they are the board's
own.

**Owner: "The overstrike key is Backspace."** True of a 2741, which
had no delete key at all -- a mistake was retyped or overstruck.
sw-apl moved overstrike to `Ctrl-]` at the CLI and in `aplterm` on
purpose, and `compose.rs` says why: a line editor needs Backspace to
delete. That reasoning holds wherever there is a line editor, and it
is not being reopened.

The board is different. It has no line editor's conventions to keep,
so it can be faithful: Backspace strikes the next glyph over the last
one, as on the machine, and a separate key erases. Ask the owner
before building it that way -- it changes what Backspace means on one
surface only, and a reader who uses both the board and a physical
keyboard will meet both meanings.

**How to make the picture tappable** (owner asked what the modern
image map is). Not `<map>` and `<area>`: their coordinates are the
image's pixels, so they do not follow the picture as the board is
resized, and every area would need rescaling on each resize. The
board is resizable, so that is a fight.

Use an SVG overlay in the picture's own `viewBox`. The original goes
in as an `<img>`, untouched; over it sits an `<svg>` with the same
`viewBox` holding one transparent `<rect>` per key, copied from the
46 rectangle pairs' coordinates. Because both share the `viewBox`,
the hit regions track the keys at every size with no code. Each rect
is `role="button"`, focusable, and named for a screen reader out of
`data/glyphs.toml`, which an `<area>` never did well. A pressed-key
highlight is drawn in the overlay, never on the picture.

This is also what keeps the licence simple: the CC BY-SA file is
served byte for byte as redistributed, and everything interactive is
in a separate file that is ours.

**Correction, 2026-09-20: the SVG may be edited.** Above says "Do
not edit the SVG's contents". That was over-cautious and is
withdrawn. The owner asked whether the licence allows derivative use,
and it does: CC BY-SA 3.0 permits adaptation. Its condition is
share-alike, not "no changes" -- an adapted file must itself be
offered under CC BY-SA 3.0 or a compatible licence, must say what was
changed, and must keep the credit. It does not pull sw-apl's MIT code
under share-alike, because the picture stays a separate file.

`images/redistributed/apl-keyboard/ATTRIBUTION.md` already has the
rules for this, written for exactly this case ("keys given
identifiers so they can be clicked"). Follow them: state the change
there, keep the adapted file in that directory beside the original
and not in `pages/`, and keep the credit wherever it is shown. The
original stays byte for byte as published beside the adaptation.

So either route is open, and it is a real choice rather than a
licence question: adapt the SVG (keys given identifiers, the ATTN key
drawn into it), or keep it untouched and put everything in an overlay.
An overlay keeps the licensing trivial; an adaptation keeps the
drawing whole. Pick one and say why.

**The ATTN key** (owner direction, 2026-09-20). The picture has an
unlabelled key at the top left, left of the 1. Make it ATTN and label
it so. Widen it -- outdented to the left, beyond where the row starts
-- so the label fits at a normal size rather than squeezed into one
key's width. That is where the 2741 had room, it is the corner a
reader's eye goes to for an escape, and it is where the attention
step's board key should end up.

Widening a key is an adaptation if it is done in the SVG; drawn in an
overlay, the overlay's viewBox has to extend left past the picture's
to hold it. Either is fine. What is not fine is a label that the
reader cannot read.
