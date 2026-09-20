# sw-apl Master Plan

This is the single source of planning truth for sw-apl. Every
instruction from the project owner lands here first; agentrail
sagas and steps are derived from the phases below, never invented
ad hoc. When direction changes, edit this file, then re-plan the
saga (`agentrail plan --update`, `agentrail insert`).

Companion docs: `parity.md` (the definition of done: every
APL\360 feature with its status), `prd.md` (what and why), `language.md` (the APL
subset), `session.md` (terminal look and feel, system commands),
`architecture.md` (crate layout), `design.md` (decisions),
`testing.md` (TDD + reg-rs), `glyph-entry.md` (typing glyphs).

## Goal

A clean-room, from-scratch implementation in Rust of pure IBM
APL\360 as it looked on a terminal: its primitives, its session
conventions, its I-beam system functions, and its `)ORIGIN`,
`)DIGITS`, `)WIDTH` settings commands. Quad and quote-quad are the
I/O forms; there are no quad-named system variables or functions
(those came with APLSV). Traditional glyphs are the only surface
syntax: Unicode input, no Latin keyword aliases, no name-to-glyph
translation layer.

Two delivery surfaces, in order:

1. `sw-apl`, a CLI interpreter for macOS and Linux (this plan).
2. A Yew/WASM interactive live demo in the browser (later phase,
   same interpreter crates compiled to wasm32).

## Non-goals (for now)

- Not APL2 and not Dyalog: no nested arrays, no each operator, no
  enclose/disclose/pick, no dfns, no union/intersection, no dyadic
  tilde ("without"), no diamond statement separator.
- Not a port of `sw-cor24-apl` (C) or of GNU APL. Those are
  references for behaviour and for the conformance corpus only.
- Not APLSV either: no quad system variables or functions
  (quad-IO, quad-CT, quad-EX, quad-NL, ...), no execute, no
  format. Their APL\360 counterparts are I-beams, `)ORIGIN`,
  `)DIGITS`, `)WIDTH`, `)ERASE`, `)FNS`, `)VARS`.
- No embedded targets, no shared variables for hardware I/O.
- No file system primitives beyond workspace save/load.

## Guiding constraints

- Rust edition 2024, one cargo workspace per `components/<name>/`,
  shared `target/` via `.cargo/config.toml`.
- `sw-checklist` metrics are design gates, not afterthoughts:
  design new code to <= 25 LOC per function, <= 4 functions per
  module, <= 4 modules per crate.
- TDD (red, green, refactor) on every step. Unit tests live next
  to the crate; end-to-end transcripts are `reg-rs` regression
  tests driven by the `samples/` corpus.
- `README.md` and every other top-level `.md` (CLAUDE.md,
  CHANGES.md, samples/README.md) are ASCII-only so GitHub renders
  them predictably; `sw-markdown-checker` gates them. `docs/*.md`
  may contain APL glyphs (UTF-8); README links to those docs
  rather than showing glyphs itself. `docs/glyphs.txt` stays the
  machine-readable source for the lexer's glyph table.
- Floating point is first-class: numbers are one semantic type
  with an integer fast path and tolerant comparison using the
  fixed APL\360 fuzz (1E-13 relative).

## Phases

Each phase becomes one agentrail saga. Step slugs are stable
identifiers; prompts are written when the saga is planned.

### Phase 0: bootstrap (saga `bootstrap`)

1. `repo-scaffold` -- COPYRIGHT, LICENSE, .gitignore,
   .gitattributes, .cargo/config.toml, justfile, scripts,
   `/mw-cp` command, CLAUDE.md project notes, README skeleton,
   samples corpus copied from sw-cor24-apl (APL\360-scope only).
2. `planning-docs` -- this plan plus prd, architecture, design,
   language, session, glyphs, testing, glyph-entry, saga docs.
3. `cli-skeleton` -- `components/cli` workspace with the `sw-apl`
   binary: `--help`, `--version`, `-f FILE`, stdin batch mode, a
   REPL loop with the six-space indent prompt that reports
   "not implemented" for every line; reg-rs harness scripts and
   the first baseline; README build instructions verified.

### Phase 1: MVP REPL, then foundations (saga `core-mvp`)

The first two steps are a deliberately thin vertical slice so the
owner can evaluate simple APL at the six-space prompt (with the
Espanso or Emacs keymaps) as early as possible. The later steps
widen each layer to the full APL\360 set.

