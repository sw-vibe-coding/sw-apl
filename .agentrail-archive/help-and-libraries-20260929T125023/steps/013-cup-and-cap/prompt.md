Owner question 2026-09-19: are cup and cap really "not APL\360"?

sw-apl puts the union glyph U+222A and the intersection glyph
U+2229 in the [[later]] table of data/glyphs.toml, so each alone is
CHARACTER ERROR with the text "(intersection, not APL\360)". That
classification was made from memory in the glyph-entry step, not
from a cited source, and it contradicts the repo itself: the
overstrike table says the lamp is struck from the cap and the
circle, which means the cap was on the 2741 keyboard -- so it
cannot also be a glyph a later APL introduced.

Separate the two claims and decide each on evidence:

- As FUNCTIONS. Union and intersection as primitives are APL2, not
  APL\360. Confirm against the IBM APL\360 Users Manual (Aug 1968)
  and the APL\360-OS/DOS manual (Dec 1970) function tables. If they
  are absent there, that half of the claim stands.
- As CHARACTERS. Did the APL\360 character set, and the 2741 type
  element, carry the cup and the cap? The repo already assumes the
  cap did. If both are characters of the set with no function
  meaning, then CHARACTER ERROR is the wrong error and the [[later]]
  classification is the wrong home for them. Find what the manual
  says a legitimate character with no meaning earns -- Table 3.1
  lists the causes of each error -- and whether that is SYNTAX ERROR
  or something else. Do not guess; if the manuals do not settle it,
  say so in the commit and choose the answer that keeps the repo
  self-consistent.

Whatever is decided, the two glyphs must be treated alike, and the
overstrike table and the [[later]] table must stop disagreeing.

Tests: RED first. Coverage is thin today -- verified at HEAD, the
cup appears in no sample and no unit test at all, and the cap only
as a CHARACTER ERROR in samples/71-overstrikes.apl. Both glyphs
deserve a unit test pinning the decided behaviour and a line in a
sample, whichever way it goes. Rebase reg-rs with the reason in the
commit message.

Docs: data/glyphs.toml comments carrying the provenance the way the
overstrike rows do, docs/glyphs.txt via scripts/gen-glyphs.sh,
docs/glyph-entry.md (the "components only" list names the cap),
docs/language.md and docs/parity.md as the decision requires.

Gates: just fmt, just test, just clippy, just gates.