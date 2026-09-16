//! Operands: the array-valued pieces a function applies to. Parsed
//! right to left, so `end` is exclusive and the return is the start.

use apl_lex::{Token, TokenKind};
use apl_value::{AplError, AplResult, Array, ErrorKind, Number};

use crate::ast::Expr;
use crate::expr::{Parsed, parse_expr};

/// Parse the operand ending just before `end`: a parenthesised
/// expression, a strand of numbers, a name, or quad input.
pub fn parse_operand(tokens: &[Token], end: usize) -> Parsed {
    let last = &tokens[end - 1];
    match &last.kind {
        TokenKind::RParen => {
            let open = matching_paren(tokens, end - 1)?;
            let (inner, start) = parse_expr(tokens, open + 1, end - 1)?;
            if start != open + 1 {
                return Err(AplError::new(ErrorKind::Syntax).at(tokens[start - 1].pos));
            }
            Ok((inner, open))
        }
        TokenKind::Number(_) => Ok(parse_strand(tokens, end)),
        TokenKind::Chars(a) => Ok((Expr::Literal(a.clone()), end - 1)),
        TokenKind::QuoteQuad => Ok((Expr::QuoteQuadIn(last.pos), end - 1)),
        TokenKind::Name(n) => Ok((Expr::Name(n.clone(), last.pos), end - 1)),
        TokenKind::Quad => Ok((Expr::QuadIn(last.pos), end - 1)),
        _ => Err(AplError::new(ErrorKind::Syntax).at(last.pos)),
    }
}

/// Consecutive numbers ending at `end` form one vector (or scalar).
fn parse_strand(tokens: &[Token], end: usize) -> (Expr, usize) {
    let mut start = end;
    let mut nums: Vec<Number> = Vec::new();
    while start > 0 {
        let TokenKind::Number(n) = tokens[start - 1].kind else {
            break;
        };
        nums.push(n);
        start -= 1;
    }
    nums.reverse();
    let array = if nums.len() == 1 {
        Array::scalar(nums[0])
    } else {
        Array::vector(nums)
    };
    (Expr::Literal(array), start)
}

/// Index of the `(` matching the `)` at `close`.
fn matching_paren(tokens: &[Token], close: usize) -> AplResult<usize> {
    let mut depth = 0usize;
    for i in (0..=close).rev() {
        match tokens[i].kind {
            TokenKind::RParen => depth += 1,
            TokenKind::LParen => {
                depth -= 1;
                if depth == 0 {
                    return Ok(i);
                }
            }
            _ => {}
        }
    }
    Err(AplError::new(ErrorKind::Syntax).at(tokens[close].pos))
}

/// The arrow at `at` assigns `value` to the name or the quad
/// immediately to its left.
pub fn apply_assign(tokens: &[Token], at: usize, value: Expr) -> Parsed {
    let value = Box::new(value);
    let target = at.checked_sub(1).map(|i| &tokens[i]);
    let expr = match target.map(|t| (&t.kind, t.pos)) {
        Some((TokenKind::Name(name), pos)) => Expr::Assign {
            name: name.clone(),
            pos,
            value,
        },
        Some((TokenKind::Quad, pos)) => Expr::QuadOut { pos, value },
        Some((TokenKind::QuoteQuad, pos)) => Expr::QuoteQuadOut { pos, value },
        _ => return Err(AplError::new(ErrorKind::Syntax).at(tokens[at].pos)),
    };
    Ok((expr, at - 1))
}
