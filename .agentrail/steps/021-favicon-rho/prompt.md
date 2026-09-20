Owner correction 2026-09-20, and a deprecation warning.

The mark is the lamp, rho and floor. Turned, the horseshoe reads
as a D; rho has a bowl and a descender and turns into a real
lower-case p, so the mark reads apl properly.

The owner also reports a console warning: the apple-mobile-web-app
-capable meta is deprecated and Chrome asks for
mobile-web-app-capable instead. Add the standard name and keep the
Apple one beside it -- older iOS reads only that, and the warning
is advice to add rather than to remove.

And one found while checking the published page: favicon.ico was
270 KB, a single 256-pixel frame, which is not what an ICO is for
and is most of what a tab bar ever downloads. Write it at the sizes
a browser asks for.
