# Standalone aplterm prototype

Two Rust binary targets, using the existing interpreter and overstrike table
as library dependencies. All prototype sources live in this directory.
The keyboard, editor, and transport are small library crates in this workspace.
The existing interpreter, CLI, and build scripts are unchanged; the project plan
and saga record track this prototype as a separate step.

From the repository root, build both binaries:

```bash
cargo build --release --manifest-path experimental/terminal2741/Cargo.toml --bins
```

Terminal 1 -- start the server:

```bash
./target/release/aplterm-server --listen 127.0.0.1:2741
```

Terminal 2 -- connect the terminal:

```bash
./target/release/aplterm --connect 127.0.0.1:2741
```

These can also be launched with `cargo run --release --manifest-path
experimental/terminal2741/Cargo.toml --bin aplterm-server` or `--bin aplterm`.
They are not installed over any existing command.

## Keyboard

The default assumes a US-layout terminal: lowercase ASCII letters are the
unshifted keys, uppercase ASCII letters and number-row punctuation are the
shifted keys. Turn Caps Lock off. An ordinary terminal supplies characters,
not physical keyboard scan codes, so it cannot distinguish Caps Lock from Shift.

- Unshifted letters echo uppercase.
- Shift-L enters quad; Shift-K enters quote.
- The US minus key enters `+`; the equals key enters multiplication.
  Shift-minus enters `-`; Shift-equals enters division.
- The US semicolon and apostrophe keys enter left and right square brackets.
  Shift-semicolon enters `(`; Shift-apostrophe enters `)`.
  Shift-apostrophe then unshifted `off` and Enter signs off.
- Shift-L, Ctrl-], Shift-K immediately composes quote-quad.
- Letter A, Ctrl-], Shift-F immediately forms underscored A.
- Shift-O, Ctrl-], Shift-P forms logarithm (circle over star).
- Ctrl-] holds the preceding cell for overstrike. Repeated presses are harmless.
- Invalid overstrikes ring the bell and retain the base; retry the second key
  or press Backspace to cancel the pending strike.
- Backspace deletes a whole glyph, including an underscored letter.
- Arrow keys, Home, End, Delete, and in-memory Up/Down history work.
- Ctrl-C or Ctrl-U clears the current input. Ctrl-D on empty input disconnects.
- F2 toggles literal Unicode input (no key translation).
- Escape quotes the next key literally, e.g. Escape then `+` enters addition.
- F4 inserts `)` directly; F4 then unshifted `off` and Enter signs off.
- Bracketed paste inserts Unicode literally, without translating letters.
  Newlines and other control characters are discarded; paste one APL line at a time.

The editor scrolls horizontally for long input lines and prints the whole line
when submitted. Underscored capitals appear as circled capitals in both input
and output; the wire uses capital plus U+0332 COMBINING LOW LINE, which is the
existing interpreter's representation. Font support determines the appearance.

`keymap.json` contains the actual Unicode glyphs and is the complete default map.
Uppercase keys are shifted letter keys. Unmapped characters pass through, with
ASCII lowercase converted to uppercase. Shift-C supplies intersection for lamp
composition; Shift-V supplies union, which the interpreter may reject alone.
Shifted brackets supply arrows; unshifted brackets remain available for indexing.
Shifted number-row keys map 2 through 0 to high minus, less, less-or-equal,
equal, greater-or-equal, greater, not-equal, or, and and. The two keys to the
right of zero enter plus and multiply unshifted, minus and divide shifted.
Overstrikes using a minus stroke (first-axis reverse, reduce, and scan) therefore
end with Shift-minus. The underbar for underscored letters remains Shift-F.
Use Escape or F2 for literal punctuation on these keys.

Edit a copy to match your keycaps, then load it explicitly (no rebuild required):

```bash
./target/release/aplterm --keymap experimental/terminal2741/keymap.json
```

For a keyboard already emitting APL Unicode, use `--literal`.

## Server and protocol

The server defaults to localhost TCP port 2741 and the checkout root as its
library directory. Override the latter with `--library PATH`. Each connection
has its own workspace; numbered libraries and saves use the selected filesystem
directory. There are at most 16 concurrent sessions. Stop the server with Ctrl-C.

This is a TCP socket protocol, without Telnet negotiation. Client-to-server
messages are JSON strings containing fully composed Unicode APL input lines,
followed by newline. Server messages are newline-delimited JSON objects with
`lines`, `prompt`, and `off`. Framing preserves partial quote-quad prompts and
input requested from inside an executing function. Control-character input is
rejected, and protocol lines are bounded to 1 MiB.

Prototype limit: Ctrl-C clears client input, but remote interruption of running
APL code is not implemented. A nonterminating computation requires stopping the
server. Disconnection while waiting for ordinary or quad input releases the session.
This local test server has no authentication or encryption.

## Verification

```bash
cargo test --manifest-path experimental/terminal2741/Cargo.toml --workspace
cargo clippy --manifest-path experimental/terminal2741/Cargo.toml --workspace --all-targets -- -D warnings
```

The Rust tests cover key composition, editing, blocking quad/quote-quad input,
definitions, underscored identifiers, and disconnects. A Rust PTY test drives the
actual client and checks display before Enter and exact Unicode sent over TCP.
