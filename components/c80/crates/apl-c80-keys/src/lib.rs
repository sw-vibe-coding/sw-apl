//! APL80's characters, as (C) '80 types and shows them (docs/mode-c.md,
//! The keys). APL80 has no APL keyboard: a shifted letter is the
//! lower-case code, which its screen draws with a dot, so `p` is rho;
//! `% @ $ & "` stand for divide, log, not-equal, and and jot; the
//! minus is the negative sign and the underscore subtract; a monadic
//! up or down arrow grades; round brackets index; `1d` chooses the
//! axis. Names may hold `.` and `-`.
//!
//! The lexer reads glyphs, so a line typed in (C) is translated into
//! them when it runs, knowing which names are functions then, and what
//! is shown is translated back. Inside the glyphs a name's `.` is `⍙`
//! and its `-` is `∆`, neither of which APL80 can type.

mod chars;
mod marks;
mod tokens;
mod typed;

pub use chars::{name, named, shown};
pub use marks::text;
pub use typed::{Typed, glyphs};
