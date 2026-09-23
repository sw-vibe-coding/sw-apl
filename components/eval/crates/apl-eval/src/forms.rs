//! The non-function forms: quad and quote-quad, bracket indexing,
//! indexed assignment.

use apl_ast::{Expr, Indexes};
use apl_prims::{index, indexed_assign};
use apl_quad::{characters, evaluated};
use apl_value::{AplError, AplResult, Array, ErrorKind};
use apl_workspace::{Output, Workspace, render};

use crate::eval::{eval_expr, eval_line};

/// The quad forms: output on the left of an assignment, input on the
/// right. Quote-quad output leaves the line open, so a prompt and the
/// answer typed after it share a line.
///
/// # Errors
/// INTERRUPT when a read finds no more input; anything an evaluated
/// reply raises.
pub fn quad(ws: &mut Workspace, expr: &Expr) -> AplResult<Array> {
    match expr {
        Expr::QuadOut { value, .. } => {
            let v = eval_expr(ws, value)?;
            ws.output.push(Output::Value(v.clone()));
            Ok(v)
        }
        Expr::QuoteQuadOut { value, .. } => {
            let v = eval_expr(ws, value)?;
            let text = render(&Output::Value(v.clone()), ws.saved.print).join("\n");
            ws.output.push(Output::Bare(text));
            Ok(v)
        }
        Expr::QuadIn(pos) => evaluated(ws, eval_line).map_err(|e| e.at(*pos)),
        Expr::QuoteQuadIn(pos) => characters(ws).map_err(|e| e.at(*pos)),
        _ => unreachable!("quad only receives quad forms"),
    }
}

/// `array[indexes]`: the array, then the indexes right to left.
///
/// # Errors
/// Evaluation errors; selection errors carry the bracket's position.
pub fn indexed(
    ws: &mut Workspace,
    array: &Expr,
    indexes: &Indexes,
    pos: usize,
) -> AplResult<Array> {
    let idx = eval_indexes(ws, indexes)?;
    let base = eval_expr(ws, array)?;
    index(&base, &idx, ws.saved.env.io).map_err(|e| e.at(pos))
}

/// `name[indexes]←value`: the value first, then the indexes, then the
/// variable; yields the value. A system variable is read, changed and
/// assigned back, so what it may hold is checked as for `⎕IO←v`.
///
/// # Errors
/// VALUE ERROR at the name when it is undefined; fit and selection
/// errors, and a value a system variable cannot take, carry the
/// bracket's position.
pub fn assign_indexed(
    ws: &mut Workspace,
    name: &str,
    indexes: &Indexes,
    value: &Expr,
    pos: usize,
) -> AplResult<Array> {
    let v = eval_expr(ws, value)?;
    let idx = eval_indexes(ws, indexes)?;
    let system = name.starts_with('⎕');
    let base = match ws.get(name) {
        _ if system => apl_sysvars::read(ws, name).map_err(|e| e.at(pos))?,
        Some(held) => held.clone(),
        None => return Err(AplError::new(ErrorKind::Value).at(pos)),
    };
    let updated = indexed_assign(&base, &idx, &v, ws.saved.env.io).map_err(|e| e.at(pos))?;
    if system {
        apl_sysvars::assign(ws, name, &updated).map_err(|e| e.at(pos))?;
    } else {
        ws.set(name, updated)?;
    }
    Ok(v)
}

/// Evaluate an index list right to left; elided axes stay `None`.
fn eval_indexes(ws: &mut Workspace, indexes: &Indexes) -> AplResult<Vec<Option<Array>>> {
    let mut out: Vec<Option<Array>> = Vec::with_capacity(indexes.len());
    for idx in indexes.iter().rev() {
        out.push(idx.as_ref().map(|e| eval_expr(ws, e)).transpose()?);
    }
    out.reverse();
    Ok(out)
}
