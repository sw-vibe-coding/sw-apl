# sw-apl Product Requirements

## Summary

sw-apl is a clean-room APL interpreter written in Rust that
reproduces the experience of classic IBM APL on a terminal:
traditional glyphs, the six-space indent prompt, printer-style
transcript output, del-editor function definition, and the system
commands for workspaces. It targets macOS and Linux first and a
browser (Yew/WASM) second, sharing the same interpreter crates.

It has two modes (owner direction 2026-09-21):

- **(A) '70, APL\360**, as on the IBM 2741. Everything below
  describes this mode unless it says otherwise.
- **(B) '75**, modelled on the APL of the IBM 5100, 5110 and 5120
  desktop computers -- APLSV as IBM adapted it for one user and a
  64-column screen. (A) with execute, format, and the quad system
  variables and functions added, and the I-beams and the `)ORIGIN`,
  `)DIGITS`, `)WIDTH` commands removed, as the IBM 5110 APL
  Reference Manual has it.

The mode is chosen at the CLI and the service with a flag and in the
browser with a tab, and nothing done for (B) changes (A).

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
- The APL\360 system interface: I-beam functions (time, date,
  workspace space, state indicator), `)ORIGIN`, `)DIGITS`, and
  `)WIDTH` settings, quad and quote-quad I/O. In (A), no APLSV quad
  system variables or functions, no execute, no format.
- In (B): execute, format, and the quad system variables and
  functions, as the IBM 5110 APL Reference Manual defines them. The
  I-beams and the settings commands are not in (B); `⎕IO`, `⎕PW`
  and `⎕PP` replace the commands. `docs/mode-b.md` has the whole
  list.
- Domino (matrix divide) in a later phase.

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
- A workspace names the modes it runs in -- (A), (A)(B) or (B) --
  from what it uses, and is listed and loaded only in those modes.
  Library 1 differs by mode where the workspaces do.
- Each shipped workspace carries a DESCRIBE that prints what the
  workspace holds and how to use it. `)LOAD` prints only the line
  saying when the workspace was saved, as APL\360 did; typing
  DESCRIBE is the reader's move.

### Input

- Glyphs are entered as Unicode. The interpreter provides no
  Latin keyword aliases and does no name-to-glyph translation.
- Documentation covers typing glyphs with Espanso and with Emacs.

### Delivery

- `sw-apl` binary: interactive REPL, `-f FILE` batch, stdin batch.
- Builds and runs on macOS (Apple silicon and Intel) and Linux.
- Later: browser build via Yew and Trunk, deployed to GitHub Pages.

## What it must not do

- No nested arrays, each, enclose, pick, dfns, or diamonds, in
  either mode: they are APL2 and later, and APL2 is not planned.
- No shared variables, in either mode: they exist to talk to other
  processes and to devices, and sw-apl has one user and no hardware.
- No hardware I/O. The I-beams are APL\360's own and no more.
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
