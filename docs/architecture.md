# sw-apl Architecture

## Repository shape

The repository is a multi-workspace monorepo. Each directory under
`components/` is its own top-level cargo workspace holding one or
more small crates under `crates/`. `.cargo/config.toml` points
every workspace at the shared `target/` at the repo root.

```
sw-apl/
  components/
    value/     apl-value, apl-error        element and array model
    display/   apl-display                 APL\360 output formatting
    lex/       apl-lex                     Unicode tokenizer
    parse/     apl-ast                     AST and Defn types
               apl-scan                    brackets, segments, del header
               apl-parse                   right-to-left parser
    prims/     apl-prims-scalar            scalar primitives
               apl-prims-mixed             structural primitives
               apl-prims-ops               reduce/scan/inner/outer
    eval/      apl-console                 output, rendering, the terminal
               apl-workspace               symbol table, frames, env
               apl-call                    defined-function calls
               apl-quad                    reading a line mid-statement
               apl-clock                   the clock, and how a span of
                                           time reads
               apl-space                   what a value costs, and the quota
               apl-store                   somewhere to keep a saved
                                           workspace: a disc, a browser
               apl-modes                   the modes, the `⍝!MODES` line,
                                           and what the host decides
               apl-shelves                 the libraries as each mode
                                           sees them
               apl-uses                    which modes a workspace runs
                                           in, from what its code uses
               apl-eval                    interpreter
    session/   apl-library                 which workspace a command means,
                                           and what to say when there is not one
               apl-copy                    taking names out of a stored one
               apl-inquiry                 listing names, groups, erasing
               apl-session                 system commands, del editor,
                                           workspace files, libraries
               apl-settings                index origin, digits, width,
                                           checked; the directives set them
    a70/       apl-ibeam                   the I-beams                 } only (A) '70
               apl-a70-commands            )ORIGIN )DIGITS )WIDTH,     } reaches
                                           )GROUP )GRP )GRPS           } these
    cli/       sw-apl                      terminal REPL and batch
    web/       apl-wire                    the terminal protocol: a
                                           frame out, a typed line in
               apl-serve                   one session per connection,
                                           held over a link
               sw-apl-server               the two listeners, and the
                                           terminal page
    web/       apl-board                   the 2741 keyboard in a
                                           browser
               apl-wasm                    the session on a worker,
                                           over a shared channel
    term/      apl-keyboard                2741 keys, overstrikes, the
                                           line being typed
               apl-paper                   what the carriage put on
                                           the page
               apl-typing                  reading one line at the
                                           keyboard
               aplterm                     the 2741 itself
  docs/        planning and reference docs
  samples/     conformance corpus (.apl transcripts)
  tests/reg-rs reg-rs regression baselines
  scripts/     gen-changes, run-samples, reg wrappers
  ws/lib1/     shipped library workspaces: LIFE, RACE, EDIT, BIRDS
```

Workspaces are created by the saga step that first needs them;
the tree above is the target layout, not a promise that every
directory exists today.

## Dependency flow

```
value -> display -> console
value -> lex -> scan -> parse
value -> prims (scalar, mixed, ops)
prims + console -> workspace -> call, quad
parse + prims + call + quad -> eval
eval -> session -> cli
eval -> session -> web (wire, serve, server)
lex (strike) -> term (keyboard, paper, typing, aplterm) -> wire
```

- `apl-value` has no dependencies inside the repo.
- `apl-lex`, `apl-scan`, and `apl-parse` never depend on
  `apl-prims` or `apl-eval`; the parser produces an AST, it does
  not evaluate.
- `apl-scan` holds what the parser reads straight off the token
  stream before it recurses: matching brackets, top-level
  semicolon segments, what ends an operand, and the del header.
- `apl-workspace` owns the symbol table (variables and defined
  functions) and the call frames that make scoping dynamic.
- `apl-call` applies a defined function: valence, frame, body,
  result. It takes "evaluate one line" as a function pointer, so
  it sits below the evaluator rather than inside it.
- `apl-console` owns what a statement produced, how that reads as
  transcript lines, and the `Console` trait: where a statement gets
  a line when it reads one. It sits below the evaluator because a
  read happens part way through a statement, after whatever the
  statement has already shown, so the prompt has to land after that
  and not before it. The workspace holds a `Console`, which is the
  seam the web build replaces.
- `apl-ibeam` answers the I-beams, and owns the `Clock` the
  workspace holds. A clear workspace has a clock that does not
  move, so nothing reads the real world until a host installs one
  that does, and a transcript made without a terminal reproduces.
- `apl-space` says what a value, a name and a defined function cost
  in bytes, and whether one more will fit. It measures nothing: the
  figures are what APL\360 would have charged, so a workspace is the
  same size on every machine and in every build, and a test can
  state a number. It sits below `apl-workspace`, which checks the
  quota in `set` and `define` so that no assignment can go round it.
- `apl-quad` reads that line: quad evaluates the reply, quote-quad
  takes it as characters. Like `apl-call` it is handed `Run` rather
  than depending on the evaluator.
- `apl-eval` owns the state indicator and dispatch from AST to
  primitives.
- `apl-library` resolves `[lib] name` to a file and owns the
  trouble reports -- WS NOT FOUND, OBJECT NOT FOUND, IMPROPER
  LIBRARY REFERENCE, INCORRECT COMMAND -- so that every command
  which names a workspace fails the same way and the reports have
  one spelling.
- `apl-copy` works out what a `)COPY` brings and what a `)PCOPY`
  leaves: the names asked for, the members a group among them
  brings, and the ones this workspace already holds. It is its own
  crate because that is four separate questions and the command
  crate had no room for them.