1. `mvp-scalar-arithmetic` -- create `components/value`
   (`apl-value`: Array, Number Int/Float, Char, errors),
   `components/lex` (`apl-lex`: numbers with high minus, strands,
   names, the glyphs plus minus times divide upstile downstile
   stile star left-arrow lamp, parentheses), `components/parse`
   (`apl-parse`: right-to-left, monadic/dyadic, strands,
   assignment, parentheses), `components/eval` (`apl-eval`:
   scalar extension, variables), `components/display`
   (`apl-display`: integers, floats to quad-PP, high minus,
   vectors) and wire `sw-apl` to them. `2+2`, `1 2 3 times 4`,
   `A left-arrow 5`, `A divide 2` work at the prompt; errors print
   in APL\360 form with the caret. reg-rs baselines for
   `samples/01` to `03`.
2. `mvp-iota-rho-reduce` -- iota, rho (shape and reshape), ravel
   and catenate, reduce over the last axis, matrix display with
   aligned columns, the index origin. `plus reduce iota 10` and
   `2 3 rho iota 6` work. reg-rs baselines for `samples/04` to
   `06`. This is the MVP: the owner can play.
3. `value-model-complete` -- tolerant equality (fixed fuzz),
   promotion and demotion rules, empty arrays, rank > 2, full
   error enum.
4. `display-complete` -- exponential form, `)WIDTH` wrapping with
   six-space continuation, character arrays, empty output, mixed
   int/float columns.
5. `lexer-complete` -- every glyph in `glyphs.txt`, quoted
   strings with doubled quotes, delta letters, system
   command lines, del sentinel, CHARACTER ERROR for lookalikes.
6. `parser-complete` -- operators reduce/scan/inner/outer with
   axis brackets, bracket indexing, indexed assignment, branch,
   quad and quote-quad on both sides of assignment, first-axis
   glyphs, mixed output (`'TEXT';X` semicolon lists, the APL\360
   feature that stands in for the later format primitive).
7. `scalar-functions-complete` -- every monadic and dyadic scalar
   primitive with rank and length checking, boolean results,
   residue and floor with tolerance, circular, factorial and
   binomial, roll.

### Phase 2: mixed functions and operators (saga `core-mixed`)

1. `ravel-catenate-laminate` (axis forms; iota and rho land in
   Phase 1).
2. `take-drop-reverse-rotate-transpose` (monadic and dyadic
   transpose, first-axis variants).
3. `compress-expand-membership-indexof`.
4. `grade-encode-decode-deal-roll` (workspace random link, which
   starts from the APL\360 initial value 16807 in a clear
   workspace, so transcripts that use `?` are reproducible).
5. `reduce-scan` first-axis forms, axis brackets, scan, identity
   elements for empty reductions.
6. `inner-outer-product`.
7. `indexing-and-indexed-assignment` honouring the index origin.
8. `axis-operator` for the structural functions that take one.

### Phase 3: functions, control, and the session (saga `core-session`)

1. `user-functions` -- del header forms (niladic, monadic,
   dyadic, with or without result), locals, dynamic scoping,
   recursion, the state indicator.
2. `branch-and-labels` -- right arrow forms, labels as local
   constants, exit and empty branch.
3. `del-editor` -- definition mode prompt `[n]`, display,
   insert, replace, delete, fractional line insertion, header
   edit, lock (del-tilde).
4. `error-display` -- APL\360 error text, statement echo, caret
   line, function name and line prefix inside functions, state
   indicator accumulation and clearing with right arrow.
5. `quad-io` -- quad and quote-quad input and output, including
   quad input inside defined functions.
6. `i-beams` -- the I-beam system functions: 20 time of day, 21
   CPU time, 22 workspace available, 23 terminals connected, 24
   sign-on time, 25 date, 26 current line, 27 state indicator
   lines (all in sixtieths of a second where APL\360 used them);
   `)ORIGIN`, `)DIGITS`, `)WIDTH` with the `WAS n` reply.
7. `reg-normalization` -- use reg-rs as it is meant to be used:
   a preprocess filter so a baseline can hold output that varies
   between runs, rather than samples contorted into determinism.
   (Owner direction 2026-09-17.)
8. `terminal-feel` -- six-space indent prompt, printer-style
   scrollback, interrupt handling, line editing with Unicode
   input, history file.
