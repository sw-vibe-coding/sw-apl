//! Mixed (structural) primitive functions: iota, rho, ravel,
//! catenate.

mod counts;
mod iota;
mod shape;

pub use counts::{bool_vector, int_vector, non_negative_int};
pub use iota::iota;
pub use shape::{ravel, reshape, shape};
