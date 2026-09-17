//! Primitive dispatch: one place that maps a glyph to its scalar,
//! mixed, or operator implementation, plus the evaluation
//! environment the primitives need (index origin, random link).

mod dispatch;
mod random;

pub use apl_prims_index::{index, indexed_assign};
pub use apl_prims_ops::{inner, outer, reduce, scan};
pub use apl_prims_select::axis_index;
pub use dispatch::{apply_dyadic, apply_monadic};
pub use random::{Env, deal, roll};
