Phase 8 step 2 (docs/plan.md). The paper and the overstrikes.

Make the client a 2741. It is a printing terminal: output starts in
column one, input is indented six spaces, definition mode prompts
`[n]`, and a line `⍞←` left open carries into what is typed next --
the CLI learned that in Phase 7 and the protocol already carries the
flag, so the terminal should need no new decisions about it.

The distinctive part is overstriking. On a 2741 you form a glyph by
typing one character, backspacing, and typing another over it: `⍟`
is `○` backspace `*`, `⍋` is `∆` backspace `|`. Only the terminal
sees those keystrokes; by the time a line reaches the server it is
the composed glyph. So the terminal owns:

- The overstrike table. It belongs in `data/glyphs.toml`, which
  says of itself that nothing else may hold a copy of this data,
  and `scripts/gen-glyphs.sh` regenerates `docs/glyphs.txt` from
  it. Add the pairs there, regenerate, and let the terminal read
  the generated table -- that is the reason this is Rust compiled
  to wasm rather than JavaScript with a second copy.
- Backspace itself. Decide what backspace does when there is
  nothing to strike over: on real paper it moves the carriage and
  the next character overstrikes whatever was there. Whether the
  terminal models the carriage or only the composed line is the
  design question of this step, and the answer should be written
  down rather than implied by the code.
- ATTN. The 2741's attention key is what interrupted a running
  function; the CLI uses Ctrl-C and answers INTERRUPT. Wire it, so
  a runaway `⍳` can be stopped.

Get the overstrike pairs from a source, not from memory. The APL
character set's overstrikes are documented; say in the commit where
the table came from and which pairs you could not confirm.

The transcript must be the CLI's. Same input, same output,
character for character -- that is a test, not an intention: run a
sample through both and compare.

Keep it usable: selectable and copyable text, and a screen reader
should be able to read the transcript.
