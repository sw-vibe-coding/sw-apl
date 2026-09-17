//! Expression evaluation.

use apl_ast::Expr;
use apl_parse::parse;
use apl_value::{AplError, AplResult, Array, ErrorKind};
use apl_workspace::{Output, Workspace};

use crate::{apply, forms};

/// Parse and evaluate one line. `Output::Nothing` when there is
/// nothing to display: a blank line, a comment, an assignment
/// (including `⎕←`, which displays through the output buffer).
///
/// # Errors
/// Any lexical, syntax, or evaluation error, with a caret.
pub fn eval_line(ws: &mut Workspace, line: &str) -> AplResult<Output> {
    let Some(expr) = parse(line)? else {
        return Ok(Output::Nothing);
    };
    if let Expr::Mixed(parts) = &expr {
        let values = parts
            .iter()
            .map(|p| eval_expr(ws, p))
            .collect::<AplResult<_>>()?;
        return Ok(Output::Mixed(values));
    }
    let silent = matches!(
        expr,
        Expr::Assign { .. }
            | Expr::IndexedAssign { .. }
            | Expr::QuadOut { .. }
            | Expr::QuoteQuadOut { .. }
    );
    let value = eval_expr(ws, &expr)?;
    Ok(if silent {
        Output::Nothing
    } else {
        Output::Value(value)
    })
}

/// Evaluate an expression tree.
///
/// # Errors
/// VALUE ERROR for unknown names; primitive errors carry the glyph's
/// position as the caret; NOT IMPLEMENTED for branch.
pub fn eval_expr(ws: &mut Workspace, expr: &Expr) -> AplResult<Array> {
    match expr {
        Expr::Literal(a) => Ok(a.clone()),
        Expr::Name(n, pos) => ws
            .get(n)
            .cloned()
            .ok_or_else(|| AplError::new(ErrorKind::Value).at(*pos)),
        Expr::Assign { name, value, .. } => {
            let v = eval_expr(ws, value)?;
            ws.set(name, v.clone());
            Ok(v)
        }
        Expr::Monadic { .. } => apply::eval_monadic(ws, expr),
        Expr::Dyadic { .. } => apply::eval_dyadic(ws, expr),
        Expr::Branch { pos, .. } => Err(AplError::new(ErrorKind::NotImplemented).at(*pos)),
        Expr::Mixed(_) => Err(AplError::new(ErrorKind::Syntax)),
        _ => eval_form(ws, expr),
    }
}

/// The quad and bracket forms.
fn eval_form(ws: &mut Workspace, expr: &Expr) -> AplResult<Array> {
    match expr {
        Expr::Index {
            array,
            pos,
            indexes,
        } => forms::indexed(ws, array, indexes, *pos),
        Expr::IndexedAssign {
            name,
            pos,
            indexes,
            value,
        } => forms::assign_indexed(ws, name, indexes, value, *pos),
        _ => forms::quad(ws, expr),
    }
}
