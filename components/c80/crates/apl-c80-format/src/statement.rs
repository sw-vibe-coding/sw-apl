//! Setting the format, and laying out what is shown under it.

use apl_modes::Feature;
use apl_value::{AplError, AplResult, Array, Data, ErrorKind};
use apl_workspace::{Output, Run, Workspace};

/// A line that is a format statement, run: the rest of it is evaluated
/// with `run` and becomes the format. `None` for any other line, or in
/// a mode without the statement.
pub fn statement(ws: &mut Workspace, line: &str, run: Run) -> Option<AplResult<Output>> {
    if !ws.mode.profile().has(Feature::FormatStatement) {
        return None;
    }
    let rest = line.trim_start().strip_prefix('⍕')?;
    // The new format replaces the old before its own pairs are shown.
    ws.format = None;
    Some(run(ws, rest).and_then(|shown| {
        let Output::Value(pairs) = shown else {
            return Err(AplError::new(ErrorKind::Syntax));
        };
        ws.format = Some(pairs_of(pairs)?);
        Ok(Output::Nothing)
    }))
}

/// The format, checked: a vector of numbers, width and decimals in
/// pairs.
///
/// # Errors
/// DOMAIN ERROR for characters, RANK ERROR for more than a vector,
/// LENGTH ERROR for an odd count.
fn pairs_of(pairs: Array) -> AplResult<Array> {
    if matches!(pairs.data, Data::Char(_)) {
        return Err(AplError::new(ErrorKind::Domain));
    }
    if pairs.shape.len() > 1 {
        return Err(AplError::new(ErrorKind::Rank));
    }
    if pairs.data.count() % 2 != 0 {
        return Err(AplError::new(ErrorKind::Length));
    }
    Ok(pairs)
}

/// What a statement shows, under the format if one is set: each array
/// of numbers laid out by it. An array of characters ends the format
/// and is shown as it is.
///
/// # Errors
/// DOMAIN ERROR from the layout, for a number too wide for its field.
pub fn shaped(ws: &mut Workspace, shown: Output) -> AplResult<Output> {
    let Some(format) = ws.format.clone() else {
        return Ok(shown);
    };
    let mut lay = |a: Array| match a.data {
        Data::Char(_) => {
            ws.format = None;
            Ok(a)
        }
        Data::Num(_) => apl_format::dyadic(&format, &a),
    };
    Ok(match shown {
        Output::Value(a) => Output::Value(lay(a)?),
        Output::Mixed(parts) => {
            Output::Mixed(parts.into_iter().map(lay).collect::<AplResult<_>>()?)
        }
        other => other,
    })
}
