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
    parse/     apl-parse                   right-to-left parser, AST
    prims/     apl-prims-scalar            scalar primitives
               apl-prims-mixed             structural primitives
               apl-prims-ops               reduce/scan/inner/outer
    eval/      apl-eval                    interpreter, symbol table
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
value -> display
value -> lex -> parse
value -> prims (scalar, mixed, ops)
parse + prims + display -> eval
eval -> session -> cli
eval -> session -> web
```

- `apl-value` has no dependencies inside the repo.
- `apl-lex` and `apl-parse` never depend on `apl-prims` or
  `apl-eval`; the parser produces an AST, it does not evaluate.
- `apl-eval` owns the symbol table, the state indicator, and
  dispatch from AST to primitives.
- `apl-session` owns everything that begins with a right
  parenthesis, the del editor, workspace files, and the library
  directory map. It exposes a line-oriented `Session` API: feed a
  line, receive output lines. The CLI and the web demo are both
  thin shells over that API.

## Pipeline for one input line

1. `apl-session` classifies the line: system command, del editor
   line (when in definition mode), or immediate-execution
   statement.
2. `apl-lex` turns a statement into tokens (glyphs, numbers,
   strings, names, quad names).
3. `apl-parse` builds the AST right to left.
4. `apl-eval` evaluates the AST against the workspace, producing
   a value or an error with a caret position.
5. `apl-display` formats the value (or the error transcript)
   with quad-PP and quad-PW.
6. The shell prints the lines and shows the next prompt.

## Value model

`Array { shape: Vec<usize>, data: Data }` where `Data` is either
`Num(Vec<Number>)` or `Char(Vec<char>)`. `Number` is `Int(i64)` or
`Float(f64)`; arithmetic promotes to `Float` on overflow or
non-integral results and demotes back to `Int` when a result is
exactly integral within quad-CT. Scalars are rank-0 arrays.

There is no nested `Data` variant. That is deliberate: this is
APL\360, not APL2.

## Error model

`apl-error::AplError { kind, caret: Option<usize>, context }` with
kinds SYNTAX, VALUE, DOMAIN, RANK, LENGTH, INDEX, WS FULL, DEFN,
CHARACTER, DEPTH, and INTERRUPT. The caret is a character offset
into the source line so the session can print the APL\360 caret
line. Each crate maps its own failures into `AplError`.

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
