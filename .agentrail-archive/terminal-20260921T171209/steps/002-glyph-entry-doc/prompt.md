Owner direction 2026-09-19, mid-session: the glyph entry documentation.

docs/input-methods.md is named for a list of ways in, but what a
reader wants is a page about entering glyphs. Rename it to fit,
and update every reference (README.md, CLAUDE.md, docs/*.md,
anything under samples/).

Keep the Espanso and Emacs coverage -- both stay, in full.

What the page is short on is the overstrike. It has a section,
but it reads as a footnote to the keymaps when it is the one
method that is APL\360 rather than a modern convenience. Document
it properly: what a 2741 did and why the carriage move is the
whole trick, the Ctrl-] key and why not Ctrl-H, the 0x08 form a
file may carry, either order, the full table of which pairs form
which glyph and where each pair is attested, what an illegitimate
overstrike reports, and which characters are foundational (keycap)
versus struck. A reader should be able to type every struck glyph
in APL\360 from this page without opening glyphs.txt.

glyphs.txt stays the machine-readable table the lexer tests read;
the page is the prose. They must agree -- check the pairs against
it rather than retyping them from memory.

Gates: just gates (sw-markdown-checker + sw-checklist). docs/ may
use glyphs; README.md and CLAUDE.md may not.