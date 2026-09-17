//! The active workspace: variables, the evaluation environment
//! (index origin, random link), pending quad output, and the shape
//! of what a statement produces for the terminal.

mod workspace;

pub use apl_prims::Env;
pub use workspace::{Output, Workspace};
