Phase 5 step 3 (docs/plan.md, owner request 2026-09-18). Write
docs/i-beam-reference.md.

The I-beams are documented today as an eight-row table in
docs/language.md, which says what each one is and nothing about how
to use it. The units are the sort of thing a reader has to be told:
sixtieths of a second since midnight, a date as the integer MMDDYY,
a space figure in bytes. A reader who wants the time of day has to
work out `(⌶20)÷60` for themselves, and a reader who wants the date
readable has to take it apart with `⊤`.

Write the reference. For each of ⌶20 through ⌶27: what it reports,
in what units, a worked example of turning it into something a
person would want, and what it does when there is nothing real to
report -- a clear workspace has a stopped clock, ⌶23 is always 1
because there are no other terminals, and ⌶26 and ⌶27 are empty in
immediate execution.

Say what a left argument does: DOMAIN ERROR, because the I-beam is
monadic and there is nothing for a left argument to mean. Say what
an argument outside 20 to 27 does. Say that these are APL\360's
I-beams and not the APLSV quad-names that replaced them, and that
sw-apl has none of those.

⌶22 is the one with a story: it reports space still free, the
workspace has a size set by --ws-size, and WS FULL is what happens
when something will not fit. docs/workspaces.md covers that; link to
it rather than repeating it.

The user-facing docs rule applies: what and how, never when or
plans. No saga names, no step numbers, no "new". See CLAUDE.md.

docs/*.md may use glyphs, so write the glyphs. Check the examples by
running them -- samples/58-ibeams.apl is the existing worked example
and the transcripts in the reference must match what the binary
actually prints. Link the doc from README.md's docs list and from
language.md's I-beam table.

No code change is expected. If writing the reference turns up a
behaviour that is wrong rather than merely undocumented, do not fix
it here: say so and add a step.
