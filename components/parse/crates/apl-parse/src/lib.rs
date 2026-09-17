//! sw-apl parser: tokens to an AST, right to left, with long right
//! scope for functions, a single array as the left argument, derived
//! functions (reduce, scan, inner and outer product, axis), bracket
//! indexing, assignment forms, calls to defined functions, branch,
//! and mixed output.

mod expr;
mod index;
mod operand;

pub use apl_ast::{Defn, Expr, Function, Indexes};
pub use apl_scan::{Funcs, parse_header};
pub use expr::parse;
