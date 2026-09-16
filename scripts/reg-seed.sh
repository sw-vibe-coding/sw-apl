#!/usr/bin/env bash
# Seed one reg-rs transcript test per samples/*.apl. Existing
# tests are skipped; rebase intentionally with scripts/reg.sh rebase.
# Usage: scripts/reg-seed.sh [pattern]
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
export REG_RS_DATA_DIR="$PWD/tests/reg-rs"
mkdir -p "$REG_RS_DATA_DIR"
(cd components/cli && cargo build --release --quiet -p sw-apl)
pattern="${1:-}"
for f in samples/*.apl; do
    base="$(basename "$f" .apl)"
    [ -n "$pattern" ] && [[ "$base" != *"$pattern"* ]] && continue
    name="apl-sample-$base"
    if [ -f "$REG_RS_DATA_DIR/$name.rgt" ]; then
        echo "  skip $name"; continue
    fi
    echo "  create $name"
    reg-rs create -t "$name" -c "target/release/sw-apl -f $f" \
        --timeout 60 --desc "Transcript of $f"
done
reg-rs run -q && echo "ALL PASS" || echo "SOME FAILURES"
