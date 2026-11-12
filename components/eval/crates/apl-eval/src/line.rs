//! A line as typed, in whichever characters the mode types.

use apl_modes::Feature;
use apl_value::AplResult;
use apl_workspace::{Output, Workspace};

use crate::eval::eval_glyphs;

/// Evaluate one line as it was typed. In (C) it is typed in APL80's
/// characters and translated into glyphs first, knowing which names
/// are functions now (`apl-c80-keys`); an error's caret is put back
/// under what was typed. Every other mode types glyphs.
///
/// # Errors
/// Any lexical, syntax, or evaluation error, with a caret.
pub fn eval_line(ws: &mut Workspace, line: &str) -> AplResult<Output> {
    if !ws.mode.profile().has(Feature::Apl80Keys) {
        return eval_glyphs(ws, line);
    }
    let typed = apl_c80_keys::glyphs(line, &|n| ws.is_function(n))?;
    eval_glyphs(ws, &typed.text).map_err(|e| typed.place(e))
}