- `apl-inquiry` answers the commands that report what a workspace
  holds -- `)FNS`, `)VARS`, `)GRPS`, `)GRP`, `)SI`, `)SIV`,
  `)SYMBOLS` -- and owns the group facility and `)ERASE`. It is
  `apl-commands`' neighbour rather than its contents: that crate
  was at its module budget, and listing names has little to do
  with reading and writing workspace files.
- `apl-session` owns everything that begins with a right
  parenthesis, the del editor, workspace files, and the library
  directory map. It exposes a line-oriented `Session` API: feed a
  line, receive output lines. The CLI and the web demo are both
  thin shells over that API.

## Pipeline for one input line

1. `apl-session` classifies the line: system command, del header,
   del editor line (when in definition mode), or
   immediate-execution statement.
2. `apl-lex` turns a statement into tokens (glyphs, numbers,
   strings, names, quad names).
3. `apl-parse` builds the AST right to left. It is told which
   names hold functions, because `F B` is a call only when `F`
   is one.
4. `apl-eval` evaluates the AST against the workspace, producing
   a value or an error with a caret position.
5. `apl-display` formats the value (or the error transcript)
   with the `)DIGITS` and `)WIDTH` settings.
6. The shell prints the lines and shows the next prompt.

## Value model

`Array { shape: Vec<usize>, data: Data }` where `Data` is either
`Num(Vec<Number>)` or `Char(Vec<char>)`. `Number` is `Int(i64)` or
`Float(f64)`; arithmetic promotes to `Float` on overflow or
non-integral results and demotes back to `Int` when a result is
exactly integral within the fixed fuzz. Scalars are rank-0 arrays.

There is no nested `Data` variant. That is deliberate: this is
APL\360, not APL2.

## Error model

`apl-error::AplError { kind, caret: Option<usize>, context }` with
kinds SYNTAX, VALUE, DOMAIN, RANK, LENGTH, INDEX, WS FULL, DEFN,
CHARACTER, DEPTH, and INTERRUPT. The caret is a character offset
into the source line so the session can print the APL\360 caret
line. Each crate maps its own failures into `AplError`.

## Generated tables

`data/glyphs.toml` is the single source of truth for every glyph:
the primitives with their monadic and dyadic names, the punctuation
and sentinels, the lookalikes a CHARACTER ERROR should redirect, and
the later-APL glyphs it should name. Two consumers read it, and
nothing else may hold a copy:

- `components/value/.../build.rs` emits Rust consts into `OUT_DIR`,
  which `apl-value`'s `glyphs` module includes. The lexer takes its
  accepted set from there, and the error display its hint tables.
- `scripts/gen-glyphs.sh` regenerates `docs/glyphs.txt`.

Adding a glyph is therefore one edit plus `scripts/gen-glyphs.sh`,
and a test asserts the generated tables agree with each other.

## Metrics as architecture

`sw-checklist` gates shape the crate boundaries: at most a few
modules per crate and a few functions per module. When a crate
approaches the gate, the next step splits it (for example
`apl-prims-mixed` into selection and structural halves) before
adding to it. `lib.rs` files are facades that re-export; logic
lives in named modules (`parse.rs`, `format.rs`, `run.rs`).

## The service, and the terminals on it

The interpreter does not run in a browser. It runs in
`sw-apl-server`, a local process holding one `Session` per
connection on a thread of its own, and terminals dial in. This is
the 1968 arrangement, and it is what makes the session work at all:
`Console::read` is synchronous, so a statement that reads stops
until a line arrives. A thread waiting on a socket may stop. A
browser holding the interpreter could not, and `⎕`, `⍞` and every
line of the del editor would have gone with it.

The seam is `apl_wire::Link`: send a frame, block for a line. Three
transports implement it and the service cannot tell them apart --
a raw socket for `aplterm` and for `nc`, a WebSocket for a browser
talking to the service, and a `SharedArrayBuffer` for a browser with
no service to talk to, where `apl-serve` itself is compiled to
WebAssembly and runs on a Web Worker. The worker's thread is allowed
to block in `Atomics.wait`; the page's is not, which is exactly why
the session lives on the worker. A frame is `Reply` as JSON:
the transcript lines, the prompt to type the next line at, and
whether the session has ended. A line `⍞←` left open is carried as
the prompt, because the carriage stopped there.

Composition belongs to the terminal. Only the terminal sees the
keystrokes, so `⍟` is formed there from `○` and `*` and the service
is sent the glyph. That is why `apl-keyboard` holds the whole
overstrike state machine and depends on nothing but `apl-strike`:
one table, one state machine, and every terminal reads it. It
compiles to `wasm32-unknown-unknown` unchanged, and the browser uses
it through `apl-board` in place of `apl-paper` and `apl-typing`, the
crossterm halves.

`apl-keyboard` and `apl-wire` are shared foundations rather than
either client's property, which is why the arrows between `term/`
and `web/` run both ways: `aplterm` takes the protocol from `web/`
and `apl-board` takes the keyboard from `term/`. No crate depends on
a crate that depends on it; the pair simply sits under both clients,
and the directories are named for where each client lives rather
than for who owns what.

What bounds the service is a fixed number of sessions, refused at
the door rather than queued. Both listeners bind to the loopback
address unless told otherwise: a service holding `)SAVE` and
`)LOAD` writes files for whoever can reach it, and APL\360's answer
to that -- sign-on numbers and passwords -- is out of scope.
