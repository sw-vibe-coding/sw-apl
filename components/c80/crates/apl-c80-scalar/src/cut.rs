//! What this thread's session leaves out, and the two questions the
//! scalar dispatcher asks of it.

use std::cell::Cell;

use apl_value::{AplError, AplResult, ErrorKind};

thread_local! {
    /// Whether the hyperbolic circle functions are left out.
    static HYPERBOLIC: Cell<bool> = const { Cell::new(false) };
    /// Whether factorial and binomial take only whole numbers.
    static WHOLE: Cell<bool> = const { Cell::new(false) };
}

/// Set what the session on this thread leaves out. The evaluator sets
/// it from the session's mode before every line, so a thread that ran
/// a (C) session and then an (A) one carries nothing over.
pub fn leave_out(hyperbolic: bool, whole_factorial: bool) {
    HYPERBOLIC.with(|cut| cut.set(hyperbolic));
    WHOLE.with(|cut| cut.set(whole_factorial));
}

/// Whether `left ○ x` may be taken.
///
/// # Errors
/// NONCE ERROR for a hyperbolic function, `5○` to `7○` and their
/// inverses, where they are left out.
pub fn circular(left: f64) -> AplResult<()> {
    let k = left.abs();
    let hyperbolic = k.fract() == 0.0 && (5.0..=7.0).contains(&k);
    if hyperbolic && HYPERBOLIC.with(Cell::get) {
        return Err(AplError::new(ErrorKind::Nonce));
    }
    Ok(())
}

/// Whether `x` may be an argument of factorial or binomial.
///
/// # Errors
/// DOMAIN ERROR for a number that is not whole, where only whole ones
/// are taken.
pub fn factorial(x: f64) -> AplResult<()> {
    if x.fract() != 0.0 && WHOLE.with(Cell::get) {
        return Err(AplError::new(ErrorKind::Domain));
    }
    Ok(())
}
