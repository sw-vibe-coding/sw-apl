//! Mixed (structural) primitive functions: iota, rho, ravel,
//! catenate.

mod iota;
mod join;
mod shape;

pub use iota::{iota, non_negative_int};
pub use join::{catenate, ravel};
pub use shape::{reshape, shape};
