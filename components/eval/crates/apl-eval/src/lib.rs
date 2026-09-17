//! sw-apl evaluator: walks the AST against a workspace of variables
//! and dispatches primitives.

mod apply;
mod eval;
mod forms;

pub use apl_workspace::{Env, Output, Workspace};
pub use eval::{eval_expr, eval_line};
