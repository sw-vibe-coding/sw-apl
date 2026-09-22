Owner direction 2026-09-20, tenth: the footer in the house style,
and the keyboard picture attributed where the keyboard is.

Three things, from the owner looking at the published page.

The colophon does not look like the footers on the other live
demos. `sw-embed/web-sw-cor24-apl/src/lib.rs` has the house form:
one line of items separated by middots -- licence, copyright, a
project link, the Software Wrighter channels, then build host, SHA
and timestamp. Three paragraphs of prose is not that. Make it
concise and make it match.

The paragraph about the keyboard picture is verbose and, as
written, misleading. It says the picture is "under Help", which
reads as though Help is where a reader would look for it; the
reader's actual experience is a Keyboard button and a board. The
picture does render in Help -- 410 by 125, confirmed on the
published page -- but it is near the bottom of a scrolling dialog
and the owner did not find it. Move it up so it is findable, and
take the paragraph out of the footer entirely.

Attribution belongs where the keyboard is. Whenever the board is
shown, show a small link beside it to

  https://github.com/sw-vibe-coding/sw-apl/tree/main/images/redistributed/apl-keyboard

which is where the picture, its LICENSE and its ATTRIBUTION.md all
live. That is a better discharge of the obligation than a sentence
in a footer nobody scrolls to, because it is next to the thing it
is about.

Check the published page afterwards, not just a local one.
