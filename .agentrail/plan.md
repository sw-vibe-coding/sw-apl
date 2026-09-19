# terminal

Phase 8 of docs/plan.md, owner direction 2026-09-19: the browser
runs a 2741 terminal and the interpreter runs in a local server.

This is the architecture APL\360 had. A typewriter terminal talked
to a time-sharing service; the self-contained browser interpreter
that was planned here was the anachronism, and it could not have
done the del editor. `Console::read` is synchronous and a browser
cannot stop mid-statement to wait for a keystroke. A server can.

What each side owns:

- The terminal owns the keyboard and the paper. Overstruck
  characters are formed there, by backspace, because only the
  terminal sees the keystrokes; `⍟` is `○` backspace `*` and the
  server never learns it was typed that way. So are the six-space
  indent, the `[n]` prompt, and a line `⍞←` left open.
- The server owns the session. One per connection, holding a
  `Session`, reading lines from the connection and writing the
  transcript back. `)SAVE`, `)LOAD` and `ws/lib1/` work because it
  has a filesystem.

The interpreter does not change. If a step finds itself altering
`Session` or anything below it to suit a browser, that is the
signal to stop and ask. The CLI is the control: its reg-rs suite
must stay green throughout, and the same input typed at the
terminal and at the CLI must produce the same transcript.

Local only. `just demo` starts the server and opens the terminal.
No always-on service, no accounts, no ops, nothing sent anywhere
but to a process on the reader's own machine -- and the page should
say so, since a terminal talking to a server looks like one that
might not.

Every step: format first, then tests, clippy, and gates (see
/mw-cp); TDD; reg-rs for anything run through the CLI binary;
commit, push, report.

## Steps

1. terminal-server -- the service, the protocol, and a blocking
   read that works.
2. terminal-2741 -- the paper and the overstrikes.
3. glyph-keyboard -- the keyboard, the expansions, and a first
   screen worth arriving at.