9. `script-files` -- an executable `.apl` file: skip a leading
   `#!` line so a shebang script runs without a CHARACTER ERROR,
   and document running a workspace file from the shell.
   (Owner direction 2026-09-17.)
10. `cli-tests-to-reg` -- reg-rs is for CLI regression testing in
    general, not only for transcripts that vary. Move what
    `cli_tests.rs` checks by spawning the binary into reg-rs
    tests; Rust tests keep the unit, function and integration
    testing of the libraries. (Owner direction 2026-09-17.)

### Phase 4: workspaces (saga `workspaces`)

1. `ws-model` -- the workspace as a value: symbol table, saved
   settings, state indicator, DESCRIBE convention.
2. `ws-file-format` -- plain-text UTF-8 workspace file that is
   both human readable and re-executable.
3. `system-commands-ws` -- )CLEAR )WSID )SAVE )LOAD )DROP )LIB
   )COPY )PCOPY )CONTINUE )OFF. Library 0 is `work/`, which is not
   tracked; the first `)SAVE` creates it rather than failing.
   `)LOAD` runs the file, so it takes the saved origin; `)COPY`
   must take the definitions and leave the settings, or it
   changes the origin under code already written. See
   `index-origin-considerations.md`. `)LOAD` prints the SAVED
   line and nothing else: no DESCRIBE hint, no latent
   expression, strict parity. (Owner direction 2026-09-17.)
4. `workspace-quota` -- a workspace is a bounded space, not the
   whole of the process's memory. Give it a size, with a sensible
   default and a way to change it, so that `⌶22` reports
   something true and `WS FULL` can be raised: on assignment, on
   a definition, and on a `)LOAD` or `)COPY` of a workspace too
   big to fit. The figure is an account of what the workspace
   holds, not of Rust allocations, so it must be stable across
   runs and machines. The quota belongs to the session, not the
   workspace: like the console and the clock it is not saved, so
   a workspace saved under a large quota may not load under a
   small one -- which is what happened on a real APL\360.
   (Owner direction 2026-09-17.)
5. `system-commands-inquiry` -- )FNS )VARS )GRPS )GRP )GROUP
   )ERASE )SI )SIV )ORIGIN )DIGITS )WIDTH )SYMBOLS.
6. `library-workspaces` -- numbered public libraries (`)LOAD 1
   CLASS` style) mapped to directories; ship starter workspaces
   in `ws/lib1/`, each carrying a DESCRIBE function. Tracked
   workspaces are ones we wrote: give each a provenance line and
   gate on it, so material from elsewhere cannot be committed by
   mistake. (Owner direction 2026-09-17.) Give the library root a
   command-line option too: a sample that runs `)SAVE` or `)LIB`
   writes into, and reads, the user's own library 0, so a reg-rs
   baseline changes the moment the user saves a workspace of their
   own. Sample 61 lost its `)LIB` line for exactly that reason.
7. `locked-workspace-file` -- a workspace holding a locked
   function is not written as plain text: `)SAVE` obscures it
   (rot-13 to begin with) and `)LOAD` reads both forms. A text
   workspace shows what the del editor refuses to, which is the
   one place sw-apl's lock is weaker than APL\360's binary
   workspaces. Obscuring is not encryption and the docs must not
   claim it is. (Owner direction 2026-09-17, deferred.)
8. `command-error-messages` -- a command that names a workspace
   which is not there answers INCORRECT COMMAND, which is wrong:
   the manual keeps that for a command given an argument it does
   not take, and has `WS NOT FOUND` for a name that is not there,
   `OBJECT NOT FOUND` for an object a `)COPY` cannot find, and
   `IMPROPER LIBRARY REFERENCE` for a library number that is not
   one. Work through the trouble-report table in the APL\360
   User's Manual (Aug 1968) and give each command the reply it
   should give, `)DROP` included. (Owner report 2026-09-18:
   `)LOAD RACE` and `)LOAD 1 RAXE` both said INCORRECT COMMAND.)

### Phase 5: numerics (saga `numerics`)

1. `domino` -- matrix inverse and least-squares divide.
2. `numeric-edge-cases` -- overflow, tolerance, large iota,
   exponent notation round trips, `)DIGITS` extremes.
3. `ibeam-reference` -- `docs/i-beam-reference.md`: what each
   I-beam reports, in what units, what it does on a machine with
   no such thing to report, and what a left argument does. The
   eight-row table in `language.md` says what they are and not
   how to use them, and the units (sixtieths of a second, MMDDYY)
   are the sort of thing a reader needs spelled out with worked
   examples. (Owner request 2026-09-18.)

