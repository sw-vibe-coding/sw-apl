//! APL80's characters, one at a time (docs/mode-c.md, The keys): each
//! with the glyph it stands for, as data -- in a string, a comment, a
//! reply to quote-quad -- and shown back. What a character means by
//! where it stands -- a round bracket, an arrow, a minus -- is
//! `apl-c80-keys`'.

mod shown;
mod tables;

pub use shown::{named, shown};
pub use tables::{data, name, text};
