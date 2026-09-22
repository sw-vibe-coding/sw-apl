//! The workspace settings -- index origin, printing precision,
//! printing width and random link -- each checked against the values
//! it may take -- and the clear workspace each mode starts with.
//!
//! Three things set them: the directives a saved workspace carries,
//! which every mode reads; the `)ORIGIN`, `)DIGITS` and `)WIDTH`
//! commands, which only '70 has and `apl-a70-commands` holds; and
//! `⎕IO`, `⎕PP`, `⎕PW` and `⎕RL`, which only '75 has and `apl-sysvars`
//! holds. All go through `setting`, so none can set what another
//! would refuse, and a directive always reads back what was saved.

mod clear;
mod setting;

pub use clear::clear;
pub use setting::setting;
