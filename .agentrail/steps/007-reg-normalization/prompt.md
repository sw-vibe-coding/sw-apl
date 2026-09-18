Phase 3 step 7 (docs/plan.md, owner direction 2026-09-17). Use reg-rs the way it is meant to be used.

reg-rs create takes -P/--preprocess (a filter command run over the output before comparison), -M/--diff-mode, --expects and --flaky-note. sw-apl uses none of them: every .rgt here carries only command, timeout, exit_code and desc, and docs/testing.md does not mention that the options exist. sw-ml-study/sw-mlpl is the worked example to follow -- every demo baseline there carries preprocess = "bash scripts/normalize-mlpl-output.sh", a shared sed filter that masks an SVG byte count and float tails so harmless drift passes while a real regression still fails. Read that script before writing this one.

The cost so far is samples/58-ibeams.apl. Believing a baseline had to be byte-identical, it hides the values the I-beams exist to report behind predicates such as a comparison proving the time of day falls inside a day. It should print the values and let the filter mask them.

Work:

1. Add scripts/normalize-apl-output.sh, a stdin-to-stdout sed filter, with a header comment saying what it masks and why, in the style of the mlpl one. Settle a convention for marking a value that varies between runs so the filter can find it without masking ordinary numbers, and say what the convention is in the script and in docs/testing.md. A label inside the sample's own mixed output is the obvious candidate, since it reads as APL and is visible in the transcript.

2. scripts/reg-seed.sh passes -P for every sample. Add --expects or --flaky-note where a test genuinely warrants one; do not add them everywhere for the sake of it.

3. Rewrite samples/58-ibeams.apl to print the real values of I-beams 20, 21, 24 and 25 under the convention, keeping 22, 23, 26 and 27 exact as they already are. Prove the result by running it twice and diffing, as before.

4. Apply the filter to the whole corpus so the baselines are uniform: remove and recreate each test with -P. The filter is a no-op on deterministic text, so keep a copy of the .out files first and diff afterwards to prove nothing but 58 changed. If anything else changes, stop and say why rather than accepting it.

5. Document it in docs/testing.md: the options reg-rs offers, the filter, the convention, and when to reach for --flaky-note rather than a filter. That section's absence is what let this go unnoticed.

Do not weaken a baseline to make a flaky test pass: masking is for values that cannot be reproduced, such as a clock, never for a value that merely looks inconvenient. TDD does not apply directly here, but the diff in step 4 is the proof. Update docs/parity.md only if a row changes. Commit, push, report.
