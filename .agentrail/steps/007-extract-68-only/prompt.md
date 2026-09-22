Phase 9 step 5 (docs/plan.md, owner direction 2026-09-21). Pull the
'68-only parts out of the shared core into crates of their own.

The owner's arrangement: what exists is the shared core; what APLSV
dropped from APL\360 is pulled out of it, so that it is plainly
APL\360's and '75 does not carry it; '75 is added as new code. The
list of what to pull out is docs/aplsv.md's, and nothing not on it
moves.

Move, do not rewrite: use git mv so that history follows each moved
file, and keep a move and any change to what moved in separate
commits. The owner's point is that this makes the history easy to
read -- a reader of git log sees what left the shared core and why,
apart from what '75 adds.

The profile decides whether the '68-only crates are reachable: in
'68 they are, and nothing about '68 changes. reg-rs 81 of 81 with no
rebase. The sw-checklist budgets hold in the new crates as in the
old.

**This step completes Milestone 1 (owner, 2026-09-21): every APL\360
test and demo works in (A) mode.** Before completing it, check that
and say so in the commit: the whole reg-rs corpus, the unit tests and
just check-pages, with the mode explicitly (A) where the host takes a
mode, all green and with no transcript moved.
