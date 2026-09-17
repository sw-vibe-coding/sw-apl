#!/usr/bin/env bash
# Regenerate docs/glyphs.txt from data/glyphs.toml, the single source
# of truth for every glyph sw-apl knows. The Rust tables come from the
# same file via components/value/crates/apl-value/build.rs.
#
# Run after editing data/glyphs.toml and commit both files.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
python3 - <<'PY' > docs/glyphs.txt
import tomllib

tables = tomllib.load(open("data/glyphs.toml", "rb"))


def point(glyph):
    return f"U+{ord(glyph):04X}"


def row(glyph, name, a="", b=""):
    return f"{glyph:<3} {point(glyph):<7} {name:<16}{a:<23}{b}".rstrip()


print("""sw-apl glyph table
==================

Generated from data/glyphs.toml by scripts/gen-glyphs.sh. Do not edit
this file; edit the TOML and regenerate. The interpreter's tables come
from the same source, so the two cannot disagree.

Column 1 is the glyph, column 2 its Unicode code point, column 3 the
traditional name, columns 4 and 5 the monadic and dyadic meanings
(blank means the glyph has no meaning in that position).

Primitive functions and operators
---------------------------------""")
for p in tables["primitive"]:
    print(row(p["glyph"], p["name"], p["monadic"], p["dyadic"]))

print("""
Syntax
------""")
for s in tables["syntax"]:
    print(row(s["glyph"], s["name"], s["meaning"]))

print("""
Lookalikes NOT accepted (CHARACTER ERROR names the glyph meant)
--------------------------------------------------------------""")
for l in tables["lookalike"]:
    print(row(l["typed"], "", f"use {l['meant']} {point(l['meant'])}"))

print("""
Later-APL glyphs NOT accepted (APL\\360 only)
--------------------------------------------""")
for l in tables["later"]:
    print(row(l["glyph"], "", l["name"]))
PY
echo "Wrote docs/glyphs.txt from data/glyphs.toml"
