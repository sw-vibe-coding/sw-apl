# sw-apl Architecture

## Repository shape

The repository is a multi-workspace monorepo. Each directory under
`components/` is its own top-level cargo workspace holding one or
more small crates under `crates/`. `.cargo/config.toml` points
every workspace at the shared `target/` at the repo root.

```
sw-apl/
  components/
    value/     apl-attn            attention: the flag that stops a run,
                                   per session
               apl-value           value model: numbers, arrays, errors
    glyphs/    apl-glyphs          glyph tables, generated from
                                   data/glyphs.toml
    display/   apl-display         APL\360-style output formatting
    lex/       apl-lex             tokenizer for traditional APL glyphs
               apl-strike          overstrikes: forming a glyph from two
                                   characters, as on a 2741
    parse/     apl-ast             abstract syntax tree types
               apl-parse           right-to-left parser and AST
               apl-scan            token-level scanning: brackets,
                                   segments, del headers
    prims/     apl-prims           primitive dispatch: glyph to scalar,
                                   mixed, or operator implementation
               apl-prims-index     bracket indexing and indexed
                                   assignment
               apl-prims-join      catenate and laminate along an axis
               apl-prims-mask      compress and expand along an axis
               apl-prims-matrix    matrix inverse and matrix divide: the
                                   domino primitive
               apl-prims-mixed     mixed (structural) primitive functions
               apl-prims-ops       operators: reduce and scan along an
                                   axis, inner and outer products
               apl-prims-radix     encode and decode in mixed radix
               apl-prims-scalar    scalar primitive functions with scalar
                                   extension
               apl-prims-search    membership and index-of
               apl-prims-select    take, drop, reverse, rotate, transpose
               apl-scalar-arith    scalar arithmetic: plus, minus, times,
                                   divide, maximum, minimum, residue,
                                   power, log, exponential
               apl-scalar-circ     scalar circular functions, factorial
                                   and binomial via the gamma function
               apl-scalar-logic    scalar comparisons with the fuzz,
                                   boolean functions
    eval/      apl-call            defined-function calls: valence,
                                   frame, body, branch
               apl-clock           clock: the time, the date and the
                                   processor time, how a span and a
                                   moment read, and whether a session has
                                   a clock
               apl-console         terminal: what a statement shows, and
                                   where it reads a line
               apl-eval            evaluator and workspace
               apl-libraries       libraries beyond 0 and 1: configured,
                                   read-only, from a directory or from
                                   fetched text
               apl-modes           modes: which modes a workspace runs
                                   in, and the libraries each mode sees
               apl-quad            quad and quote-quad input: prompting
                                   and reading a line
               apl-saved           saved half of a workspace: symbol
                                   table, activation stack, settings
               apl-shelves         libraries as each mode sees them:
                                   listing, loading, saving and dropping
                                   by mode
               apl-space           workspace accounting: what a value
                                   costs and what a quota allows
               apl-store           workspace libraries: somewhere to keep
                                   a saved workspace
               apl-transcript      console on paper: keeps what it is
                                   shown, answers reads from loaded lines
               apl-uses            which modes a workspace runs in, from
                                   what its code uses
               apl-workspace       active workspace: variables,
                                   functions, environment, output
               apl-wsfile          workspace file: a saved workspace as
                                   re-executable APL
    session/   apl-commands        system commands: the lines that begin
                                   with a parenthesis
               apl-copy            )COPY and )PCOPY: taking names out of
                                   a stored workspace
               apl-editor          del editor: definition mode, line
                                   editing, display
               apl-extensions      own system commands, which no
                                   historical system had: )DIALECT,
                                   )HELP, )LIBS
               apl-inquiry         inquiry commands: listing names,
                                   groups, erasing
               apl-library         workspace libraries: where a file is,
                                   and what to say when it is not there
               apl-reply           session reply: the lines one input
                                   produced, and what the shell does next
               apl-session         session: line in, transcript lines out
               apl-settings        workspace settings: index origin,
                                   digits, width and random link, each
                                   checked against what it may be
    a70/       apl-a70-commands    system commands only the (A) '70 mode
                                   has: the settings and group commands
               apl-ibeam           I-beam system functions, which only
                                   the (A) '70 mode has
                                   (only (A) '70 reaches these)
    b75/       apl-console-control console control: the IBM 5110's own
                                   system function, which only the (B)
                                   '75 mode has
               apl-execute         execute: a character vector run as a
                                   line of APL, which only the (B) '75
                                   mode has
               apl-fix             canonical representation and fix: a
                                   defined function as characters and
                                   back, which only the (B) '75 mode has
               apl-format          (B) '75: format, monadic and dyadic
               apl-numeral         (B) '75: a number rounded and laid out
                                   in decimal or scaled form, for format
               apl-sysfns          system functions: quad-named functions
                                   on the workspace's names and
                                   definitions, which only the (B) '75
                                   mode has
               apl-sysvars         system variables: the quad-named
                                   settings and values, which only the
                                   (B) '75 mode has
                                   (only (B) '75 reaches these)
    c80/       apl-c80-cuts        cuts: what the (C) '80 mode leaves
                                   out of a statement and the mixed
                                   functions, after APL80's Limitations
               apl-c80-scalar      scalar cuts: the scalar functions the
                                   (C) '80 mode leaves out, the
                                   hyperbolic circle functions and
                                   factorial of a fraction
                                   (each asks the session's profile, so
                                   only (C) '80 is cut)
    cli/       apl-config          library configuration: --lib
                                   N=DIR[,NAME] and sw-apl.toml, for the
                                   CLI and the service
               sw-apl              clean-room APL interpreter, (A) '70,
                                   (B) '75 and (C) '80: terminal REPL
                                   and batch runner
    web/       apl-board           the 2741 keyboard in a browser: one
                                   keystroke in, the line out
               apl-serve           service: one session per connection,
                                   over any link
               apl-wasm            in a browser: the session in a worker,
                                   on a shared channel
               apl-wasm-store      in a browser: the libraries a session
                                   keeps, from the page's storage, the
                                   bundle, and URLs
               apl-wire            terminal protocol: one frame of
                                   transcript and prompt
               sw-apl-server       service: the APL\360 session a 2741
                                   dials into
    term/      apl-keyboard        2741 keyboard: key translation,
                                   overstrike composition, the line being
                                   typed
               apl-paper           2741 paper: what the carriage has put
                                   on the page
               apl-typing          2741 keystrokes: reading one line at
                                   the keyboard
               aplterm             the 2741 terminal: a keyboard, a page,
                                   and a line to the service
  data/        glyphs.toml, the glyph tables; help.txt, the )HELP pages
  docs/        reference, plan and design; literate/, the literate
               documents; emacs/, the Emacs mode and Org Babel language
  pages/       the browser demo, built and committed; literate/ its pages
  samples/     conformance corpus (.apl transcripts)
  tests/reg-rs reg-rs regression baselines
  scripts/     changes, samples, reg-rs, pages, literate, gates
  ws/lib1/     shipped library workspaces: LEARN, LIFE, RACE, EDIT,
               BIRDS (one for each mode), TTTML ((B) only)
```

