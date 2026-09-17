//! Brackets and parentheses: matching, index lists, indexed operands.

use apl_ast::{Expr, Indexes};
use apl_lex::{Token, TokenKind};
use apl_value::{AplError, AplResult, ErrorKind};

use crate::expr::{Parsed, parse_expr};
use crate::operand::parse_operand;

/// Index of the opener matching the closer at `close` (the lexer has
/// already checked balance).
pub fn matching(tokens: &[Token], close: usize) -> usize {
    let mut depth = 0usize;
    for i in (0..=close).rev() {
        match tokens[i].kind {
            TokenKind::RParen | TokenKind::RBracket => depth += 1,
            TokenKind::LParen | TokenKind::LBracket => {
                depth -= 1;
                if depth == 0 {
                    return i;
                }
            }
            _ => {}
        }
    }
    0
}

/// Split `tokens[lo..hi]` at top-level semicolons into `(lo, hi)`
/// ranges; an empty range is an elided or missing segment.
pub fn segments(tokens: &[Token], lo: usize, hi: usize) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let (mut depth, mut start) = (0usize, lo);
    for (i, tok) in tokens.iter().enumerate().take(hi).skip(lo) {
        match tok.kind {
            TokenKind::LParen | TokenKind::LBracket => depth += 1,
            TokenKind::RParen | TokenKind::RBracket => depth -= 1,
            TokenKind::Semicolon if depth == 0 => {
                ranges.push((start, i));
                start = i + 1;
            }
            _ => {}
        }
    }
    ranges.push((start, hi));
    ranges
}

/// The index list closed by the bracket at `close`; returns it with
/// the index of the opening bracket.
pub fn parse_indexes(tokens: &[Token], close: usize) -> AplResult<(Indexes, usize)> {
    let open = matching(tokens, close);
    let mut indexes = Vec::new();
    for (lo, hi) in segments(tokens, open + 1, close) {
        if lo == hi {
            indexes.push(None);
            continue;
        }
        let (expr, start) = parse_expr(tokens, lo, hi)?;
        if start != lo {
            return Err(AplError::new(ErrorKind::Syntax).at(tokens[start - 1].pos));
        }
        indexes.push(Some(expr));
    }
    Ok((indexes, open))
}

/// An indexed operand `array[...]` whose closing bracket is at `close`.
pub fn parse_indexed(tokens: &[Token], close: usize) -> Parsed {
    let (indexes, open) = parse_indexes(tokens, close)?;
    if open == 0 || !tokens[open - 1].kind.ends_operand() {
        return Err(AplError::new(ErrorKind::Syntax).at(tokens[open].pos));
    }
    let (array, start) = parse_operand(tokens, open)?;
    let (array, pos) = (Box::new(array), tokens[open].pos);
    Ok((
        Expr::Index {
            array,
            pos,
            indexes,
        },
        start,
    ))
}
