//! The accounting: a known value has a known size. These numbers are
//! deliberately spelled out rather than computed from the constants,
//! because the point of the accounting is that it is stable -- a test
//! that recomputes it agrees with any change, including a mistake.

use std::collections::HashMap;
use std::rc::Rc;

use apl_ast::Defn;
use apl_space::{DEFAULT, Funcs, Groups, Vars, free, of_function, of_name, of_value, room, used};
use apl_value::{Array, Data, ErrorKind, Number};

fn ints(v: &[i64]) -> Array {
    Array::vector(v.iter().map(|i| Number::Int(*i)).collect())
}

#[test]
fn a_value_costs_its_descriptor_its_axes_and_its_elements() {
    // Scalar: descriptor only, plus the one number.
    assert_eq!(of_value(&Array::scalar(Number::Int(1))), 16 + 8);
    // Vector of three: one axis, three numbers.
    assert_eq!(of_value(&ints(&[1, 2, 3])), 16 + 4 + 24);
    // A float costs the same as an integer, so the accounting does
    // not move when 4/2 comes back as an Int.
    assert_eq!(
        of_value(&Array::scalar(Number::Float(0.5))),
        of_value(&Array::scalar(Number::Int(1)))
    );
    // Characters are a byte each, as they were on a 360, whatever
    // they take in UTF-8.
    let chars = Array::new(vec![3], Data::Char("AB\u{2373}".chars().collect())).unwrap();
    assert_eq!(of_value(&chars), 16 + 4 + 3);
    // A matrix has two axes.
    let m = Array::new(vec![2, 3], Data::Num(vec![Number::Int(0); 6])).unwrap();
    assert_eq!(of_value(&m), 16 + 8 + 48);
}

#[test]
fn an_empty_array_still_costs_its_descriptor() {
    let empty = Array::new(vec![0], Data::Num(Vec::new())).unwrap();
    assert_eq!(of_value(&empty), 16 + 4);
}

#[test]
fn a_name_costs_its_entry_and_its_characters() {
    assert_eq!(of_name("A"), 8 + 1);
    assert_eq!(of_name("HYP"), 8 + 3);
}

#[test]
fn a_function_costs_its_header_names_and_its_lines() {
    let defn = Defn {
        name: "HYP".to_string(),
        result: Some("R".to_string()),
        left: Some("A".to_string()),
        right: Some("B".to_string()),
        locals: Vec::new(),
        body: vec!["R\u{2190}A+B".to_string()],
        locked: false,
    };
    // Descriptor, three one-character names, and one body line of
    // five characters with its own per-line cost.
    assert_eq!(of_function(&defn), 16 + 3 * (8 + 1) + (4 + 5));
}

#[test]
fn the_symbol_table_is_every_name_and_what_it_holds() {
    let mut vars: Vars = HashMap::new();
    vars.insert("A".to_string(), ints(&[1, 2, 3]));
    let mut funcs: Funcs = HashMap::new();
    funcs.insert("F".to_string(), Rc::new(Defn::default()));
    let want = (of_name("A") + of_value(&ints(&[1, 2, 3])))
        + (of_name("F") + of_function(&Defn::default()));
    assert_eq!(used(&vars, &funcs, &Groups::new()), want);
}

#[test]
fn a_group_costs_its_own_name_and_the_names_it_lists() {
    let mut groups: Groups = HashMap::new();
    groups.insert("G".to_string(), vec!["A".to_string(), "LONGER".to_string()]);
    let want = of_name("G") + of_name("A") + of_name("LONGER");
    assert_eq!(used(&Vars::new(), &Funcs::new(), &groups), want);
    // A member that holds nothing still costs the entry recording
    // it, which is why a group is charged for names and not for
    // whatever they refer to.
    assert_eq!(want, 9 + 9 + 14);
}

#[test]
fn free_is_the_quota_less_what_is_held() {
    assert_eq!(free(1000, 0), 1000);
    assert_eq!(free(1000, 400), 600);
    // A quota lowered below what is already held reports nothing
    // free rather than a negative figure.
    assert_eq!(free(100, 400), 0);
}

#[test]
fn room_refuses_what_will_not_fit_and_counts_what_is_given_back() {
    // 100 held, quota 200: 50 more fits, 150 more does not.
    assert!(room(200, 100, 50, 0).is_ok());
    assert_eq!(
        room(200, 100, 150, 0).unwrap_err().kind,
        ErrorKind::WsFull,
        "WS FULL, not some other error"
    );
    // Replacing a 150-byte name with a 150-byte one fits, because
    // the old value is given back first.
    assert!(room(200, 160, 150, 150).is_ok());
    // Exactly filling the quota is allowed; one byte more is not.
    assert!(room(200, 100, 100, 0).is_ok());
    assert!(room(200, 100, 101, 0).is_err());
}

#[test]
fn the_default_quota_is_a_megabyte() {
    assert_eq!(DEFAULT, 1_048_576);
}
