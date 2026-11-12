//! The status operator and the conversion, where the evaluator asks.

use apl_ast::Function;
use apl_ibeam::{argument, system_value};
use apl_modes::Feature;
use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};
use apl_workspace::{Workspace, free, used};

use apl_c80_screen as screen;

use crate::codes::{code, glyph};
use crate::special::obey;

/// `⌶r` or `#r`, where the evaluator asks: in a mode with status, the
/// I-beam is APL80's status; in one with the I-beams it reads them, and
/// in any other it is a NONCE ERROR (`apl-ibeam`). `#` is APL80's
/// conversion.
///
/// # Errors
/// As `status`, `apl_ibeam::system_value` and `convert`.
pub fn system(ws: &mut Workspace, func: &Function, r: &Array) -> AplResult<Array> {
    match func {
        Function::Prim('⌶') if ws.mode.profile().has(Feature::Status) => status(ws, r),
        Function::Prim('⌶') => system_value(ws, r),
        _ => convert(ws, r),
    }
}

/// `s n`: 0 the time since sign-on in 25-millisecond ticks; 1, 2 and 3
/// the read pointer, the update pointer and the records in the last
/// file, all 0 while (C) has no files; 4 the line now running; 5 the
/// bytes free.
///
/// # Errors
/// DOMAIN ERROR for anything but one of those six.
fn status(ws: &Workspace, r: &Array) -> AplResult<Array> {
    let value = match argument(r)? {
        // The clock counts sixtieths of a second: 25 ms is 1.5 of them.
        0 => ((ws.clock)().now - ws.signed_on) * 2 / 3,
        1..=3 => 0,
        4 => ws
            .si()
            .last()
            .map_or(0, |a| i64::try_from(a.line).unwrap_or(0)),
        5 => {
            let held = used(&ws.saved.vars, &ws.saved.funcs, &ws.saved.groups);
            i64::try_from(free(ws.quota, held)).unwrap_or(i64::MAX)
        }
        _ => return Err(AplError::new(ErrorKind::Domain)),
    };
    Ok(Array::scalar(Number::Int(value)))
}

/// `#r`: characters to their codes, codes to their characters. A code
/// APL80 obeys rather than prints -- `#6`, randomize -- is obeyed and
/// gives no character; one that controls the screen gives the character
/// that does it when shown (`apl-c80-screen`).
///
/// # Errors
/// DOMAIN ERROR for a character APL80 has not got, or a number that is
/// not a code; NONCE ERROR for what `special` does not do.
fn convert(ws: &mut Workspace, r: &Array) -> AplResult<Array> {
    let ns = match &r.data {
        Data::Num(ns) => ns,
        Data::Char(cs) => {
            let code = |c: &char| code(*c).map(i64::from).or_else(|| screen::code(*c));
            let codes = cs.iter().map(|c| code(c).map(Number::Int));
            let codes = codes
                .collect::<Option<_>>()
                .ok_or_else(|| AplError::new(ErrorKind::Domain))?;
            return Array::new(r.shape.clone(), Data::Num(codes));
        }
    };
    let mut shown = Vec::new();
    for n in ns {
        shown.extend(character(ws, *n)?);
    }
    let shape = if r.shape.is_empty() && shown.len() == 1 {
        Vec::new()
    } else {
        vec![shown.len()]
    };
    Array::new(shape, Data::Char(shown))
}

/// The character for one code, or none for a code that is obeyed.
fn character(ws: &mut Workspace, n: Number) -> AplResult<Option<char>> {
    let Number::Int(n) = n else {
        return Err(AplError::new(ErrorKind::Domain));
    };
    if obey(ws, n)? {
        return Ok(None);
    }
    let printed = || u8::try_from(n).ok().and_then(glyph);
    let c = screen::character(n).or_else(printed);
    c.map(Some).ok_or_else(|| AplError::new(ErrorKind::Domain))
}
