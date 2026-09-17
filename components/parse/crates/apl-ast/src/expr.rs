//! Expressions.

use apl_value::Array;

use crate::function::Function;

/// An index list: one entry per axis, `None` when elided.
pub type Indexes = Vec<Option<Expr>>;

/// An expression. Positions are character offsets for carets.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// A numeric or character literal (strands already joined).
    Literal(Array),
    /// A variable reference and its position.
    Name(String, usize),
    /// Bare quad on the right: evaluated input.
    QuadIn(usize),
    /// Quote-quad on the right: character input.
    QuoteQuadIn(usize),
    /// `f right`, with an optional axis `f[k]`.
    Monadic {
        func: Function,
        pos: usize,
        axis: Option<Box<Expr>>,
        right: Box<Expr>,
    },
    /// `left f right`, with an optional axis.
    Dyadic {
        func: Function,
        pos: usize,
        axis: Option<Box<Expr>>,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    /// `array[i;j]`.
    Index {
        array: Box<Expr>,
        pos: usize,
        indexes: Indexes,
    },
    /// `name ← value`; yields the value.
    Assign {
        name: String,
        pos: usize,
        value: Box<Expr>,
    },
    /// `name[i;j] ← value`; yields the value.
    IndexedAssign {
        name: String,
        pos: usize,
        indexes: Indexes,
        value: Box<Expr>,
    },
    /// `⎕ ← value`: display now; yields the value.
    QuadOut { pos: usize, value: Box<Expr> },
    /// `⍞ ← value`: character output without a newline.
    QuoteQuadOut { pos: usize, value: Box<Expr> },
    /// `→ target`, or a bare `→` (no target).
    Branch {
        pos: usize,
        target: Option<Box<Expr>>,
    },
    /// `'TEXT';X;...`: mixed output, top level only.
    Mixed(Vec<Expr>),
}

impl Expr {
    /// `func right`.
    #[must_use]
    pub fn monadic(func: Function, pos: usize, axis: Option<Box<Expr>>, right: Expr) -> Expr {
        let right = Box::new(right);
        Expr::Monadic {
            func,
            pos,
            axis,
            right,
        }
    }

    /// `left func right`.
    #[must_use]
    pub fn dyadic(
        func: Function,
        pos: usize,
        axis: Option<Box<Expr>>,
        left: Expr,
        right: Expr,
    ) -> Expr {
        let (left, right) = (Box::new(left), Box::new(right));
        Expr::Dyadic {
            func,
            pos,
            axis,
            left,
            right,
        }
    }

    /// `name[indexes] ← value`.
    #[must_use]
    pub fn indexed_assign(name: &str, pos: usize, indexes: Indexes, value: Box<Expr>) -> Expr {
        let name = name.to_string();
        Expr::IndexedAssign {
            name,
            pos,
            indexes,
            value,
        }
    }

    /// `→ target`.
    #[must_use]
    pub fn branch(pos: usize, target: Option<Expr>) -> Expr {
        let target = target.map(Box::new);
        Expr::Branch { pos, target }
    }
}
