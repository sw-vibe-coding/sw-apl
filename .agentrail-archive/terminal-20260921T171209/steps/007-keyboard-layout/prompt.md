Owner direction 2026-09-20. The keyboard is the 2741's, and the
picture of it comes with a licence.

The owner types on a keyboard relabelled as a 2741's, so the map is
a physical fact to match, not a convenience to choose. Two keys are
wrong today: the semicolon key must send right bracket, and shifted
it must send right parenthesis.

Do not take those two corrections and stop there. The article about
the 2741 carries no key table, so the layout exists only in the
picture of the keyboard, and the picture is therefore the source:
read the whole map out of it and reconcile it against keymap.json
key by key. Where the two disagree, the picture wins and the commit
says which keys moved and why. Where the picture is unreadable or
ambiguous, say so rather than guessing, and leave what is there.

The picture is File:APL-keybd2.svg by the Wikimedia Commons user
Rursus, under CC BY-SA 3.0. The owner wants it redistributed with
sw-apl. Put it in images/redistributed/apl-keyboard/ with the full
licence text beside it and an attribution naming the author, linking
the licence, and saying whether the file was changed. Keep borrowed
material apart from ours, which is the distinction ws/ and work/
already make; check whether scripts/check-provenance.sh needs to
know about it.

The image is also what the clickable board in the glyph-keyboard
step will be drawn from, so leave it in a state that step can use.

Tests: the keyboard tests assert what each key sends, so they move
with the map. Add one that reads the map and the picture together if
that can be done honestly; if it cannot, say why.

Do not do the attention key here. It is its own step, and it needs
the per-session interrupt flag first.
