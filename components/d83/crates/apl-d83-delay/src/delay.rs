//! `⎕DL n`.

use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};

/// `⎕DL n`: wait `n` seconds, a fraction if need be, and give back the
/// seconds waited. A session that cannot wait -- a transcript -- gives
/// back `n`, so it prints the same every time.
///
/// # Errors
/// RANK ERROR past a vector, LENGTH ERROR for other than one number,
/// DOMAIN ERROR for a character.
pub fn delay(r: &Array) -> AplResult<Array> {
    let error = |kind| Err(AplError::new(kind));
    match (r.shape.len(), &r.data) {
        (2.., _) => error(ErrorKind::Rank),
        (_, Data::Num(ns)) if ns.len() == 1 => {
            let waited = apl_attn::delay(ns[0].as_f64());
            Ok(Array::scalar(Number::from_f64(waited)))
        }
        (_, Data::Num(_)) => error(ErrorKind::Length),
        _ => error(ErrorKind::Domain),
    }
}
