//! Arithmetic scalar functions on `f64`: plus, minus, times, divide,
//! maximum, minimum, residue, power, logarithm, exponential, floor
//! and ceiling with the fuzz. The caller demotes results to integers.

mod dyadic;
mod expo;
mod monadic;

pub use dyadic::dyadic_arith;
pub use monadic::monadic_arith;
