Phase 8 step 1 (docs/plan.md). The session, compiled for wasm32 and
driven from JavaScript.

No UI in this step. The deliverable is a crate that builds for
wasm32-unknown-unknown, exposes a session, and is proved to work by
tests -- so that the next step is only a terminal, and any trouble
found there is the terminal's.

`components/web/` is the workspace; it does not exist yet.
`wasm32-unknown-unknown` is installed, and so are trunk and
wasm-pack.

Three seams, two of which already exist:

- `Console` is a trait. The CLI has `Terminal` and `Script`; the
  web build needs its own. See `components/cli/crates/sw-apl/host.rs`.
- `Clock` is a `fn() -> Time` the host installs. `Session::default`
  leaves a stopped clock, which is why a transcript reproduces. The
  web host installs one reading the browser's clock; check whether
  apl-ibeam's chrono dependency builds for wasm32 at all, and if it
  does not, note that the web host does not need `system()` and say
  how it gets a time instead.
- Where a workspace lives is NOT a seam. `apl-library`'s path.rs
  and `apl-commands`' save.rs and load.rs call `std::fs` directly.

That third one is the work. In a browser `std::fs` compiles and then
fails at runtime, so `)LOAD 1 LIFE` would answer WS NOT FOUND and
the shipped workspaces -- the best thing the demo has to show --
would be unreachable.

Put the store behind a trait in `apl-library`, since that crate
exists to answer which file a command means. The CLI keeps a
filesystem implementation and must behave exactly as it does today:
the reg-rs suite is the check, and it should pass untouched. The web
build gets one serving library 1 from memory, with the shipped
workspaces compiled in -- they are small, tracked, and ours, so
`include_str!` is honest here. Library 0 is the user's; decide
whether it is memory only for now or browser storage, and say why.

Then decide what a read does. `Console::read` is synchronous and a
browser cannot block in the middle of a statement. Batch mode
answers INTERRUPT at end of input and that is already specified
behaviour, so it is available -- but a demo where `⎕` always
interrupts is a demo that cannot run sample 57. Consider instead
whether the host can be given the lines in advance, as `Script`
does. Whatever is chosen, write it in the docs as a difference
between the two hosts rather than leaving it to be met.

`)OFF` has nowhere to go in a browser. Decide what the session does
with `Reply.off` there.

Tests: the wasm crate needs its own, run with wasm-pack or
wasm-bindgen-test, and the CI story for them stated even if not
wired up. Keep the CLI's suite green throughout -- it is the control.

Do not change the interpreter to suit the browser. If something
seems to require it, stop and say so.

Update docs/architecture.md with the new workspace and the seam,
and docs/parity.md if any row's answer differs between hosts.