The tree is each crate's own `description` from its `Cargo.toml`, so
it can be checked against them.

## Dependency flow

```
glyphs -> value -> display -> console
value -> lex (and strike) -> scan -> parse
value -> c80 (scalar cuts) -> prims (dispatch; scalar, mixed, ops and the rest)
prims + console + clock + modes -> workspace -> call, quad
parse + prims + call + quad + a70 + b75 + c80 -> eval
eval -> session (commands, editor, library, extensions) -> cli (+ config)
eval -> session -> web (wire, serve, server; wasm, board, wasm-store)
lex (strike) -> term (keyboard, paper, typing, aplterm) -> wire
```

- `apl-glyphs` generates its tables from `data/glyphs.toml`, the
  one list of every glyph sw-apl knows; `apl-value` re-exports them.
  Nothing else holds a copy.
- `apl-value` depends only on `apl-glyphs`.
- `apl-lex`, `apl-scan`, and `apl-parse` never depend on
  `apl-prims` or `apl-eval`; the parser produces an AST, it does
  not evaluate.
- `apl-scan` holds what the parser reads straight off the token
  stream before it recurses: matching brackets, top-level
  semicolon segments, what ends an operand, and the del header.
- `apl-workspace` owns the symbol table (variables and defined
  functions) and the call frames that make scoping dynamic. A local
  name hides whatever the name holds, a function as well as a
  variable, and gives it back on return.
- `apl-saved` is the half of a workspace a file holds, as plain
  data: the symbol table, the activation stack with what each call's
  locals hid, and the settings. `unwound` gives it with every call
  returned, which is what `)SAVE` writes.
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
- `apl-clock` owns the `Clock` the workspace holds, and whether a
  session has a clock at all (`--clock`). A clear workspace has a
  clock that does not move, so nothing reads the real world until a
  host installs one that does, and a transcript made without a
  terminal reproduces. `apl-ibeam`, which only (A) reaches, answers
  the I-beams from it; (B)'s `⎕TS` reads it only with a clock.
