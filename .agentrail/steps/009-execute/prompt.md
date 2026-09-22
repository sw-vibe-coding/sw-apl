Phase 9 step 6 (docs/plan.md). The glyph table learns the mode a glyph
arrives in; '75 gains execute.

data/glyphs.toml's [[later]] entries are what the lexer refuses as
not APL\360. Give each glyph the mode it arrives in, so that the lexer
reads the profile: execute and format are '75 glyphs, and in '68 they
are refused exactly as now -- the same CHARACTER ERROR, the same
words. Everything still later than APLSV stays refused in both.

Then execute, in a new '75-only crate: monadic, a character vector
evaluated as a line in the current workspace, per docs/aplsv.md. Its
errors are the line's errors. Sample for '75 (the sample harness
needs a way to run a sample in a mode; add it). reg-rs for every '68
sample unchanged.
