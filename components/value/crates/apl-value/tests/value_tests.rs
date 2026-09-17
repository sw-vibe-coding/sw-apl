//! Value model: numbers promote and demote, arrays know their shape,
//! errors carry a kind and an optional caret.

use apl_value::{AplError, Array, Data, ErrorKind, Number};

#[test]
fn from_f64_demotes_exact_integers() {
    assert_eq!(Number::from_f64(3.0), Number::Int(3));
    assert_eq!(Number::from_f64(-0.0), Number::Int(0));
    assert_eq!(Number::from_f64(2.5), Number::Float(2.5));
}

#[test]
fn from_f64_keeps_huge_values_as_float() {
    assert_eq!(Number::from_f64(1e20), Number::Float(1e20));
}

#[test]
fn as_f64_round_trips() {
    assert!((Number::Int(7).as_f64() - 7.0).abs() < f64::EPSILON);
    assert!((Number::Float(0.5).as_f64() - 0.5).abs() < f64::EPSILON);
}

#[test]
fn scalar_and_vector_constructors_set_shape() {
    let s = Array::scalar(Number::Int(1));
    assert_eq!(s.shape, Vec::<usize>::new());
    let v = Array::vector(vec![Number::Int(1), Number::Int(2)]);
    assert_eq!(v.shape, vec![2]);
    assert_eq!(v.data.count(), 2);
    assert!(matches!(v.data, Data::Num(_)));
}

#[test]
fn new_checks_shape_against_data_length() {
    let ok = Array::new(vec![2, 3], Data::Num(vec![Number::Int(0); 6]));
    assert!(ok.is_ok());
    let bad = Array::new(vec![2, 3], Data::Num(vec![Number::Int(0); 5]));
    assert_eq!(bad.unwrap_err().kind, ErrorKind::Length);
}

#[test]
fn error_kind_names_match_apl360() {
    assert_eq!(ErrorKind::Syntax.to_string(), "SYNTAX ERROR");
    assert_eq!(ErrorKind::Value.to_string(), "VALUE ERROR");
    assert_eq!(ErrorKind::Domain.to_string(), "DOMAIN ERROR");
    assert_eq!(ErrorKind::Rank.to_string(), "RANK ERROR");
    assert_eq!(ErrorKind::Length.to_string(), "LENGTH ERROR");
    assert_eq!(
        ErrorKind::Character('\u{3c1}').to_string(),
        "CHARACTER ERROR: U+03C1 (use \u{2374} U+2374)"
    );
    assert_eq!(ErrorKind::NotImplemented.to_string(), "NOT IMPLEMENTED");
}

#[test]
fn error_caret_is_optional_and_settable() {
    let e = AplError::new(ErrorKind::Domain);
    assert_eq!(e.caret, None);
    let e = e.at(4);
    assert_eq!(e.caret, Some(4));
    assert_eq!(e.at(9).caret, Some(4), "first caret wins");
}

#[test]
fn tolerant_equality_uses_relative_tolerance() {
    let ct = 1e-13;
    assert!(Number::Float(0.1 + 0.2).tolerant_eq(Number::Float(0.3), ct));
    assert!(Number::Int(3).tolerant_eq(Number::Float(3.0 + 1e-14), ct));
    assert!(!Number::Int(3).tolerant_eq(Number::Float(3.001), ct));
    assert!(
        !Number::Float(1e-20).tolerant_eq(Number::Int(0), ct),
        "zero has no relative slack"
    );
    assert!(Number::Float(1e20).tolerant_eq(Number::Float(1e20 + 1e6), ct));
    assert!(!Number::Int(1).tolerant_eq(Number::Int(2), 0.0));
}

#[test]
fn exact_integer_arithmetic_beyond_2_to_53() {
    let big = Number::Int(9_007_199_254_740_993); // 2^53 + 1
    assert_eq!(Number::exact_int('+', big, Number::Int(0)), Some(big));
    assert_eq!(
        Number::exact_int('-', big, Number::Int(1)),
        Some(Number::Int(9_007_199_254_740_992))
    );
    assert_eq!(
        Number::exact_int('\u{d7}', Number::Int(3), Number::Int(4)),
        Some(Number::Int(12))
    );
    assert_eq!(
        Number::exact_int('+', Number::Int(i64::MAX), Number::Int(1)),
        None,
        "overflow falls back"
    );
    assert_eq!(
        Number::exact_int('+', Number::Float(1.5), Number::Int(1)),
        None
    );
    assert_eq!(
        Number::exact_int('\u{f7}', Number::Int(4), Number::Int(2)),
        None,
        "divide is not exact-int"
    );
}

#[test]
fn every_error_kind_has_its_apl360_text() {
    let cases = [
        (ErrorKind::Index, "INDEX ERROR"),
        (ErrorKind::WsFull, "WS FULL"),
        (ErrorKind::Defn, "DEFN ERROR"),
        (ErrorKind::Depth, "DEPTH ERROR"),
        (ErrorKind::Interrupt, "INTERRUPT"),
    ];
    for (kind, text) in cases {
        assert_eq!(kind.to_string(), text);
    }
}

#[test]
fn arrays_of_any_rank_including_empties() {
    let cube = Array::new(vec![2, 3, 4], Data::Num(vec![Number::Int(0); 24])).unwrap();
    assert_eq!(cube.shape.len(), 3);
    for shape in [vec![0], vec![2, 0], vec![0, 3], vec![0, 0, 5]] {
        let a = Array::new(shape.clone(), Data::Num(vec![])).unwrap();
        assert_eq!(a.shape, shape);
        assert_eq!(a.data.count(), 0);
    }
    let chars = Array::new(vec![2], Data::Char(vec!['A', 'B'])).unwrap();
    assert_eq!(chars.data.count(), 2);
}

#[test]
fn character_error_names_the_intended_glyph_for_lookalikes() {
    assert_eq!(
        ErrorKind::Character('\u{3c1}').to_string(),
        "CHARACTER ERROR: U+03C1 (use \u{2374} U+2374)"
    );
    assert_eq!(
        ErrorKind::Character('\u{2212}').to_string(),
        "CHARACTER ERROR: U+2212 (use - U+002D)"
    );
    assert_eq!(
        ErrorKind::Character('#').to_string(),
        "CHARACTER ERROR: U+0023"
    );
}

#[test]
fn character_error_names_later_apl_glyphs() {
    assert_eq!(
        ErrorKind::Character('{').to_string(),
        "CHARACTER ERROR: U+007B (dfn brace, not APL\\360)"
    );
    assert_eq!(
        ErrorKind::Character('\u{a8}').to_string(),
        "CHARACTER ERROR: U+00A8 (each, not APL\\360)"
    );
    assert_eq!(
        ErrorKind::Character('\u{2282}').to_string(),
        "CHARACTER ERROR: U+2282 (enclose, not APL\\360)"
    );
}
