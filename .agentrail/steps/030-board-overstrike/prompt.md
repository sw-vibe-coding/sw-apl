Bug found by the owner, 2026-09-20: an overstrike made on the board
does not strike.

At a physical keyboard, A Ctrl-] Shift-F makes the underscored A. On
the board, A then the picture's backspace -- which is the overstrike
key there, as on a 2741 -- then the key carrying `_` does not: the
`_` lands beside the A instead of over it.

The cause: every glyph tapped on the board reaches the keyboard as
`Board::paste`, and `Keyboard::paste` begins by cancelling any pending
strike and then inserts literally. A key typed at a physical keyboard
goes through `type_key`, which strikes when one is pending. So the two
paths disagree about the one thing an overstrike is.

It was never supported rather than regressed: the board has sent its
glyphs by paste since it existed.

Fix it in the keyboard, not the page: a single character arriving
while a strike is pending should strike, however it arrived. That is
the same rule a typed key follows, and it is also right for a paste
at a terminal right after Ctrl-] and for an expander such as Espanso
sending a glyph -- the pending strike was waiting for the next input,
and this is the next input. More than one character still cancels the
strike and goes in as typed, as now. A pair that forms no glyph
rings the bell and leaves the base, as a typed key does.

Tests: in `apl-keyboard`, the strike by paste, the bell, and a
multi-character paste still cancelling. In the browser, the owner's
own sequence on the board gives exactly what the physical keyboard
gives, compared rather than assumed.
