# web-demo

Phase 8 of docs/plan.md: sw-apl in a browser, as a live demo.

The interpreter is already host-independent in two of the three
places it needs to be. `Console` is a trait the host implements, so
where a statement reads a line is the host's business; `Clock` is a
plain function pointer the host installs, and a workspace that has
not been given one does not move. The CLI implements both, and the
web build implements them differently.

The third place is not abstracted: `apl-library` and `apl-commands`
reach `std::fs` directly. In a browser there is no filesystem, so
either the workspace commands stop working -- which would take
`)LOAD 1 LIFE` and the shipped workspaces out of the demo, and they
are the best thing to show -- or where a workspace lives becomes a
seam like the other two. `apl-library` exists to answer "which file
does this command mean", so it is where the seam belongs.

Two things the browser cannot do that the terminal can, both to be
decided rather than discovered:

- It cannot block. `Console::read` is synchronous, and a browser
  has no way to stop and wait for a keystroke in the middle of a
  statement. Batch mode already faces this and answers INTERRUPT at
  end of input; whether that is the right answer for a demo is a
  question for the step, not a default to fall into.
- There is no `)OFF`. A session that signs off has nowhere to go.

Nothing about APL changes. The same `Session` answers the same
lines; what differs is who reads, who keeps the workspaces, and what
the clock says. If a step finds itself changing the interpreter to
suit the browser, that is the signal to stop and ask.

Every step: format first, then tests, clippy, and gates (see
/mw-cp); TDD; reg-rs for anything run through the CLI binary, which
keeps working throughout; commit, push, report.

## Steps

1. wasm-facade -- the session compiled and driven from wasm32,
   with the library store behind a seam.
2. yew-terminal -- a printer-style terminal with a glyph keyboard.
3. pages-deploy -- the build and the deploy.
