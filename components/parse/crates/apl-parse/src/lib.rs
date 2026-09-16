//! sw-apl parser: tokens to an AST, right to left, with long right
//! scope for functions and a single array as the left argument.

mod ast;
mod expr;
mod operand;

pub use ast::Expr;
pub use expr::parse;