- `apl-modes` knows the two modes, what a host decides (`Host`: the
  quota, the libraries, the mode, the clock), and the `⍝!MODES` line;
  `apl-shelves` shows each mode only the workspaces that run in it.
  `apl-libraries` adds libraries 2 and up, read-only, and
  `apl-config` (in `cli/`) reads them from `--lib` and `sw-apl.toml`
  for the CLI and the service; `apl-wasm-store` does the same for
  the browser.
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
  `apl-commands`' neighbor rather than its contents: that crate
  was at its module budget, and listing names has little to do
  with reading and writing workspace files.
- `apl-extensions` answers the commands sw-apl adds -- `)DIALECT`,
  `)LIBS`, `)HELP` -- and `apl-reply` is what the session hands back
  for one line.
- `apl-session` owns everything that begins with a right
  parenthesis, the del editor, and a system command typed in reply
  to quad input. It exposes a line-oriented `Session` API: feed a
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
   with the printing precision and width: `)DIGITS` and `)WIDTH`
   in (A), `⎕PP` and `⎕PW` in (B), one setting either way.
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

`apl_value::AplError { kind, caret: Option<usize>, context }` with
kinds SYNTAX, VALUE, DOMAIN, RANK, LENGTH, INDEX, WS FULL, DEFN,
CHARACTER, DEPTH, INTERRUPT, and NONCE (in (B), for what the 5110
does not do). The caret is a character offset
into the source line so the session can print the APL\360 caret
line. Each crate maps its own failures into `AplError`.

## Adding a mode

(A) '70 and (B) '75 are built, and (C) '80 is being built (`mode-c.md`);
(D) '83, IBM PC APL, a punched-card APL like Maryland's, and the
APL\1130 subset of APL\360 may follow. Each is added without moving
the modes before it, in five layers:

1. **The core stays APL\360.** The shared crates know nothing of
   modes. What only some modes have is a crate of its own, under the
   directory of the mode that first needed it (`a70/`, `b75/`, `c80/`), and a
   later mode reuses it rather than copying: (D) is mostly (B)'s
   crates, (C) and APL\1130 mostly the core with parts turned off.
2. **A mode is a profile, not a match.** Code asks what the session's
   profile has -- execute, the del editor, the I-beams, its limits on
   functions and lines -- not which mode it is in. Adding a mode adds
   a profile and a letter to `apl-modes`, not an arm to every `match`
   on `Mode`. `Mode::profile` answers `has(Feature)` -- the I-beams,
   the settings commands, scan, a latent expression -- and
   `cuts(Cut)`, with the limits on functions and lines; the checks
   that were matches on `Mode` ask it, each move covered by the tests
   already there.
3. **Typing and display are translated at the edges.** The lexer and
   everything after it see glyphs. Between them and the screen, a
   mode's surface turns what is typed into glyphs and what is shown
   back: APL80's `.P`, `$` and `%` and its dotted letters on a 64-by-16
   screen, a keypunch's upper case, a code page. (A) and (B) have the
   identity surface. A workspace is stored in glyphs, whatever the
   mode that runs it.
4. **What the grammar itself changes is a parser option.** APL80's
   round-bracket indexing and names with `.` and `-`, and
   `)DEF`/`)EDIT` in place of the del, are not spellings: they change
   how a line parses or how a function is entered. Each is an option
   the profile sets, off by default, so that (A) and (B) parse exactly
   as before; each is tested in both states.
5. **Nothing before moves, and that is proved.** Every sample has a
   reg-rs transcript in its mode, and a new mode brings its own. A
   change to an earlier mode's transcript fails the gate unless it is
   a correctness fix to that mode, with its source. The CLI starts in
   (A), the demo's `?mode=A` and `?mode=B` keep their meaning, and a
   workspace's mode letters (`NAME.<modes>.apl.ws` and its `⍝!MODES`
   line) keep it out of every mode it was not written for.

The order follows the risk: (C) first, since it exercises the surface
and the parser options hardest; then APL\1130, mostly a profile with
parts off; then (D), mostly (B); then the punched-card mode, whose
input arrives as a deck and needs the most new session work.

## Generated tables

`data/glyphs.toml` is the single source of truth for every glyph:
the primitives with their monadic and dyadic names, the punctuation
and sentinels, the lookalikes a CHARACTER ERROR should redirect, the
later-APL glyphs it should name (and which of them (B) accepts), the
characters of the set with no meaning, which are a SYNTAX ERROR,
the overstrikes, and the underscored alphabet. Two consumers read
it, and nothing else may hold a copy:

- `components/glyphs/crates/apl-glyphs/build.rs` emits Rust consts
  into `OUT_DIR`, which `apl-glyphs` includes and `apl-value`
  re-exports. The lexer takes its accepted set from there, and the
  error display its hint tables.
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