### Phase 6: the system commands, said properly (saga `commands`)

Raised by the owner 2026-09-18, on finding no reference for the
system commands and no mention anywhere of `)MSG`, `)OPR` and
`)PORTS`. Checking that turned up two parity gaps behind the
documentation one.

1. `commands-reference` -- `docs/commands-reference.md`: every
   system command APL\360 had, what it does, what it replies, what
   it refuses and with which trouble report. Including the ones
   sw-apl does not implement and will not: `)NUMBER`, `)OFF HOLD`,
   `)CONTINUE HOLD`, `)MSG`, `)MSGN`, `)OPR`, `)OPRN`, `)PORTS` are
   the multi-user surface of a shared machine with accounts, ports
   and an operator, and saying so per command is the deliverable.
   `session.md` keeps its tables and links here; `parity.md` keeps
   the status.
2. `command-abbreviation` -- the manual: "Where the first word of a
   command form is more than four characters long, only the first
   four are significant. The others are included only for mnemonic
   reasons, and may be dropped or replaced, as desired. For
   example, )CLEAR, )CLEA, )CLEAVER, etc., are all equivalent."
   sw-apl requires the exact spelling, so `)CLEA` and `)ORIG 0`
   answer INCORRECT COMMAND. Add the rule, and a parity row, which
   it has never had.
3. `copy-replies` -- `)COPY` and `)PCOPY` print nothing. The
   manual gives both "SAVED, followed by the time of day and the
   date that the source workspace was last stored", and `)PCOPY`
   also "NOT COPIED:, followed by the names of objects not
   copied". A protected copy that quietly skipped the name you
   asked for cannot be told from one that worked. (Found while
   writing the reference, 2026-09-18.)
4. `save-lock-syntax` -- `)SAVE NAME:PASSWORD` stores a workspace
   literally called `NAME:PASSWORD`, because the `[LOCK]` and
   `[KEY]` password forms are not parsed and the colon is taken as
   part of the name. Locks are the multi-user surface again and are
   not wanted; a colon in a workspace name should be refused rather
   than silently making a strangely named workspace. Decide the
   reply against the manual's table and record it.

### Phase 7: the last parity rows (saga `parity`)

Owner direction 2026-09-18: clear the checklist before starting a
new component. Two of the remaining rows are real and two are
restrictions to be written down rather than closed.

1. `valence-syntax-error` -- a glyph used in a valence it does not
   have answers NOT IMPLEMENTED (`1~0`, `1⍋2`), and so does a
   reduction or product by a glyph with no dyadic scalar form, and
   an axis form belonging to APL2. In APL\360 there is no such
   function, so the sentence does not parse: all of them are SYNTAX
   ERROR. Removing `ErrorKind::NotImplemented` altogether is what
   takes the "must reach zero" row to zero.
2. `open-definition-guard` -- NOT WITH OPEN DEFINITION, the
   manual's report 6. `)COPY`, `)PCOPY`, `)SAVE` and `)CONTINUE`
   carry it; `)LOAD` does not, which is worth understanding before
   implementing rather than after.
4. `open-output-line` -- `⍞←` leaves the output line open, and it
   survives inside a function but not across a statement in
   immediate execution, because `Reply` carries complete lines and
   an unterminated one cannot cross that boundary. Worth fixing
   before the web demo rather than after, since the demo is a third
   caller of the same API. (Found sorting the rows, 2026-09-18.)
3. `documented-restrictions` -- a saved workspace does not keep a
   suspended function, and WS LOCKED and NOT SAVED, WS QUOTA USED
   UP cannot arise here. These are not work to be done; they are
   what this implementation is. Give `parity.md` a Restrictions
   section that says so, and stop carrying them as todo.

### Phase 8: the 2741 and a local service (saga `terminal`)

Owner direction 2026-09-19, Linux prototype: first build and test an
isolated Rust CLI terminal and TCP server in `experimental/terminal2741`.
The `aplterm` client translates physical keyboard input and composes
overstrikes into Unicode before sending complete lines. Keep the existing
interpreter and CLI unchanged. Publish the tested prototype on a feature
branch before integrating the broader server and browser work below.

Linux validation follow-up: make the legacy bare-shebang regression portable
and correct the quoted heredoc delimiters in the obscured-load and workspace-file
regression commands. These three existing fixtures fail on Linux independently
of the prototype; their baselines remain unchanged in this slice.

