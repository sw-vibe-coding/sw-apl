//! The statement shapes that do not evaluate to one value, which the
//! evaluator tries before it evaluates an expression.

mod statement;

pub use statement::{Eval, statement};
