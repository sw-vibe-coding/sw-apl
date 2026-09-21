The service half of the attention step, moved out so the board could
come first (owner, 2026-09-20: "When can I see a proper keyboard?").

The attention step shipped ATTN for the interpreter, the CLI and the
browser demo: a per-session flag in `apl-attn`, polled between lines
and inside every primitive, and in a browser a slot in the shared
channel written by Escape, Ctrl-[ and a board key. What it did not
do is the service, and these are the three parts left, each with the
problem that stopped it:

- **The listening reader, for TCP.** The session thread reads the
  socket only when it wants a line, so an attention sent during a run
  sits unread until the run ends. Give each connection a reader
  thread that owns the read half -- `Socket` already holds separate
  read and write handles -- and routes an attention to the session's
  flag and every other line into a channel the session reads.
- **The protocol.** The terminal sends a JSON string per line, or raw
  text from `nc`, so an attention is a JSON object: `{"attn":true}`.
  It cannot be sent by accident at `nc`, because `{` and `"` are not
  APL. Pin that with a test over real APL lines.
- **`aplterm`.** While it waits for a frame nothing reads the
  keyboard, and a thread watching stdin for Escape must hand stdin
  back to the line editor the moment a prompt arrives, or it steals
  the first keystroke of the next line. That handoff is the design
  problem, not the key.
- **The WebSocket page `sw-apl-server` serves.** `tungstenite`'s
  `WebSocket` is one object for both directions, and splitting it
  across a reader thread and a writer breaks on control frames: a
  Ping read on one side queues its Pong in that side's write buffer.
  Decide how, or ask the owner whether that page still earns its
  keep beside the published demo.

The test the original step asked for belongs here: two sessions on
one server, one sent an attention, the other not stopped.
