//! A workspace file in glyphs, ported to (C) '80: each `∇` definition
//! as a `)DEF` block, every line in APL80's characters, and the `∆`
//! helpers, which APL80 cannot type, renamed with a `D.` prefix
//! (docs/mode-c.md, "Names with ∆"). What the port makes is checked by
//! loading it in (C); a workspace that needs more than this is changed
//! by hand after it.

mod branch;
mod port;
mod sample;

pub use port::port;
pub use sample::sample;
