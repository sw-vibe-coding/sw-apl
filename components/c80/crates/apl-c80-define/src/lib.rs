//! Definition mode in (C) '80, as APL80's manual describes it: `)DEF`
//! with a header opens a new function and `)EDIT` with a name an old
//! one, each asking for the first unused line; `)?` shows the
//! function, `)P n` fourteen lines of it, `)n` replaces line n, `)0` the
//! header, and `)E n` offers line n to type over. Every change
//! renumbers at once. BREAK leaves, which a line-at-a-time terminal
//! types as an empty line. There is no del (docs/mode-c.md).

mod keyed;
mod line;

pub use keyed::{Keyed, Typed};
pub use line::header;
