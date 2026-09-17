//! Bracket index lists, indexed operands, and assignment targets.

use apl_ast::{Expr, Indexes};
use apl_lex::{Token, TokenKind};
use apl_scan::{Funcs, ends_operand, matching, segments};
use apl_value::{AplError, AplResult, ErrorKind};

use crate::expr::{Parsed, parse_expr};
use crate::operand::parse_operand;

/// The index list closed by the bracket at `close`; returns it with
/// the index of the opening bracket.
pub fn parse_indexes(tokens: &[Token], close: usize, funcs: Funcs) -> AplResult<(Indexes, usize)> {
    let open = matching(tokens, close);
    let mut indexes = Vec::new();
    for (lo, hi) in segments(tokens, open + 1, close) {
        if lo == hi {
            indexes.push(None);
            continue;
        }
        let (expr, start) = parse_expr(tokens, lo, hi, funcs)?;
        if start != lo {
            return Err(AplError::new(ErrorKind::Syntax).at(tokens[start - 1].pos));
        }
        indexes.push(Some(expr));
    }
    Ok((indexes, open))
}

/// An indexed operand `array[...]` whose closing bracket is at `close`.
pub fn parse_indexed(tokens: &[Token], close: usize, funcs: Funcs) -> Parsed {
    let (indexes, open) = parse_indexes(tokens, close, funcs)?;
    if open == 0 || !ends_operand(tokens, open - 1, funcs) {
        return Err(AplError::new(ErrorKind::Syntax).at(tokens[open].pos));
    }
    let (array, start) = parse_operand(tokens, open, funcs)?;
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

/// The arrow at `at` assigns `value` to the name, indexed name, quad,
/// or quote-quad immediately to its left.
pub fn apply_assign(tokens: &[Token], at: usize, value: Expr, funcs: Funcs) -> Parsed {
    let syntax = || AplError::new(ErrorKind::Syntax).at(tokens[at].pos);
    let value = Box::new(value);
    let (indexes, end) = match tokens.get(at.wrapping_sub(1)).map(|t| &t.kind) {
        Some(TokenKind::RBracket) => {
            parse_indexes(tokens, at - 1, funcs).map(|(i, open)| (Some(i), open))?
        }
        _ => (None, at),
    };
    let target = end.checked_sub(1).map(|i| &tokens[i]).ok_or_else(syntax)?;
    let pos = target.pos;
    let expr = match (&target.kind, indexes) {
        (TokenKind::Name(n), Some(indexes)) => Expr::indexed_assign(n, pos, indexes, value),
        (TokenKind::Name(name), None) => Expr::Assign {
            name: name.clone(),
            pos,
            value,
        },
        (TokenKind::Quad, None) => Expr::QuadOut { pos, value },
        (TokenKind::QuoteQuad, None) => Expr::QuoteQuadOut { pos, value },
        _ => return Err(syntax()),
    };
    Ok((expr, end - 1))
}
