# The 2741 and the service

sw-apl runs as a local service that terminals dial into. This is the
arrangement APL\360 had: a typewriter on one end of a line, a
time-sharing system on the other. It is also what makes the session
work, because a statement that reads -- `⎕`, `⍞`, every line of the
del editor -- stops until a line arrives, and the thread holding the
session may stop.

Two terminals come with it: `aplterm`, an IBM 2741 in a terminal
window, and a page in a browser. Both speak one protocol.

Everything runs on your own machine. Both listeners bind to the
loopback address unless told otherwise, and nothing typed at either
terminal goes anywhere else.

## Starting it

```bash
just demo
```

builds the binaries, starts the service, and opens the browser
terminal at `http://127.0.0.1:8360/`. Library 0 is `target/demo/work`,
so a session saves into scratch rather than into the checkout's
`work/`, and `ws/` is linked through so `)LOAD 1 LIFE` finds the
shipped workspaces.

By hand, and with a 2741 instead:

```bash
just release
target/release/sw-apl-server --library .
target/release/aplterm --connect 127.0.0.1:2741
```

`sw-apl-server --help` describes both listeners, `--library`,
`--ws-size` and `--sessions`. `sw-apl` with no service, holding its
session in process, is unchanged and is still the daily interpreter.

## The keyboard

Base, overstrike key, overstrike is how an APL glyph is typed, as it
was on a 2741: `⍟` is `○`, the key, `*`. The keys themselves assume a
US-layout keyboard with Caps Lock off, because an ordinary terminal
receives characters rather than scan codes and cannot tell Caps Lock
from Shift.

- Unshifted letters echo uppercase.
- Shift-L enters quad; Shift-K enters quote.
- The minus key enters `+` and the equals key enters multiplication;
  shifted, they enter `-` and division. Overstrikes made with a minus
  stroke -- first-axis reverse, reduce, scan -- therefore end with
  Shift-minus.
- The semicolon and apostrophe keys enter `[` and `]`; shifted, `(`
  and `)`. Shift-apostrophe then `off` and Enter signs off, and F4
  types `)` as well. The brackets are on the home row because that is
  where the 2741 put them -- it has two keys right of `L` and only
  one right of `P`, where a US keyboard has two and two.
- That one key right of `P`, which a US keyboard labels `[`, carries
  both arrows: `→` unshifted and `←` shifted.
- Shift-A, Shift-W, Shift-Z and Shift-X enter `⍺`, `⍵`, `⊂` and `⊃`,
  and Shift-1 enters `¨`. They are on the typeball and mean nothing;
  see below.
- Shifted number-row keys 2 through 0 enter high minus, less,
  less-or-equal, equal, greater-or-equal, greater, not-equal, or, and
  and.
- Shift-C enters intersection, which the lamp `⍝` is struck from.
  Shift-V enters union.
- Shift-F is the underbar: `A`, the overstrike key, Shift-F is
  underscored `A`.

### Four keys with nothing behind them

The typeball carries `⍺` on A, `⍵` on W, `⊂` on Z and `⊃` on X, and
none of the four means anything in APL\360. They are not functions:
`⍺` and `⍵` became the arguments of a direct definition in Dyalog, and
`⊂` and `⊃` became enclose and disclose in APL2, all of them years
later. They are not letters either -- a name is built from `A` to `Z`,
the underscored alphabet, `∆`, `⍙` and digits, and these are not in
that alphabet. And unlike `∩`, which exists so that the lamp `⍝` can
be struck from it and `○`, none of the four is a part of any
overstrike.

What is left is the one thing every character of the set can do: be
character data.

```apl
      ⍴'⍺⍵⊂⊃'
4
      'THE ⊂ KEY'
THE ⊂ KEY
```

That is their whole use, and it is a real one -- APL\360's character
set is what a quoted vector may hold, and what `⍞` may print.

Typing one bare is an error, as it should be, because there is no
function there to apply. sw-apl's message for it is wrong today: it
says `not APL\360`, which is false of a glyph the 2741 printed.
Correcting that is the same work as correcting it for `∩` and `∪`,
which `docs/plan.md` already has as a step; these four join it.

`¨`, on the shifted `1` key, is in the same position.

Ctrl-] is the overstrike key. Backspace cannot be it -- a line editor
needs backspace for deleting -- and neither can Ctrl-H, which *is*
the backspace byte.

- Ctrl-] holds the cell the carriage just passed. Pressing it twice
  is harmless.
- A pair that forms no glyph rings the bell and keeps the base: press
  the second key again, or Backspace to cancel the strike.
- Backspace deletes a whole glyph, an underscored letter included.
- Arrow keys, Home, End and Delete edit the line; Up and Down recall
  earlier lines.
- Ctrl-C and Ctrl-U clear the line. Ctrl-D on an empty line hangs up.
- Esc quotes the next key: Esc then `+` enters addition.
- F2 takes keys as the Unicode they already are, for a keyboard that
  sends APL glyphs itself. `--literal` starts that way.
