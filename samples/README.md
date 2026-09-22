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

`71-overstrikes.apl` forms glyphs the way a 2741 did, by striking
one character over another: log from circle and star, domino from
quad and divide, the lamp from a character that is not an APL\360
glyph at all. The file carries the backspace a 2741 sent; at a
terminal the key is Ctrl-].

`72-birds.apl` loads the BIRDS workspace from library 1 and flies
every combinator APL\360 can write, shows the limit of the table of
primitives the birds reach, and prints what cannot be written and
why. See docs/birds.md.

`73-set-operations.apl` builds intersection, difference, union and
unique from membership, compression and the index generator, since
APL\360 has no primitive for any of them -- on numbers and on
characters. Unique is the nub idiom, which needs the index generator
to take the shape of a vector.

`74-single-element-extension.apl` shows which arguments extend: a
scalar or a one-element array of any rank, with a scalar function;
a one-element left argument to compression, but not a scalar right
one; either argument of decode; and an axis in brackets.

`70-open-definition.apl` types eight system commands into an open
function definition: none becomes a body line, most run at once, and
the four that would store or copy a half-changed workspace are
refused. A comment, being an APL statement, does go in.

`69-no-such-function.apl` shows what a glyph used where it has no
such form answers, and what separates that from a bad argument: a
function APL\360 has not got is a SYNTAX ERROR, while a function it
has, given something outside its domain, is a DOMAIN ERROR.

`68-command-names.apl` shows how much of a command name has to be
typed: four characters for a long one, and anything after the fourth
ignored rather than forgiven, while a short name must be exact.

`67-numeric-edges.apl` walks the boundaries: the fuzz and what it
does to comparison, floor and counting; where a number stops being
exact; what `)DIGITS` bounds and what it does not; exponential form
in and out; and the point where a value is too large to keep.
`docs/parity.md` records which of these are the manual's and which
are ours.

`66-domino.apl` inverts a matrix, checks the inverse by matrix
product, solves a system, and fits a line to four points by least
squares. Its examples are chosen so that every answer is exact in
binary floating point, since a transcript prints what it is given.

`65-trouble-reports.apl` walks what a system command says when it
cannot do what was asked, and what separates the replies: INCORRECT
COMMAND is about the command, while WS NOT FOUND, OBJECT NOT FOUND
and IMPROPER LIBRARY REFERENCE are about what was asked for.

`64-library.apl` lists the shipped library with `)LIB 1`, loads a
workspace from it with `)LOAD 1 EDIT`, and fixes the off-by-one that
workspace carries on purpose -- one line found with a bracketed quad
and one line written back.

`63-names-and-groups.apl` lists what a workspace holds with `)FNS`
and `)VARS`, shows that `)VARS` reports globals even while a call's
locals are in scope, gathers names into a group and erases them
through it, and shows the one thing `)ERASE` will not take: a
function the state indicator is still holding.

`62-workspace-space.apl` shows how much room a workspace has: what a
name, a value, a character and a defined function each cost, what is
given back when a name is reassigned or the workspace cleared, and
what WS FULL looks like. What a workspace holds is bounded; what an
expression builds on the way to a result is not.

`61-save-load.apl` saves a workspace, clears, loads it back, and then
copies from it -- showing that copying takes the names but leaves the
index origin alone, which is the difference that matters.

`60-workspace.apl` names a workspace with `)WSID` and clears it with
`)CLEAR`, showing what belongs to the workspace -- names, settings,
and a suspended function -- and what a clear one starts with.

`59-sign-off.apl` is the sign-off `)OFF` prints. Its three values
come from the clock, so the reg-rs filter keeps their shape and masks
what they say.

`58-ibeams.apl` shows the I-beam system functions and what they
refuse -- a left argument, an argument out of range or not whole, an
axis bracket -- which pins the claims `docs/i-beam-reference.md`
makes. It includes the four
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
