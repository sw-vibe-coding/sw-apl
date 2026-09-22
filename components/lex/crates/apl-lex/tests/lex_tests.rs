//! Tokenizer: numbers with high minus, strands, names, glyphs,
//! comments, and CHARACTER ERROR for anything outside the table.

use apl_lex::{Token, TokenKind, tokenize};
use apl_value::{Array, ErrorKind, Number};

fn num(v: i64) -> TokenKind {
    TokenKind::Numbers(Array::scalar(Number::Int(v)))
}
fn strand(v: &[Number]) -> TokenKind {
    TokenKind::Numbers(Array::vector(v.to_vec()))
}

fn kinds(line: &str) -> Vec<TokenKind> {
    tokenize(line, "")
        .unwrap()
        .into_iter()
        .map(|t| t.kind)
        .collect()
}

#[test]
fn numbers_including_high_minus_float_and_exponent() {
    assert_eq!(
        kinds("3 \u{af}4 2.5 1E3 \u{af}1.5E\u{af}2"),
        vec![strand(&[
            Number::Int(3),
            Number::Int(-4),
            Number::Float(2.5),
            Number::Int(1000),
            Number::Float(-0.015),
        ])]
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
            num(2),
            TokenKind::Prim('\u{d7}'),
            TokenKind::Name("B".to_string()),
            TokenKind::RParen,
        ]
    );
}

#[test]
fn ascii_minus_is_a_function_not_a_sign() {
    assert_eq!(kinds("-3"), vec![TokenKind::Prim('-'), num(3)]);
}

#[test]
fn positions_are_char_indexes() {
    let toks: Vec<Token> = tokenize("\u{2374} 1 2 +3", "").unwrap();
    assert_eq!(toks[0].pos, 0);
    assert_eq!(toks[1].pos, 2, "a strand sits at its first number");
    assert_eq!(toks[2].pos, 6);
    assert_eq!(toks[3].pos, 7);
}

#[test]
fn lamp_comment_ends_the_line() {
    assert_eq!(kinds("1 \u{235d} 2 + anything \u{3c1}"), vec![num(1)]);
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
        let err = tokenize(text, "").unwrap_err();
        assert_eq!(err.kind, ErrorKind::Character(bad));
        assert_eq!(
            err.caret,
            Some(text.chars().position(|c| c == bad).unwrap())
        );
    }
}

#[test]
fn bad_number_is_a_syntax_error() {
    let err = tokenize("1.2.3", "").unwrap_err();
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
        let err = tokenize(&bad.to_string(), "").unwrap_err();
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
    let err = tokenize("1 2 'abc", "").unwrap_err();
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
            num(1),
            TokenKind::Semicolon,
            num(2),
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
    assert_eq!(tokenize(")OFF", "").unwrap()[0].pos, 0);
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
    assert_eq!(tokenize("1X", "").unwrap_err().kind, ErrorKind::Syntax);
}

#[test]
fn unbalanced_brackets_are_syntax_errors_at_the_offender() {
    for (line, caret) in [
        ("(1+2", 0),
        ("1+2)", 3),
        ("A[1;2", 1),
        ("A[1)]", 3),
        ("(A[1)]", 4),
    ] {
        let err = tokenize(line, "").unwrap_err();
        assert_eq!(err.kind, ErrorKind::Syntax, "{line}");
        assert_eq!(err.caret, Some(caret), "{line}");
    }
}

#[test]
fn an_underscored_letter_is_a_letter_in_a_name() {
    // A̲ through Z̲ are characters of the APL\360 set, each a letter
    // and a combining low line here, and each valid in a name.
    assert_eq!(
        kinds("X\u{332} A\u{332}B1 \u{2206}C\u{332}"),
        vec![
            TokenKind::Name("X\u{332}".into()),
            TokenKind::Name("A\u{332}B1".into()),
            TokenKind::Name("\u{2206}C\u{332}".into()),
        ]
    );
}

#[test]
fn a_letter_and_the_same_letter_underscored_are_two_names() {
    // The manual's point: they were distinct characters, not
    // decoration, so X and X̲ name two different things.
    let two = kinds("X X\u{332}");
    assert_ne!(two[0], two[1]);
    assert_eq!(two[1], TokenKind::Name("X\u{332}".into()));
}

#[test]
fn a_low_line_with_no_letter_before_it_is_a_character_error() {
    // It underscores a letter; it is not a name start and not a
    // glyph of its own.
    for (text, caret) in [("\u{332}", 0), ("1\u{332}", 1), ("\u{2206}\u{332}", 1)] {
        let err = tokenize(text, "").unwrap_err();
        assert_eq!(err.kind, ErrorKind::Character('\u{332}'), "{text:?}");
        assert_eq!(err.caret, Some(caret), "{text:?}");
    }
}
