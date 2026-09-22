//! sw-apl evaluator: walks the AST against a workspace of variables
//! and defined functions, and dispatches primitives.

mod apply;
mod eval;
mod forms;

pub use apl_modes::{Host, Mode};
pub use apl_store::{Files, Memory, Shelf, Store};
pub use apl_workspace::{
    Activation, Clock, Console, Defn, Env, Groups, INDENT, Output, Print, QUOTA, Run, Saved, Shown,
    Time, Transcript, Workspace, error_lines, free, hms, of_name, render, render_all, system, used,
};
pub use eval::{eval_expr, eval_line};
