//! Ravel and catenate.

use apl_value::{AplError, AplResult, Array, Data, ErrorKind};

/// `,r`: all elements as a vector.
#[must_use]
pub fn ravel(r: &Array) -> Array {
    Array {
        shape: vec![r.data.count()],
        data: r.data.clone(),
    }
}

/// `l,r` for scalars and vectors (matrices wait for the axis forms).
///
/// # Errors
/// DOMAIN ERROR when mixing numbers and characters; NOT IMPLEMENTED
/// above rank 1.
pub fn catenate(l: &Array, r: &Array) -> AplResult<Array> {
    if l.shape.len() > 1 || r.shape.len() > 1 {
        return Err(AplError::new(ErrorKind::NotImplemented));
    }
    let data = match (&l.data, &r.data) {
        (Data::Num(a), Data::Num(b)) => Data::Num([a.as_slice(), b].concat()),
        (Data::Char(a), Data::Char(b)) => Data::Char([a.as_slice(), b].concat()),
        _ => return Err(AplError::new(ErrorKind::Domain)),
    };
    Ok(Array {
        shape: vec![data.count()],
        data,
    })
}
