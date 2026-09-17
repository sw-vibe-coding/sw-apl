//! The abstract syntax tree for one APL\360 statement. Types only;
//! `apl-parse` builds them and `apl-eval` walks them.

mod defn;
mod expr;
mod function;

pub use defn::Defn;
pub use expr::{Expr, Indexes};
pub use function::Function;
