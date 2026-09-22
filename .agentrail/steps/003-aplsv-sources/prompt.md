Phase 9 step 3 (docs/plan.md). Find the APLSV sources, and record how
'72 differs from '68, before any '72 behaviour is built.

Everything so far has been checked against the IBM APL\360 User's
Manual. '72 needs the same: the APLSV manual -- IBM's APL Shared
Variables system, the terminal user's manual or equivalent -- and
anything else primary. Find what is available, say where, and say
what could not be found.

Write docs/aplsv.md: what APLSV added (execute, format, the quad
system variables and functions -- each with what it does), what it
changed, and what it dropped from APL\360. The last list is what the
extraction step pulls out into '68-only crates, so it must be
sourced, not remembered: the I-beams and the )ORIGIN, )DIGITS and
)WIDTH commands are candidates, not facts, until a source says. Mark
anything that is a guess as one, and cite the page or section for
everything that is not.

Also settle, from the sources if they say: whether APLSV could load
an APL\360 workspace, which decides whether '72 may read '68's
libraries.

No code. docs/ may carry glyphs; this is a planning document and may
say what is to come.
