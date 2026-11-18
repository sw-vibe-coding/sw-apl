//! Which of (D)'s dyadic additions, if any.

use apl_ast::Function;
use apl_d83_alternate::value as alternate;
use apl_d83_collate::grade;
use apl_modes::Feature;
use apl_value::{AplResult, Array};
use apl_workspace::{Run, Workspace};

/// `l func r` when it is one of (D)'s, in a mode that has it: `⎕EA`,
/// which runs lines by `run`, or `⍋` or `⍒` with no axis, which sort
/// by a collating sequence. The caret of an error in the grade is
/// put at `pos`. `None` for anything else.
pub fn dyadic(
    ws: &mut Workspace,
    func: &Function,
    pos: usize,
    (l, r): (&Array, &Array),
    run: Run,
) -> Option<AplResult<Array>> {
    let profile = ws.mode.profile();
    match func {
        Function::System(name) if name == "⎕EA" && profile.has(Feature::ExecuteAlternate) => {
            Some(alternate(ws, l, r, pos, run))
        }
        Function::Prim(g @ ('⍋' | '⍒')) if profile.has(Feature::CollatingGrade) => {
            Some(grade(l, r, *g == '⍒', ws.saved.env.io).map_err(|e| e.at(pos)))
        }
        _ => None,
    }
}
