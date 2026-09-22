//! Running a character vector as a line.

use apl_ast::{Expr, Function};
use apl_value::{AplError, AplResult, Array, Data, ErrorKind};
use apl_workspace::{Output, Run, Workspace};

/// `⍎r` as a whole statement: the line `r` spells, run by `run`, and
/// whatever it would show when typed -- nothing for an assignment or
/// an empty line, a branch that takes effect as one. The manual's
/// example is `⍎(A=B)/'A+B'`, which does nothing when A and B differ.
///
/// An error in the line is the statement's error, with the caret on
/// the execute at `pos`, since the executed line is not on the paper.
/// An error inside a function the line calls keeps its own place.
///
/// # Errors
/// DOMAIN ERROR unless `r` is characters, RANK ERROR unless it is a
/// scalar or vector, and whatever the line reports.
pub fn execute(ws: &mut Workspace, r: &Array, pos: usize, run: Run) -> AplResult<Output> {
    let line = text(r)?;
    run(ws, &line).map_err(|mut err| {
        if err.context.is_none() {
            err.caret = Some(pos);
        }
        err
    })
}

/// The statement `expr`, when it is an execute and nothing more: its
/// argument evaluated by `eval`, and then as `execute`. `None` for any
/// other statement.
pub fn whole(
    ws: &mut Workspace,
    expr: &Expr,
    eval: fn(&mut Workspace, &Expr) -> AplResult<Array>,
    run: Run,
) -> Option<AplResult<Output>> {
    let Expr::Monadic {
        func: Function::Prim('⍎'),
        pos,
        axis: None,
        right,
    } = expr
    else {
        return None;
    };
    Some(eval(ws, right).and_then(|r| execute(ws, &r, *pos, run)))
}

/// `⍎r` inside an expression, where it must have a value.
///
/// # Errors
/// As `execute`, and VALUE ERROR when the line has no value to give:
/// an empty line, an assignment, a branch. Whether an executed
/// assignment gave its value in APLSV or on the 5100 the manuals do
/// not say; sw-apl gives none, as a typed assignment shows none.
pub fn value(ws: &mut Workspace, r: &Array, pos: usize, run: Run) -> AplResult<Array> {
    match execute(ws, r, pos, run)? {
        Output::Value(v) => Ok(v),
        _ => Err(AplError::new(ErrorKind::Value).at(pos)),
    }
}

/// The line a character scalar or vector spells.
fn text(r: &Array) -> AplResult<String> {
    if r.shape.len() > 1 {
        return Err(AplError::new(ErrorKind::Rank));
    }
    match &r.data {
        Data::Char(chars) => Ok(chars.iter().collect()),
        Data::Num(_) => Err(AplError::new(ErrorKind::Domain)),
    }
}
