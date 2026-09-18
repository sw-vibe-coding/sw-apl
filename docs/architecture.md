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
               apl-ibeam                   I-beam values and the clock
               apl-eval                    interpreter
    session/   apl-session                 system commands, del editor,
                                           workspace files, libraries
    cli/       sw-apl                      terminal REPL and batch
    web/       (later) Yew/WASM demo
  docs/        planning and reference docs
  samples/     conformance corpus (.apl transcripts)
  tests/reg-rs reg-rs regression baselines
  scripts/     gen-changes, run-samples, reg wrappers
  ws/lib1/     shipped library workspaces (DESCRIBE convention)
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
eval -> session -> web
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
- `apl-quad` reads that line: quad evaluates the reply, quote-quad
  takes it as characters. Like `apl-call` it is handed `Run` rather
  than depending on the evaluator.
- `apl-eval` owns the state indicator and dispatch from AST to
  primitives.
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

## Web build (later)

`apl-session` must compile for `wasm32-unknown-unknown`: no
threads, no blocking stdin, file access behind a trait so the
browser can back workspaces with local storage.
