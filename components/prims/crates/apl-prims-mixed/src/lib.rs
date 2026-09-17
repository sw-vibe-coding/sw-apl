//! Mixed (structural) primitive functions: iota, rho, ravel,
//! catenate.

mod iota;
mod shape;

pub use iota::{bool_vector, int_vector, iota, non_negative_int};
pub use shape::{ravel, reshape, shape};
