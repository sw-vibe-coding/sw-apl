//! Reading and assigning `⎕FC`, and the characters it holds now.

use apl_d83_picture::CLEAR;
use apl_modes::Feature;
use apl_value::{AplError, AplResult, Array, Data, ErrorKind};
use apl_workspace::Workspace;

/// The format control in force: what was assigned, or the clear one.
#[must_use]
pub fn control(ws: &Workspace) -> Vec<char> {
    match ws.saved.format.as_ref().map(|a| &a.data) {
        Some(Data::Char(chars)) => chars.clone(),
        _ => CLEAR.chars().collect(),
    }
}

/// The value of the system variable `name`, when it is `⎕FC` in a mode
/// that has it: the evaluator asks here for a name it does not know.
///
/// # Errors
/// SYNTAX ERROR for any other name, as for any name the system has not
/// got.
pub fn read(ws: &Workspace, name: &str) -> AplResult<Array> {
    if name != "⎕FC" || !ws.mode.profile().has(Feature::PictureFormat) {
        return Err(AplError::new(ErrorKind::Syntax));
    }
    let chars = control(ws);
    Ok(Array {
        shape: vec![chars.len()],
        data: Data::Char(chars),
    })
}

/// `⎕FC←v`: characters, a scalar or a vector, whose fifth -- printed as
/// a blank -- is not one picture format reads as a digit, a point or a
/// comma. What is not given of the five is the clear one's.
///
/// # Errors
/// RANK ERROR above a vector, DOMAIN ERROR for numbers or such a fifth;
/// SYNTAX ERROR for a name other than `⎕FC`, or a mode without it.
pub fn assign(ws: &mut Workspace, name: &str, v: &Array) -> AplResult<()> {
    read(ws, name)?;
    if v.shape.len() > 1 {
        return Err(AplError::new(ErrorKind::Rank));
    }
    let Data::Char(chars) = &v.data else {
        return Err(AplError::new(ErrorKind::Domain));
    };
    if chars.get(4).is_some_and(|c| ",.0123456789".contains(*c)) {
        return Err(AplError::new(ErrorKind::Domain));
    }
    let chars: Vec<char> = CLEAR
        .chars()
        .enumerate()
        .map(|(i, c)| chars.get(i).copied().unwrap_or(c))
        .collect();
    ws.saved.format = Some(Array {
        shape: vec![chars.len()],
        data: Data::Char(chars),
    });
    Ok(())
}
