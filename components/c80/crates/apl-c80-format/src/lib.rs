//! APL80's format statement (docs/mode-c.md). Where APL writes
//! `6 3⍕OUTPUT`, APL80 writes the format first, as a statement of its
//! own -- `f 6 3`, which (C) types as the format glyph it stands for --
//! and every number shown after it is laid out so, until a new format,
//! an error, a system command, or character output. The layout is
//! (B)'s dyadic format, reused.

mod statement;

pub use statement::{shaped, statement};
