//! The status operator and the conversion, where the evaluator asks.

use apl_ast::Function;
use apl_ibeam::argument;
use apl_modes::Feature;
use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};
use apl_workspace::{Workspace, free, used};

use crate::codes::{code, glyph};

/// `f r` when `f` is one of APL80's own: status, where the mode has
/// it, and `#`. `None` for any other function.
#[must_use]
pub fn monadic(ws: &Workspace, func: &Function, r: &Array) -> Option<AplResult<Array>> {
    match func {
        Function::Prim('⌶') if ws.mode.profile().has(Feature::Status) => Some(status(ws, r)),
        Function::Prim('#') => Some(convert(r)),
        _ => None,
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

/// `#r`: characters to their codes, codes to their characters.
///
/// # Errors
/// DOMAIN ERROR for a character APL80 has not got, or a number that is
/// not a code; NONCE ERROR for a code that controls the screen rather
/// than printing, which (C) does not send yet.
fn convert(r: &Array) -> AplResult<Array> {
    let domain = || AplError::new(ErrorKind::Domain);
    let data = match &r.data {
        Data::Char(cs) => Data::Num(
            cs.iter()
                .map(|c| code(*c).map(|n| Number::Int(n.into())).ok_or_else(domain))
                .collect::<AplResult<_>>()?,
        ),
        Data::Num(ns) => Data::Char(ns.iter().map(|n| character(*n)).collect::<AplResult<_>>()?),
    };
    Array::new(r.shape.clone(), data)
}

/// The character for one code.
fn character(n: Number) -> AplResult<char> {
    let Number::Int(n) = n else {
        return Err(AplError::new(ErrorKind::Domain));
    };
    let byte = u8::try_from(n).map_err(|_| AplError::new(ErrorKind::Nonce))?;
    glyph(byte).ok_or_else(|| AplError::new(ErrorKind::Nonce))
}
