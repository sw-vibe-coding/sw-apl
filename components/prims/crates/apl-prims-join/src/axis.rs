//! Axis specification and argument conformance.

use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};

/// Which axis a structural function works along (0-based).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    /// No bracket: the last axis.
    Last,
    /// `[k]`: an existing axis.
    At(usize),
    /// A fractional bracket: a new axis inserted at this position.
    Laminate(usize),
}

/// Turn an evaluated axis bracket into an [`Axis`] for arguments of
/// the given (larger) rank, in the index origin `io`.
///
/// # Errors
/// RANK ERROR unless the bracket is a scalar; DOMAIN ERROR for
/// character data; INDEX ERROR when the axis is out of range.
pub fn resolve_axis(axis: Option<&Array>, io: i64, rank: usize) -> AplResult<Axis> {
    let Some(axis) = axis else {
        return Ok(Axis::Last);
    };
    let x = scalar_number(axis)?.as_f64();
    let laminate = x.fract() != 0.0;
    let Number::Int(k) = Number::from_f64(x.ceil()) else {
        return Err(AplError::new(ErrorKind::Index));
    };
    let limit = if laminate { rank } else { rank.max(1) - 1 };
    match usize::try_from(k - io) {
        Ok(p) if p <= limit && laminate => Ok(Axis::Laminate(p)),
        Ok(p) if p <= limit => Ok(Axis::At(p)),
        _ => Err(AplError::new(ErrorKind::Index)),
    }
}

/// The single number in a scalar array.
fn scalar_number(a: &Array) -> AplResult<Number> {
    if !a.shape.is_empty() {
        return Err(AplError::new(ErrorKind::Rank));
    }
    match &a.data {
        Data::Num(v) => Ok(v[0]),
        Data::Char(_) => Err(AplError::new(ErrorKind::Domain)),
    }
}

/// Give `x` the rank of `other` for joining along axis `k`: a scalar
/// spreads to `other`'s shape (with axis `k` of length 1 unless
/// laminating); an array of one rank less gains a length-1 axis at
/// `k` (not when laminating, where shapes must match).
///
/// # Errors
/// RANK ERROR when the ranks cannot be reconciled.
pub fn conform(x: &Array, other: &Array, k: usize, laminate: bool) -> AplResult<Array> {
    let (rx, ro) = (x.shape.len(), other.shape.len());
    if rx >= ro {
        return Ok(x.clone());
    }
    if rx == 0 {
        let mut shape = other.shape.clone();
        if !laminate {
            shape[k] = 1;
        }
        return Array::new(shape.clone(), fill(&x.data, shape.iter().product()));
    }
    if laminate || rx + 1 != ro {
        return Err(AplError::new(ErrorKind::Rank));
    }
    let mut shape = x.shape.clone();
    shape.insert(k, 1);
    Array::new(shape, x.data.clone())
}

/// `count` copies of the single element of `data`.
fn fill(data: &Data, count: usize) -> Data {
    match data {
        Data::Num(v) => Data::Num(vec![v[0]; count]),
        Data::Char(v) => Data::Char(vec![v[0]; count]),
    }
}
