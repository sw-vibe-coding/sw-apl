//! The call itself.

use apl_ast::Defn;
use apl_value::{AplError, AplResult, Array, ErrorKind};
use apl_workspace::{Output, Workspace};

/// How to evaluate one body line. The evaluator passes its own
/// `eval_line`; nothing here needs to know what that does.
pub type Run = fn(&mut Workspace, &str) -> AplResult<Output>;

/// Apply the function `name` holds. `None` comes back when the header
/// declares no result, which only a whole statement may ignore.
///
/// # Errors
/// SYNTAX ERROR when the valence written is not the one declared,
/// DEPTH ERROR when calls nest too deeply, and anything the body
/// raises (with the caret dropped, since it pointed into the body).
pub fn call(
    ws: &mut Workspace,
    name: &str,
    left: Option<Array>,
    right: Option<Array>,
    run: Run,
) -> AplResult<Option<Array>> {
    let defn = ws
        .function(name)
        .ok_or_else(|| AplError::new(ErrorKind::Value))?;
    if left.is_some() != defn.left.is_some() || right.is_some() != defn.right.is_some() {
        return Err(AplError::new(ErrorKind::Syntax));
    }
    let frame = ws.enter(&defn.names())?;
    for (n, v) in [(&defn.left, left), (&defn.right, right)] {
        if let (Some(n), Some(v)) = (n, v) {
            ws.set(n, v);
        }
    }
    let ran = run_body(ws, &defn, run).map_err(|e| AplError::new(e.kind));
    let result = defn.result.as_ref().and_then(|r| ws.get(r).cloned());
    ws.leave(frame);
    ran.map(|()| result)
}

/// A call used for its value: a function with no result has none.
///
/// # Errors
/// VALUE ERROR when the function declares no result; otherwise
/// whatever the call raises, with `pos` as the caret.
pub fn value(
    ws: &mut Workspace,
    name: &str,
    pos: usize,
    args: (Option<Array>, Option<Array>),
    run: Run,
) -> AplResult<Array> {
    call(ws, name, args.0, args.1, run)
        .map_err(|e| e.at(pos))?
        .ok_or_else(|| AplError::new(ErrorKind::Value).at(pos))
}

/// Run the body lines in order. Anything a line displays joins the
/// pending output, so it reaches the terminal ahead of the result.
fn run_body(ws: &mut Workspace, defn: &Defn, run: Run) -> AplResult<()> {
    for line in &defn.body {
        let shown = run(ws, line)?;
        if shown != Output::Nothing {
            ws.output.push(shown);
        }
    }
    Ok(())
}
