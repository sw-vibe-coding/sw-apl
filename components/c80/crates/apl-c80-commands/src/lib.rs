//! The system commands (C) '80 has of APL80's own (docs/mode-c.md):
//! `)TRON` and `)TROFF`, a trace of each line before it runs; `)PS`
//! and `)PD`, numbers printed in six digits or fifteen; `)AUTO`, a
//! latent expression; `)RAM`, which opens the memory to peek, poke and
//! call; and `)DOS`. `)EXEC`, which runs the latent expression, `)DEF`
//! and `)EDIT`, which open a definition, and the lines DOS takes are
//! the session's.

mod command;

pub use command::{READY, command};
