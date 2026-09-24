<p align="center">
  <img src="images/sw-apl-logo.png" alt="sw-apl logo" width="256">
</p>

# sw-apl

A clean-room APL interpreter, written in Rust from scratch, for the
terminal, a local service, and the browser. It brings back classic
APL the way it was used: traditional glyphs typed as Unicode, the
six-space prompt, a printed transcript, the del editor, and
workspaces saved and loaded by name.

It has two modes:

| Mode | Modelled on | What it has |
|---|---|---|
| (A) '70 | APL\360 on an IBM 2741 terminal | The APL\360 language; I-beams for system values; `)ORIGIN`, `)DIGITS` and `)WIDTH` for the settings; groups |
| (B) '75 | The APL of the IBM 5100 family of desktop computers | The same core, plus execute, format, and the quad system variables and functions; no I-beams, no settings commands, no groups |

Neither mode is APL2 or Dyalog: arrays are flat, with no nesting and
no each. sw-apl is not a port; the C
interpreter `sw-cor24-apl` and GNU APL served only as references for
expected behaviour and for the conformance corpus in `samples/`.

## Try it

**[sw-apl in your browser](https://sw-apl.softwarewrighter.com/)**

The interpreter runs in the tab, compiled to WebAssembly; nothing
typed is sent anywhere. The page opens in (B) '75, and the tabs at
the top switch mode (a new session, in a clear workspace). A keyboard
on the page gives every glyph a key, so a phone works too.

Some things to type:

```
      )LIB 1               the workspaces sw-apl ships
      )LOAD 1 TTTML        in (B): a machine that learns tic-tac-toe
      PLAY 1               play it; you move first
      )LOAD 1 BIRDS        combinators, a version for each mode
      HOWBIRDS
      )LOAD 1 LIFE         Conway's Life
      GLIDER
      RUN 4
```

`)SAVE` keeps a workspace in the browser's own storage, and Help on
the page covers the rest.

## Run it on your machine

Needs a Rust toolchain (edition 2024, stable) and, for the commands
below, [just](https://github.com/casey/just).

```bash
just release                                  # builds the three binaries
target/release/sw-apl                         # (A) '70 at the terminal
target/release/sw-apl --mode 75               # (B) '75
target/release/sw-apl -f samples/06-reduce.apl
target/release/sw-apl --help
```

Run it from the repository root, or pass `--library DIR`, so that
`)LOAD 1 NAME` finds the shipped workspaces in `ws/lib1`.

## A 2741 terminal and a service

sw-apl also runs the way APL\360 ran: a service holding one session
per connection, and terminals connecting to it. Everything stays on
your machine -- the service listens on the loopback address only.

`sw-apl-server` is the service, and its mode is every session's
mode. `aplterm` is a 2741 in a terminal window, with the 2741
keyboard and its overstrikes; it takes the mode from the service.

```bash
# (A) '70
target/release/sw-apl-server --mode 70
target/release/aplterm                        # connects to 127.0.0.1:2741

# (B) '75
target/release/sw-apl-server --mode 75
target/release/aplterm
```

Both at once, each service on ports of its own:

```bash
target/release/sw-apl-server --mode 70 --listen 127.0.0.1:2741 --http 127.0.0.1:8360
target/release/sw-apl-server --mode 75 --listen 127.0.0.1:2775 --http 127.0.0.1:8375
target/release/aplterm --connect 127.0.0.1:2741    # (A)
target/release/aplterm --connect 127.0.0.1:2775    # (B)
```

`just demo` builds everything, starts the service and opens a
terminal page in your browser at `http://127.0.0.1:8360/`; `just
demo 75` does the same in (B). `nc 127.0.0.1 2741` works too, as an
emergency client: the protocol is one line each way.

## Library 1

The workspaces sw-apl ships, in `ws/lib1/`. Each is a plain text
file, and each has a `DESCRIBE` that says what it holds. A file's
name says the modes it runs in: `NAME.apl.ws` for both,
`NAME.a-70.apl.ws` or `NAME.b-75.apl.ws` for one.

| Workspace | Modes | What it is |
|---|---|---|
| LIFE | both | Conway's Life on a torus: `GLIDER`, then `RUN 4` |
| RACE | both | A horse race, written to be read |
| EDIT | both | A workspace to practise the del editor on; its `FACT` is wrong by one on purpose |
| BIRDS | a version for each | Combinators, after Smullyan's birds. In (A), the ones APL\360 can write; in (B), every one in the list, by execute |
| TTTML | (B) | A machine that learns tic-tac-toe by playing itself, then plays you |

## What works

| Area | |
|---|---|
| Language | Every APL\360 primitive and operator on arrays of any rank; bracket indexing and indexed assignment; characters; mixed output; quad and quote-quad input and output |
| (B) additions | Execute and format; the quad system variables (`IO`, `CT`, `PP`, `PW`, `RL`, `LX` and the rest), which can be made local; the quad system functions (`CR`, `FX`, `EX`, `NL`, `NC`, `CC`) |
| Functions | The del editor and definition mode, locals (functions as well as variables), labels and branching, recursion, locking with del-tilde |
| Errors | The APL\360 error display with its caret; a failing function suspends, `)SI` shows where, and a branch takes it up again; Ctrl-C (ATTN) stops a run |
| Workspaces | `)SAVE`, `)LOAD`, `)COPY`, `)DROP`, `)LIB` over numbered libraries; a saved workspace is plain APL you could have typed |
| Session | The six-space prompt, a printed transcript, output shown as it is printed, batch runs of a script, `)OFF` |
| Input | The 2741 keyboard and overstrikes in `aplterm` and the browser; Espanso and Emacs keymaps |

`docs/parity.md` is the row-by-row picture for each mode.

## Documentation

This README is plain ASCII, so it reads the same everywhere; the
documents show real APL glyphs.

Using sw-apl:

- [Language reference](docs/language.md) -- the shared core, what (B)
  adds, what (A) has that (B) has not, and exactly which Unicode is
  accepted
- [Session](docs/session.md) -- the prompt, error display, system
  commands, libraries, the DESCRIBE convention
- [System commands reference](docs/commands-reference.md) -- every
  command, what it replies and refuses, and which mode has it
- [Using the del editor](docs/del-editor-guide.md) -- writing and
  changing a function, line by line
- [Workspaces](docs/workspaces.md) -- what one holds, the libraries
  and their file names, saving and loading, locking
- [Entering glyphs](docs/glyph-entry.md) -- keymaps, OS layouts, and
  every 2741 overstrike
- [The 2741 and the service](docs/terminal.md) -- the keyboard, the
  protocol, and what a session is

The modes and the workspaces:

- [The (B) '75 mode](docs/mode-b.md) -- what (B) is modelled on, its
  sources, and every way it differs from (A)
- [APL timeline](docs/apl-timeline.md) -- the APLs sw-apl models,
  and the ones around them
- [Learning tic-tac-toe in 64 KB](docs/learn-tic-tac-toe-strategy.md)
  -- how TTTML learns, and why it fits a 5110
- [BIRDS](docs/birds.md) and the [combinators
  reference](docs/combinators.md)
- [I-beam reference](docs/i-beam-reference.md) and [index origin
  considerations](docs/index-origin-considerations.md)

The project:

- [Parity checklist](docs/parity.md) -- the definition of done, a
  column for each mode
- [Master plan](docs/plan.md) -- phases, decisions, what comes next
- [Glyph table](docs/glyphs.txt), [Testing](docs/testing.md),
  [Architecture](docs/architecture.md), [Design decisions](docs/design.md),
  [Requirements](docs/prd.md)

## Building and testing

Each directory under `components/` is its own cargo workspace; they
share one `target/` at the repository root.

```bash
just test            # cargo test in every workspace
just clippy          # clippy, warnings as errors
just fmt-check
just gates           # markdown and code-shape checks
scripts/reg.sh run   # transcript regressions (reg-rs), one per sample
just pages           # the browser build, into pages/
just check-pages     # the browser build, checked in Chrome
```

## Repository layout

```
components/   one cargo workspace per component: the interpreter,
              the CLI, the service (web), the 2741 (term), and the
              parts only one mode has (a70, b75)
docs/         reference, plan, requirements, architecture
pages/        the browser demo: a page, a worker, and the session
              compiled to WebAssembly
samples/      conformance corpus: glyph-form APL programs
ws/lib1/      the workspaces sw-apl ships
scripts/      change log, sample runner, reg-rs wrappers, gates
images/       logo, and redistributed material
```

## Links

- Blog: [Software Wrighter Lab](https://software-wrighter-lab.github.io/)
- Discord: [Join the community](https://discord.com/invite/Ctzk5uHggZ)
- YouTube: [Software Wrighter](https://www.youtube.com/@SoftwareWrighter)

## Copyright

Copyright (c) 2026 Michael A Wright. See [COPYRIGHT](COPYRIGHT).

## License

MIT License. See [LICENSE](LICENSE). sw-apl's own code, documentation
and workspaces are all under it.

One directory is not, and deliberately so. `images/redistributed/`
holds material sw-apl did not write and redistributes under the terms
it came with, one directory per work, each with its own `LICENSE` and
an `ATTRIBUTION.md` naming the author and saying whether the file was
changed. Today that is the IBM 2741 APL keyboard layout
([images/redistributed/apl-keyboard](images/redistributed/apl-keyboard)),
by the Wikimedia Commons user Rursus, under CC BY-SA 3.0 -- a
share-alike licence, so any modified version of it stays under CC
BY-SA and lives in that same directory rather than becoming MIT by
moving. A build or a page that ships the picture ships its licence
with it. `scripts/check-provenance.sh` is the gate: borrowed material
cannot be committed there without the terms it travels under.
