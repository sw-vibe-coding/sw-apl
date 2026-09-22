Owner direction 2026-09-21: the keyboard gets the same treatment as
the workspaces -- '70 has the 2741's, and '75 may have the IBM 5100's
where it differs, and only as much as it differs.

Build from docs/aplsv.md's findings: how the 5100's APL keyboard
differs from the 2741's, and whether the 5100, 5110 and 5120 are the
same. If they do not differ in any key a reader types, say so and
share the board; do not invent a difference to have one.

If there is a 5100 picture to use, it needs its own licence and
attribution, kept as the 2741's is in images/redistributed/ with
ATTRIBUTION.md, and the same rules for adapting it. The board and the
keymap follow the mode the tab selects. APLSV was used from 2741s as
well, so say in the docs that the 5100 board is '75's look and not a
claim that APLSV needed it.

**Carried from step 012 (execute):** the IBM 5100 manual forms
execute by overstriking ⊥ and ∘, and format by ⊤ and ∘. The
overstrike table in data/glyphs.toml is mode-blind today, so those
two pairs were not added: in (A) they must stay an illegitimate
overstrike. Add them for (B) with the keyboard, and have composition
read the mode.
