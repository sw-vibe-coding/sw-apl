Owner direction 2026-09-20, third (docs/plan.md). A demo that fits
a phone.

The owner opened the demo on a simulated phone in Chrome. The
footer prose takes more of the screen than the transcript does: the
paper is squeezed into the top third and the line being typed is
pushed off the bottom. The text is not wrong, it is in the wrong
place.

Make the page paper and prompt, and as little else as will do.

- The explanation moves behind something asked for -- a help
  dialog, or links that open one. A first-time reader needs it and
  must be able to find it without knowing to look; a reader who has
  come back must not pay for it with two thirds of the screen.
- What cannot be got back stays reachable wherever it is put: that
  nothing typed leaves the browser is a claim worth keeping in
  sight, and the licence and the keyboard picture's attribution are
  a redistribution obligation, not a nicety.
- The line being typed is the one thing that must never be off
  screen. Check it with the on-screen keyboard raised, which is
  what a phone actually does to the viewport -- dvh units and
  visualViewport, not vh.

`sw-fun/suduko` is the owner's pointer for styling: look at its
`pages/index.html` head and its stylesheet, and take the parts that
suit a printing terminal rather than the whole look. sw-apl's page
is paper and a carriage and should stay that.

Keep it to layout. The manifest, the icons and offline caching are
the next two steps and this one should leave them a place to stand;
do not start them here.

Check it the way the bug was found: Chrome's device toolbar at
phone width, portrait and landscape, and the transcript still
readable with a long RACE run on the paper. `just check-pages`
must stay green -- add a case for the line being on screen at phone
width.
