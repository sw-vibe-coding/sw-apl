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
        "CHARACTER ERROR: U+03C1"
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
