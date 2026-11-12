//! The system commands (C) '80 has of APL80's own (docs/mode-c.md):
//! `)TRON` and `)TROFF`, a trace of each line before it runs; `)PS`
//! and `)PD`, numbers printed in six digits or fifteen; and `)AUTO`, a
//! latent expression. `)EXEC`, which runs it, and `)DEF` and `)EDIT`,
//! which open a definition, are the session's.

mod command;

pub use command::command;
