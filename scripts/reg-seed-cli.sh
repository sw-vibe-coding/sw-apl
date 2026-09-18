#!/usr/bin/env bash
# Seed the reg-rs tests that check the CLI itself: how the binary
# answers the shell, rather than what a sample program prints. Rust
# tests cover the unit, function and integration testing of the
# libraries; this is the other half. See docs/testing.md.
#
# Each is created with the same normalize filter the samples use, so
# the sign-off does not pin a clock and the version block does not pin
# the machine it was built on.
#
# Two of these are not transcripts but properties, so the command is
# written to make its own answer the output: whether -V agrees with
# --version, and whether -h is shorter than --help.
#
# A description may not begin with a dash: reg-rs reads it with clap,
# which refuses a flag-like value, so each is worded to start with a
# word.
#
# Existing tests are skipped; pass --all to recreate them.
#
# Usage: scripts/reg-seed-cli.sh [--all]
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
export REG_RS_DATA_DIR="$PWD/tests/reg-rs"
mkdir -p "$REG_RS_DATA_DIR"
(cd components/cli && cargo build --release --quiet -p sw-apl)
filter="bash scripts/normalize-apl-output.sh"
recreate="${1:-}"
apl="target/release/sw-apl"

# seed NAME DESCRIPTION COMMAND
seed() {
    local name="$1" desc="$2" command="$3"
    if [ -f "$REG_RS_DATA_DIR/$name.rgt" ]; then
        [ "$recreate" != "--all" ] && { echo "  skip $name"; return; }
        reg-rs remove -p "$name" >/dev/null
    fi
    echo "  create $name"
    reg-rs create -t "$name" -c "$command" \
        --timeout 60 --desc "$desc" --preprocess "$filter"
}

seed apl-cli-version \
    "The version block: name, copyright, licence, repository, build" \
    "$apl --version"
seed apl-cli-version-short \
    "The short form -V gives the same block as --version" \
    "if [ \"\$($apl -V)\" = \"\$($apl --version)\" ]; then echo 'same'; else echo 'DIFFER'; fi"
seed apl-cli-help \
    "The long help, including the agent instructions" \
    "$apl --help"
seed apl-cli-help-short \
    "The short form -h gives less than --help" \
    "if [ \"\$($apl -h | wc -c)\" -lt \"\$($apl --help | wc -c)\" ]; then echo 'shorter'; else echo 'NOT SHORTER'; fi"
seed apl-cli-stdin \
    "Batch from stdin, echoing each line, stopping at )OFF" \
    "printf '2+2\n)OFF\n3+3\n' | $apl"
seed apl-cli-no-echo \
    "The --no-echo flag prints the output without the input" \
    "printf '2+2\n)OFF\n' | $apl --no-echo"
seed apl-cli-file \
    "A file run with -f" \
    "$apl -f tests/scripts/two-lines.apl"
seed apl-cli-missing-file \
    "A file that is not there: a message, and a non-zero status" \
    "$apl -f /nonexistent/x.apl 2>&1"
seed apl-cli-bad-utf8 \
    "A line that is not UTF-8 is reported and the run carries on" \
    "$apl -f tests/scripts/bad-utf8.apl"
seed apl-cli-definition-echo \
    "Definition lines echo behind the bracketed prompt" \
    "printf '\342\210\207R\342\206\220DOUBLE N\nR\342\206\220N+N\n\342\210\207\nDOUBLE 4\n)OFF\n' | $apl"
seed apl-cli-quad-from-script \
    "A read takes the next line of the script" \
    "printf 'X\342\206\220\342\216\225\n2 3 4\nX\303\2272\n)OFF\n' | $apl"
seed apl-cli-quote-quad-line \
    "A quote-quad prompt and its answer share a line" \
    "$apl -f tests/scripts/greet.apl"
seed apl-cli-open-line-ends \
    "A statement ends the line quote-quad left open, unlike a function" \
    "$apl -f tests/scripts/open-line.apl"
seed apl-cli-workspace-file \
    "A saved workspace file is a program: running it rebuilds the workspace" \
    "cat tests/scripts/saved-workspace.apl.ws - <<'"'"'APL'"'"' | $apl --no-echo
SUM 10
3 HYP 4
M
T
)WSID
)OFF
APL"
seed apl-cli-shebang-env \
    "An executable .apl file run through its env -S shebang" \
    "./tests/scripts/hello.apl"
seed apl-cli-shebang-bare \
    "An executable .apl file run through a bare shebang" \
    "./tests/scripts/bare-shebang.apl"
seed apl-cli-hash-elsewhere \
    "A hash after the first line is still a CHARACTER ERROR" \
    "./tests/scripts/hash-elsewhere.apl"
seed apl-cli-shebang-as-file \
    "A file with a shebang, run with -f rather than executed" \
    "$apl --no-echo -f tests/scripts/hello.apl"
# A workspace holding a locked function is written obscured, so it is
# not a program the way a plain one is. It says so itself rather than
# printing a screen of CHARACTER ERRORs.
seed apl-cli-obscured-as-file \
    "An obscured workspace run as a program says what it is and signs off" \
    "$apl -f tests/scripts/work/VAULT.apl.ws"
# The same file through )LOAD, which is the way in. --library points
# library 0 at the fixture directory, so the test does not read or
# write the user's own work/.
seed apl-cli-obscured-load \
    "An obscured workspace loads, runs, and keeps its function locked" \
    "$apl --library tests/scripts --no-echo <<'"'"'APL'"'"'
)LOAD VAULT
SECRET
OPEN 4
∇SECRET
)FNS
)OFF
APL"

reg-rs run -q && echo "ALL PASS" || echo "SOME FAILURES"
