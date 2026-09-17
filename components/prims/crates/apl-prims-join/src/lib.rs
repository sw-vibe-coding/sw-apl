//! Catenate and laminate along an axis, with scalar and
//! rank-minus-one conformance and origin-aware axis resolution.

mod axis;
mod join;

pub use axis::{Axis, resolve_axis};
pub use join::catenate;
