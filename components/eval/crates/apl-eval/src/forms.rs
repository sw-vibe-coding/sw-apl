//! The non-function forms: quad output and input, bracket indexing,
//! indexed assignment.

use apl_ast::{Expr, Indexes};
use apl_prims::{index, indexed_assign};
use apl_value::{AplError, AplResult, Array, ErrorKind};
use apl_workspace::Workspace;

use crate::eval::eval_expr;

/// Quad forms: quad output and quad input.
///
/// # Errors
/// NOT IMPLEMENTED for input and quote-quad output, for now.
pub fn quad(ws: &mut Workspace, expr: &Expr) -> AplResult<Array> {
    match expr {
        Expr::QuadOut { value, .. } => {
            let v = eval_expr(ws, value)?;
            ws.output.push(v.clone());
            Ok(v)
        }
        Expr::QuadIn(pos) | Expr::QuoteQuadIn(pos) | Expr::QuoteQuadOut { pos, .. } => {
            Err(AplError::new(ErrorKind::NotImplemented).at(*pos))
        }
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
    index(&base, &idx, ws.env.io).map_err(|e| e.at(pos))
}

/// `name[indexes]←value`: the value first, then the indexes, then the
/// variable; yields the value.
///
/// # Errors
/// VALUE ERROR at the name when it is undefined; fit and selection
/// errors carry the bracket's position.
pub fn assign_indexed(
    ws: &mut Workspace,
    name: &str,
    indexes: &Indexes,
    value: &Expr,
    pos: usize,
) -> AplResult<Array> {
    let v = eval_expr(ws, value)?;
    let idx = eval_indexes(ws, indexes)?;
    let base = ws
        .get(name)
        .cloned()
        .ok_or_else(|| AplError::new(ErrorKind::Value).at(pos))?;
    let updated = indexed_assign(&base, &idx, &v, ws.env.io).map_err(|e| e.at(pos))?;
    ws.set(name, updated);
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
