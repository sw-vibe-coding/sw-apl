//! The abstract syntax tree for one statement.

use apl_value::Array;

/// An expression. Positions are character offsets for carets.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// A numeric literal or strand.
    Literal(Array),
    /// A variable reference and its position.
    Name(String, usize),
    /// `f right`.
    Monadic {
        f: char,
        pos: usize,
        right: Box<Expr>,
    },
    /// `left f right`.
    Dyadic {
        f: char,
        pos: usize,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    /// `name ← value`; yields the value.
    Assign {
        name: String,
        pos: usize,
        value: Box<Expr>,
    },
}
