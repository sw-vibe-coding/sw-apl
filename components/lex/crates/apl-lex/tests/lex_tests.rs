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
    for g in "×÷⌈⌊⍟○∧∨⍲⍱≤≥≠⍳⍴⌽⊖⍉↑↓⌿⍀⊥⊤∊⍋⍒⌹⌶∘+-*|!~<=>?,./\\".chars()
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

#[test]
fn quad_is_always_a_bare_token() {
    assert_eq!(
        kinds("\u{2395}\u{2190}\u{2395}IO"),
        vec![
            TokenKind::Quad,
            TokenKind::Assign,
            TokenKind::Quad,
            TokenKind::Name("IO".to_string()),
        ]
    );
}

#[test]
fn glyphs_from_later_apls_are_character_errors() {
    for bad in [
        '\u{234e}', '\u{2355}', '\u{237a}', '\u{2375}', '\u{2282}', '\u{a8}',
    ] {
        let err = tokenize(&bad.to_string()).unwrap_err();
        assert_eq!(err.kind, ErrorKind::Character(bad), "{bad}");
    }
}

fn chars_of(kind: &TokenKind) -> String {
    match kind {
        TokenKind::Chars(a) => match &a.data {
            apl_value::Data::Char(v) => v.iter().collect(),
            apl_value::Data::Num(_) => panic!("numeric"),
        },
        other => panic!("{other:?}"),
    }
}

#[test]
fn quoted_literals_with_doubled_quotes_and_any_unicode_inside() {
    let k = kinds("'HELLO' 'it''s' '' 'A' '\u{3c1}\u{235d}x'");
    assert_eq!(chars_of(&k[0]), "HELLO");
    assert_eq!(chars_of(&k[1]), "it's");
    assert_eq!(chars_of(&k[2]), "");
    assert_eq!(chars_of(&k[3]), "A");
    assert_eq!(chars_of(&k[4]), "\u{3c1}\u{235d}x");
    let TokenKind::Chars(one) = &k[3] else {
        panic!()
    };
    assert_eq!(one.shape, Vec::<usize>::new(), "one character is a scalar");
    let TokenKind::Chars(five) = &k[0] else {
        panic!()
    };
    assert_eq!(five.shape, vec![5]);
    let TokenKind::Chars(none) = &k[2] else {
        panic!()
    };
    assert_eq!(none.shape, vec![0]);
}

#[test]
fn unterminated_quote_is_a_syntax_error_at_the_quote() {
    let err = tokenize("1 2 'abc").unwrap_err();
    assert_eq!(err.kind, ErrorKind::Syntax);
    assert_eq!(err.caret, Some(4));
}

#[test]
fn punctuation_and_sentinel_tokens() {
    assert_eq!(
        kinds("A[1;2] \u{2192}L L: \u{2207}R\u{2190}F X \u{236b} \u{235e}"),
        vec![
            TokenKind::Name("A".into()),
            TokenKind::LBracket,
            TokenKind::Number(Number::Int(1)),
            TokenKind::Semicolon,
            TokenKind::Number(Number::Int(2)),
            TokenKind::RBracket,
            TokenKind::Branch,
            TokenKind::Name("L".into()),
            TokenKind::Name("L".into()),
            TokenKind::Colon,
            TokenKind::Del,
            TokenKind::Name("R".into()),
            TokenKind::Assign,
            TokenKind::Name("F".into()),
            TokenKind::Name("X".into()),
            TokenKind::DelTilde,
            TokenKind::QuoteQuad,
        ]
    );
}

#[test]
fn a_line_starting_with_a_right_paren_is_a_system_command() {
    assert_eq!(
        kinds("  )VARS A  "),
        vec![TokenKind::SystemCommand("VARS A".to_string())]
    );
    assert_eq!(tokenize(")OFF").unwrap()[0].pos, 0);
}

#[test]
fn names_may_hold_delta_letters_and_digits() {
    assert_eq!(
        kinds("\u{2206}X1 \u{2359}Y \u{2206}"),
        vec![
            TokenKind::Name("\u{2206}X1".into()),
            TokenKind::Name("\u{2359}Y".into()),
            TokenKind::Name("\u{2206}".into()),
        ]
    );
    assert_eq!(tokenize("1X").unwrap_err().kind, ErrorKind::Syntax);
}
