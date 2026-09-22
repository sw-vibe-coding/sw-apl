Phase 9 step 11 (docs/plan.md). '72 is usable, so the reader is told.

- The browser: the Ⓑ '72 tab beside Ⓐ '68, with its own libraries,
  and the tooltip "APLSV compatible". Switching mode says what happens
  to the workspace in hand.
- The docs: docs/language.md becomes a shared core with what '72
  adds, and what '68 has that '72 does not; docs/parity.md gains a
  column per mode; session.md and commands-reference.md say which
  commands each mode has. User-facing rules still hold: what and how,
  no plans.
- The README says sw-apl has two modes and links docs/aplsv.md.

**This step completes Milestone 2 (owner, 2026-09-21): APLSV's own
tests and demos, except format, work in (B) mode.** Format is the
next step, not this one. Before completing it, check and say in the
commit that every '72 sample and test is green in (B), and that
Milestone 1 still holds in (A).

**No IBM names in the interface (owner, 2026-09-21):** the app is
sw-apl and its modes are (A) '68 and (B) '75, so as not to borrow
IBM's possibly trademarked names. The (A) tab's tooltip and
aria-label in pages/index.html read "APL\360 compatible" today;
reword both, and give (B) a tooltip with no IBM name either. IBM
names may appear in docs only to say what a mode is modelled on.
See docs/mode-b.md, decision 5.
