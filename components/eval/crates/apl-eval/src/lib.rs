//! sw-apl evaluator: walks the AST against a workspace of variables
//! and dispatches primitives.

mod apply;
mod eval;
mod workspace;

pub use apl_prims::Env;
pub use eval::{eval_expr, eval_line};
pub use workspace::{Output, Workspace};
