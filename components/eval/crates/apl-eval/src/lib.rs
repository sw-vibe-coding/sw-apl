//! sw-apl evaluator: walks the AST against a workspace of variables
//! and defined functions, and dispatches primitives.

mod apply;
mod eval;
mod forms;
mod line;

pub use apl_modes::{Cut, Feature, Host, Mode, Profile};
pub use apl_store::{Files, Memory, Shelf, Store};
pub use apl_workspace::{
    Activation, Clock, Console, Defn, Env, Groups, INDENT, Output, Print, QUOTA, Referent, Run,
    Saved, Shown, Time, TimeSource, Transcript, Workspace, error_lines, free, hms, of_name, render,
    render_all, system, used,
};
pub use eval::eval_expr;
pub use line::eval_line;
