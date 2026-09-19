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

print("""
Overstrikes: struck from two characters, as on a 2741
-----------------------------------------------------
Type the base, press the overstrike key, type the other over it.
Either order forms the same glyph. A pair not listed here is a
CHARACTER ERROR -- the manual's own answer, which gives
"Illegitimate overstrike" as the cause of one.

"source" says how the pair is known: from the IBM manuals, by
mirroring a pair they state, or from the glyph being visibly its
two parts.""")
for o in tables["overstrike"]:
    struck = f"{o['base']} {point(o['base'])} over {o['over']} {point(o['over'])}"
    print(row(o["glyph"], "", struck, o["source"]))

u = tables["underscored"]
print(f"""
The underscored alphabet: a letter struck with the underbar
-----------------------------------------------------------
A rule, not a table: {u['struck']} {point(u['struck'])} struck over any of the twenty-six
letters gives a further character of the APL\\360 set, a letter in
its own right and distinct from the plain one -- X and X{u['mark']} are two
names in one workspace. Either order forms it, as above.

Unicode has no precomposed underscored Latin letter, so each is
written as the letter followed by {point(u['mark'])} COMBINING LOW LINE: two code
points, one column. data/glyphs.toml records why.""")
for letter in u["letters"]:
    struck = f"{letter} {point(letter)} over {u['struck']} {point(u['struck'])}"
    print(f"{letter}{u['mark']:<2} {point(letter)}+{point(u['mark'])} {'':<5}{struck:<23}rule".rstrip())

foundational = sorted(
    {c for o in tables["overstrike"] for c in (o["base"], o["over"])}
)
struck = {o["glyph"] for o in tables["overstrike"]}
print("""
Foundational characters: what a keycap carries
----------------------------------------------
Every glyph above is struck from two of these, and none of them is
itself struck. A 2741's keyboard carried them and nothing else.""")
for c in foundational:
    note = "" if c not in struck else "ALSO STRUCK -- check the table"
    print(row(c, "", note))
PY
echo "Wrote docs/glyphs.txt from data/glyphs.toml"
