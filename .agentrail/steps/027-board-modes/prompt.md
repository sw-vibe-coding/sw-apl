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
