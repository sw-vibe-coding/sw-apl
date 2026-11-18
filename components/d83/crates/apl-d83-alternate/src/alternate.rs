//! Running the line, and the alternate when it fails.

use apl_execute::execute;
use apl_value::{AplError, AplResult, Array, Data, ErrorKind};
use apl_workspace::{Output, Run, Workspace};

/// `l ⎕EA r`: what the line `r` shows when typed, run by `run`; or,
/// when it fails, what `l` shows. The functions `r` called and left
/// suspended are taken off the state indicator, since the failure is
/// answered and nothing is left to take up. An error in `l` is
/// reported as any other, with the caret on `⎕EA` at `pos`.
///
/// # Errors
/// RANK ERROR unless both are scalars or vectors, DOMAIN ERROR unless
/// both are characters, checked before either runs; and whatever `l`
/// reports.
pub fn alternate(
    ws: &mut Workspace,
    l: &Array,
    r: &Array,
    pos: usize,
    run: Run,
) -> AplResult<Output> {
    line(r)?;
    line(l)?;
    let depth = ws.si().len();
    execute(ws, r, pos, run).or_else(|_| {
        while ws.si().len() > depth {
            ws.leave();
        }
        execute(ws, l, pos, run)
    })
}

/// A character scalar or vector, which is a line to run.
fn line(a: &Array) -> AplResult<()> {
    if a.shape.len() > 1 {
        return Err(AplError::new(ErrorKind::Rank));
    }
    match a.data {
        Data::Char(_) => Ok(()),
        Data::Num(_) => Err(AplError::new(ErrorKind::Domain)),
    }
}
