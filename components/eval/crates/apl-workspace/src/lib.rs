//! The active workspace: variables, defined functions, the evaluation
//! environment (index origin, random link), the activation stack that
//! makes scoping dynamic and doubles as the state indicator, pending
//! output, and the shape of what a statement produces.

mod frame;
mod funcs;
mod workspace;

pub use apl_ast::Defn;
pub use apl_clock::{Clock, Time, hms, stopped, system};
pub use apl_console::{Console, INDENT, Output, Print, Shown, error_lines, render, render_all};
pub use apl_prims::Env;
pub use apl_saved::{Activation, Referent, Saved};
pub use apl_space::{DEFAULT as QUOTA, Groups, free, of_function, of_name, of_value, room, used};
pub use apl_transcript::Transcript;
pub use workspace::{Command, Run, Workspace};
