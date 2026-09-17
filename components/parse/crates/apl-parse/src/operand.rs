//! Operands (the array-valued pieces a function applies to), function
//! resolution, axis brackets, and calls to defined functions. Parsed
//! right to left, so `end` is exclusive and the return is the start.

use apl_ast::{Expr, Function};
use apl_lex::{Token, TokenKind};
use apl_scan::{Funcs, ends_operand, matching};
use apl_value::{AplError, AplResult, ErrorKind};

use crate::expr::{Parsed, parse_all};
use crate::index::{parse_indexed, parse_indexes};

/// Parse the operand ending just before `end`: a parenthesised
/// expression, an indexed operand, a literal, a name, or quad input.
pub fn parse_operand(tokens: &[Token], end: usize, funcs: Funcs) -> Parsed {
    let last = &tokens[end - 1];
    match &last.kind {
        TokenKind::RParen => {
            let open = matching(tokens, end - 1);
            Ok((parse_all(tokens, open + 1, end - 1, funcs)?, open))
        }
        TokenKind::RBracket => parse_indexed(tokens, end - 1, funcs),
        TokenKind::Numbers(a) | TokenKind::Chars(a) => Ok((Expr::Literal(a.clone()), end - 1)),
        TokenKind::Name(n) => Ok((Expr::Name(n.clone(), last.pos), end - 1)),
        TokenKind::Quad => Ok((Expr::QuadIn(last.pos), end - 1)),
        TokenKind::QuoteQuad => Ok((Expr::QuoteQuadIn(last.pos), end - 1)),
        _ => Err(AplError::new(ErrorKind::Syntax).at(last.pos)),
    }
}

/// The function whose rightmost glyph is at `at`: a derived function
/// when an operator pattern precedes it (`f/`, `f⌿`, `f\`, `f⍀`,
/// `f.g`, `∘.f`), else the primitive. Returns the function, the index
/// of its leftmost token, and that token's position.
pub fn resolve_function(tokens: &[Token], lo: usize, at: usize) -> (Function, usize, usize) {
    let glyph = |i: usize| match tokens.get(i).map(|t| &t.kind) {
        Some(TokenKind::Prim(g)) if i >= lo => Some(*g),
        _ => None,
    };
    let f = glyph(at).expect("resolve_function is called on a primitive");
    let (prev, prev2) = (
        at.checked_sub(1).and_then(glyph),
        at.checked_sub(2).and_then(glyph),
    );
    let is_fn = |g: char| !"/⌿\\⍀.∘".contains(g);
    let first = matches!(f, '⌿' | '⍀');
    let derived = match (prev2, prev, f) {
        (_, Some(g), '/' | '⌿') if is_fn(g) => Some((Function::Reduce { f: g, first }, at - 1)),
        (_, Some(g), '\\' | '⍀') if is_fn(g) => Some((Function::Scan { f: g, first }, at - 1)),
        (Some('∘'), Some('.'), g) if is_fn(g) => Some((Function::Outer { f: g }, at - 2)),
        (Some(h), Some('.'), g) if is_fn(h) && is_fn(g) => {
            Some((Function::Inner { f: h, g }, at - 2))
        }
        _ => None,
    };
    let (func, start) = derived.unwrap_or((Function::Prim(f), at));
    (func, start, tokens[start].pos)
}

/// An axis bracket closed at `close`: exactly one expression, and a
/// function glyph must precede the opening bracket. Returns the axis
/// and the index of that glyph.
pub fn parse_axis(
    tokens: &[Token],
    lo: usize,
    close: usize,
    funcs: Funcs,
) -> AplResult<(Box<Expr>, usize)> {
    let (mut idx, open) = parse_indexes(tokens, close, funcs)?;
    let after_function = open > lo && matches!(tokens[open - 1].kind, TokenKind::Prim(_));
    match (idx.len(), idx.pop()) {
        (1, Some(Some(axis))) if after_function => Ok((Box::new(axis), open - 1)),
        _ => Err(AplError::new(ErrorKind::Syntax).at(tokens[open].pos)),
    }
}

/// The defined function named at `at` applied to `right`: dyadic when
/// an operand ends immediately to its left, monadic otherwise. A
/// defined function never takes an axis.
pub fn apply_defined(tokens: &[Token], lo: usize, at: usize, right: Expr, funcs: Funcs) -> Parsed {
    let TokenKind::Name(name) = &tokens[at].kind else {
        unreachable!("apply_defined is called on a name")
    };
    let (func, pos) = (Function::Defined(name.clone()), tokens[at].pos);
    if at == lo || !ends_operand(tokens, at - 1, funcs) {
        return Ok((Expr::monadic(func, pos, None, right), at));
    }
    let (left, start) = parse_operand(tokens, at, funcs)?;
    Ok((Expr::dyadic(func, pos, None, left, right), start))
}
