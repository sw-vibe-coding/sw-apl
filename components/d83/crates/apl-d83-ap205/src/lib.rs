//! (D) '83's AP205, the full-screen auxiliary processor of IBM PC APL
//! (APL Reference, May 1983, chapter 3), as a processor the shared
//! variables couple to. A program offers it a control variable, whose
//! name begins with C, and a data variable, beginning with D; then
//! each value set in the control variable is a command, answered with
//! a return code, and the data variable carries what the command
//! writes or reads. The screen it keeps is `apl-d83-screen`'s. With no
//! keyboard to wait on, interactive mode (commands 3 and 8) answers as
//! though Enter were pressed at once. See docs/ibm-pc-apl-plan.md.

mod command;
mod processor;

pub use processor::Ap205;
