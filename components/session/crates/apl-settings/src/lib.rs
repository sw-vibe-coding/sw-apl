//! The workspace settings -- index origin, printing precision and
//! printing width -- each checked against the values it may take.
//!
//! Two things set them: the directives a saved workspace carries,
//! which every mode reads, and the `)ORIGIN`, `)DIGITS` and `)WIDTH`
//! commands, which only '68 has and `apl-a70-commands` holds. Both go
//! through `setting`, so a directive can never set what the command
//! would refuse.

mod setting;

pub use setting::setting;
