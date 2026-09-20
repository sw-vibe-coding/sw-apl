Phase 8 step 5 (docs/plan.md, owner direction 2026-09-19 fourth).
Workspace libraries with no filesystem under them.

In the browser, )LIB 1 and )LOAD 1 NAME must work, and )SAVE must
write library 0 into the browser local storage rather than being
refused. A saved workspace is small UTF-8 text, so nothing about it
needs a file.

The seam is small and already surveyed. Five call sites do real
file I/O, and everything else is already text in and text out:

  apl-library/src/path.rs        fs::read_to_string   read a workspace
  apl-commands/src/save.rs       fs::create_dir_all   make library 0
  apl-commands/src/save.rs       fs::write            write a workspace
  apl-commands/src/save.rs       fs::remove_file      )DROP
  apl-commands/src/load.rs       fs::read_dir         )LIB

Put a trait behind those five: read a named workspace, write one,
drop one, list a library. Workspace::libraries is the one
configuration point today and the natural place for it to become a
store rather than a path. apl-wsfile already turns a workspace into
a String and back, so the format does not change and neither does
what a saved workspace looks like.

Two implementations: the filesystem, which is what the CLI and the
service use and whose behaviour must not move at all, and the
browser one -- library 1 baked into the bundle and read-only,
library 0 in local storage.

Say in the commit what a browser reader sees when local storage is
full or refused, and whether a workspace saved in one browser is
visible in another (it is not; say so where a reader will find it).

Tests: the existing session and library tests are the check that
the filesystem implementation did not move -- they must pass
untouched. Add tests for the store trait itself against an
in-memory implementation, which is also what the browser one is
tested against. reg-rs is the harder control and must stay green.
