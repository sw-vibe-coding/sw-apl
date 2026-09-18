#!/usr/bin/env bash
# Seed one reg-rs transcript test per samples/*.apl. Existing
# tests are skipped; rebase intentionally with scripts/reg.sh rebase.
#
# Every test is created with the normalize filter, so a sample may
# print a value that cannot come back the same -- a clock reading, a
# processor time -- by labelling it (VARIES). See the filter's own
# comments and docs/testing.md. The filter changes nothing in a
# transcript with no such label, so it is on for all of them and one
# less thing to remember when a sample later grows one.
#
# Usage: scripts/reg-seed.sh [pattern]
#        scripts/reg-seed.sh --all      recreate every test
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
export REG_RS_DATA_DIR="$PWD/tests/reg-rs"
mkdir -p "$REG_RS_DATA_DIR"
(cd components/cli && cargo build --release --quiet -p sw-apl)
# Samples deliberately left unseeded, and why. Without this the --all
# recreate sweeps them back in, which is how they arrived once before.
#   14-multiline    a comment-only leftover from sw-cor24-apl that
#                   describes )LIST and )RUN, which are not APL\360.
skip_seed="14-multiline"

pattern="${1:-}"
recreate=""
[ "$pattern" = "--all" ] && { recreate="yes"; pattern=""; }
filter="bash scripts/normalize-apl-output.sh"
for f in samples/*.apl; do
    base="$(basename "$f" .apl)"
    [[ " $skip_seed " == *" $base "* ]] && continue
    [ -n "$pattern" ] && [[ "$base" != *"$pattern"* ]] && continue
    name="apl-sample-$base"
    if [ -f "$REG_RS_DATA_DIR/$name.rgt" ]; then
        [ -z "$recreate" ] && { echo "  skip $name"; continue; }
        reg-rs remove -p "$name" >/dev/null
    fi
    echo "  create $name"
    reg-rs create -t "$name" -c "target/release/sw-apl -f $f" \
        --timeout 60 --desc "Transcript of $f" --preprocess "$filter"
done
reg-rs run -q && echo "ALL PASS" || echo "SOME FAILURES"
