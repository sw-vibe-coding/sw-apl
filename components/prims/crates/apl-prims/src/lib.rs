//! Primitive dispatch: one place that maps a glyph to its scalar,
//! mixed, or operator implementation, plus the evaluation
//! environment the primitives need (index origin, random link).

mod dispatch;
mod random;

pub use apl_prims_ops::reduce;
pub use dispatch::{apply_dyadic, apply_monadic};
pub use random::{Env, deal, roll};
