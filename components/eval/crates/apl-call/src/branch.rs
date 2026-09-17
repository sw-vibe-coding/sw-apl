//! Where a branch goes.

use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};

/// The line a branch selects: the first element of its value, or
/// `None` when the value is empty, which falls through to the next
/// line. Whether the line exists is the body's business; 0 is the
/// traditional way to write a number that cannot be one.
///
/// # Errors
/// RANK ERROR above rank 1, DOMAIN ERROR for character data and for
/// a number that is not a whole line number.
pub fn target(value: &Array) -> AplResult<Option<i64>> {
    if value.shape.len() > 1 {
        return Err(AplError::new(ErrorKind::Rank));
    }
    let Data::Num(v) = &value.data else {
        return Err(AplError::new(ErrorKind::Domain));
    };
    match v.first() {
        None => Ok(None),
        Some(Number::Int(n)) => Ok(Some(*n)),
        Some(Number::Float(_)) => Err(AplError::new(ErrorKind::Domain)),
    }
}
