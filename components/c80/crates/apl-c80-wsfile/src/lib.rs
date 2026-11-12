//! A workspace saved in (C) '80, written as APL80 would type it: each
//! function as `)DEF` with its header, its lines, and the empty line
//! that stands for BREAK; the latent expression as `)AUTO`. It runs in
//! (C) alone (owner, 2026-11-11): the heading, the variables and the
//! settings are written as every mode writes them, by `apl-wsfile`.

mod write;

pub use write::write;
