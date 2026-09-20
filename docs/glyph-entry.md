# Entering APL glyphs

sw-apl reads Unicode glyphs and nothing else -- there are no keyword
aliases -- so you need a way to type them. There are two kinds of
answer, and they work together:

Espanso and the Emacs input method work in the browser demo too,
and nothing had to be done to make them: an OS-level expander
watches the keystrokes before the browser sees them, so the 2741
map cannot hide a trigger from it, and the backtick it triggers on
is not a key the map claims. Espanso replaces what was typed either
by sending backspaces and the glyph as a keystroke or by pasting,
and the page takes a glyph either way -- a character that is not in
the map is inserted as itself. `just check-pages` pins all three.

In the browser demo there is a third: a board on the page, drawn from
the same `keymap.json` the terminal compiles in, whose keys insert
the glyph they carry. It is the only way in on a touch screen, which
has no keyboard to intercept, and it is laid out like the 2741 so
that tapping `⍴` also shows where Shift-R is. **Keyboard** shows it,
**ABC** switches it to letters and digits, and it remembers whether
it was up. See `terminal.md`.

- **Expansion.** Espanso, an Emacs input method, or an OS keyboard
  layout turns a prefix and a letter into the glyph. This is the
  modern convenience, it needs no special hardware, and it is what
  most readers will use every day.
- **The overstrike.** Type one character, move the carriage back, and
  type another over it. This is not a convenience: it is how APL\360
  was typed, on a terminal whose keyboard carried the foundational
  characters listed below and formed the rest by striking two of them
  together. sw-apl
  accepts it at the prompt and in a file.

The glyph inventory with code points is in `glyphs.txt`, which is the
machine-readable table the lexer's own tests read. This page is the
prose; where the two disagree, `glyphs.txt` is right.

## The keymap

The expansion methods all use the same idea: a prefix key followed by
the letter in the glyph's position on the classic APL typeball
keyboard, the layout IBM shipped with the 2741 terminal. Rho sits on
R, iota on I, quad on L, left arrow on the left bracket, and so on.
Dyalog and gnu-apl-mode use these same positions, so the muscle
memory transfers.

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
absent. `H` for delta-underbar is an sw-apl choice; the other
positions are the classic ones.

## Espanso

[Espanso](https://espanso.org) is a cross-platform text expander. It
watches what you type and replaces a trigger with its expansion in
any application, including a terminal running sw-apl.

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

`docs/espanso/apl.yml` holds one match per key in the table above.
Type backtick then `r` and the two characters are replaced by rho. If
the backtick prefix collides with shell or markdown use, change the
`trigger` strings or add an `apps` filter so the matches apply only
in your terminal.

Tips:

- Espanso needs accessibility permission on macOS the first time.
- The terminal font must cover the APL block (U+2336 to U+237A).
  APL385 Unicode, APL333, DejaVu Sans Mono, Iosevka, and JuliaMono
  all do.

## Emacs

### Self-contained input method (no packages)

`docs/emacs/apl-input.el` defines a Quail input method named `sw-apl`
with the keymap above. Load it and switch to it:

```elisp
(load-file "path/to/sw-apl/docs/emacs/apl-input.el")
(set-input-method "sw-apl")       ; or C-\ then sw-apl
```

`C-\` toggles the input method in the current buffer. It works in any
buffer, including `M-x shell`, `M-x term`, `vterm`, and `eshell`, so
you can run sw-apl inside Emacs and type glyphs directly. Add the
`load-file` line to your init file to make it permanent.

### Without any configuration

`C-x 8 RET` (`insert-char`) accepts a Unicode name or code point:
`C-x 8 RET APL FUNCTIONAL SYMBOL RHO RET`, or `C-x 8 RET 2374 RET`.
Slow, but always available.

### MELPA modes

- `gnu-apl-mode` provides the `APL-Z` input method (default prefix is
  a period, customisable through `gnu-apl-key-prefix`) and syntax
  highlighting for `.apl` files. The key positions match the table
  above.
- `dyalog-mode` provides an input method with the backtick prefix and
  highlighting; it knows the Dyalog superset of glyphs.

Either mode's highlighting works for sw-apl source; only the input
method prefix differs.

## OS keyboard layouts

- Linux (X11 and most Wayland compositors): xkeyboard-config ships an
  `apl` layout. `setxkbmap -layout "us,apl" -option grp:switch` puts
  the APL glyphs on the right Alt key with the classic positions.
- macOS: Dyalog publishes a free APL keyboard layout for macOS that
  uses the backtick as a dead key, matching the table.
- Windows: not a target for sw-apl, but the Dyalog layout exists
  there too.

## Overstrikes

### What a 2741 did

An IBM 2741 printed with a golf ball on a moving carriage, and its
keyboard carried the foundational characters listed at the end of
this section. Every other APL glyph was made by typing one of them,
pressing backspace, and typing a second one. Backspace did not erase -- there was nothing to erase,
the ink was already on the paper -- it only moved the carriage back
one position, so the second character struck the same spot as the
first and the two impressions together were the glyph.

That is the whole trick, and two consequences follow from it that
matter when you type an overstrike in sw-apl:

- **Order does not matter.** Both impressions land on one position,
  so on paper `⎕` struck with `'` and `'` struck with `⎕` are the
  same mark. sw-apl accepts either order. (Whether APL\360 itself
  accepted both is not stated in its manuals. It is unambiguous here
  because no two pairs use the same two characters, which the lexer's
  tests check.)
- **Not every pair is a glyph.** Striking `Q` with `Z` puts two
  letters on one spot and means nothing. The manual calls that an
  illegitimate overstrike and lists it as a cause of CHARACTER ERROR,
  which is what sw-apl reports.

### The key is Ctrl-]

sw-apl accepts the 2741 gesture with one change forced by the
century: a line editor needs backspace for deleting, so the carriage
move is on a key of its own.

**Press `Ctrl-]`.** Type a character, press it, type the character to
strike over the first:

```
      A←○     press Ctrl-]     then *     then 3
      A←⍟3
