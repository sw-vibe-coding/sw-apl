//! Mixed output, a branch, and a bare call to a defined function.

use apl_ast::Expr;
use apl_call::branch_target;
use apl_value::{AplResult, Array};
use apl_workspace::{Output, Run, Workspace};

/// How to evaluate an expression. The evaluator passes its own
/// `eval_expr`, as it passes `eval_line` as the `Run`.
pub type Eval = fn(&mut Workspace, &Expr) -> AplResult<Array>;

/// The statement shapes that do not evaluate to one value: mixed
/// output, a branch, and a bare call to a defined function, which
/// displays nothing when the header declares no result. `None` when
/// the statement is an ordinary expression.
///
/// # Errors
/// Whatever evaluating its parts raises, and a call's errors with the
/// caret on the function's name.
pub fn statement(
    ws: &mut Workspace,
    expr: &Expr,
    eval: Eval,
    run: Run,
) -> AplResult<Option<Output>> {
    match expr {
        Expr::Mixed(parts) => {
            let parts = parts.iter().map(|p| eval(ws, p));
            Ok(Some(Output::Mixed(parts.collect::<AplResult<_>>()?)))
        }
        Expr::Branch { target, .. } => branch(ws, target.as_deref(), eval).map(Some),
        _ => call(ws, expr, eval, run),
    }
}

/// `→target`, or `→` alone, which leaves the suspended function.
fn branch(ws: &mut Workspace, target: Option<&Expr>, eval: Eval) -> AplResult<Output> {
    let Some(target) = target else {
        return Ok(Output::Branch(None));
    };
    let line = branch_target(&eval(ws, target)?)?;
    Ok(line.map_or(Output::Nothing, |n| Output::Branch(Some(n))))
}

/// A defined function called by the whole statement, its arguments
/// right first; `None` when the statement is not one.
fn call(ws: &mut Workspace, expr: &Expr, eval: Eval, run: Run) -> AplResult<Option<Output>> {
    let Some((name, pos, left, right)) = expr.defined_call().filter(|c| ws.is_function(c.0)) else {
        return Ok(None);
    };
    let right = right.map(|e| eval(ws, e)).transpose()?;
    let left = left.map(|e| eval(ws, e)).transpose()?;
    let got = apl_call::call(ws, name, left, right, run).map_err(|e| e.at(pos))?;
    Ok(Some(got.map_or(Output::Nothing, Output::Value)))
}
