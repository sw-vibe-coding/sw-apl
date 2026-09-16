//! Tokenizer: numbers with high minus, strands, names, glyphs,
//! comments, and CHARACTER ERROR for anything outside the table.

use apl_lex::{Token, TokenKind, tokenize};
use apl_value::{ErrorKind, Number};

fn kinds(line: &str) -> Vec<TokenKind> {
    tokenize(line)
        .unwrap()
        .into_iter()
        .map(|t| t.kind)
        .collect()
}

#[test]
fn numbers_including_high_minus_float_and_exponent() {
    assert_eq!(
        kinds("3 \u{af}4 2.5 1E3 \u{af}1.5E\u{af}2"),
        vec![
            TokenKind::Number(Number::Int(3)),
            TokenKind::Number(Number::Int(-4)),
            TokenKind::Number(Number::Float(2.5)),
            TokenKind::Number(Number::Int(1000)),
            TokenKind::Number(Number::Float(-0.015)),
        ]
    );
}

#[test]
fn names_glyphs_parens_and_assignment() {
    assert_eq!(
        kinds("AB\u{2206}1 \u{2190} (2 \u{d7} B)"),
        vec![
            TokenKind::Name("AB\u{2206}1".to_string()),
            TokenKind::Assign,
            TokenKind::LParen,
            TokenKind::Number(Number::Int(2)),
            TokenKind::Prim('\u{d7}'),
            TokenKind::Name("B".to_string()),
            TokenKind::RParen,
        ]
    );
}

#[test]
fn ascii_minus_is_a_function_not_a_sign() {
    assert_eq!(
        kinds("-3"),
        vec![TokenKind::Prim('-'), TokenKind::Number(Number::Int(3))]
    );
}

#[test]
fn positions_are_char_indexes() {
    let toks: Vec<Token> = tokenize("\u{2374} 1 2").unwrap();
    assert_eq!(toks[0].pos, 0);
    assert_eq!(toks[1].pos, 2);
    assert_eq!(toks[2].pos, 4);
}

#[test]
fn lamp_comment_ends_the_line() {
    assert_eq!(
        kinds("1 \u{235d} 2 + anything \u{3c1}"),
        vec![TokenKind::Number(Number::Int(1))]
    );
    assert_eq!(kinds("\u{235d} only"), vec![]);
}

#[test]
fn every_glyph_in_the_table_is_a_primitive_token() {
    for g in "×÷⌈⌊⍟○∧∨⍲⍱≤≥≠⍳⍴⌽⊖⍉↑↓⌿⍀⊥⊤∊⍋⍒⌹⍎⍕∘+-*|!~<=>?,./\\".chars()
    {
        assert_eq!(kinds(&g.to_string()), vec![TokenKind::Prim(g)], "{g}");
    }
}

#[test]
fn lookalikes_and_controls_are_character_errors() {
    for (text, bad) in [("\u{3c1}5", '\u{3c1}'), ("1\t2", '\t'), ("a#b", '#')] {
        let err = tokenize(text).unwrap_err();
        assert_eq!(err.kind, ErrorKind::Character(bad));
        assert_eq!(
            err.caret,
            Some(text.chars().position(|c| c == bad).unwrap())
        );
    }
}

#[test]
fn bad_number_is_a_syntax_error() {
    let err = tokenize("1.2.3").unwrap_err();
    assert_eq!(err.kind, ErrorKind::Syntax);
}
