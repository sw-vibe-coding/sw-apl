//! (D) '83's transfer files, `NAME.AIO`: what `)OUT` writes and `)IN`
//! reads. IBM PC APL's own were binary; sw-apl's are UTF-8 text, like
//! its workspaces -- a line saying when the file was written, then
//! each object's transfer form (`apl-d83-transfer`) on a line of its
//! own. A form's shape says how long it is, so a character value
//! holding a line ending is read back whole.

mod file;
mod shelf;

pub use file::{Contents, compose, contents};
pub use shelf::{key, list};
