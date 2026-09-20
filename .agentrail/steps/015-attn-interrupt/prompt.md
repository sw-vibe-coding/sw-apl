Phase 8 step 4 (docs/plan.md, owner direction 2026-09-19 fourth).
Escape is ATTN.

The 2741 had an attention key that stopped a running statement. The
owner has chosen Escape for it, which is Ctrl-[ -- the same byte,
exactly as Ctrl-H is backspace. Escape already quotes the next key
in aplterm and the two never collide: while a line is being typed
the terminal is reading keys and Escape quotes; while the service
is working the terminal is waiting for a frame and Escape is ATTN.
The mode decides. Do not rebind anything, and check that claim
rather than assuming it.

Three pieces:

- The interrupt flag. apl-call keeps it as one process-global
  static STOP, set by the CLI signal handler. A server holding
  sixteen sessions would interrupt all of them. Make it
  per-session state, with the CLI setting its own session's like
  any other client. This is below apl-serve, and the owner has
  asked for it in docs/plan.md, so it is in scope -- but keep the
  change as small as it can be and say in the commit what moved.
- The protocol. The terminal needs to say something unprompted,
  and a typed line must never be mistaken for it. Decide the
  shape, keeping nc usable: a reader typing APL must not be able
  to send an attention by accident.
- The listening connection. The session thread is inside eval when
  the attention arrives, not reading the socket, so the connection
  needs a reader that is listening then, feeding typed lines to the
  session over a channel. This is the channel-fed read the service
  was always described as having; it arrives now because this is
  what needs it.

Both clients: Escape in aplterm while it waits for a frame, and
Escape in the browser page. In a browser, say what happens to the
copy shortcut if Ctrl-[ is taken as well, and prefer not breaking it.

Tests: an attention that stops a long-running statement, and an
attention that reaches one session and not another on the same
server. The second is the one that pins why the flag moved. Also
pin that a line of APL typed at nc cannot be an attention.

The CLI is the control: reg-rs stays green, and Ctrl-C at the CLI
keeps doing what it does today.
