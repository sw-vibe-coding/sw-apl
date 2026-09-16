//! sw-apl evaluator: walks the AST against a workspace of variables
//! and dispatches primitives.

mod eval;
mod system;
mod workspace;

pub use eval::{eval_expr, eval_line};
pub use workspace::Workspace;
