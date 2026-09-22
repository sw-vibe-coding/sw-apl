//! `⎕CC`: the answers the 5110 gives, with nothing done.

use apl_console_control::{control, national};
use apl_value::{Array, Data, ErrorKind, Number};

fn chars(s: &str) -> Array {
    let c: Vec<char> = s.chars().collect();
    Array {
        shape: vec![c.len()],
        data: Data::Char(c),
    }
}

fn ints(v: &[i64]) -> Array {
    Array::vector(v.iter().map(|n| Number::Int(*n)).collect())
}

fn answer(a: &Array) -> i64 {
    match a.data {
        Data::Num(ref n) => match n[0] {
            Number::Int(i) => i,
            Number::Float(_) => panic!("{a:?}"),
        },
        Data::Char(_) => panic!("{a:?}"),
    }
}

#[test]
fn a_national_character_set_is_one_character_of_the_list() {
    for c in [".", "0", "9", "÷", "×", "-", "+"] {
        assert_eq!(answer(&national(&chars(c)).unwrap()), 1, "{c}");
    }
    for c in ["Q", "", "01"] {
        assert_eq!(answer(&national(&chars(c)).unwrap()), 0, "{c}");
    }
    assert_eq!(national(&ints(&[1])).unwrap_err().kind, ErrorKind::Domain);
}

#[test]
fn each_operation_takes_its_own_values() {
    let cc = |op: i64, v: &[i64]| answer(&control(&ints(&[op]), &ints(v)).unwrap());
    assert_eq!(cc(1, &[0, 1]), 1);
    assert_eq!(cc(1, &[2]), 0);
    assert_eq!(cc(2, &[2, 2, 2, 2]), 1);
    assert_eq!(cc(3, &[1]), 1);
    assert_eq!(cc(4, &[-10, 5]), 1);
    assert_eq!(cc(4, &[17]), 0);
    assert_eq!(cc(5, &[131]), 1);
    assert_eq!(cc(5, &[0]), 0);
    assert_eq!(cc(6, &[0]), 0);
    let e = control(&ints(&[1]), &chars("A")).unwrap_err();
    assert_eq!(e.kind, ErrorKind::Domain);
}
