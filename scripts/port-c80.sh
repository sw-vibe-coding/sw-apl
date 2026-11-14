#!/usr/bin/env bash
# Port workspaces in glyphs to (C) '80's form (docs/mode-c.md, "The
# workspaces"): NAME.ab.apl.ws or NAME.a.apl.ws to NAME.c.apl.ws beside
# it. apl-c80-port does the definitions, the characters, the indexing
# and the D. names; this rewords what APL80 would show badly in prose:
# a dash (APL80 shows a minus as _) and APL\360 (it shows \ as w).
# What comes out is loaded and run in (C) before it is committed.
#
#   scripts/port-c80.sh ws/lib1/RACE.ab.apl.ws ...
set -euo pipefail
cd "$(dirname "$0")/.."
cargo build -q --release --manifest-path components/c80/Cargo.toml -p apl-c80-port --example apl-c80-port
for from in "$@"; do
  base="$(basename "$from")"
  to="$(dirname "$from")/${base%%.*}.c.apl.ws"
  target/release/examples/apl-c80-port "$from" |
    perl -CSD -Mutf8 -pe 'next if /^⍝!/; s/ __ /: /g; s/APLw360/APL\/360/g; s/(_{3,})/"=" x length($1)/ge' > "$to"
  echo "$to"
done
