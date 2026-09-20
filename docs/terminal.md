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
  types `)` as well.
- Shifted number-row keys 2 through 0 enter high minus, less,
  less-or-equal, equal, greater-or-equal, greater, not-equal, or, and
  and.
- Shift-C enters intersection, which the lamp `⍝` is struck from.
  Shift-V enters union.
- Shift-F is the underbar: `A`, the overstrike key, Shift-F is
  underscored `A`.

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

`SharedArrayBuffer` needs the page to be cross-origin isolated, which
means two headers a static host will not send. The page installs a
service worker that adds them to its own responses and then loads
once more under it, so the first visit loads twice and later ones do
not. A browser that refuses service workers gets a page that says so
rather than a broken prompt.

What does not work yet: `)SAVE` and `)LOAD` have no filesystem under
them and say so, and the keyboard is whatever your own sends --
overstrikes and the 2741 layout are not there yet. Type or paste APL
glyphs directly.

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
