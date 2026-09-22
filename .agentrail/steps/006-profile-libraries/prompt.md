Phase 9 step 4 (docs/plan.md). A profile on the workspace, libraries
per mode, and the mode in what is saved -- with '68 unchanged.

- A mode on the workspace, set by the host like the quota and the
  store: `--mode 68|72` at the CLI and at sw-apl-server, defaulting
  to 68; the tab in the browser.
- Libraries per mode. The store (Phase 8, step 009) is the seam:
  library 0 and library 1 are kept per mode -- on disc under a mode
  directory, in a browser under a mode key -- so a workspace saved in
  '75 is in '75's library 0 and `)LIB` in '68 never lists it. Decide
  where the existing ws/lib1 and work/ go so that a '68 reader sees
  exactly what they saw before, and say so.
- The mode in a saved workspace, as a directive beside the others.
  Loading a workspace saved in the other mode does what docs/aplsv.md
  settled: refused, or allowed one way. A workspace with no mode
  directive -- every one saved so far -- is '68.

Nothing '75 does differently yet: '75 is '68 with its own libraries.
reg-rs stays 81 of 81 with no rebase, which is the proof '68 did not
move. Tests for the store per mode, for the directive, and for
loading across modes.

**Owner direction, 2026-09-21 -- this replaces "libraries per mode"
above.** A workspace is listed and loaded in the modes it runs in:

- Each workspace carries a line near the top naming its modes: `(A)`
  for '68, `(B)` for '75, `(A)(B)` for both, later modes added as they
  come. Every workspace saved so far, and every shipped one, is given
  its line; a file with none is `(A)`.
- A workspace using what only one mode has is listed and loaded in
  that mode alone. One using only what both share is listed and loaded
  in both.
- `)SAVE` decides the line from what the workspace uses, not from the
  mode it was saved in, so a '75 save that uses nothing '75 added is
  `(A)(B)`. Two workspaces may share a name when their modes do not
  overlap.
- Keeping a workspace that runs in both once, and showing it in both,
  is an optimisation the reader does not see.
- The settings: if docs/aplsv.md says APLSV dropped `)ORIGIN`,
  `)DIGITS` and `)WIDTH`, write them in a form both modes read -- a
  `⍝!` directive, as the random link already is -- so that a workspace
  is not '68-only merely for having an index origin. The '68
  transcripts must not change.
