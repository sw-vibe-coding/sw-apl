//! The abstract syntax tree for one APL\360 statement. Types only;
//! `apl-parse` builds them and `apl-eval` walks them.

mod expr;
mod function;

pub use expr::{Expr, Indexes};
pub use function::Function;
