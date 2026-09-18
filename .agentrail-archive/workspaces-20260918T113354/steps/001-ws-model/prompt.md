Phase 4 step 1 (docs/plan.md). The workspace as a value.

Today Workspace holds the symbol table (variables and defined functions), the activation stack that is both dynamic scoping and the state indicator, the environment (index origin and random link), the print settings, pending output, a console and a clock. The last two are the terminal's, and the sign-on time is the session's: none of them can be written to a file and read back. Separate what belongs to the workspace from what belongs to the session it is running in, so a later step can save one and load it without carrying a terminal along.

Do not move things merely to tidy them. The test is whether a value survives )SAVE and )LOAD: the symbol table, the index origin, the print precision and width, the random link (so a loaded workspace carries on its sequence and a transcript that uses roll still reproduces), and the workspace identifier. The console, the clock, the sign-on time and the pending output do not.

Add the workspace identifier, which APL\360 calls the workspace id: CLEAR WS until it is named. )WSID with no argument shows it, )WSID NAME sets it and replies WAS the old one, following the WAS convention the settings commands already use. )CLEAR gives a fresh workspace and prints CLEAR WS.

Decide and document what )CLEAR does to a suspended function. APL\360 clears the state indicator with everything else; say so in docs/session.md rather than leaving it to be discovered, and test it.

Watch the module and crate gates: apl-workspace is at its cap already, so expect to split rather than add, and let the split follow the save/session boundary rather than cutting across it.

TDD. Rust tests for the library, reg-rs for anything run through the binary, per docs/testing.md. Add a sample showing )CLEAR and )WSID and seed it. Update the docs/parity.md rows for )CLEAR and )WSID in the same commit. Commit, push, report.
