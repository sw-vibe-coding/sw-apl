//! Assigning a system variable.

use apl_settings::setting;
use apl_value::{AplError, AplResult, Array, Data, ErrorKind, FUZZ, Number};
use apl_workspace::Workspace;

/// Give the system variable `name` the value `v`.
///
/// A setting takes it only if it is one the setting may take, by the
/// same bounds the '70 commands use. The 5110 ignores an assignment
/// to `⎕LC` and `⎕WA` (manual, Chapter 5); sw-apl ignores one to the
/// other values it only reports, `⎕AV`, `⎕TT` and `⎕UL`, likewise.
/// `⎕AI` and `⎕TS` take any numbers, as the manual allows.
///
/// # Errors
/// DOMAIN ERROR for a value a setting cannot take -- where the 5110
/// took it and gave IMPLICIT ERROR when it was next used, sw-apl
/// refuses it at once, as APLSV did. NONCE ERROR for a comparison
/// tolerance other than its own. SYNTAX ERROR for a name the system
/// does not have.
pub fn assign(ws: &mut Workspace, name: &str, v: &Array) -> AplResult<()> {
    let domain = || AplError::new(ErrorKind::Domain);
    let which = match name {
        "⎕IO" => "ORIGIN",
        "⎕PP" => "DIGITS",
        "⎕PW" => "WIDTH",
        "⎕RL" => "LINK",
        "⎕CT" => return tolerance(v),
        "⎕LX" => return line(v).map(|lx| ws.saved.latent = lx),
        "⎕AI" | "⎕TS" if matches!(v.data, Data::Num(_)) => {
            ws.compatible.insert(name.to_string(), v.clone());
            return Ok(());
        }
        "⎕AI" | "⎕TS" => return Err(domain()),
        "⎕LC" | "⎕WA" | "⎕AV" | "⎕TT" | "⎕UL" => return Ok(()),
        _ => return Err(AplError::new(ErrorKind::Syntax)),
    };
    let n = whole(v).ok_or_else(domain)?;
    setting(&mut ws.saved, which, &n.to_string())
        .map(|_| ())
        .ok_or_else(domain)
}

/// The one whole number `v` holds, if it holds one and nothing else.
fn whole(v: &Array) -> Option<i64> {
    let Data::Num(numbers) = &v.data else {
        return None;
    };
    match numbers.as_slice() {
        [Number::Int(n)] => Some(*n),
        [Number::Float(x)] => match Number::from_f64(*x) {
            Number::Int(n) => Some(n),
            Number::Float(_) => None,
        },
        _ => None,
    }
}

/// `⎕CT←v`. sw-apl's comparison tolerance is fixed, so it takes only
/// the value it has.
///
/// # Errors
/// NONCE ERROR for any other: a tolerance that can be set is not
/// implemented.
fn tolerance(v: &Array) -> AplResult<()> {
    let Data::Num(numbers) = &v.data else {
        return Err(AplError::new(ErrorKind::Domain));
    };
    match numbers.as_slice() {
        [n] if (n.as_f64() - FUZZ).abs() <= FUZZ * 1e-9 => Ok(()),
        _ => Err(AplError::new(ErrorKind::Nonce)),
    }
}

/// `⎕LX←v`: the line to keep, `None` for an empty one.
///
/// # Errors
/// DOMAIN ERROR unless `v` is characters, RANK ERROR unless it is a
/// scalar or vector.
fn line(v: &Array) -> AplResult<Option<Array>> {
    if v.shape.len() > 1 {
        return Err(AplError::new(ErrorKind::Rank));
    }
    match &v.data {
        Data::Char(chars) if chars.is_empty() => Ok(None),
        Data::Char(_) => Ok(Some(v.clone())),
        Data::Num(_) => Err(AplError::new(ErrorKind::Domain)),
    }
}
