//! The scalar primitive family applied element by element with
//! scalar extension. The families themselves live in the
//! `apl-scalar-*` crates; this crate dispatches and extends.

mod dispatch;
mod extend;

pub use dispatch::{DYADIC, MONADIC, apply_dyadic, apply_monadic};
pub use extend::{dyadic, monadic, numbers};
