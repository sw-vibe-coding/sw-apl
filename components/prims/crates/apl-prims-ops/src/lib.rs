//! Operators: reduce and scan along an axis, inner and outer products.

mod product;
mod reduce;
mod scan;

pub use product::{inner, outer};
pub use reduce::reduce;
pub use scan::scan;
