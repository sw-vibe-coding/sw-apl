//! Statement and expression parsing, right to left.

use apl_lex::{Token, TokenKind, tokenize};
use apl_value::{AplError, AplResult, ErrorKind};

use crate::ast::Expr;
use crate::operand::{ends_operand, parse_operand};

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
pub fn parse_expr(tokens: &[Token], lo: usize, end: usize) -> AplResult<(Expr, usize)> {
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

/// The glyph at `at` applies to `right`; dyadic when an operand ends
/// immediately to its left, monadic otherwise.
fn apply_function(
    tokens: &[Token],
    lo: usize,
    at: usize,
    f: char,
    right: Expr,
) -> AplResult<(Expr, usize)> {
    let (pos, right) = (tokens[at].pos, Box::new(right));
    if !(at > lo && ends_operand(&tokens[at - 1].kind)) {
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

/// The arrow at `at` must have a name to its left.
fn apply_assign(tokens: &[Token], at: usize, value: Expr) -> AplResult<(Expr, usize)> {
    match at.checked_sub(1).map(|i| &tokens[i]) {
        Some(Token {
            kind: TokenKind::Name(name),
            pos,
        }) => Ok((
            Expr::Assign {
                name: name.clone(),
                pos: *pos,
                value: Box::new(value),
            },
            at - 1,
        )),
        _ => Err(AplError::new(ErrorKind::Syntax).at(tokens[at].pos)),
    }
}
