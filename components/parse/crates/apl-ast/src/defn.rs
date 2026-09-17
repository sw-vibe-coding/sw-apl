//! Defined functions: the del header, the body as typed, and the
//! call an expression makes to one.

use crate::expr::Expr;
use crate::function::Function;

/// A function defined with the del form. The body holds the lines as
/// they were entered; they are parsed when the function runs, so a
/// function can call itself and functions defined after it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Defn {
    /// The function's own name.
    pub name: String,
    /// The result variable, when the header starts `R←`.
    pub result: Option<String>,
    /// The left argument's name, when the header is dyadic.
    pub left: Option<String>,
    /// The right argument's name, unless the header is niladic.
    pub right: Option<String>,
    /// The names written after semicolons.
    pub locals: Vec<String>,
    /// Body lines, numbered from 1.
    pub body: Vec<String>,
}

impl Defn {
    /// Every name a call makes local: the result, the arguments, and
    /// the names after the semicolons.
    #[must_use]
    pub fn names(&self) -> Vec<String> {
        let args = [&self.result, &self.left, &self.right];
        args.into_iter()
            .flatten()
            .chain(self.locals.iter())
            .cloned()
            .collect()
    }
}

impl Expr {
    /// The defined function this expression calls outright, as
    /// `(name, position, left, right)` with the arguments still
    /// unevaluated. A bare name qualifies: the evaluator knows
    /// whether it holds a function or a variable.
    #[must_use]
    pub fn defined_call(&self) -> Option<(&str, usize, Option<&Expr>, Option<&Expr>)> {
        match self {
            Expr::Name(n, pos) => Some((n, *pos, None, None)),
            Expr::Monadic {
                func: Function::Defined(n),
                pos,
                right,
                ..
            } => Some((n, *pos, None, Some(right))),
            Expr::Dyadic {
                func: Function::Defined(n),
                pos,
                left,
                right,
                ..
            } => Some((n, *pos, Some(left), Some(right))),
            _ => None,
        }
    }
}
