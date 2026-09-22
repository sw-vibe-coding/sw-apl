Owner direction 2026-09-20: the updated logo, with transparent
corners.

`~/Downloads/sw-apl-logo.jpg` is a new version of the badge. It is
a JPEG with white corners; it replaces `images/sw-apl-logo.png`,
which the README shows and which the demo's install icons are made
from, so both follow from the one file.

The corners have to come out. Flood-filling from them leaves a
white fringe -- the source is a JPEG and its edge pixels are part
white -- so mask to the circle instead, which is what the badge is
and which gives a clean anti-aliased edge. Check it against a dark
background, a light one and a saturated one; a fringe is invisible
on white and obvious everywhere else.

Regenerate the demo's icons from it afterwards: `icon-192`,
`icon-512` and the maskable one all come from this file.

The logo is ours, so nothing changes for `check-provenance`.
