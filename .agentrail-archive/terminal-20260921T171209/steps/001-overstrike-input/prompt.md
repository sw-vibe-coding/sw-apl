Phase 8 step 1 (docs/plan.md, owner direction 2026-09-19). Typing an
APL glyph by striking one character over another.

On a 2741 you formed `⍟` by typing `○`, backspace, `*`: the golf
ball struck both on one position. The owner's keyboard carries only
the foundational glyphs on its caps, as a 2741's did, so this is the
input method for daily use and it lands in the CLI first.

Read `sw-embed/web-sw-tos/crates/swtos-input/` before starting. Its
`dispatch.rs` and `translate.rs` are the same problem solved once
already: a frontend that assists with keystrokes rather than
forwarding them, and a prefix key chosen against stated criteria.
Two of its lessons are requirements here.

## The table

The overstrike pairs go in `data/glyphs.toml`, which says of itself
that nothing else may hold a copy of this data, and
`scripts/gen-glyphs.sh` regenerates `docs/glyphs.txt`. Put them
there, regenerate, commit both.

Get the pairs from a source, not from memory. Say in the commit
where the table came from and name any pair you could not confirm
rather than guessing. Only APL\360 glyphs: a pair forming a later
APL's glyph does not belong, and should keep whatever CHARACTER
ERROR that glyph gets today.

Mark which glyphs are foundational and which are struck. The
keyboard step wants exactly the foundational set for its keycaps,
and the owner is printing those keycaps.

## The key

Backspace cannot be it: a line editor needs backspace for delete,
and a mode or a remembered-deletion trick would make one key mean
two things. It is a separate keystroke, and the owner's keyboard
sends it from a repurposed, relabelled keypad key.

`Ctrl-H` cannot be it either, for two independent reasons worth
keeping in the code comment: `Ctrl-H` *is* `0x08`, byte-identical
to backspace -- the same trap `swtos-input/dispatch.rs` documents
for `Ctrl-[` and Escape -- and rustyline already binds it. The
control characters rustyline claims are A B C D E F G H I K L N P Q
R S T U V W X Y and `Ctrl-_`.

Default `Ctrl-]` (0x1D): free in rustyline, not a terminal signal
(SIGQUIT is `Ctrl-\`, 0x1C), and claimed by no browser shortcut
known -- verify that on the three platforms rather than trusting
it, as `Ctrl-B` was. `Ctrl-^` (0x1E) is the fallback if a platform
surprises you.

Name it in one constant with its label beside it, so the help text
cannot come to name a different key from the one bound. That is
`PREFIX_LABEL`'s lesson and its comment says what it cost upstream.

## The composing

One state machine, front end, both hosts. The CLI has no server, so
composition happens there regardless; doing it front-end in both
means one mechanism and one table fed two ways rather than two
composition points that can disagree. This step builds it and wires
the CLI; the browser wires the same thing later.

Decide and write down what the state machine does when the pair is
not a documented overstrike -- the two characters both stand, or
the sequence is refused, or something else -- and what happens when
the key is pressed with nothing before it.

The wire protocol is unaffected: it carries ordinary lines.

Keep a way in that needs no keyboard. The espanso and emacs
expansions already shipping are one; the glyph board comes later.
A key can be lost to a platform nobody tested.

TDD. The table and the state machine are ordinary unit tests. The
rustyline binding is the part tests cannot reach: say so, and say
what you did instead to be sure of it. `docs/input-methods.md`
gains the section, naming which glyphs are struck and which are
foundational.
