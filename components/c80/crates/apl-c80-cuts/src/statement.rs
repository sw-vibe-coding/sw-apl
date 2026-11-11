//! One specification a statement, at its left.

use apl_ast::Expr;
use apl_modes::{Cut, Profile};
use apl_value::{AplError, AplResult, ErrorKind};

/// Ready a line to run under `profile`: set the scalar functions it
/// leaves out, and check the statement against its cuts.
///
/// # Errors
/// SYNTAX ERROR, at the second arrow, for a statement that specifies
/// twice or uses a specification's value -- `X←Y←9`, `÷X←9` -- where
/// a statement may specify only once; APL80 has them split in two.
pub fn enter(profile: Profile, expr: &Expr) -> AplResult<()> {
    let hyperbolic = profile.cuts(Cut::Hyperbolic);
    apl_c80_scalar::leave_out(hyperbolic, profile.cuts(Cut::FractionalFactorial));
    if !profile.cuts(Cut::MultipleSpecification) {
        return Ok(());
    }
    let inner = match expr {
        Expr::Assign { value, .. }
        | Expr::QuadOut { value, .. }
        | Expr::QuoteQuadOut { value, .. } => specified(value),
        Expr::IndexedAssign { value, indexes, .. } => {
            let mut parts = indexes.iter().flatten().chain([value.as_ref()]);
            parts.find_map(specified)
        }
        other => specified(other),
    };
    inner.map_or(Ok(()), |pos| Err(AplError::new(ErrorKind::Syntax).at(pos)))
}

/// Where `expr` specifies a name, anywhere inside it.
fn specified(expr: &Expr) -> Option<usize> {
    match expr {
        Expr::Assign { pos, .. } | Expr::IndexedAssign { pos, .. } => Some(*pos),
        _ => parts(expr).into_iter().find_map(specified),
    }
}

/// The expressions `expr` is made of.
fn parts(expr: &Expr) -> Vec<&Expr> {
    match expr {
        Expr::Monadic { axis, right, .. } => {
            axis.iter().chain([right]).map(AsRef::as_ref).collect()
        }
        Expr::Dyadic {
            axis, left, right, ..
        } => axis
            .iter()
            .chain([left, right])
            .map(AsRef::as_ref)
            .collect(),
        Expr::Index { array, indexes, .. } => {
            indexes.iter().flatten().chain([array.as_ref()]).collect()
        }
        Expr::QuadOut { value, .. } | Expr::QuoteQuadOut { value, .. } => vec![value],
        Expr::Branch { target, .. } => target.iter().map(AsRef::as_ref).collect(),
        Expr::Mixed(all) => all.iter().collect(),
        _ => Vec::new(),
    }
}
