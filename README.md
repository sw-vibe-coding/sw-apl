<p align="center">
  <img src="images/sw-apl-logo.png" alt="sw-apl logo" width="256">
</p>

# sw-apl

A clean-room APL\360 interpreter written in Rust, from scratch.

sw-apl brings back classic IBM APL the way it was used on a
terminal: traditional glyphs typed as Unicode, the six-space
indent prompt, printer-style transcript output, the del editor
for defining functions, and the APL\360 system commands for
workspaces (`)CLEAR`, `)WSID`, `)SAVE`, `)LOAD`, `)FNS`,
`)VARS`, ...). It is a command-line program for macOS and Linux;
a browser version built with Yew and WebAssembly follows.

It is deliberately not APL2 and not Dyalog: flat arrays only, no
nested arrays, no each. It is also not a port. The C interpreter
`sw-cor24-apl` and GNU APL served only as references for expected
behaviour and for the conformance corpus in `samples/`.

## Status

Early. The `sw-apl` binary evaluates scalar arithmetic at the
six-space prompt: numeric literals with the high minus, strands,
plus, minus, times, divide, maximum, minimum, residue, power
(monadic and dyadic, with scalar extension), parentheses,
assignment, variables, monadic rho, and APL\360 error display
with the caret. Anything else answers `NOT IMPLEMENTED` or a
CHARACTER ERROR naming the code point. Batch mode, `)OFF`, help,
and the version block work. Implementation proceeds phase by
phase per `docs/plan.md`: iota, reshape, and reduce next, then
the full value, display, lexer, parser, and scalar-function
layers, mixed functions and operators, defined functions and the
session, workspaces, numerics, then the web demo.

## Summary

| Area | What sw-apl provides |
|---|---|
| Syntax | Traditional glyphs only (see `docs/glyphs.txt`), right-to-left evaluation, strands, axis brackets |
| Data | Flat arrays of any rank; one numeric type with integer fast path and floating point; characters |
| Primitives | The APL\360 scalar and mixed functions; reduce, scan, inner and outer product; indexing |
| Functions | Del editor, niladic/monadic/dyadic headers, locals, labels, branching, recursion |
| System | Quad system variables and functions (APLSV names), APL\360 system commands, workspaces on disk, the DESCRIBE convention |
| Session | Six-space indent prompt, APL\360 error display with caret, batch transcripts |
| Input | Espanso and Emacs keymaps for typing glyphs (`docs/input-methods.md`) |

This README is plain ASCII so it renders the same everywhere; the
documents below show real APL glyphs:

- [Master plan](docs/plan.md) -- phases, decisions, what comes next
- [Language reference](docs/language.md) -- the APL\360 subset and
  exactly which Unicode is accepted
- [Glyph table](docs/glyphs.txt) -- every glyph with its code point
- [Session](docs/session.md) -- prompt, error display, system
  commands, the DESCRIBE convention
- [Typing glyphs](docs/input-methods.md) -- Espanso and Emacs
  keymaps, OS layouts
- [Testing](docs/testing.md), [Architecture](docs/architecture.md),
  [Design decisions](docs/design.md), [Requirements](docs/prd.md)

## Building

Requires a Rust toolchain (edition 2024, stable). Each directory
under `components/` is its own cargo workspace and they share one
`target/` at the repository root.

```bash
# Debug build and tests for the CLI workspace
cd components/cli
cargo test
cargo build

# Release binary at target/release/sw-apl (from the repo root)
just release

# Run
target/release/sw-apl                    # interactive session
target/release/sw-apl -f samples/06-reduce.apl
printf '2+2\n)OFF\n' | target/release/sw-apl
target/release/sw-apl --help
```

With [just](https://github.com/casey/just) installed, `just test`,
`just clippy`, `just fmt-check`, and `just gates` run the project
gates across every workspace. Regression transcripts use
[reg-rs](https://github.com/softwarewrighter) via `scripts/reg.sh`.

## Repository layout

```
components/   one cargo workspace per component (cli today)
docs/         plan, requirements, architecture, language, session
samples/      conformance corpus: glyph-form APL programs
scripts/      change log, sample runner, reg-rs wrappers
images/       logo
```

## Links

- Blog: [Software Wrighter Lab](https://software-wrighter-lab.github.io/)
- Discord: [Join the community](https://discord.com/invite/Ctzk5uHggZ)
- YouTube: [Software Wrighter](https://www.youtube.com/@SoftwareWrighter)

## Copyright

Copyright (c) 2026 Michael A Wright. See [COPYRIGHT](COPYRIGHT).

## License

MIT License. See [LICENSE](LICENSE).
