//! An array written as the APL that produces it.

use apl_value::{Array, Data, Number};
use apl_wsfile::literal;

fn ints(v: &[i64]) -> Vec<Number> {
    v.iter().map(|i| Number::Int(*i)).collect()
}

fn chars(s: &str) -> Data {
    Data::Char(s.chars().collect())
}

#[test]
fn a_scalar_is_written_as_itself() {
    assert_eq!(literal(&Array::scalar(Number::Int(5))), "5");
    assert_eq!(literal(&Array::scalar(Number::Int(-5))), "¯5");
    assert_eq!(literal(&Array::scalar(Number::Float(2.5))), "2.5");
    let ch = Array::new(Vec::new(), chars("A")).unwrap();
    assert_eq!(literal(&ch), "'A'");
}

#[test]
fn a_vector_of_more_than_one_is_written_as_its_elements() {
    assert_eq!(literal(&Array::vector(ints(&[1, 2, 3]))), "1 2 3");
    assert_eq!(literal(&Array::vector(ints(&[-1, 2])),), "¯1 2");
    let text = Array::new(vec![3], chars("ABC")).unwrap();
    assert_eq!(literal(&text), "'ABC'");
}

#[test]
fn a_vector_of_one_is_reshaped_so_it_stays_a_vector() {
    // `5` and `'A'` are scalars in APL, so a one-element vector has
    // to say so or it comes back the wrong rank.
    assert_eq!(literal(&Array::vector(ints(&[5]))), "1⍴5");
    let one = Array::new(vec![1], chars("A")).unwrap();
    assert_eq!(literal(&one), "1⍴'A'");
}

#[test]
fn rank_and_shape_survive() {
    let m = Array::new(vec![2, 3], Data::Num(ints(&[1, 2, 3, 4, 5, 6]))).unwrap();
    assert_eq!(literal(&m), "2 3⍴1 2 3 4 5 6");
    let cube = Array::new(vec![2, 1, 2], Data::Num(ints(&[1, 2, 3, 4]))).unwrap();
    assert_eq!(literal(&cube), "2 1 2⍴1 2 3 4");
    let text = Array::new(vec![2, 2], chars("ABCD")).unwrap();
    assert_eq!(literal(&text), "2 2⍴'ABCD'");
}

#[test]
fn an_empty_array_still_has_something_to_reshape() {
    assert_eq!(literal(&Array::vector(Vec::new())), "0⍴0");
    let none = Array::new(vec![0, 3], Data::Num(Vec::new())).unwrap();
    assert_eq!(literal(&none), "0 3⍴0");
    let text = Array::new(vec![0], chars("")).unwrap();
    assert_eq!(literal(&text), "0⍴''");
}

#[test]
fn a_quote_inside_a_literal_is_doubled() {
    let text = Array::new(vec![4], chars("IT'S")).unwrap();
    assert_eq!(literal(&text), "'IT''S'");
}

#[test]
fn a_float_is_written_to_as_many_figures_as_it_takes() {
    let third = Array::scalar(Number::Float(1.0 / 3.0));
    assert_eq!(literal(&third), "0.3333333333333333");
    // The high minus and the capital E are APL's spelling.
    assert_eq!(literal(&Array::scalar(Number::Float(-1e-10))), "¯1E¯10");
}
