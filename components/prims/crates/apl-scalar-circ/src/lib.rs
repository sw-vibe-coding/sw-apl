//! Circular functions (`○`), factorial and binomial (`!`) with the
//! gamma function for non-integers.

mod circle;
mod gamma;

pub use circle::circular;
pub use gamma::{binomial, factorial, gamma};
