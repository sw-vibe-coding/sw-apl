//! The six comparisons, tolerant where APL\360 is.

use apl_value::{FUZZ, Number};

/// `l f r` for `< ≤ = ≥ > ≠`; `None` when `f` is not a comparison.
/// Equality is tolerant, so `<` and `>` exclude tolerantly equal
/// pairs and `≤` and `≥` include them.
#[must_use]
pub fn compare(f: char, left: Number, right: Number) -> Option<bool> {
    let eq = left.tolerant_eq(right, FUZZ);
    let (lhs, rhs) = (left.as_f64(), right.as_f64());
    Some(match f {
        '=' => eq,
        '≠' => !eq,
        '<' => !eq && lhs < rhs,
        '≤' => eq || lhs < rhs,
        '≥' => eq || lhs > rhs,
        '>' => !eq && lhs > rhs,
        _ => return None,
    })
}