```

The glyph appears on the line as soon as the second character is
typed. Backspace still deletes, as it does everywhere else.

It is not `Ctrl-H`, which would be the obvious choice and cannot
work: `Ctrl-H` *is* `0x08`, byte-identical to the backspace key, so a
terminal cannot tell the two apart -- and rustyline binds it besides.
`Ctrl-]` is `0x1D`, claimed by neither rustyline nor the terminal's
line discipline (SIGQUIT is `Ctrl-\`, `0x1C`) and by no browser
shortcut known. `Ctrl-^`, `0x1E`, is the fallback if some platform
turns out to claim it.

If you have a programmable keyboard, a spare key sending `Ctrl-]` --
a keypad key relabelled, say -- gives you the 2741 gesture with
keycaps that carry only the foundational characters, as a 2741's did.

Pressing `Ctrl-]` at the start of a line, or twice in a row, does
nothing: there is no character to take back, which is where the
carriage would have met the left margin.

### In a file

A file may carry the byte a 2741 actually transmitted, `0x08`,
between the two characters, and sw-apl reads it the same way it reads
`Ctrl-]`. `samples/71-overstrikes.apl` is written that way; run it
with `sw-apl -f samples/71-overstrikes.apl` to see the transcript.
Both markers mean the same thing, so a file written by another
terminal, or by a script emitting `0x08`, needs no conversion.

You never have to use either marker in a file: a file holding the
composed glyph `⍟` directly is read as `⍟`, and that is what most of
`samples/` does.

### Every struck glyph

These are every struck glyph in APL\360. Type the first
character, press `Ctrl-]`, then type the second -- or the other way
round, which forms the same glyph.

The **Known by** column says how the pair is attested: *manual* means
the IBM manuals state it, *symmetry* means the manuals state the
pair's mirror (the row above it in the same family) and this one
follows it, and *shape* means the printed glyph is visibly its two
parts.

| Glyph | Code point | Name | Struck from | Known by |
|---|---|---|---|---|
| `⌿` | U+233F | slash-bar (compress first) | `/` U+002F and `-` U+002D | manual |
| `⊖` | U+2296 | circle-bar (reverse first) | `○` U+25CB and `-` U+002D | manual |
| `⍋` | U+234B | grade-up | `∆` U+2206 and `\|` U+007C | manual |
| `⍝` | U+235D | lamp (comment) | `∩` U+2229 and `○` U+25CB | manual |
| `⌶` | U+2336 | i-beam | `⊥` U+22A5 and `⊤` U+22A4 | manual |
| `⍫` | U+236B | del-tilde (lock) | `∇` U+2207 and `~` U+007E | manual |
| `⍞` | U+235E | quote-quad | `⎕` U+2395 and `'` U+0027 | manual |
| `⌹` | U+2339 | domino (matrix divide) | `⎕` U+2395 and `÷` U+00F7 | manual |
| `⌽` | U+233D | circle-stile (reverse) | `○` U+25CB and `\|` U+007C | symmetry |
| `⍒` | U+2352 | grade-down | `∇` U+2207 and `\|` U+007C | symmetry |
| `⍀` | U+2340 | backslash-bar (expand first) | `\` U+005C and `-` U+002D | symmetry |
| `⍟` | U+235F | log | `○` U+25CB and `*` U+002A | shape |
| `⍉` | U+2349 | circle-backslash (transpose) | `○` U+25CB and `\` U+005C | shape |
| `⍲` | U+2372 | nand | `∧` U+2227 and `~` U+007E | shape |
| `⍱` | U+2371 | nor | `∨` U+2228 and `~` U+007E | shape |
| `⍙` | U+2359 | delta-underbar (a letter) | `∆` U+2206 and `_` U+005F | shape |
| `!` | U+0021 | shriek (factorial) | `'` U+0027 and `.` U+002E | shape |

