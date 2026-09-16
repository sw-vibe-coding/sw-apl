//! The abstract syntax tree for one statement.

use apl_value::Array;

/// An expression. Positions are character offsets for carets.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// A numeric literal or strand.
    Literal(Array),
    /// A variable reference and its position.
    Name(String, usize),
    /// Bare quad on the right: evaluated input.
    QuadIn(usize),
    /// Quote-quad on the right: character input.
    QuoteQuadIn(usize),
    /// `⍞ ← value`: character output without a newline.
    QuoteQuadOut { pos: usize, value: Box<Expr> },
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
    /// `f/right`: reduce along the last axis.
    Reduce {
        f: char,
        pos: usize,
        right: Box<Expr>,
    },
    /// `name ← value`; yields the value.
    Assign {
        name: String,
        pos: usize,
        value: Box<Expr>,
    },
    /// `⎕ ← value`: display now; yields the value.
    QuadOut { pos: usize, value: Box<Expr> },
}
