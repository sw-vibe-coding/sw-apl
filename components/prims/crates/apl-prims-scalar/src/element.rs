//! A number or a character: what = and ≠ compare.
//!
//! The APL\360 User's Manual, page 3.8: of the scalar functions, the
//! relations = and ≠ are defined on characters as well as on numbers.
//! Every other scalar function takes numbers only.

use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};

use crate::dispatch::apply_dyadic;

/// One element of either kind of array.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Element {
    /// A number.
    Num(Number),
    /// A character.
    Char(char),
}

/// The elements of `a`, for scalar function `f`.
///
/// # Errors
/// DOMAIN ERROR when `a` holds characters and `f` is not = or ≠ --
/// even when `a` is empty, as it always was.
pub fn elements(a: &Array, f: char) -> AplResult<Vec<Element>> {
    match &a.data {
        Data::Num(v) => Ok(v.iter().map(|n| Element::Num(*n)).collect()),
        Data::Char(v) if matches!(f, '=' | '≠') => {
            Ok(v.iter().map(|c| Element::Char(*c)).collect())
        }
        Data::Char(_) => Err(AplError::new(ErrorKind::Domain)),
    }
}

/// Scalar function `f` on two elements. Two numbers go to the number
/// family, tolerance and all. A character equals only the same
/// character, and never a number: the manual does not say what a
/// character compared with a number gives, and sw-apl answers rather
/// than refusing.
///
/// # Errors
/// DOMAIN ERROR for a character with any function but = and ≠, and
/// whatever the number family reports.
pub fn related(f: char, left: Element, right: Element) -> AplResult<Number> {
    let same = match (left, right) {
        (Element::Num(l), Element::Num(r)) => return apply_dyadic(f, l, r),
        (Element::Char(l), Element::Char(r)) => l == r,
        _ => false,
    };
    match f {
        '=' => Ok(Number::Int(i64::from(same))),
        '≠' => Ok(Number::Int(i64::from(!same))),
        _ => Err(AplError::new(ErrorKind::Domain)),
    }
}

/// `n` results of `apply` on elements paired as scalar extension
/// pairs them: a side of one element is reused.
///
/// # Errors
/// Whatever `apply` reports.
pub fn pairs<T: Copy>(
    l: &[T],
    r: &[T],
    n: usize,
    apply: impl Fn(T, T) -> AplResult<Number>,
) -> AplResult<Vec<Number>> {
    let pick = |v: &[T], i: usize| v[if v.len() == 1 { 0 } else { i }];
    (0..n).map(|i| apply(pick(l, i), pick(r, i))).collect()
}
