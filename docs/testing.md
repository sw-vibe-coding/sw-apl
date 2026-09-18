# sw-apl Testing

## Test-first, always

Every saga step follows red, green, refactor:

1. RED: write failing unit tests that define the behaviour in the
   step prompt. Run them and watch them fail for the right reason.
2. GREEN: write the least code that passes.
3. REFACTOR: split to the sw-checklist gates, keep tests green.

Unit tests live in the crate: `tests/*.rs` for public behaviour,
`*_tests.rs` sibling modules for internals. Keep test modules out
of production files when they distort readability.

## Transcript regressions with reg-rs

End-to-end behaviour is pinned with `reg-rs`. Each sample program
in `samples/` is a test whose baseline is the transcript sw-apl
prints for it. Tests are stored in the repo under `tests/reg-rs/`
(`.rgt` spec plus `.out` baseline) by exporting
`REG_RS_DATA_DIR=tests/reg-rs`; `scripts/reg.sh` sets that and
forwards to `reg-rs`.

Baselines are seeded once the MVP evaluates the early samples
(Phase 1 of `plan.md`); until then the scripts exist but
`tests/reg-rs/` is empty. Seed one test per sample:

```bash
scripts/reg-seed.sh              # creates missing tests only
scripts/reg.sh run               # run all, summary line
scripts/reg.sh run -vv -p 06     # full diff for one sample
```

Rebase a baseline only when the change is intentional, and say so
in the commit message (`reg-rs: rebased 06-reduce, identity of
empty reduce`).

### Output that cannot come back the same

`reg-rs` is built for commands that are not byte-identical twice.
`reg-rs create` takes:

| Option | What it is for |
|---|---|
| `-P`, `--preprocess CMD` | A filter run over the output before it is compared |
| `-M`, `--diff-mode MODE` | How differences are normalized (default `text`) |
| `--expects TEXT` | What the test is meant to show |
| `--flaky-note TEXT` | Why a test is unreliable, when it is |

sw-apl uses the filter. `scripts/normalize-apl-output.sh` is it, and
`scripts/reg-seed.sh` gives it to every test, because it changes
nothing in a transcript that has no value to mask and is then one
less thing to remember when a sample later grows one.

APL prints bare numbers, so a filter cannot tell a clock reading from
arithmetic. The convention is that the sample says which is which. A
value that cannot reproduce is printed behind an upper-case label
ending `(VARIES): `, in the sample's own mixed output:

```apl
'TIME OF DAY (VARIES): ';⌶20
```

The filter replaces what follows with `...`, and only on an output
line: output starts in column one, while an echoed input line is
indented six spaces or headed by its `[n]` prompt, so the statement
stays legible beside its masked answer. `samples/58-ibeams.apl` is
the worked example.

Mask only what genuinely cannot reproduce -- a clock, a processor
time, a host name. A sample twisted into determinism, printing
`((⌶20)≥0)∧(⌶20)<5184000` rather than the time, is a worse sample: it
stops showing what the feature returns. A value masked because it is
merely inconvenient is worse still, because the test then pins
nothing. Where a whole test is unreliable rather than one value in
it, say so with `--flaky-note` instead of widening the filter.

`scripts/reg-seed.sh` also carries the list of samples deliberately
left unseeded, each with its reason, so `--all` cannot quietly sweep
one back in.

### Tests of the CLI itself

`scripts/reg-seed-cli.sh` seeds the tests that check how the binary
answers the shell rather than what a sample prints: the executable
`.apl` files in `tests/scripts/`, run through both shebang forms and
also through `-f`. They are reg-rs tests because that is what reg-rs
is for. Rust tests are for the unit, function and integration testing
of the libraries; `components/cli/crates/sw-apl/tests/cli_tests.rs`
still holds checks that belong here, and moving them is its own step.

The command each test runs is `target/release/sw-apl -f
samples/NAME.apl`; `scripts/reg-seed.sh` builds the release
binary first. The transcript includes the echoed input line so
the baseline reads like a terminal session.

## Parity checklist

`docs/parity.md` lists every APL\360 feature with its status and
the test that pins it. A step that changes a row updates the
file in the same commit.

## Conformance corpus

`samples/*.apl` are glyph-form programs adapted from the
`sw-cor24-apl` GNU APL comparison set, filtered to APL\360 scope.
Each ends with `)OFF`. Add a sample whenever a step adds a
primitive or a session behaviour; the sample is the executable
spec for that feature.

Samples from the original corpus that used features outside
APL\360 (execute, format, a quad-named random seed) were removed;
the index-origin sample uses `)ORIGIN`.

## Generated files

`docs/glyphs.txt` and the Rust glyph tables are generated from
`data/glyphs.toml` (see `architecture.md`). After editing the TOML,
run `scripts/gen-glyphs.sh` and commit both files; a unit test checks
that the generated tables are consistent.

## Gates before every commit

- `cargo test` in the changed workspaces.
- `cargo clippy --workspace --all-targets -- -D warnings` there.
- `cargo fmt --all -- --check` there.
- `sw-markdown-checker` on README.md and the other top-level
  markdown when they changed (`just gates`); `docs/*.md` may hold
  glyphs and is not ASCII-gated.
- `sw-checklist` always; zero failures, warnings kept at zero.
- `scripts/reg.sh run -q` when interpreter behaviour changed.

`/mw-cp` (in `.claude/commands/mw-cp.md`) runs this list in
order and commits.
