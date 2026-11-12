//! The TRS-80's memory, as (C) '80 models it once `)RAM` opens it
//! (docs/mode-c.md): the 64-by-16 screen, which is what (C) has shown
//! and so what a peek there reads, and plain RAM above it. There is no
//! ROM in it -- the ROM is not ours to ship -- and no keyboard or
//! devices; what reaches them is `apl-c80-memory`'s to refuse.

mod machine;

pub use machine::{Machine, RAM, SCREEN};
