#!/usr/bin/env bash
# Put the on-screen board's data into pages/.
#
# Both files are copies of something that already exists, made at
# build time so that nothing is copied in the repository:
#
#   keymap.json       what each key sends. The same file apl-keyboard
#                     compiles in, so the board on the page and the
#                     keyboard at a terminal cannot disagree.
#   glyph-names.json  what sw-apl calls each glyph, out of
#                     data/glyphs.toml, which is the single source of
#                     truth for that. A board is unusable to a screen
#                     reader without it.
#
# Build output: pages/ carries neither under version control. See
# docs/terminal.md.
set -euo pipefail
cd "$(dirname "$0")/.."

cp components/term/crates/apl-keyboard/keymap.json pages/keymap.json

python3 - <<'PY'
import json
import pathlib
import tomllib

data = tomllib.loads(pathlib.Path("data/glyphs.toml").read_text())
names = {}
for section in data.values():
    if not isinstance(section, list):
        continue
    for row in section:
        glyph, name = row.get("glyph"), row.get("name")
        if glyph and name and glyph not in names:
            names[glyph] = name
out = pathlib.Path("pages/glyph-names.json")
out.write_text(json.dumps(names, ensure_ascii=False, indent=0, sort_keys=True))
print(f"gen-board: {out} has {len(names)} glyphs")
PY

# Where each key is on the 2741 picture, measured out of the picture
# so the board cannot drift from the file it is drawn over.
scripts/board-keys.py > pages/board-keys.json

# Commands and idioms: tracked in data/, where a test runs every one.
cp data/board.json pages/board.json