`!` is on the list because a 2741 had no exclamation mark, but your
keyboard does; type it directly and save the gesture.

Every other glyph is a key. The bar family is the pattern worth
remembering: `⌿ ⊖ ⍀` are `/ ○ \` struck with a minus, and `⌽ ⍋ ⍒` are
`○ ∆ ∇` struck with a stile.

### Foundational characters: what a keycap carries

These are what a 2741's keyboard carried. Every struck glyph above is
made from two of them, and none of them is itself struck.

```
'  U+0027      *  U+002A      -  U+002D      .  U+002E
/  U+002F      \  U+005C      _  U+005F      |  U+007C
~  U+007E      ÷  U+00F7      ∆  U+2206      ∇  U+2207
∧  U+2227      ∨  U+2228      ∩  U+2229      ⊤  U+22A4
⊥  U+22A5      ⎕  U+2395      ○  U+25CB
```

Two of them are components only, which surprises people printing
keycaps:

- `∩` U+2229, intersection, is **not** an APL\360 function. Typing it
  alone is a CHARACTER ERROR. It is on the keyboard only so that the
  lamp `⍝` can be struck from it and `○`.
- `_` U+005F, underbar, is likewise a component: it makes `⍙` and
  the underscored alphabet of the next section, and alone it is a
  CHARACTER ERROR.

### The underscored alphabet

A 2741 could strike the underbar over any letter, and APL\360 used
that: A̲ through Z̲ are twenty-six further characters of the set, each
a letter struck with `_`, and each valid in a variable or function
name. They are distinct characters, not decoration -- `X` and `X̲` are
two different names in the same workspace. Delta-underbar `⍙` is the
same idea applied to delta.

Type one the way you type any overstrike: the letter, `Ctrl-]`, then
`_`. Either order forms it, as everywhere else.

```
      X←2
      X̲←3
      X×X̲
6
      )VARS
X X̲
```

They are a rule rather than a table of twenty-six pairs: any letter
struck with the underbar gives that letter underscored. The one table
pair that uses the underbar strikes it over `∆`, which is no letter,
so nothing collides.

#### Two code points, one column

Unicode has no precomposed underscored Latin letter, so sw-apl writes
each as the letter followed by U+0332 COMBINING LOW LINE. That is
what any editor produces and what a file written elsewhere will hold,
so `X̲` pasted in from one is the same name as `X̲` struck at the
keyboard -- U+0332 has no precomposed form, so normalization leaves
it alone.

It prints in one position, as it did on paper, and sw-apl counts it
that way: `)WIDTH` wrapping, `)FNS` and `)VARS` columns, and the
caret under an error all treat the low line as no column of its own.

As character data, though, each underscored letter is two elements
rather than one: `⍴'X̲'` is 2. APL\360 had one character of its 256
there. sw-apl's characters are Unicode scalars and its quoted
literals already accept any of them, so this is that difference
showing rather than a new one.

The low line underscores the letter before it, and needs one:

```
      ̲
CHARACTER ERROR: U+0332 (combining low line, not on a letter)
      ̲
      ^
```

Striking a third character over an underscored letter forms nothing,
which is what three impressions on one position are:

```
      A_B
CHARACTER ERROR: U+0042 struck over U+0332 forms no glyph
```

The transcript shows the three characters side by side, as it does
for any refused strike: the paper never held a glyph to show.

Names use letters, the underscored letters, digits, `∆` and `⍙`; see
`language.md`.

### When a strike forms nothing

A pair that forms no glyph is refused as it is typed, and the line is
reported as a CHARACTER ERROR naming both code points:

```
      QZ
CHARACTER ERROR: U+005A struck over U+0051 forms no glyph
```

The transcript shows `QZ` -- the two characters side by side -- rather
than a struck glyph, because none was formed. The message names the
character struck over the first, then the first.

A character that is a component but not a glyph in its own right, or
a lookalike from the wrong Unicode block, is a different and shorter
report naming just the one code point:

```
      ∩
CHARACTER ERROR: U+2229 (intersection, not APL\360)
      ∩
      ^
```

## Checking your setup

Run sw-apl and type a few glyphs at the six-space prompt. When a
glyph is a lookalike from the wrong Unicode block -- Greek rho
instead of APL rho, for instance -- sw-apl reports CHARACTER ERROR
and names the code point it saw. `glyphs.txt` lists the common
confusions, and the overstrike table above is generated from the same
source the lexer uses, so a pair listed there will always form its
glyph.