- Paste is literal Unicode, one line at a time; control characters in
  it are dropped.

Long lines scroll sideways while being typed and are printed whole
when sent. Underscored capitals are shown as circled capitals at both
terminals, because most fonts draw a combining low line badly; the
wire carries the capital and U+0332, which is what the interpreter
reads and writes.

`keymap.json`, beside the `apl-keyboard` crate, is the map itself and
is the complete default. Copy it, edit it to match your keycaps, and
pass it:

```bash
target/release/aplterm --keymap my-keymap.json
```

The browser terminal takes APL glyphs typed or pasted directly. Its
keyboard and overstrikes are not implemented yet.

## A browser on its own

`pages/` is the same session with no service at all: the interpreter
compiled to WebAssembly and running in the tab.

```bash
just pages-serve
```

builds it and serves it at `http://127.0.0.1:8361/`, the way a static
host would. It needs the wasm toolchain:

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

The blocking read works here too, and that is the whole trick. The
session runs on a Web Worker and `Link::recv` parks that thread in
`Atomics.wait` until the page puts a line in a `SharedArrayBuffer`.
The page's own thread is never blocked -- it is not allowed to be --
and `Console::read` stays synchronous, so `⎕`, `⍞` and the del editor
read as they do everywhere else.

The page, the worker and the WebAssembly are one build served as
three files, and a browser caches them separately. So only
`index.html` is fetched fresh -- it is the navigation -- and it asks
for `version.txt` uncached, which the build writes as a digest of
what a visitor actually runs. Everything below is fetched at that
version: `apl.js?v=`, then `worker.js?v=`, then the bundle. A cached
worker cannot meet a newer page, and a rebuild that changed nothing
leaves a visitor's cache standing.

Two things guard what the stamp cannot. `start` takes the whole
message the page posted and pulls the channel and library 0 out of
it, rather than taking them as separate arguments, so a worker from
an older build still makes a call it understands. And a session that
has not announced itself within ten seconds is reported on the paper
rather than left as a prompt that ignores typing: a reader cannot
tell that from a slow load, and a plain reload does not replace a
worker a browser has already cached.

`just check-pages` is that check, in a browser: a first visit, a
visit holding the worker from an older bundle, and a worker that
never answers.

`SharedArrayBuffer` needs the page to be cross-origin isolated, which
means two headers:

    Cross-Origin-Opener-Policy: same-origin
    Cross-Origin-Embedder-Policy: require-corp

`sw-apl-server` sends them with the terminal page and
`just pages-serve` sends them with everything, so a local visit is
isolated on the first response, loads once, and never registers a
service worker at all.

A static host will not send them, and that is what `sw.js` is for:
the published page installs a worker that adds them to its own
responses and then loads once more under it, so the first visit there
loads twice and later ones do not. It is the fallback, not the path.
A browser that refuses service workers -- a private window does --
gets one line saying there is no shared memory and pointing at Help,
which carries the reason and what to do.

`just check-pages` serves without the headers on purpose, to keep the
service-worker path covered, and has one case that serves with them
and asserts the page is isolated on a single load.

The keyboard is the 2741's here too, and it is the same keyboard:
`apl-keyboard` compiled to WebAssembly, holding the keymap and the
overstrike table that `aplterm` uses. The page sends it keystrokes
and draws what it hands back, so `⍟` is `○`, `Ctrl-]`, `*` in a
browser exactly as it is in a terminal, and there is no second copy
of the table in JavaScript.

The demo is published by `.github/workflows/static.yml`, which
builds nothing: it uploads the committed `pages/` as the site. So
what is served is byte for byte what is in the commit
`build-info.json` names. "Deploy from a branch" cannot do this --
it offers the repository root or `/docs` and nothing else -- which
is why the Pages source is GitHub Actions.

The demo installs. `manifest.json` names it, asks for a standalone
window, and carries the icons; `start_url` and `scope` are relative,
so it installs from a project page's sub-path as readily as from a
root. The favicon is `∩⊃⌊` turned 33 degrees: three APL glyphs that,
turned, read as the letters APL. Bright green on nothing, because a
favicon with a background is a square of somebody else's colour in a
strip of tabs. The install icons are the project logo the README
shows, scaled down, with a maskable copy inside the safe zone for a
launcher that crops to a circle.
`scripts/gen-icons.sh` makes all of them and they are tracked, like
the rest of `pages/`.

Installed, it still needs the network for the first load: offline is
`offline-shell`'s, and it is last because it is the part that can
break the version stamp.

Below the session is a colophon in the form the other live demos
use: one line of middot-separated items -- licence, copyright, the
repository, the Software Wrighter channels, and the build -- wrapping
on a phone, where a PWA window is a phone. The build facts come from
`build-info.json`, which `just publish` writes, so there is nowhere
else for them to go stale. It is below the fold and scrolled
to: the session owns the window exactly, because the bar under the
paper has been cut twice for taking room the transcript wanted.

