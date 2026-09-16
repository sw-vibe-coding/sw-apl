//! Primitive dispatch: one place that maps a glyph to its scalar,
//! mixed, or operator implementation.

mod dispatch;

pub use apl_prims_ops::reduce;
pub use dispatch::{apply_dyadic, apply_monadic};
