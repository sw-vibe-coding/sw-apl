//! The call itself.

use apl_ast::Defn;
use apl_scan::{labels, without_label};
use apl_value::{AplError, AplResult, Array, ErrorKind, Number};
use apl_workspace::{Frame, Output, Workspace};

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
    let frame = bind(ws, &defn, (left, right))?;
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

/// Shadow everything the call makes local -- result, arguments,
/// locals, labels -- then give the arguments and the labels their
/// values. A label holds the number of the line it names.
///
/// # Errors
/// DEPTH ERROR when calls nest too deeply.
fn bind(ws: &mut Workspace, defn: &Defn, args: (Option<Array>, Option<Array>)) -> AplResult<Frame> {
    let labels = labels(&defn.body);
    let mut names = defn.names();
    names.extend(labels.iter().map(|(n, _)| n.clone()));
    let frame = ws.enter(&names)?;
    for (n, v) in [(&defn.left, args.0), (&defn.right, args.1)] {
        if let (Some(n), Some(v)) = (n, v) {
            ws.set(n, v);
        }
    }
    for (n, line) in labels {
        let line = i64::try_from(line).unwrap_or_default();
        ws.set(&n, Array::scalar(Number::Int(line)));
    }
    Ok(frame)
}

/// Run the body from line 1, following branches. A branch to a line
/// the function does not have -- 0, by convention -- returns. What a
/// line displays joins the pending output, so it reaches the terminal
/// ahead of the result.
fn run_body(ws: &mut Workspace, defn: &Defn, run: Run) -> AplResult<()> {
    let mut line = 1usize;
    while let Some(text) = defn.body.get(line - 1) {
        match run(ws, &without_label(text))? {
            Output::Branch(to) => match usize::try_from(to) {
                Ok(n) if (1..=defn.body.len()).contains(&n) => line = n,
                _ => return Ok(()),
            },
            Output::Nothing => line += 1,
            shown => {
                ws.output.push(shown);
                line += 1;
            }
        }
    }
    Ok(())
}
