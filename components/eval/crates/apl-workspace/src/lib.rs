//! The active workspace: variables, defined functions, the evaluation
//! environment (index origin, random link), the activation stack that
//! makes scoping dynamic and doubles as the state indicator, pending
//! output, and the shape of what a statement produces.

mod frame;
mod funcs;
mod workspace;

pub use apl_ast::Defn;
pub use apl_prims::Env;
pub use frame::Activation;
pub use workspace::{Output, Workspace};
