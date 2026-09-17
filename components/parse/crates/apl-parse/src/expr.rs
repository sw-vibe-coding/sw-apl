//! Statement and expression parsing, right to left.

use apl_ast::{Expr, Function};
use apl_lex::{Token, TokenKind, tokenize};
use apl_value::{AplError, AplResult, ErrorKind};

use crate::bracket::segments;
use crate::operand::{apply_assign, parse_axis, parse_operand, resolve_function};

/// An expression and the token index where it starts.
pub type Parsed = AplResult<(Expr, usize)>;

/// Parse one statement line. `None` for a blank or comment-only line.
/// Top-level semicolons make a mixed-output statement.
///
/// # Errors
/// Lexical errors and SYNTAX ERROR, each with a caret.
pub fn parse(line: &str) -> AplResult<Option<Expr>> {
    let tokens = tokenize(line)?;
    if let [
        Token {
            kind: TokenKind::Branch,
            pos,
        },
    ] = tokens.as_slice()
    {
        return Ok(Some(Expr::branch(*pos, None)));
    }
    if tokens.is_empty() {
        return Ok(None);
    }
    let ranges = segments(&tokens, 0, tokens.len());
    if ranges.len() == 1 {
        return parse_all(&tokens, 0, tokens.len()).map(Some);
    }
    let parts = ranges
        .into_iter()
        .map(|(lo, hi)| parse_all(&tokens, lo, hi))
        .collect::<AplResult<Vec<_>>>()?;
    Ok(Some(Expr::Mixed(parts)))
}

/// The expression that fills `tokens[lo..hi]` exactly.
pub fn parse_all(tokens: &[Token], lo: usize, hi: usize) -> AplResult<Expr> {
    if lo >= hi {
        let near = tokens.get(lo).or_else(|| tokens.get(lo.wrapping_sub(1)));
        return Err(AplError::new(ErrorKind::Syntax).at(near.map_or(0, |t| t.pos)));
    }
    let (expr, start) = parse_expr(tokens, lo, hi)?;
    if start != lo {
        return Err(AplError::new(ErrorKind::Syntax).at(tokens[start - 1].pos));
    }
    Ok(expr)
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
            TokenKind::Prim(_) => apply_function(tokens, lo, start - 1, None, right)?,
            TokenKind::RBracket => {
                let (axis, at) = parse_axis(tokens, lo, start - 1)?;
                apply_function(tokens, lo, at, Some(axis), right)?
            }
            TokenKind::Assign => apply_assign(tokens, start - 1, right)?,
            TokenKind::Branch if start - 1 == lo => (Expr::branch(tok.pos, Some(right)), lo),
            TokenKind::LParen => break,
            _ => return Err(AplError::new(ErrorKind::Syntax).at(tok.pos)),
        };
    }
    Ok((right, start))
}

/// The function whose rightmost glyph is at `at` applies to `right`:
/// dyadic when an operand ends immediately to its left, monadic
/// otherwise. Reduce and scan take no left argument; inner and outer
/// products need one; a lone dot or jot is not a function.
fn apply_function(
    tokens: &[Token],
    lo: usize,
    at: usize,
    axis: Option<Box<Expr>>,
    right: Expr,
) -> Parsed {
    let (func, start, pos) = resolve_function(tokens, lo, at);
    let has_left = start > lo && tokens[start - 1].kind.ends_operand();
    let bad = match func {
        Function::Prim('.' | '∘') => true,
        Function::Reduce { .. } | Function::Scan { .. } => has_left,
        Function::Inner { .. } | Function::Outer { .. } => !has_left,
        Function::Prim(_) => false,
    };
    if bad {
        return Err(AplError::new(ErrorKind::Syntax).at(pos));
    }
    if !has_left {
        return Ok((Expr::monadic(func, pos, axis, right), start));
    }
    let (left, start) = parse_operand(tokens, start)?;
    Ok((Expr::dyadic(func, pos, axis, left, right), start))
}
