# workspaces

Phase 4 of docs/plan.md: the workspace as a thing you can name,
clear, save, load and look inside. Phase 3 left a Workspace that
holds the symbol table, the settings, the state indicator, a
console and a clock; the last two are the terminal's and must not
be saved. Separating them is the first step, and the file format
follows from it.

The workspace file is plain UTF-8 that a person can read and APL
can re-execute: del form for functions, assignment for variables,
settings commands for the environment. That makes a saved
workspace a sample, and a sample a saved workspace.

Pure APL\360 throughout: numbered public libraries, the DESCRIBE
convention, no quad names. Every step: format first, then tests,
clippy, and gates (see /mw-cp); TDD; reg-rs for anything run
through the binary, Rust tests for the libraries; update
docs/parity.md rows in the same commit; commit, push, report.

## Steps

1. ws-model -- the workspace as a value: what is saved with it
   (symbol table, index origin, print precision and width, the
   random link, the workspace identifier) and what belongs to the
   session it runs in (the console, the clock, the sign-on time).
   )CLEAR gives a fresh one and prints CLEAR WS; )WSID shows or
   sets the identifier. Say what clearing does to a suspended
   function.
2. ws-file-format -- a plain-text UTF-8 workspace file that is
   both human readable and re-executable: del form for functions,
   assignment for variables, settings commands for the
   environment. Round-trip it in a test, and pin the file itself
   as a fixture so its shape is a baseline.
3. system-commands-ws -- )SAVE )LOAD )DROP )LIB )COPY )PCOPY
   )CONTINUE, with the timestamps and replies docs/session.md
   describes. )CONTINUE saves and signs off.
4. system-commands-inquiry -- )FNS )VARS )GRPS )GRP )GROUP )ERASE
   )SYMBOLS, in the order and column layout APL\360 used.
5. library-workspaces -- numbered public libraries mapped to
   directories, the library form )LOAD 1 CLASS, and starter
   workspaces in ws/lib1/ each carrying a DESCRIBE function that
   the session points at after a load.
