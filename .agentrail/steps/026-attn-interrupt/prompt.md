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

---

Findings from the owner's questions, 2026-09-20, before this step
was started. Checked against the code, not assumed.

**There is no tight-loop case to test with.** The corpus tops out
at `+/⍳1000000`, which runs in ten milliseconds because iota is
lazy. Write one first: a defined function with `→` back to line 1,
which is where an APL\360 loop lives. It is the fixture the whole
step needs and nothing in `samples/` can stand in for it.

**The render thread is safe by construction.** The interpreter is
on a Web Worker and the page's own thread only draws, so a tight
loop cannot block rendering, and the keystroke handler that has to
notice ATTN stays live however long the loop runs. Nothing is
needed here; do not add anything.

**The CPU is not tuned and this step is not where to fix it.** The
worker spins one core at 100% for the duration, with no yielding
and no budget -- heat and battery on a phone. Worth its own step;
note it and leave it.

**ATTN cannot arrive by `postMessage`.** A worker services
`onmessage` only when its thread returns to the event loop, and a
synchronous eval never does. So the page cannot tell the worker to
stop by messaging it, and the listening-reader design in this
prompt -- which is right for the service, where another thread can
watch the socket -- does not carry over. In the browser the flag
must go through the `SharedArrayBuffer` the channel already uses,
with the interpreter polling it. Decide the slot and say so.

**Polling granularity is a decision, not a detail.** `interrupted()`
is read in exactly one place, `run_body`, between the lines of a
defined function; `branch.rs` says a statement that has not finished
a line "cannot yet be stopped". So `→` loops are interruptible and
one enormous primitive is not, at the CLI as well as in the browser.
Either accept that and say so in the docs, or push a check into the
primitive loops and pay for it there. Do not leave it unstated.

**The board needs an ATTN key** (owner direction, 2026-09-20). A
touch screen has no Escape key at all, so without one on the board
a reader on a phone can start a loop and have no way to stop it.
That is the worst state this page can be in, and it is reachable
today. The key goes on the board with the other controls, and it
must work while the session is busy -- which is the same shared-
memory path as the physical key, not a tap the worker has to be
free to receive.

Escape is the physical key, as the owner chose. `Ctrl-[` is the
same byte and should work too where a browser reports it
distinguishably. Say in the docs which keys are ATTN, on the page
under Help and in `terminal.md`, and say what a reader does on a
touch screen. The board's key needs a name a screen reader can
read, like every other key on it.

**Owner decision, 2026-09-20: the tight loop should be loose.**
Asked to choose between polling only between lines (A) and pushing
a check into the primitive loops (B), the owner chose B and gave
the reason: this is not time-sensitive production code, and a loop
that cannot be stopped is worse than a loop that runs slower. So
poll inside the loops.

Two things that decision covers, and they are not the same thing.
Do not conflate them, and do not let the second delay the first.

*Stopping.* ATTN works only if the running code polls a flag. The
interpreter must read it inside the primitive loops -- reduce,
scan, inner and outer products are where the time goes -- as well
as between the lines of a body. An atomic read every few thousand
iterations costs nothing measurable; find the granularity, say what
was chosen, and keep it in one place rather than spread through the
primitives.

*Power.* The worker pegs a core for the duration. That is real on a
phone, and it is a separate matter: the page's own thread is never
blocked, so key events, taps and drawing all happen already no
matter how long the loop runs. Yielding buys battery, not
responsiveness. If a periodic yield is cheap, take it; if it costs
throughput, leave it and raise it as its own step. Do not claim it
makes the page responsive -- the page already is.

One tension to resolve rather than discover: the flag is to become
per-session, and the primitives do not obviously have a session to
read it from. Decide how a primitive reaches it -- a handle passed
down, or a thread-local the session installs -- before writing the
polling, because that choice is most of the work.

A candidate for that tension, found by looking: the primitives take
no session -- `reduce(f: char, r: &Array, k: usize)` and its
neighbours take arrays and nothing else -- so reaching a per-session
flag through their arguments means changing every signature in
`apl-prims-*`, which is most of the crate for a flag.

A thread-local holding a *handle* to the flag avoids that. Each
session already owns a thread: one per connection at the service,
the worker in the browser, the process at the CLI. The session
installs the handle on its own thread when it starts, and
`interrupted()` reads it from there, so it is per-session by
construction and no signature moves. The handle is what differs --
an `Arc<AtomicBool>` natively, a view into the shared channel on
wasm, which is the only thing the page can write to while the
worker is busy. Check this before adopting it; it is a suggestion
from reading the signatures, not a decision.
