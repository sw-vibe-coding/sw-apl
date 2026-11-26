//! A (U) '74 batch run: a deck of cards in, a line printer's listing
//! out (docs/mode-u.md). The run starts with a clear workspace, lists
//! every card, runs the APL ones between `@APL` and `@FIN` through an
//! ordinary session in mode U, and reads data cards for `⎕` and `⍞`.

mod batch;
mod control;
mod statement;

pub use batch::run;
