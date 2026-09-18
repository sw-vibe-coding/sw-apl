#!/usr/bin/env bash
# Seed the reg-rs tests that check the CLI itself rather than a
# sample's transcript: how the binary answers the shell. Each is
# created with the same normalize filter the samples use, so the
# sign-off does not pin a clock.
#
# Existing tests are skipped, as in reg-seed.sh; pass --all to
# recreate them.
#
# Usage: scripts/reg-seed-cli.sh [--all]
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
export REG_RS_DATA_DIR="$PWD/tests/reg-rs"
mkdir -p "$REG_RS_DATA_DIR"
(cd components/cli && cargo build --release --quiet -p sw-apl)
filter="bash scripts/normalize-apl-output.sh"
recreate="${1:-}"

# name|command|description
tests=(
"apl-cli-shebang-env|./tests/scripts/hello.apl|An executable .apl file run through its env -S shebang"
"apl-cli-shebang-bare|./tests/scripts/bare-shebang.apl|An executable .apl file run through a bare shebang"
"apl-cli-hash-elsewhere|./tests/scripts/hash-elsewhere.apl|A hash after the first line is still a CHARACTER ERROR"
"apl-cli-shebang-as-file|target/release/sw-apl --no-echo -f tests/scripts/hello.apl|A file with a shebang, run with -f rather than executed"
)

for spec in "${tests[@]}"; do
    IFS='|' read -r name command desc <<< "$spec"
    if [ -f "$REG_RS_DATA_DIR/$name.rgt" ]; then
        [ "$recreate" != "--all" ] && { echo "  skip $name"; continue; }
        reg-rs remove -p "$name" >/dev/null
    fi
    echo "  create $name"
    reg-rs create -t "$name" -c "$command" \
        --timeout 60 --desc "$desc" --preprocess "$filter"
done
reg-rs run -q && echo "ALL PASS" || echo "SOME FAILURES"
