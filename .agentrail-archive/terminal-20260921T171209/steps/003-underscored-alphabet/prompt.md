Owner observation 2026-09-19: the underscored alphabet.

A 2741 struck the underbar over any letter, and APL\360 used it: A
through Z underscored were twenty-six further characters of the
set, valid in variable and function names and distinct from the
plain letter -- X and X-underscored were two different names in one
workspace. Delta-underbar is the same idea applied to delta, and
sw-apl already has that one.

sw-apl does not have the letters. Verified at HEAD: underbar struck
over A is an illegitimate overstrike, and the combining low line
U+0332 is a CHARACTER ERROR. docs/parity.md now carries the
overstrike row as part, naming this as what is missing; the row
goes back to done in this step.

Decide and say why in the commit:

- The representation. There is no precomposed Unicode for these.
  Letter followed by U+0332 (combining low line) is the obvious
  answer and is what a file written in any editor will hold, but it
  makes a name a sequence of two code points per underscored
  letter, which touches the lexer, the workspace name tables, the
  display width calculation, and every place a name is compared.
  Check what that costs before committing to it. GNU APL and the
  APL\360 tape transcriptions may have precedent worth citing.
- Display width. An underscored letter is one column on paper. If
  the combining form is chosen, the display crate must not count
  U+0332 as a column, or every column of output holding one of
  these names goes out by one.
- The overstrike table. Twenty-six pairs are not seventeen hand-
  written rows; a letter struck with underbar is a rule, not a
  table entry. Decide whether strike() grows a rule or the
  generated table grows the rows, and keep the property that no two
  pairs share their characters.
- Names. docs/language.md states what a name is made of; it changes
  here.

Tests: RED first. A unit test that the strike forms it, a lexer
test that it is a name character, a session test that X and
X-underscored are different names, a display test for the column
width. A sample -- extend samples/71-overstrikes.apl or add one --
and rebase reg-rs with the reason in the commit message.

Docs: docs/glyph-entry.md has a section saying this is not
implemented; replace it with how to type them. docs/language.md,
docs/glyphs.txt (via data/glyphs.toml and scripts/gen-glyphs.sh),
docs/parity.md.

Gates: just fmt, just test, just clippy, just gates.