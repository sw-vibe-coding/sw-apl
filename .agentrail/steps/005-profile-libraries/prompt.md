Phase 9 step 4 (docs/plan.md). A profile on the workspace, libraries
per mode, and the mode in what is saved -- with '68 unchanged.

- A mode on the workspace, set by the host like the quota and the
  store: `--mode 68|72` at the CLI and at sw-apl-server, defaulting
  to 68; the tab in the browser.
- Libraries per mode. The store (Phase 8, step 009) is the seam:
  library 0 and library 1 are kept per mode -- on disc under a mode
  directory, in a browser under a mode key -- so a workspace saved in
  '72 is in '72's library 0 and `)LIB` in '68 never lists it. Decide
  where the existing ws/lib1 and work/ go so that a '68 reader sees
  exactly what they saw before, and say so.
- The mode in a saved workspace, as a directive beside the others.
  Loading a workspace saved in the other mode does what docs/aplsv.md
  settled: refused, or allowed one way. A workspace with no mode
  directive -- every one saved so far -- is '68.

Nothing '72 does differently yet: '72 is '68 with its own libraries.
reg-rs stays 81 of 81 with no rebase, which is the proof '68 did not
move. Tests for the store per mode, for the directive, and for
loading across modes.
