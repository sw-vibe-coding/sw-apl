Phase 4 step 6 (docs/plan.md, owner direction 2026-09-17, deferred at the time it was raised). A workspace holding a locked function is not written as plain text.

The problem is real and is already documented in docs/workspaces.md. Closing a definition with del-tilde locks a function: it runs, but it cannot be displayed, cannot be reopened, and cannot be unlocked. APL\360 workspaces were binary, so a locked function was genuinely opaque to whoever you sent it to. A sw-apl workspace is text, and the writer puts a locked function into it in full, closed with del-tilde so that loading reproduces the lock -- which means anyone with the file can read the body and retype it unlocked.

When a )SAVE would write a workspace holding at least one locked function, obscure the file instead: rot-13 is enough to begin with, and the point is that it is not casually readable rather than that it is secure. )LOAD reads both forms, telling them apart by what the file starts with. A workspace with no locked function stays plain text, because that is the format's whole virtue: readable, diffable, re-executable.

Say plainly, in the code comments and in docs/workspaces.md, that this is obscuring and not encryption. Anyone determined can read a rot-13 file, and the docs must not suggest otherwise; what it stops is reading a locked function by accident, or by curiosity, which is what APL\360's binary format stopped.

Decide and document what an obscured workspace does about being run as a program. A plain workspace file is also a sample -- `sw-apl -f work/NAME.apl.ws` works -- and an obscured one cannot be, so say so rather than leaving it to be discovered.

TDD: a round trip through both forms, a workspace that gains a locked function and is saved again, and a reg-rs test that an obscured file is not readable as APL. Update docs/parity.md and docs/workspaces.md in the same commit. Commit, push, report.
