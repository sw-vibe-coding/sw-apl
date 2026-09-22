Phase 8 step 3 (docs/plan.md, owner direction 2026-09-19 fourth).
The interpreter in the browser, and a local pages bundle.

GitHub Pages is static, so the public demo has no service to dial:
the interpreter has to run in the browser. What makes that possible
is that the blocking read now has somewhere to block. Run
apl-serve in a Web Worker and implement Link a third time over a
SharedArrayBuffer with Atomics.wait, so recv parks the worker until
the page posts a line. Console::read stays synchronous and Session
is not touched. If this step finds itself altering Session or
anything below it, stop and ask.

Build pages/ and serve it locally. The owner wants to try it before
GitHub Pages is configured at all, so a plain static file server
over the built directory is the deliverable, with a just recipe
that builds and serves it and says the URL.

Settle, and say why in the commit:

- Cross-origin isolation. SharedArrayBuffer needs COOP and COEP,
  which Pages will not set. coi-serviceworker is the usual answer;
  check it is, say what it costs on a first visit, and say what
  happens in a browser that refuses service workers.
- The wasm toolchain. wasm32-unknown-unknown plus whatever bundles
  it. Say what a reader has to install, and keep just test and the
  existing gates working without it.
- What does not work yet. SAVE and LOAD have no filesystem under
  them in the browser; they report that, and the next step gives
  them one. Say exactly what the reader sees.
- Size. Say how big the bundle is, since a demo nobody waits for is
  not a demo.

The client is the plain HTML page the service already ships, moved
or copied into the bundle and pointed at the worker instead of a
WebSocket. No 2741 fidelity here; that is step 6.

Tests: the worker and the blocking read are what is new. A headless
browser test is the honest check, and if that costs too much, say
so and pin what can be pinned natively: the SharedArrayBuffer Link
against a scripted far end, with the same protocol tests the socket
Link already passes.

The CLI is the control: reg-rs stays green, and sw-apl is untouched.
