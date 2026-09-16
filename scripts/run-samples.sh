#!/usr/bin/env bash
# Run every samples/*.apl through the release sw-apl binary and
# print the transcripts. Usage: scripts/run-samples.sh [pattern]
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
BIN=target/release/sw-apl
[ -x "$BIN" ] || (cd components/cli && cargo build --release --quiet -p sw-apl)
pattern="${1:-}"
for f in samples/*.apl; do
    base="$(basename "$f" .apl)"
    [ -n "$pattern" ] && [[ "$base" != *"$pattern"* ]] && continue
    echo "==== $base"
    "$BIN" -f "$f" || echo "(exit $?)"
done
