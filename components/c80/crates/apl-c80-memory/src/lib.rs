//! (C) '80's reach into the machine (docs/mode-c.md): peek and poke,
//! the shifted U, which (C) holds as `⌷`, and Call, dyadic `#`, on the
//! memory `)RAM` opens (`apl-c80-machine`). Status and the rest of `#`
//! are `apl-c80-status`'s; the evaluator asks here for all of them.

mod entry;
mod memory;

pub use entry::system;
