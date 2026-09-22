//! Where the evaluator asks for a system value.

use apl_modes::Mode;
use apl_value::{AplError, AplResult, Array, ErrorKind};
use apl_workspace::{Workspace, free, used};

use crate::ibeam::{argument, ibeam};

/// `⌶r` in `ws`: the system value `r` selects, read from the
/// workspace -- its state indicator, the space it has left, when its
/// session signed on, and its clock.
///
/// # Errors
/// NONCE ERROR outside (A): the IBM 5100 family replaced the I-beams
/// with system variables and functions (docs/mode-b.md), and the
/// IBM 5110 manual gives NONCE for an I-beam used there. Otherwise as
/// `argument` and `ibeam`.
pub fn system_value(ws: &Workspace, r: &Array) -> AplResult<Array> {
    if ws.mode != Mode::A {
        return Err(AplError::new(ErrorKind::Nonce));
    }
    let si = ws.si().iter().rev();
    let lines: Vec<i64> = si.filter_map(|a| a.line.try_into().ok()).collect();
    let held = used(&ws.saved.vars, &ws.saved.funcs, &ws.saved.groups);
    let left = i64::try_from(free(ws.quota, held)).unwrap_or(i64::MAX);
    ibeam(argument(r)?, (ws.clock)(), ws.signed_on, &lines, left)
}
