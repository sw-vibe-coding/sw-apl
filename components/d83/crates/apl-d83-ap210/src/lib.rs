//! (D) '83's AP210, the DOS file processor of IBM PC APL (APL
//! Reference, May 1983, pages 3-21 to 3-28), as a processor the shared
//! variables couple to. Up to four pairs of variables, a control
//! variable whose name begins with C and a data variable with the same
//! name after a D. A character vector set in the control variable is
//! a command -- open to read (IR), to read and write (IW), delete
//! (DL), rename (RN) -- and, once a file is open, a numeric vector is
//! a subcommand: read or write, which record, how long; an empty one
//! closes the file. Each is answered with a return code in the
//! control variable, and a read with the record in the data variable.
//! A drive's files are library 0's (`apl-d83-dosfile`). See
//! docs/ibm-pc-apl-plan.md.

mod command;
mod processor;
mod transfer;

pub use processor::Ap210;
