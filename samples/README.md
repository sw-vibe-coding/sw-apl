# Conformance samples

Glyph-form APL programs, one feature area per file, each ending
with `)OFF`. They are the executable specification for sw-apl
and the source of the reg-rs transcript baselines (see
`docs/testing.md`).

Origin: adapted from the GNU APL comparison set in
`sw-cor24-apl`, filtered to APL\360 scope. Files that used APL2
features (enclose, pick, each, union, intersection, dyadic tilde)
were left out. Two files use APLSV features and stay pending
until Phase 5:

- `25-horse-race-simple.apl` (format)
- `42-execute.apl` (execute)

Run the corpus against the release binary:

```bash
scripts/run-samples.sh          # all samples, transcript to stdout
scripts/run-samples.sh 06       # only matching names
```
