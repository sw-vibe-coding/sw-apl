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

`50-horse-race.apl` is a target program written in pure APL\360
(character matrix for names, mixed output, the branch idiom); it
runs once defined functions, branching, indexing, comparisons, and
roll land, and its transcript is reproducible because the random
link starts from a fixed value.

Run the corpus against the release binary:

```bash
scripts/run-samples.sh          # all samples, transcript to stdout
scripts/run-samples.sh 06       # only matching names
```
