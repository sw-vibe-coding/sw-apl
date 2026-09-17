//! Bracket matching, semicolon segments, and what ends an operand.

use apl_lex::{Token, TokenKind};

/// Which names hold a defined function that takes an argument on its
/// right. APL cannot tell `F B` (a call) from `A B` (a syntax error)
/// without it. A niladic function is not one: it produces a value
/// where it stands, so it parses as an operand and the evaluator
/// calls it.
pub type Funcs<'a> = &'a dyn Fn(&str) -> bool;

/// Index of the opener matching the closer at `close` (the lexer has
/// already checked balance).
#[must_use]
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
#[must_use]
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

/// True when the token at `i` ends an operand: a glyph immediately to
/// its right then has an array on its left and is dyadic. A name that
/// holds a function taking a right argument is a function, not an
/// array, so it ends nothing; a niladic one stands for its value and
/// does.
#[must_use]
pub fn ends_operand(tokens: &[Token], i: usize, funcs: Funcs) -> bool {
    match &tokens[i].kind {
        TokenKind::Name(n) => !funcs(n),
        kind => kind.ends_operand(),
    }
}
