//! Expression evaluation.

use apl_ast::Expr;
use apl_call::{branch_target, value};
use apl_parse::parse;
use apl_value::{AplError, AplResult, Array, ErrorKind};
use apl_workspace::{Output, Workspace};

use crate::{apply, forms};

/// Parse and evaluate one line. `Output::Nothing` when there is
/// nothing to display: a blank line, a comment, an assignment
/// (including `⎕←`, which displays through the output buffer), or a
/// call to a defined function that declares no result.
///
/// # Errors
/// Any lexical, syntax, or evaluation error, with a caret.
pub fn eval_line(ws: &mut Workspace, line: &str) -> AplResult<Output> {
    let takes_argument = |n: &str| ws.function(n).is_some_and(|d| d.right.is_some());
    let Some(expr) = parse(line, ws.mode.glyphs(), &takes_argument)? else {
        return Ok(Output::Nothing);
    };
    // An execute that is the whole statement shows what its line shows.
    if let Some(done) = apl_execute::whole(ws, &expr, eval_expr, eval_line) {
        return done;
    }
    if let Some(shown) = statement(ws, &expr)? {
        return Ok(shown);
    }
    let silent = matches!(
        expr,
        Expr::Assign { .. }
            | Expr::IndexedAssign { .. }
            | Expr::QuadOut { .. }
            | Expr::QuoteQuadOut { .. }
    );
    let value = eval_expr(ws, &expr)?;
    Ok((!silent)
        .then_some(value)
        .map_or(Output::Nothing, Output::Value))
}

/// The statement shapes that do not evaluate to one value: mixed
/// output, a branch, and a bare call to a defined function, which
/// displays nothing when the header declares no result. `None` when
/// the statement is an ordinary expression.
fn statement(ws: &mut Workspace, expr: &Expr) -> AplResult<Option<Output>> {
    if let Expr::Mixed(parts) = expr {
        let parts = parts
            .iter()
            .map(|p| eval_expr(ws, p))
            .collect::<AplResult<_>>()?;
        return Ok(Some(Output::Mixed(parts)));
    }
    if let Expr::Branch { target, .. } = expr {
        let Some(target) = target else {
            return Ok(Some(Output::Branch(None)));
        };
        let line = branch_target(&eval_expr(ws, target)?)?;
        return Ok(Some(
            line.map_or(Output::Nothing, |n| Output::Branch(Some(n))),
        ));
    }
    let Some((name, pos, left, right)) = expr.defined_call().filter(|c| ws.is_function(c.0)) else {
        return Ok(None);
    };
    let right = right.map(|e| eval_expr(ws, e)).transpose()?;
    let left = left.map(|e| eval_expr(ws, e)).transpose()?;
    let got = apl_call::call(ws, name, left, right, eval_line).map_err(|e| e.at(pos))?;
    Ok(Some(got.map_or(Output::Nothing, Output::Value)))
}

/// Evaluate an expression tree. A system variable, a name only (B)
/// lexes, is read and assigned by `apl-sysvars`.
///
/// # Errors
/// VALUE ERROR for unknown names; primitive errors carry the glyph's
/// position as the caret; SYNTAX ERROR for a branch, which is a
/// statement and not a value.
pub fn eval_expr(ws: &mut Workspace, expr: &Expr) -> AplResult<Array> {
    if let Some(done) = apl_sysvars::form(ws, expr, eval_expr) {
        return done;
    }
    match expr {
        Expr::Literal(a) => Ok(a.clone()),
        Expr::Name(n, pos) if ws.is_function(n) => value(ws, n, *pos, (None, None), eval_line),
        Expr::Name(n, pos) => ws
            .get(n)
            .cloned()
            .ok_or_else(|| AplError::new(ErrorKind::Value).at(*pos)),
        Expr::Assign { name, value, .. } => {
            let v = eval_expr(ws, value)?;
            ws.set(name, v.clone())?;
            Ok(v)
        }
        Expr::Monadic { .. } => apply::eval_monadic(ws, expr),
        Expr::Dyadic { .. } => apply::eval_dyadic(ws, expr),
        Expr::Branch { pos, .. } => Err(AplError::new(ErrorKind::Syntax).at(*pos)),
        Expr::Mixed(_) => Err(AplError::new(ErrorKind::Syntax)),
        _ => eval_form(ws, expr),
    }
}

/// The bracket forms, and the quad forms behind them.
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
