Phase 3 step 10 (docs/plan.md, owner direction 2026-09-17). reg-rs is the tool for CLI regression testing in general, not only for sample transcripts and not only where output varies. Rust tests are for unit tests, and for the function and integration testing of the libraries.

components/cli/crates/sw-apl/tests/cli_tests.rs is on the wrong side of that line. It spawns the binary and asserts on its stdout, stderr and exit code, which is what reg-rs captures: the version block, the help text, batch from stdin, --no-echo, a missing file and its exit status, invalid UTF-8, the bracketed definition prompt in a batch echo, a read fed from the script, and the quote-quad prompt sharing a line with its answer.

Move each of those to a reg-rs test. The commands are ordinary shell, so stdin cases are a printf piped into the binary. reg-rs records the exit code, so the missing-file case keeps its non-zero status as part of the baseline rather than an assertion.

Two need thought rather than translation:

- The version block carries a git hash and a build timestamp, so it cannot be pinned as it stands. Extend scripts/normalize-apl-output.sh, or give that test its own preprocess, and say in the script's comments which lines are masked and why. This is the case the VARIES convention does not fit, because the output is not a sample's and cannot be labelled from inside.
- "-h is shorter than --help" is a property, not a transcript. Either express it as a command whose output is the answer, or keep it as the one Rust test that remains, and say which was chosen and why.

Seed the new tests through scripts/reg-seed.sh or a sibling script if the samples loop does not fit them; do not create them by hand, so the corpus stays reproducible. Delete what has moved, and leave cli_tests.rs only if something genuinely belongs there.

Update docs/testing.md to say which tool covers which kind of test, since that division is what was missing. Update the gates list in docs/testing.md and .claude/commands/mw-cp.md if reg-rs now has to run for a CLI change. Commit, push, report.
