//! The active workspace: variables, defined functions, the evaluation
//! environment (index origin, random link), call frames for dynamic
//! scoping, pending output, and the shape of what a statement
//! produces for the terminal.

mod frame;
mod funcs;
mod workspace;

pub use apl_ast::Defn;
pub use apl_prims::Env;
pub use frame::Frame;
pub use workspace::{Output, Workspace};
