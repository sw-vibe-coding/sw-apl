#!/usr/bin/env bash
# Run every samples/*.apl through the release sw-apl binary and
# print the transcripts. Usage: scripts/run-samples.sh [pattern]
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
BIN=target/release/sw-apl
[ -x "$BIN" ] || (cd components/cli && cargo build --release --quiet -p sw-apl)
pattern="${1:-}"
# A sample whose first line is `⍝!MODES (B)` runs in (B) '75, and one
# whose first line is `⍝!MODES (C)` in (C) '80, and `⍝!MODES (D)` in (D) '83.
# A deck, samples/*.deck, runs in (U) '74.
mode_of() {
    if [[ "$1" == *.deck ]]; then echo "--mode 74"
    elif head -1 "$1" | grep -q '^⍝!MODES (B)$'; then echo "--mode 75"
    elif head -1 "$1" | grep -q '^⍝!MODES (C)$'; then echo "--mode 80"
    elif head -1 "$1" | grep -q '^⍝!MODES (D)$'; then echo "--mode 83"
    fi
}
for f in samples/*.apl samples/*.deck; do
    base="$(basename "$f")"; base="${base%.*}"
    [ -n "$pattern" ] && [[ "$base" != *"$pattern"* ]] && continue
    echo "==== $base"
    # shellcheck disable=SC2046  # the mode is zero or two words
    "$BIN" $(mode_of "$f") -f "$f" || echo "(exit $?)"
done
