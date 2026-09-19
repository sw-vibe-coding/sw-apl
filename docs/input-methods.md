# Typing APL glyphs

sw-apl reads Unicode glyphs and nothing else, so you need a way to
type them. This page covers four: Espanso (any application, any
OS), Emacs (a self-contained input method plus the MELPA modes),
OS keyboard layouts, and overstrikes, which is how a 2741 did it.
The glyph inventory with code points is in `glyphs.txt`.

Any of them will do, and they work together. Espanso and Emacs are
the ones that need no special hardware, and they are not going
anywhere.

All three use the same idea: a prefix key followed by the letter
in the glyph's position on the classic APL typeball keyboard,
the layout IBM shipped with the 2741 terminal. rho sits on R,
iota on I, quad on L, left arrow on the left bracket, and so on.
Dyalog and gnu-apl-mode use these same positions, so the muscle
memory transfers.

## The keymap

Prefix is the backtick. Lowercase keys, then shifted keys.

```
`2 high minus   `3 <   `4 <=   `5 =   `6 >=   `7 >   `8 /=
`9 or   `0 and   `- times   `= divide
`q query   `e epsilon   `r rho   `t tilde   `y take
`u drop   `i iota   `o circle   `p star   `[ left arrow
`] right arrow
`s upstile   `d downstile   `f underbar   `g del
`h delta   `j jot   `k quote   `l quad
`b decode   `n encode   `m stile   `, lamp   `. scan-first
`/ reduce-first
`@ del-tilde   `# grade-down   `$ grade-up   `% reverse
`^ transpose   `& reverse-first   `* log   `( nor   `) nand
`_ shriek   `+ domino   `{ quote-quad   `H delta-underbar
`! i-beam
```

Keys for glyphs outside APL\360 (alpha, omega, execute, format,
enclose, disclose, union, intersection, zilde) are intentionally
absent. `H` for delta-underbar
is an sw-apl choice; the other positions are the classic ones.

## Espanso

[Espanso](https://espanso.org) is a cross-platform text expander.
It watches what you type and replaces a trigger with its
expansion in any application, including a terminal running
sw-apl.

Install:

```bash
# macOS
brew tap espanso/espanso && brew install espanso
espanso service register && espanso start
# Linux: see https://espanso.org/install/ (deb, AppImage, or snap)
```

Then copy the match file and restart:

```bash
espanso path            # prints the config directory
cp docs/espanso/apl.yml "$(espanso path config)/match/apl.yml"
espanso restart
```

`docs/espanso/apl.yml` holds one match per key in the table
above. Type backtick then `r` and the two characters are
replaced by rho. If the backtick prefix collides with shell or
markdown use, change the `trigger` strings or add an `apps`
filter so the matches apply only in your terminal.

Tips:

- Espanso needs accessibility permission on macOS the first time.
- The terminal font must cover the APL block (U+2336 to U+237A).
  APL385 Unicode, APL333, DejaVu Sans Mono, Iosevka, and JuliaMono
  all do.

## Emacs

### Self-contained input method (no packages)

`docs/emacs/apl-input.el` defines a Quail input method named
`sw-apl` with the keymap above. Load it and switch to it:

```elisp
(load-file "path/to/sw-apl/docs/emacs/apl-input.el")
(set-input-method "sw-apl")       ; or C-\ then sw-apl
```

`C-\` toggles the input method in the current buffer. It works
in any buffer, including `M-x shell`, `M-x term`, `vterm`, and
`eshell`, so you can run sw-apl inside Emacs and type glyphs
directly. Add the `load-file` line to your init file to make it
permanent.

### Without any configuration

`C-x 8 RET` (`insert-char`) accepts a Unicode name or code point:
`C-x 8 RET APL FUNCTIONAL SYMBOL RHO RET`, or `C-x 8 RET 2374
RET`. Slow, but always available.

### MELPA modes

- `gnu-apl-mode` provides the `APL-Z` input method (default
  prefix is a period, customisable through `gnu-apl-key-prefix`)
  and syntax highlighting for `.apl` files. The key positions
  match the table above.
- `dyalog-mode` provides an input method with the backtick prefix
  and highlighting; it knows the Dyalog superset of glyphs.

Either mode's highlighting works for sw-apl source; only the
input method prefix differs.

## OS keyboard layouts

- Linux (X11 and most Wayland compositors): xkeyboard-config
  ships an `apl` layout. `setxkbmap -layout "us,apl" -option
  grp:switch` puts the APL glyphs on the right Alt key with the
  classic positions.
- macOS: Dyalog publishes a free APL keyboard layout for macOS
  that uses the backtick as a dead key, matching the table.
- Windows: not a target for sw-apl, but the Dyalog layout exists
  there too.

## Checking your setup

Run sw-apl and type a few glyphs at the six-space prompt. When a
glyph is a lookalike from the wrong Unicode block (Greek rho
instead of APL rho, for instance) sw-apl reports CHARACTER ERROR
and names the code point it saw; `glyphs.txt` lists the common
confusions.

## Overstrikes

A 2741 had no key for most APL glyphs. You typed one character,
pressed backspace, and typed another over it: the carriage did not
erase, it only moved, so the golf ball struck both on one position.
`⍟` is `○` struck with `*`. `⌹` is `⎕` struck with `÷`.

sw-apl accepts this, with one change forced by the century: a line
editor needs backspace for deleting, so the carriage move is on a
key of its own.

```
      A←○     press Ctrl-]     then *     then 3
      A←⍟3
```

**The key is `Ctrl-]`.** Not `Ctrl-H`, which would be the obvious
choice and cannot work: `Ctrl-H` *is* `0x08`, the backspace byte, so
a terminal cannot tell them apart. `0x1D` is claimed by neither
rustyline nor the terminal's line discipline. If you have a
programmable keyboard, a spare key sending `Ctrl-]` -- a keypad key
relabelled, say -- gives you the 2741 gesture with keycaps that
carry only the foundational characters, as a 2741's did.

A file may use the backspace a 2741 actually sent, `0x08`, between
the two characters; `samples/71-overstrikes.apl` does. Both markers
mean the same thing.

Either order forms the same glyph. On paper there is no difference
-- both impressions land on one spot -- so `⎕` then `'` and `'` then
`⎕` are both `⍞`. Whether APL\360 accepted both is not recorded in
its manuals; sw-apl does, and it is unambiguous because no two
pairs use the same two characters.

A pair that forms no glyph is a CHARACTER ERROR naming both. The
manual calls it an illegitimate overstrike and gives it as a cause
of exactly that error.

### Which glyphs are struck, and which are keys

`glyphs.txt` has two sections for this: **Overstrikes**, listing
each struck glyph with its two characters and how the pair is known,
and **Foundational characters**, which is what a keycap carries.
Every struck glyph is made from two foundational ones, and no
foundational character is itself struck.

One surprise worth knowing before you print keycaps: `∩` is on the
list. Intersection is not an APL\360 function and typing it alone is
a CHARACTER ERROR -- it is there only because the lamp `⍝` is struck
from `∩` and `○`. The underbar `_` is likewise a component, of `⍙`,
and nothing else.