An OS-level expander -- Espanso, or the Emacs input method -- works
here as it does anywhere else, and needs nothing from sw-apl. It
sees the keystrokes before the browser does, backtick is not a key
the 2741 map claims, and a glyph it sends back arrives as itself
whether it comes as a keystroke or as a paste.

A reader with no APL keycaps, or no keyboard at all, taps the board
on the page instead. **Keyboard** shows it and **ABC** switches it
between the glyph each key carries and the letter or digit it is
painted with; it remembers whether it was up, and it is reachable by
tab and named for a screen reader out of `data/glyphs.toml`.

The layout is the IBM 2741's, and the drawing of it that sw-apl
redistributes is somebody else's work under a share-alike licence.
A link beside the board credits it and goes to the picture, its
LICENSE and its ATTRIBUTION together; it is shown exactly when the
board is, which puts the obligation next to the thing it is about
rather than in a footer nobody scrolls to.

It sits at the bottom, or at the top if a reader moves it there, and
its keys are as big as they were last set. Both are remembered, and
the controls are on the board itself rather than in the bar under
the paper. On a narrow window the keys shrink with the width until
they hit a floor; below that the board keeps its size and the
transcript gives up the room, because a keyboard too small to hit is
no use and a shorter transcript still scrolls. A reader who wants
the paper back puts the board away.

It is drawn from `keymap.json` -- the same file `apl-keyboard`
compiles in, copied into the bundle by `just pages` -- so a key sends
here what the same key sends at a terminal, and there is no second
copy of the map. A tap goes through the same `Board` a keystroke
does, so an overstrike begun by tapping can be finished by typing.
Where a 2741 put a glyph is where the board has it: `)` is the quote
key, as it was on the machine.

The board is ours. The redistributed keyboard picture is reference
only -- an Inkscape drawing whose keys are not addressable -- and is
shown under **Help** with its author and licence beside it.

Two differences from the terminal, both because a browser is not one:

- Hold `Alt` for the character a key is painted with, where the
  terminal uses `Esc`. A browser reports `Alt` on the keystroke
  itself, so there is nothing to quote ahead of.
- Underscored capitals are shown as they are, not as circled
  capitals. The circle is a workaround for terminal fonts that draw a
  combining low line badly, and a browser with an APL font does not
  need it.

Only the chords sw-apl claims are taken from the browser: `Ctrl-]`
overstrikes and `Ctrl-C` clears the line. Copy, paste and reload are
left alone.

The libraries work here with no filesystem under them. Library 1 is
baked into the bundle, so `)LIB 1` lists LIFE, RACE and EDIT and
`)LOAD 1 RACE` loads one. `)SAVE` writes library 0 into the
browser's local storage, and `)LIB`, `)LOAD NAME` and `)DROP` read
it back on a later visit.

What a browser keeps is that browser's. A workspace saved in one is
not in another, not on another machine, and not on any server:
nothing leaves the tab. A browser with nowhere to keep it -- storage
full, or refused, as a private window refuses it -- still saves and
loads for as long as the tab is open, and the page says once that it
will not last past it.

The session's thread cannot reach local storage: only the page can,
and the page's own thread is the one that must not block. So the
workspaces are held in memory on the worker and the page is told
whenever library 0 changes; it reads what it kept before the session
starts and hands it over with the channel.

What does not work yet: a touch screen has no keyboard to intercept,
so there is nothing to type with until the board itself is
clickable.

The WebAssembly is about 390 KB. `pages/wasm/` is build output and is
not tracked; the page, the worker and the service worker beside it
are.

## The protocol

One line each way, UTF-8.

The service sends one JSON object per line: `lines`, the finished
transcript lines; `prompt`, what to print before the typing; and
`off`. The prompt is the six-space indent, or the `[n]` of the del
editor, or the line `⍞←` left open -- the carriage stopped on that
line, so that is where the typing goes. A `prompt` of `null` with
`off` set is the last frame of a session.

The terminal sends the typed line as a JSON string. A line that is
not JSON is taken verbatim, so `nc` is an emergency client and a
debugging window:

```bash
nc 127.0.0.1 2741
```

Type APL and read the transcript. A bare `nc` gets no overstrike
composition, because composition belongs to the terminal: only the
terminal sees the keystrokes, and what travels is the composed line.

There is no Telnet negotiation. Protocol lines are bounded, and a
line carrying a control character ends the session -- a 2741 has no
key for one, so it is a client that has lost its framing.

## Sessions

One session per connection, each with its own workspace, each on its
own thread. `--sessions` bounds how many may be held at once; a
connection arriving when they are all held is closed rather than
queued.

A connection that drops releases its session and its workspace. It is
not `)OFF`: a statement waiting for a line gets none, which is
reported as `INTERRUPT` and leaves the function suspended, and then
the session ends. Holding a workspace for a reconnection is what
APL\360 had sign-on numbers for, and those are not implemented.
`)CONTINUE` saves before it ends, so it still saves.

Interrupting a running statement from the terminal is not
implemented. A statement that will not stop needs the service
stopped.
