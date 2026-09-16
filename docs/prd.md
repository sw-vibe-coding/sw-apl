# sw-apl Product Requirements

## Summary

sw-apl is a clean-room APL interpreter written in Rust that
reproduces the experience of classic IBM APL\360 on a terminal:
traditional glyphs, the six-space indent prompt, printer-style
transcript output, del-editor function definition, and the
APL\360 system commands for workspaces. It targets macOS and
Linux first and a browser (Yew/WASM) second, sharing the same
interpreter crates.

## Who it is for

- The project owner, for teaching and demonstrating classic APL
  the way it was used on IBM terminals through the 1970s and
  early 1980s, before APL2.
- Learners following the Software Wrighter lab material who want
  a small, readable APL they can run locally or in a browser.
- Anyone comparing array-language designs who wants a faithful,
  minimal APL\360 reference that is not APL2 or Dyalog.

## What it must do

### Language

- Flat arrays only: scalars, vectors, matrices, and higher rank.
- One numeric type semantically (floating point is first-class),
  with integer representation used when exact; characters as a
  second element type; boolean results are numbers 0 and 1.
- The complete APL\360 primitive set: scalar functions, mixed
  functions, the reduce, scan, inner product, outer product
  operators, axis brackets, indexing and indexed assignment.
- User-defined functions via the del editor: niladic, monadic,
  dyadic, with or without explicit result, local names, labels,
  branching, dynamic scoping, recursion.
- APLSV-era system variables and functions: quad-IO, quad-PP,
  quad-PW, quad-CT, quad-RL, quad-LX, quad-EX, quad-NL, quad-NC,
  quad-FX, quad-CR, quad-TS, quad-AI, quad-WA, quad-DL, quad-LC.
- Execute and format (APLSV) in a later phase; domino (matrix
  divide) in a later phase.

### Session

- Six-space indent as the input prompt; output begins in column
  one. Function definition mode prompts with `[n]` in brackets.
- Error reports in the APL\360 style: error name on one line, the
  statement echoed, a caret marking the point of detection, and
  `FN[n]` prefixes inside functions.
- System commands starting with a right parenthesis: workspace
  control, inquiry, and settings, following the APL\360 names.
- Workspaces are saved to and loaded from plain text files; a
  numbered library convention maps `)LOAD 1 NAME` to directories.
- Each shipped workspace carries a niladic DESCRIBE function that
  prints what the workspace holds and how to use it.

### Input

- Glyphs are entered as Unicode. The interpreter provides no
  Latin keyword aliases and does no name-to-glyph translation.
- Documentation covers typing glyphs with Espanso and with Emacs.

### Delivery

- `sw-apl` binary: interactive REPL, `-f FILE` batch, stdin batch.
- Builds and runs on macOS (Apple silicon and Intel) and Linux.
- Later: browser build via Yew and Trunk, deployed to GitHub Pages.

## What it must not do

- No nested arrays, each, enclose, pick, dfns, or diamonds.
- No shared variables, no hardware I/O, no I-beams.
- No dependency on GNU APL or on the C interpreter at runtime.

## Quality bar

- Every step is test-first. Unit tests per crate, reg-rs
  transcript regressions per sample program.
- `sw-checklist` passes with zero failures; warnings are kept at
  zero by designing to the gates.
- `cargo clippy -D warnings` clean, `cargo fmt` clean.
- Markdown passes `sw-markdown-checker` (ASCII only).

## Success looks like

Typing the horse race program from `samples/` into the REPL,
saving it with `)SAVE HORSES`, restarting, `)LOAD HORSES`,
typing `DESCRIBE`, and running it, all feeling like a 2741
terminal session with a modern keyboard.
