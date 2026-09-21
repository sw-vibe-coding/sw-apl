Owner direction 2026-09-20, third (docs/plan.md), narrowed after
most of it shipped: what the viewport does to the page.

This step was written from a phone screenshot where footer prose
took two thirds of the screen. Most of what it asked for has since
been done by other steps: the prose is behind Help, the bar is two
buttons, the disclaimer is gone, the colophon is below the fold in
the house style, and the session owns the window in `dvh`. One
bullet is superseded outright -- the owner had the "nothing leaves
your browser" line removed from sight.

What is left is the part its own prompt said mattered most: the
line being typed must never be off screen.

- Use `visualViewport`, not `dvh` alone. `100dvh` is the viewport
  with the browser's own UI retracted; it does not track the URL
  bar sliding in and out, and it does not track pinch-zoom. The
  session should be exactly as tall as the visual viewport is now,
  and follow it when it changes. Keep `dvh` as the fallback for
  anything without `visualViewport`.
- Note while checking: this page has no input, textarea or
  contenteditable anywhere -- keystrokes are read off the window --
  so a phone cannot raise its own keyboard over it at all. That is
  the reason the on-screen board exists. Say so in the commit,
  because the original prompt asked for the keyboard-raised case
  and it is not a case this page can be in.
- Two checks the original asked for and never got: landscape at
  phone size, and a long transcript. Run RACE, which fills the
  paper, and confirm the paper scrolls while the line and the bar
  stay put.

Keep it to layout. No new controls, nothing moved that the owner
has already placed.
