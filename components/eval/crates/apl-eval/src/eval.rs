//! Expression evaluation: a line as typed, a line in glyphs, an
//! expression, and the indexing and quad forms.

use apl_ast::Expr;
use apl_call::value;
use apl_modes::Feature;
use apl_parse::parse;
use apl_value::{AplError, AplResult, Array, ErrorKind};
use apl_workspace::{Output, Workspace};

use crate::apply::statement;
use crate::{apply, forms};

/// Evaluate one line as it was typed. In (C) it is typed in APL80's
/// characters and translated into glyphs first, knowing which names
/// are functions now (`apl-c80-keys`), and may be a format statement
/// (`apl-c80-format`); an error's caret is put back under what was
/// typed. Every other mode types glyphs.
///
/// # Errors
/// Any lexical, syntax, or evaluation error, with a caret.
pub fn eval_line(ws: &mut Workspace, line: &str) -> AplResult<Output> {
    // A workspace file's directives, `⍝!MODES (C)` and the rest, are
    // comments no keyboard of APL80's could type: the machine's lines.
    if !ws.mode.profile().has(Feature::Apl80Keys) || line.trim_start().starts_with('⍝') {
        return eval_glyphs(ws, line);
    }
    let typed = apl_c80_keys::glyphs(line, &|n| ws.is_function(n))?;
    let done = apl_c80_format::statement(ws, &typed.text, eval_glyphs);
    done.unwrap_or_else(|| eval_glyphs(ws, &typed.text))
        .map_err(|e| typed.place(e))
}

/// Parse and evaluate one line in glyphs. `Output::Nothing` when there
/// is nothing to display: a blank line, a comment, an assignment
/// (including `⎕←`, which displays through the output buffer), or a
/// call to a defined function that declares no result. What is shown
/// is laid out by (C)'s format statement when one is in force
/// (`apl-c80-format`).
///
/// # Errors
/// Any lexical, syntax, or evaluation error, with a caret.
fn eval_glyphs(ws: &mut Workspace, line: &str) -> AplResult<Output> {
    let takes_argument =
        |n: &str| apl_sysfns::is_function(n) || ws.function(n).is_some_and(|d| d.right.is_some());
    let Some(expr) = parse(line, ws.mode.glyphs(), &takes_argument)? else {
        return Ok(Output::Nothing);
    };
    apl_c80_cuts::enter(ws.mode.profile(), &expr)?;
    // An execute that is the whole statement shows what its line shows.
    if let Some(done) = apl_execute::whole(ws, &expr, eval_expr, eval_line) {
        return done;
    }
    if let Some(shown) = statement(ws, &expr)? {
        return apl_c80_format::shaped(ws, shown);
    }
    let silent = matches!(
        expr,
        Expr::Assign { .. }
            | Expr::IndexedAssign { .. }
            | Expr::QuadOut { .. }
            | Expr::QuoteQuadOut { .. }
    );
    let value = eval_expr(ws, &expr)?;
    let shown = (!silent).then_some(value);
    apl_c80_format::shaped(ws, shown.map_or(Output::Nothing, Output::Value))
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
