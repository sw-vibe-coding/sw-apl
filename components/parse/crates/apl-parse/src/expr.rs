//! Statement and expression parsing, right to left.

use apl_lex::{Token, TokenKind, tokenize};
use apl_value::{AplError, AplResult, ErrorKind};

use crate::ast::Expr;
use crate::operand::{apply_assign, parse_operand};

/// An expression and the token index where it starts.
pub type Parsed = AplResult<(Expr, usize)>;

/// Parse one statement line. `None` for a blank or comment-only line.
///
/// # Errors
/// Lexical errors and SYNTAX ERROR, each with a caret.
pub fn parse(line: &str) -> AplResult<Option<Expr>> {
    let tokens = tokenize(line)?;
    if tokens.is_empty() {
        return Ok(None);
    }
    let (expr, start) = parse_expr(&tokens, 0, tokens.len())?;
    if start != 0 {
        return Err(AplError::new(ErrorKind::Syntax).at(tokens[start - 1].pos));
    }
    Ok(Some(expr))
}

/// Parse the expression in `tokens[lo..end]`, consuming leftwards from
/// `end` as far as the grammar allows. Returns the expression and the
/// index where it starts (`lo` when everything was consumed).
pub fn parse_expr(tokens: &[Token], lo: usize, end: usize) -> Parsed {
    if end <= lo {
        let pos = tokens.get(end).map_or(0, |t| t.pos);
        return Err(AplError::new(ErrorKind::Syntax).at(pos));
    }
    let (mut right, mut start) = parse_operand(tokens, end)?;
    while start > lo {
        let tok = &tokens[start - 1];
        (right, start) = match &tok.kind {
            TokenKind::Prim(f) => apply_function(tokens, lo, start - 1, *f, right)?,
            TokenKind::Assign => apply_assign(tokens, start - 1, right)?,
            TokenKind::LParen => break,
            _ => return Err(AplError::new(ErrorKind::Syntax).at(tok.pos)),
        };
    }
    Ok((right, start))
}

/// The glyph at `at` applies to `right`: a reduce when it is a slash
/// with a function to its left, dyadic when an operand ends there,
/// monadic otherwise.
fn apply_function(tokens: &[Token], lo: usize, at: usize, f: char, right: Expr) -> Parsed {
    let (pos, right) = (tokens[at].pos, Box::new(right));
    let left = (at > lo).then(|| &tokens[at - 1]);
    if let Some(reduce) = apply_reduce(left, f, right.clone()) {
        return Ok((reduce, at - 1));
    }
    if !left.is_some_and(|t| t.kind.ends_operand()) {
        return Ok((Expr::Monadic { f, pos, right }, at));
    }
    let (left, start) = parse_operand(tokens, at)?;
    let left = Box::new(left);
    Ok((
        Expr::Dyadic {
            f,
            pos,
            left,
            right,
        },
        start,
    ))
}

/// `g/right` when `f` is the slash and the token to its left is a
/// function glyph `g`.
fn apply_reduce(left: Option<&Token>, f: char, right: Box<Expr>) -> Option<Expr> {
    match (f, left) {
        (
            '/',
            Some(Token {
                kind: TokenKind::Prim(g),
                pos,
            }),
        ) => Some(Expr::Reduce {
            f: *g,
            pos: *pos,
            right,
        }),
        _ => None,
    }
}