Owner direction 2026-09-19, fourth: a public demo, an attention key,
and what a workspace library means with no filesystem under it.

**`./pages`, tried locally first.** A static bundle in `pages/`, run
from a plain file server before GitHub Pages is configured at all.
Pages is static, so there is no service to dial: the demo runs the
interpreter in the browser after all. What makes that possible now is
that the blocking read has a place to block. `apl-serve` runs in a Web
Worker; `Link` is implemented a third time over a `SharedArrayBuffer`
with `Atomics.wait`, so `recv` parks the worker until the page posts a
line. `Console::read` stays synchronous and `Session` is not touched --
the worker blocks exactly as the service's connection thread blocks.
The terminal above it is the same terminal: three transports, one
client, one overstrike table.

Cross-origin isolation is the price. `SharedArrayBuffer` needs COOP
and COEP headers, which GitHub Pages will not set, so the bundle
carries `coi-serviceworker` to install them -- at the cost of one
reload on first visit, and of failing where service workers are
refused. Trying it locally first is what tells us whether that is
acceptable before any of it is published.

**ATTN is Escape.** Owner direction: Escape -- which *is* Ctrl-[, the
same byte, exactly as Ctrl-H is backspace -- stands in for the 2741's
ATTN key in both clients. Escape already quotes the next key in
`aplterm`, and the two do not collide, because they are never both
possible: while a line is being typed the terminal is reading keys and
Escape quotes; while the service is working the terminal is waiting
for a frame and Escape is ATTN. The mode decides, and nothing is
rebound. In the browser Escape is free, and `Ctrl-[` arrives
distinguishably besides.

Two things have to change under it. The protocol gains a message the
terminal may send unprompted, which a typed line can never be mistaken
for; and a connection gains a reader that is listening while the
session thread is inside `eval`, which is the channel-fed read the
service was always described as having. The third is below
`apl-serve` and so is asked for explicitly here: `apl-call` keeps the
interrupt flag as one process-global `static STOP`, set by the CLI's
signal handler. A server holding sixteen sessions would interrupt all
of them, so the flag becomes per-session state that the CLI's handler
sets for its own session like any other client.

**Libraries without a filesystem.** `)LIB 1` and `)LOAD 1 NAME` work
in the browser: `ws/lib1/` is baked into the bundle and read-only,
which is what it already is in spirit. `)SAVE` writes library 0 into
the browser's local storage rather than being refused -- a saved
workspace is small UTF-8 text, so there is nothing about it that needs
a file. That wants a seam under `apl-session` saying where workspaces
live and how they are listed, read and written, with the native
implementation being the filesystem it is today: the CLI's behaviour,
and reg-rs, must not move.

Order: prove the worker and the blocking read with the plain page
first, since it is the only part nobody has done before; then the
attention key, which is testable with `aplterm` and `nc` alone; then
the libraries; then the 2741 in the browser and the keyboard, which
are presentation on top of all three.

Owner direction 2026-09-19, promotion: the prototype is the
implementation. `experimental/terminal2741` moves into the component
layout under the ordinary gates -- `components/web` for the protocol,
the service and the two listeners, `components/term` for the keyboard,
the paper and `aplterm` -- and `experimental/` goes away. The JSON
frame the prototype settled on stays the protocol: it is `Reply` on
the wire, a browser parses it without help, and a client already
speaks it.

Owner question 2026-09-19, answered: the browser terminal can be Yew
and WASM, and should be. The blocking read is what made an in-browser
interpreter impossible, and it now lives in the service; what is left
in the browser is a terminal. `apl-keyboard` is the whole overstrike
state machine and depends on nothing but `apl-strike` and a keymap, so
it compiles to `wasm32-unknown-unknown` as it stands and the browser
reuses it rather than repeating it in JavaScript. What the browser
replaces is `apl-paper` and `apl-typing`, the crossterm halves: DOM
rendering, and `keydown` instead of terminal events. It can key the
2741 layout off the physical key, which a terminal cannot -- a
terminal receives characters and so cannot tell Caps Lock from Shift.
The costs are a wasm toolchain in `just demo`, the chords a browser
reserves, and IME and touch keyboards, which is why the keyless path
stays. That is step 3, not step 2; the plain HTML page the service
ships is the fallback with no build step, and the thing that proves
the protocol.

Owner direction 2026-09-19, replacing the in-browser interpreter
that was planned here. The browser runs a 2741 terminal; the
interpreter runs in a local server it talks to.

This is the 1968 architecture. A 2741 talking to a time-sharing
service is what APL\360 *was*, and the self-contained browser
interpreter was the anachronism. It also dissolves the hard part:
`Console::read` is synchronous, and a browser cannot stop and wait
for a keystroke in the middle of a statement, which would have cost
us `⎕`, `⍞` and every line of the del editor. On a server the
blocking read happens where blocking is free. The filesystem seam
disappears with it -- the server has a real one, so `)SAVE`, `)LOAD`
and `ws/lib1/` simply work.

Hosting: local only. `just demo` starts the server and opens the
terminal; the README carries a recording for readers who will not
clone. No always-on service, no accounts, no ops.

Owner direction 2026-09-19, second: base, backspace, overstrike is
how an APL glyph is typed. A 2741 formed `⍟` from `○`, backspace,
`*`, and the owner's keyboard carries only the foundational glyphs
on its caps, as a 2741's did. So this is the input method, not a
curiosity, and it lands in the CLI first where it will be used
daily.

The front end composes, both hosts, one state machine. A 2741 sent
the three characters and let the mainframe compose them, and sw-apl
does not, for a reason that settles it: the CLI has no server, so
composition has to happen in the front end there regardless. Doing
it front-end in both means one mechanism and one table fed two
ways, rather than two composition points that can disagree. The
wire protocol then carries ordinary lines, which is what
`Session::respond` wants.

The backspace key cannot be the key, because a line editor needs it
for delete. It is a separate keystroke, and `Ctrl-H` cannot be that
either: `Ctrl-H` *is* `0x08`, byte-identical to backspace, and
rustyline already binds it. `Ctrl-]` (0x1D) is free in rustyline,
is not a terminal signal (SIGQUIT is `Ctrl-\`), and is claimed by
no browser shortcut known; `Ctrl-^` (0x1E) is the fallback. The
owner's keyboard sends it from a repurposed, relabelled keypad key.

Two lessons from `sw-embed/web-sw-tos`, whose frontend assists with
keystrokes rather than forwarding them: name the key in one
constant so the help text cannot drift from the binding, and keep a
path that needs no keyboard at all, because a key can still be lost
to a platform nobody tested.

1. `overstrike-input` -- base, backspace, overstrike as a way of
   typing, and the overstrike table in `data/glyphs.toml` where
   both hosts read it.
Owner direction 2026-09-19, third: both clients talk to the same
service over a socket, so the server can run on one machine and the
terminal on another. The CLI is a client too -- `sw-apl --connect
host:port` -- and not only the browser. That is the 2741 dialling
in, and it makes the server testable without a browser at all.

One protocol, two transports: a TCP port for CLI clients, and an
HTTP/WebSocket port for browsers, which cannot open a raw socket.
Framing is plain UTF-8 lines, so `nc host port` is an emergency
client and a debugging window -- though a bare `nc` gets no
overstrike composition, because that belongs to the front end.

Not telnet: no IAC, no option negotiation.

`sw-apl` with no `--connect` holds its session in process exactly as
it does today. The reg-rs suite is the check.

The listener binds to localhost unless told otherwise. A service
holding `)SAVE` and `)LOAD` is a file-writing primitive for whoever
can reach it, and APL\360's own answer to that -- sign-on numbers
and passwords -- is out of scope here. Safe default, explicit flag
to open it to the LAN, and the docs saying which.

2. `terminal-server` -- `sw-apl-server` and the CLI client: one
   session per connection, a line protocol over TCP and WebSocket,
   and a blocking read that works because the server may block.
3. `wasm-session` -- the interpreter in a Web Worker, `Link` over a
   `SharedArrayBuffer` with `Atomics.wait`, and `pages/` built and
   served locally. The plain page is the client; `)SAVE` and `)LOAD`
   report that there is nowhere to write yet.
4. `attn-interrupt` -- Escape as ATTN: a per-session interrupt flag,
   an unprompted message in the protocol, a connection that listens
   while the session works, and the key in `aplterm` and in the page.
5. `browser-workspaces` -- a seam under `apl-session` for where
   workspaces live; the filesystem underneath it natively, library 1
   baked into the bundle and library 0 in local storage in the
   browser. `)LIB 1`, `)LOAD 1 NAME`, `)SAVE`.
6. `terminal-2741` -- the browser terminal: the printing-terminal
   feel, the carriage, ATTN as interrupt, and the overstrike
   sequence sent rather than composed -- the table it needs is
   already in `data/glyphs.toml` by then, which is the reason to
   compile the terminal from Rust rather than write it in
   JavaScript with a second copy.
8. `del-tilde-opens` -- `⍫` opens a locked definition as well as
   closing one: "used instead of ∇ to open or close a function
   definition". sw-apl takes it only to close, so `⍫R←SECRET` is a
   SYNTAX ERROR. (Found writing the overstrike sample, 2026-09-19.)
7. `glyph-keyboard` -- the IBM 2741 APL layout, a clickable board,
   and the expansions already in `docs/espanso/` and
   `docs/emacs/`; a first screen worth arriving at.

## Owner decisions (2026-09-16)

Answers to the questions raised at bootstrap, now policy:

1. Unicode acceptance is strict and documented exactly in
   `language.md` ("Accepted Unicode"): outside quotes and comments
   only the code points in `glyphs.txt` plus printable ASCII and
   space are valid; anything else, including Greek lookalikes and
   control characters, is CHARACTER ERROR naming the code point.
   Invalid UTF-8 in a file or on stdin is reported as CHARACTER
   ERROR with the byte offset, never a crash.
2. (Superseded by 5.) Execute and format were to stay in Phase 5.
3. Workspace libraries: numbered libraries map to directories via
   a config file (APL\360 `)LOAD 1 NAME` form) and plain paths
   also work.
4. Markdown: README.md and other top-level markdown stay ASCII;
   `docs/*.md` may use glyphs; README links to docs. If GitHub
   turns out not to render APL glyphs in `docs/*.md`, switch those
   files to Org. Later tooling (not scheduled yet): `ob-sw-apl`
   for literate Org/PDF/HTML with glyphs, and vhs tapes of the
   CLI to embed as images and animations in the ASCII README.
5. Pure APL\360 (2026-09-16, later the same day): the APLSV quad
   system variables and functions are out. I-beam functions
   provide the system interface; `)ORIGIN`, `)DIGITS`, `)WIDTH`
   are the settings and reply `WAS n`; the comparison fuzz is
   fixed; the random link is workspace state saved with the
   workspace; execute and format are gone. Quad and quote-quad
   input/output stay, as in APL\360.

## Owner decision (2026-09-19): cap and cup are reserved characters

Researched and settled by the owner. `∩` and `∪` were on the IBM
2741 typing element and on the 51xx keyboards, and were carried
forward through APLSV for workspace compatibility, but in neither
APL\360 nor APLSV were they primitive functions. `∩` existed so
that the lamp `⍝` could be struck from it and `○`; `∪` was reserved
and struck into nothing. Minimum and maximum, floor and ceiling,
were `⌊` and `⌈` throughout, as they still are. They became
primitives only in IBM APL2 (1984): dyadic `∩` intersection,
monadic `∪` unique, dyadic `∪` union.

So sw-apl's `[[later]]` classification of the two is wrong on its
face -- a glyph a later APL *introduced* is not one the 2741 had --
and it contradicts sw-apl's own overstrike table, which strikes the
lamp from `∩`. They are characters of the APL\360 set with no
function meaning, which is what `_` already is here. Step
`008-cup-and-cap` carries this out; what it still has to decide
from the manuals is which error a legitimate character with no
meaning earns, and how the message should read, since
"(intersection, not APL\360)" states something false.

APL\360 has no set operations. Union, intersection, difference and
unique are written with membership and compression -- `(A∊B)/A`,
`A,(~B∊A)/B`, `(~A∊B)/A`, `((A⍳A)=⍳⍴A)/A` -- which is why `∊`
exists, and is the subject of the sample in step
`010-iota-one-element-vector`.

### Phase 9: literate and recorded docs (saga `doc-tooling`, unscheduled)

1. `vhs-tapes` -- vhs tape scripts under `docs/tapes/` rendering
   CLI sessions with glyph input and output to GIF/PNG for the
   README. First tape (`mvp.tape`) shipped with the core-mvp saga;
   re-record as features land. Known vhs limits: a typed left
   arrow becomes the cursor key and the clipboard is unavailable
   headless, so tapes avoid assignment until a workaround exists.
2. `ob-sw-apl` -- an Org Babel language for sw-apl so literate
   Org documents can execute APL blocks and export to PDF/HTML.
