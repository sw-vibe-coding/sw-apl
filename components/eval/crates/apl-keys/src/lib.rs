//! What a library keeps a thing under, and what that is called as a
//! file: a workspace, `NAME.ab.apl.ws`, or a transfer file,
//! `NAME.AIO`. The stores keep text under keys; the filesystem ones
//! need a file name for each, and this is the one place that says
//! which.

mod key;

pub use key::{SUFFIX, TRANSFER, file_of, key_of};
