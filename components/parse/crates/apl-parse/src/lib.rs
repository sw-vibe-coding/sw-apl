//! sw-apl parser: tokens to an AST, right to left, with long right
//! scope for functions, a single array as the left argument, derived
//! functions (reduce, scan, inner and outer product, axis), bracket
//! indexing, assignment forms, branch, and mixed output.

mod bracket;
mod expr;
mod operand;

pub use apl_ast::{Expr, Function, Indexes};
pub use expr::parse;
