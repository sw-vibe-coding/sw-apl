//! Mixed (structural) primitive functions: iota, rho, ravel,
//! catenate.

mod iota;
mod shape;

pub use iota::{int_vector, iota, non_negative_int};
pub use shape::{ravel, reshape, shape};
