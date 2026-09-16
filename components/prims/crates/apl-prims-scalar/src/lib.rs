//! Scalar primitive functions (plus, minus, times, divide, upstile,
//! downstile, stile, star) applied element by element with scalar
//! extension.

mod dyadic;
mod extend;
mod monadic;

pub use extend::{dyadic, monadic};
