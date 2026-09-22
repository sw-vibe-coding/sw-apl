Phase 9 step 11 (docs/plan.md). '75 is usable, so the reader is told.

- The browser: the Ⓑ '75 tab beside Ⓐ '70, with its own libraries,
  and the tooltip "APLSV compatible". Switching mode says what happens
  to the workspace in hand.
- The docs: docs/language.md becomes a shared core with what '75
  adds, and what '70 has that '75 does not; docs/parity.md gains a
  column per mode; session.md and commands-reference.md say which
  commands each mode has. User-facing rules still hold: what and how,
  no plans.
- The README says sw-apl has two modes and links docs/aplsv.md.

**This step completes Milestone 2 (owner, 2026-09-21): APLSV's own
tests and demos, except format, work in (B) mode.** Format is the
next step, not this one. Before completing it, check and say in the
commit that every '75 sample and test is green in (B), and that
Milestone 1 still holds in (A).

**No IBM names in the interface (owner, 2026-09-21):** the app is
sw-apl and its modes are (A) '70 and (B) '75, so as not to borrow
IBM's possibly trademarked names. The (A) tab's tooltip and
aria-label in pages/index.html read "APL\360 compatible" today;
reword both, and give (B) a tooltip with no IBM name either. IBM
names may appear in docs only to say what a mode is modelled on.
See docs/mode-b.md, decision 5.

**Tooltip wording (owner, 2026-09-21):** the (A) tab's tooltip reads
"APL\360-inspired" and the (B) tab's "IBM 5100-inspired" -- this
settles the rewording asked for above. The page passes the mode to
the worker as a mode field in its start message ("68" or "75");
apl-wasm reads it already and a page that sends none is (A).

**Done already, in step 009 (rename to '70):** the (A) tab reads
"Ⓐ '70", its tooltip and the tap-to-show text "APL\360-inspired",
and its aria-label "1970, APL\360-inspired"; scripts/check-pages.mjs
checks all three. This step adds the (B) tab beside it.

**From step 013 (mode-aware overstrikes):** the browser board is
built for a mode -- `new Board("A")` today in pages/apl.js. The (B)
tab must start its session with a new `Board("B")` and send the
worker a `mode` field ("75"), so the board composes ⍎ and ⍕ from the
5100's pairs and the session is in (B).
