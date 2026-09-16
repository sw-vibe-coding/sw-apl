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

The command each test runs is `target/release/sw-apl -f
samples/NAME.apl`; `scripts/reg-seed.sh` builds the release
binary first. The transcript includes the echoed input line so
the baseline reads like a terminal session.

## Conformance corpus

`samples/*.apl` are glyph-form programs adapted from the
`sw-cor24-apl` GNU APL comparison set, filtered to APL\360 scope.
Each ends with `)OFF`. Add a sample whenever a step adds a
primitive or a session behaviour; the sample is the executable
spec for that feature.

Samples using APLSV features (execute, format) are kept and are
expected to fail until Phase 5; their reg-rs tests are created
when the feature lands.

## Gates before every commit

- `cargo test` in the changed workspaces.
- `cargo clippy --workspace --all-targets -- -D warnings` there.
- `cargo fmt --all -- --check` there.
- `sw-markdown-checker -f "**/*.md"` when markdown changed.
- `sw-checklist` always; zero failures, warnings kept at zero.
- `scripts/reg.sh run -q` when interpreter behaviour changed.

`/mw-cp` (in `.claude/commands/mw-cp.md`) runs this list in
order and commits.
