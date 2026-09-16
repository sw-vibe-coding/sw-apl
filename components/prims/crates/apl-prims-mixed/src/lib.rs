//! Mixed (structural) primitive functions: iota, rho, ravel,
//! catenate.

mod iota;
mod join;
mod shape;

pub use iota::iota;
pub use join::{catenate, ravel};
pub use shape::{reshape, shape};
