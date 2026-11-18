//! `⎕EA` where it stands: the whole statement, or part of one.

use apl_ast::{Expr, Function};
use apl_modes::Feature;
use apl_value::{AplError, AplResult, Array, ErrorKind};
use apl_workspace::{Output, Run, Workspace};

use crate::alternate::alternate;

/// The statement `expr`, in a mode that has `⎕EA`, when it is a
/// `⎕EA` and nothing more: its arguments evaluated by `eval`, right
/// first, then as `alternate`, so a line with no value shows nothing.
/// `None` for any other statement.
pub fn whole(
    ws: &mut Workspace,
    expr: &Expr,
    eval: fn(&mut Workspace, &Expr) -> AplResult<Array>,
    run: Run,
) -> Option<AplResult<Output>> {
    let Expr::Dyadic {
        func: Function::System(name),
        pos,
        axis: None,
        left,
        right,
    } = expr
    else {
        return None;
    };
    if name != "⎕EA" || !ws.mode.profile().has(Feature::ExecuteAlternate) {
        return None;
    }
    let args = eval(ws, right).and_then(|r| Ok((eval(ws, left)?, r)));
    Some(args.and_then(|(l, r)| alternate(ws, &l, &r, *pos, run)))
}

/// `l ⎕EA r` inside an expression, where it must have a value.
///
/// # Errors
/// As `alternate`, and VALUE ERROR when the line that ran has none.
pub fn value(ws: &mut Workspace, l: &Array, r: &Array, pos: usize, run: Run) -> AplResult<Array> {
    match alternate(ws, l, r, pos, run)? {
        Output::Value(v) => Ok(v),
        _ => Err(AplError::new(ErrorKind::Value).at(pos)),
    }
}
