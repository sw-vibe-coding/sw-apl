//! Right-to-left parsing with long right scope.

use apl_parse::{Expr, parse};
use apl_value::{ErrorKind, Number};

fn n(v: i64) -> Number {
    Number::Int(v)
}

#[test]
fn empty_line_parses_to_nothing() {
    assert_eq!(parse("").unwrap(), None);
    assert_eq!(parse("  \u{235d} c").unwrap(), None);
}

#[test]
fn strand_becomes_one_literal_vector() {
    let e = parse("1 2 3").unwrap().unwrap();
    let Expr::Literal(a) = e else { panic!("{e:?}") };
    assert_eq!(a.shape, vec![3]);
}

#[test]
fn single_number_is_a_scalar_literal() {
    let e = parse("7").unwrap().unwrap();
    let Expr::Literal(a) = e else { panic!("{e:?}") };
    assert_eq!(a.shape, Vec::<usize>::new());
}

#[test]
fn right_to_left_long_right_scope() {
    // 2+3×4  is  2 + (3 × 4)
    let e = parse("2+3\u{d7}4").unwrap().unwrap();
    let Expr::Dyadic {
        f: '+',
        left,
        right,
        ..
    } = e
    else {
        panic!("{e:?}")
    };
    assert!(matches!(*left, Expr::Literal(_)));
    assert!(matches!(*right, Expr::Dyadic { f: '\u{d7}', .. }));
}

#[test]
fn parentheses_group_and_monadic_at_line_start() {
    // -(2+3)×4  is  -( (2+3) × 4 )
    let e = parse("-(2+3)\u{d7}4").unwrap().unwrap();
    let Expr::Monadic { f: '-', right, pos } = e else {
        panic!("{e:?}")
    };
    assert_eq!(pos, 0);
    let Expr::Dyadic {
        f: '\u{d7}', left, ..
    } = *right
    else {
        panic!()
    };
    assert!(matches!(*left, Expr::Dyadic { f: '+', .. }));
}

#[test]
fn assignment_binds_a_name_to_the_whole_right() {
    let e = parse("A\u{2190}1+2").unwrap().unwrap();
    let Expr::Assign { name, value, .. } = e else {
        panic!("{e:?}")
    };
    assert_eq!(name, "A");
    assert!(matches!(*value, Expr::Dyadic { f: '+', .. }));
}

#[test]
fn dyadic_left_argument_is_a_single_array() {
    // 1 2+3 : left is the strand 1 2
    let e = parse("1 2+3").unwrap().unwrap();
    let Expr::Dyadic { left, .. } = e else {
        panic!("{e:?}")
    };
    let Expr::Literal(a) = *left else { panic!() };
    assert_eq!(a.shape, vec![2]);
    assert!(matches!(a.data, apl_value::Data::Num(ref v) if v == &[n(1), n(2)]));
}

#[test]
fn names_carry_positions_for_the_caret() {
    let e = parse("1+ABC").unwrap().unwrap();
    let Expr::Dyadic { right, .. } = e else {
        panic!()
    };
    assert!(matches!(*right, Expr::Name(ref s, 2) if s == "ABC"));
}

#[test]
fn syntax_errors_carry_carets() {
    let e = parse("(1+2").unwrap_err();
    assert_eq!(e.kind, ErrorKind::Syntax);
    assert_eq!(e.caret, Some(0));
    let e = parse("1+").unwrap_err();
    assert_eq!(e.kind, ErrorKind::Syntax);
    assert_eq!(e.caret, Some(1));
    let e = parse("A 5").unwrap_err();
    assert_eq!(e.kind, ErrorKind::Syntax);
    let e = parse("3\u{2190}4").unwrap_err();
    assert_eq!(e.kind, ErrorKind::Syntax);
}

#[test]
fn reduce_is_a_derived_monadic_function() {
    let e = parse("+/1 2 3").unwrap().unwrap();
    let Expr::Reduce {
        f: '+',
        pos: 0,
        right,
    } = e
    else {
        panic!("{e:?}")
    };
    assert!(matches!(*right, Expr::Literal(_)));
    // 2×+/1 2 3 : reduce binds first, then × is dyadic
    let e = parse("2\u{d7}+/1 2 3").unwrap().unwrap();
    let Expr::Dyadic {
        f: '\u{d7}', right, ..
    } = e
    else {
        panic!("{e:?}")
    };
    assert!(matches!(*right, Expr::Reduce { f: '+', .. }));
}

#[test]
fn slash_with_a_left_operand_is_dyadic_compress() {
    let e = parse("1 0 1/1 2 3").unwrap().unwrap();
    assert!(matches!(e, Expr::Dyadic { f: '/', .. }));
}

#[test]
fn quad_output_and_quad_input() {
    let e = parse("\u{2395}\u{2190}1+2").unwrap().unwrap();
    let Expr::QuadOut { value, .. } = e else {
        panic!("{e:?}")
    };
    assert!(matches!(*value, Expr::Dyadic { .. }));
    assert!(matches!(
        parse("\u{2395}").unwrap().unwrap(),
        Expr::QuadIn(0)
    ));
    let e = parse("1+\u{2395}").unwrap().unwrap();
    let Expr::Dyadic { right, .. } = e else {
        panic!()
    };
    assert!(matches!(*right, Expr::QuadIn(2)));
}

#[test]
fn quad_names_are_not_a_thing_in_apl360() {
    let e = parse("\u{2395}IO\u{2190}0").unwrap_err();
    assert_eq!(e.kind, ErrorKind::Syntax);
    assert_eq!(parse("\u{2395}IO").unwrap_err().kind, ErrorKind::Syntax);
}
