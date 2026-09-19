Phase 8 step 1 (docs/plan.md). The service.

`sw-apl-server`: a local process holding one `Session` per
connection, reading lines from a browser and writing the transcript
back. No terminal fidelity in this step -- a plain HTML page good
enough to type a line and see the answer is the right amount of
client. The next step makes it a 2741.

The point of the step is the blocking read. `Console::read` is
synchronous and returns the next line a statement asks for; on a
server that can block on a channel fed by the connection, so `⎕`,
`⍞` and every line of the del editor work as they do in the CLI.
Get that right and the rest of the demo is presentation.

Shape to settle, and to say why in the commit:

- The transport. A websocket per session is the obvious answer;
  check it is, and say what happens when the socket closes with a
  session mid-statement.
- The protocol. `Session::respond` takes a line and answers
  `Reply { lines, open, off }`, and `Session::prompt` says what to
  prompt with. That is close to a wire format already. Keep it that
  shape rather than inventing a richer one: the terminal needs the
  lines, whether the last is open, and the prompt.
- `Reply.off`. `)OFF` in a browser has nowhere to go. Decide, and
  note that `)CONTINUE` saves first, so it must still save.
- Threads or async. One session per connection blocking on a
  channel is the simplest thing that works; if that means a thread
  per connection, say so and say what bounds it.

Where it lives: `components/web/` is a new cargo workspace, as
every component is. The server crate depends on `apl-session` and
nothing the CLI owns -- if it wants something from
`components/cli/crates/sw-apl/host.rs`, that is a sign the piece
belongs lower down.

The clock: `Session::attached` installs the real one. Use it, so
`⌶20` answers.

Libraries: the server has a filesystem, so `)LOAD 1 LIFE` should
work from a checkout. Decide what `--library` means for the server
and whether library 0 should be somewhere other than the
repository's `work/`, which is the owner's.

Tests: the session layer is already covered, so what is new here is
the protocol and the blocking read. A test that drives a session
through the protocol and reads a `⎕` is the one that matters. Do
not test by hand only.

`just demo` starts the server and opens a browser. Add it, and a
line in the README saying what it is and that nothing leaves the
machine.
