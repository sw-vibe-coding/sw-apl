# Conformance samples

Glyph-form APL programs, one feature area per file, each ending
with `)OFF`. They are the executable specification for sw-apl
and the source of the reg-rs transcript baselines (see
`docs/testing.md`).

Origin: adapted from the GNU APL comparison set in
`sw-cor24-apl`, filtered to APL\360 scope. Files that used APL2
features (enclose, pick, each, union, intersection, dyadic tilde)
were left out, as were files that used APLSV features (execute,
format, a quad-named random seed). The index-origin sample uses
`)ORIGIN` rather than a quad variable.

`53-functions.apl` shows the del header forms, locals and dynamic
scoping, and what a function with no result may and may not be used
for. It has no branches, so every body runs from line 1 to the last
line.

`50-horse-race.apl` is a target program written in pure APL\360: a
character matrix for the names, mixed output, and the conditional
branch idiom driving two functions. Its transcript is reproducible
because the random link starts from a fixed value.

`59-sign-off.apl` is the sign-off `)OFF` prints. Its three values
come from the clock, so the reg-rs filter keeps their shape and masks
what they say.

`58-ibeams.apl` shows the I-beam system functions, including the four
that read the clock: it prints their values behind a `(VARIES)` label
that the reg-rs preprocess filter masks, so the transcript shows what
an I-beam returns without pinning a number that cannot come back. See
`docs/testing.md`. It ends with the STIR idiom, which advances the
random link from the clock.

`57-quad-input.apl` reads from its own script: quad evaluating a
typed reply, quote-quad taking characters, and a prompt written with
quote-quad that shares a line with its answer.

`56-suspension.apl` shows what a failure inside a function leaves
behind: the error naming the function and line, the locals still
readable, `)SI` and `)SIV`, taking the function up again, and
clearing the state indicator.

`55-del-editor.apl` is an edit session: reopening a function,
inserting at a fractional line number, replacing and deleting lines,
displaying with the bracketed quad, editing and renaming through the
header, and locking with del-tilde.

`54-life-function.apl` is Conway's Life rewritten as functions --
GEN for one generation, RUN for a labelled loop over several --
beside the straight-line form in `51-life.apl`.

Run the corpus against the release binary:

```bash
scripts/run-samples.sh          # all samples, transcript to stdout
scripts/run-samples.sh 06       # only matching names
```
