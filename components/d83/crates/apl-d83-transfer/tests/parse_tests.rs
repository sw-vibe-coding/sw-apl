//! Transfer forms read back: where one ends, and what is not one.

use apl_d83_transfer::{Object, parse};
use apl_value::{Array, Data, Number};

fn chars(s: &str) -> Vec<char> {
    s.chars().collect()
}

#[test]
fn a_numeric_matrix() {
    let (object, used) = parse(&chars("NTHIS 2 2 3 1 2 3 4 5 6")).unwrap();
    assert_eq!(used, 23);
    let Object::Value(name, v) = object else {
        panic!("a value")
    };
    assert_eq!(name, "THIS");
    assert_eq!(v.shape, [2, 3]);
    assert_eq!(v.data, Data::Num((1..=6).map(Number::Int).collect()));
}

#[test]
fn a_character_value_ends_where_its_shape_says() {
    let text = chars("CX 1 3 A\nB\nNY 0 5");
    let (first, used) = parse(&text).unwrap();
    let three = Array {
        shape: vec![3],
        data: Data::Char(chars("A\nB")),
    };
    assert_eq!(first, Object::Value("X".into(), three));
    assert_eq!(text[used], '\n');
    let (second, _) = parse(&text[used + 1..]).unwrap();
    assert_eq!(second.name(), "Y");
}

#[test]
fn numbers_end_at_a_line_ending() {
    let text = chars("NV 1 2 ¯1.5 2E3\nmore");
    let (object, used) = parse(&text).unwrap();
    assert_eq!(used, 15);
    let Object::Value(_, v) = object else {
        panic!("a value")
    };
    assert_eq!(
        v.data,
        Data::Num(vec![Number::Float(-1.5), Number::Int(2000)])
    );
}

#[test]
fn what_is_not_a_form() {
    for bad in [
        "",
        "N",
        "NX",
        "NX 1 3 1 2",
        "QX 0 1",
        "FX 1 1 A",
        "NX 0 -1",
        "CX 1 5 AB",
    ] {
        assert!(parse(&chars(bad)).is_none(), "{bad}");
    }
}
