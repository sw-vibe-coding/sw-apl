Owner direction 2026-09-20, eighth: a colophon below the fold.

The demo says nothing about what it is or who wrote it. A published
page should: the copyright, the licence, a link to the repository,
and the build it is running -- build host, short SHA, timestamp.

Below the fold, and scrolled to. The session fills the window and
keeps it: the bar under the paper has been cut twice already for
taking room the transcript wanted, and this must not undo that. A
reader who wants to know what they are running scrolls down for it
and a reader who does not never sees it.

The build facts come from `build-info.json`, which `just publish`
already writes; it gains the host. The page reads it, so a stale
footer is impossible -- there is no second place for the build to
be written down.

Note that the bar holding Keyboard and Help is not a footer and
should stop calling itself one now that there is a real one.

`check-provenance` and the markdown gates do not cover a page, so
say in the commit that the licence named is the one the bundle
travels under, and keep the keyboard picture's own terms where they
are -- under Help, which is where the picture is.

The board places itself relative to the page's children, so moving
the session into a wrapper will move its ground; check pinning to
the top still works afterwards.

Check it at phone width: the session must still fill the window
exactly, with the footer only reachable by scrolling.
