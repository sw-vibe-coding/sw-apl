# sw-apl Master Plan

This is the single source of planning truth for sw-apl. Every
instruction from the project owner lands here first; agentrail
sagas and steps are derived from the phases below, never invented
ad hoc. When direction changes, edit this file, then re-plan the
saga (`agentrail plan --update`, `agentrail insert`).

Companion docs: `prd.md` (what and why), `language.md` (the APL
subset), `session.md` (terminal look and feel, system commands),
`architecture.md` (crate layout), `design.md` (decisions),
`testing.md` (TDD + reg-rs), `input-methods.md` (typing glyphs).

## Goal

A clean-room, from-scratch implementation in Rust of classic IBM
APL as it looked on a terminal: APL\360 semantics and session
conventions, extended with the APLSV-era system functions and
variables (quad-EX, quad-IO, quad-NL, ...). Traditional glyphs are
the only surface syntax: Unicode input, no Latin keyword aliases,
no name-to-glyph translation layer.

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
- No embedded targets, no shared variables for hardware I/O
  (quad-SVO), no I-beam functions.
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
  with an integer fast path and tolerant comparison (quad-CT).

## Phases

Each phase becomes one agentrail saga. Step slugs are stable
identifiers; prompts are written when the saga is planned.

### Phase 0: bootstrap (saga `bootstrap`)

1. `repo-scaffold` -- COPYRIGHT, LICENSE, .gitignore,
   .gitattributes, .cargo/config.toml, justfile, scripts,
   `/mw-cp` command, CLAUDE.md project notes, README skeleton,
   samples corpus copied from sw-cor24-apl (APL\360-scope only).
2. `planning-docs` -- this plan plus prd, architecture, design,
   language, session, glyphs, testing, input-methods, saga docs.
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
   aligned columns, quad-IO. `plus reduce iota 10` and
   `2 3 rho iota 6` work. reg-rs baselines for `samples/04` to
   `06`. This is the MVP: the owner can play.
3. `value-model-complete` -- tolerant equality (quad-CT),
   promotion and demotion rules, empty arrays, rank > 2, full
   error enum.
4. `display-complete` -- exponential form, quad-PW wrapping with
   six-space continuation, character arrays, empty output, mixed
   int/float columns.
5. `lexer-complete` -- every glyph in `glyphs.txt`, quoted
   strings with doubled quotes, quad names, delta letters, system
   command lines, del sentinel, CHARACTER ERROR for lookalikes.
6. `parser-complete` -- operators reduce/scan/inner/outer with
   axis brackets, bracket indexing, indexed assignment, branch,
   quad and quote-quad on both sides of assignment, first-axis
   glyphs.
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
4. `grade-encode-decode-deal-roll` (quad-RL seeding).
5. `reduce-scan` first-axis forms, axis brackets, scan, identity
   elements for empty reductions.
6. `inner-outer-product`.
7. `indexing-and-indexed-assignment` with quad-IO.
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
5. `quad-io` -- quad and quote-quad input and output, quad-IO,
   quad-PP, quad-PW, quad-CT, quad-RL, quad-LX.
6. `system-functions` -- quad-EX, quad-NL, quad-NC, quad-FX,
   quad-CR, quad-TS, quad-AI, quad-WA, quad-DL, quad-LC.
7. `terminal-feel` -- six-space indent prompt, printer-style
   scrollback, interrupt handling, line editing with Unicode
   input, history file.

### Phase 4: workspaces (saga `workspaces`)

1. `ws-model` -- the workspace as a value: symbol table, saved
   settings, state indicator, DESCRIBE convention.
2. `ws-file-format` -- plain-text UTF-8 workspace file that is
   both human readable and re-executable.
3. `system-commands-ws` -- )CLEAR )WSID )SAVE )LOAD )DROP )LIB
   )COPY )PCOPY )CONTINUE )OFF.
4. `system-commands-inquiry` -- )FNS )VARS )GRPS )GRP )GROUP
   )ERASE )SI )SIV )ORIGIN )DIGITS )WIDTH )SYMBOLS.
5. `library-workspaces` -- numbered public libraries (`)LOAD 1
   CLASS` style) mapped to directories; ship starter workspaces
   each carrying a DESCRIBE function.

### Phase 5: numerics (saga `numerics`)

1. `domino` -- matrix inverse and least-squares divide.
2. `format-execute` -- monadic and dyadic quad-FMT style format
   (APLSV format primitive) and execute.
3. `numeric-edge-cases` -- overflow, tolerance, large iota,
   exponent notation round trips, quad-PP extremes.

### Phase 6: web demo (saga `web-demo`)

1. `wasm-facade` -- session API usable from wasm32.
2. `yew-terminal` -- printer-style terminal component with glyph
   keyboard and Espanso-style expansions.
3. `pages-deploy` -- GitHub Pages build and deploy scripts.

## Owner decisions (2026-09-16)

Answers to the questions raised at bootstrap, now policy:

1. Unicode acceptance is strict and documented exactly in
   `language.md` ("Accepted Unicode"): outside quotes and comments
   only the code points in `glyphs.txt` plus printable ASCII and
   space are valid; anything else, including Greek lookalikes and
   control characters, is CHARACTER ERROR naming the code point.
   Invalid UTF-8 in a file or on stdin is reported as CHARACTER
   ERROR with the byte offset, never a crash.
2. Execute and format (APLSV) stay in Phase 5. Pro: they are
   small, widely expected, and make the horse-race samples run.
   Con: they are not APL\360 proper; the docs label them APLSV.
3. Workspace libraries: numbered libraries map to directories via
   a config file (APL\360 `)LOAD 1 NAME` form) and plain paths
   also work.
4. Markdown: README.md and other top-level markdown stay ASCII;
   `docs/*.md` may use glyphs; README links to docs. If GitHub
   turns out not to render APL glyphs in `docs/*.md`, switch those
   files to Org. Later tooling (not scheduled yet): `ob-sw-apl`
   for literate Org/PDF/HTML with glyphs, and vhs tapes of the
   CLI to embed as images and animations in the ASCII README.

### Phase 7: literate and recorded docs (saga `doc-tooling`, unscheduled)

1. `vhs-tapes` -- vhs tape scripts under `docs/tapes/` rendering
   CLI sessions with glyph input and output to GIF/PNG for the
   README.
2. `ob-sw-apl` -- an Org Babel language for sw-apl so literate
   Org documents can execute APL blocks and export to PDF/HTML.
