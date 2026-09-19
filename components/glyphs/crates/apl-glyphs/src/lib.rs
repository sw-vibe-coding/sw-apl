//! The glyph tables: every character sw-apl knows, what it means,
//! and what to say about the ones it rejects.
//!
//! Generated from `data/glyphs.toml` by `build.rs`. Edit that file
//! and run `scripts/gen-glyphs.sh`; never edit the generated output,
//! and never keep a second copy of this data anywhere else.

mod columns;

pub use columns::{columns, pad};

include!(concat!(env!("OUT_DIR"), "/glyphs.rs"));
