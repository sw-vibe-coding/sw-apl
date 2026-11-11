//! What (C) '80 leaves out of a statement and of the mixed functions,
//! after the "Limitations" of APL80's manual (docs/mode-c.md). Each is
//! asked of the session's profile, so a mode that leaves nothing out
//! passes straight through. The scalar functions it leaves out are
//! `apl-c80-scalar`'s, which `enter` sets for the line.

mod mixed;
mod statement;

pub use mixed::mixed;
pub use statement::enter;
