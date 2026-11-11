//! The mixed functions APL80 leaves out or limits.

use apl_ast::Function;
use apl_modes::{Cut, Profile};
use apl_value::{AplError, AplResult, Array, Data, ErrorKind};

/// Whether `left func[axis] right` may be taken under `profile`.
///
/// # Errors
/// NONCE ERROR, where each is left out, for domino, lamination, the
/// diagonal case of dyadic transpose, and encode or decode with an
/// argument above a vector.
pub fn mixed(
    profile: Profile,
    func: &Function,
    axis: Option<&Array>,
    left: Option<&Array>,
    right: &Array,
) -> AplResult<()> {
    let Function::Prim(glyph) = func else {
        return Ok(());
    };
    let above_vector = |a: &Array| a.shape.len() > 1;
    let cut = match (glyph, left) {
        ('⌹', _) => profile.cuts(Cut::Domino),
        (',', _) => profile.cuts(Cut::Laminate) && axis.is_some_and(fractional),
        ('⍉', Some(l)) => profile.cuts(Cut::Diagonal) && repeats(l),
        ('⊤' | '⊥', Some(l)) => {
            profile.cuts(Cut::MatrixRadix) && (above_vector(l) || above_vector(right))
        }
        _ => false,
    };
    if cut {
        return Err(AplError::new(ErrorKind::Nonce));
    }
    Ok(())
}

/// Whether an axis is a fraction, which laminates.
fn fractional(axis: &Array) -> bool {
    numbers(axis).iter().any(|k| k.fract() != 0.0)
}

/// Whether a transpose's left argument names an axis twice.
fn repeats(left: &Array) -> bool {
    let axes = numbers(left);
    axes.iter().enumerate().any(|(i, k)| axes[..i].contains(k))
}

/// The numbers in `a`; none for characters.
fn numbers(a: &Array) -> Vec<f64> {
    match &a.data {
        Data::Num(v) => v.iter().map(|n| n.as_f64()).collect(),
        Data::Char(_) => Vec::new(),
    }
}
