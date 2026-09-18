//! sw-apl evaluator: walks the AST against a workspace of variables
//! and defined functions, and dispatches primitives.

mod apply;
mod eval;
mod forms;

pub use apl_workspace::{
    Activation, Clock, Console, Defn, Env, INDENT, Output, Print, Run, Shown, Time, Transcript,
    Workspace, error_lines, hms, render, render_all, system,
};
pub use eval::{eval_expr, eval_line};
