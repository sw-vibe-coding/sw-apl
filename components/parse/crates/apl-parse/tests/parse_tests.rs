//! Right-to-left parsing with long right scope, derived functions,
//! indexing, assignment forms, branch, and mixed output.

use apl_ast::{Expr, Function};
use apl_parse::parse;
use apl_value::{Data, ErrorKind, Number};

fn one(line: &str) -> Expr {
    parse(line)
        .unwrap()
        .unwrap_or_else(|| panic!("nothing parsed for {line}"))
}
fn syntax_at(line: &str, caret: usize) {
    let e = parse(line).unwrap_err();
    assert_eq!(e.kind, ErrorKind::Syntax, "{line}");
    assert_eq!(e.caret, Some(caret), "{line}");
}
fn lit_ints(e: &Expr) -> Vec<i64> {
    let Expr::Literal(a) = e else { panic!("{e:?}") };
    let Data::Num(v) = &a.data else {
        panic!("{e:?}")
    };
    v.iter()
        .map(|n| match n {
            Number::Int(i) => *i,
            Number::Float(_) => panic!("float"),
        })
        .collect()
}

#[test]
fn empty_line_parses_to_nothing() {
    assert_eq!(parse("").unwrap(), None);
    assert_eq!(parse("  \u{235d} c").unwrap(), None);
}

#[test]
fn strands_scalars_and_character_literals() {
    let Expr::Literal(a) = one("1 2 3") else {
        panic!()
    };
    assert_eq!(a.shape, vec![3]);
    let Expr::Literal(a) = one("7") else { panic!() };
    assert_eq!(a.shape, Vec::<usize>::new());
    let Expr::Literal(a) = one("'AB'") else {
        panic!()
    };
    assert_eq!(a.shape, vec![2]);
    assert!(matches!(a.data, Data::Char(_)));
    syntax_at("1 'A' 2", 2);
}

#[test]
fn right_to_left_long_right_scope_and_parentheses() {
    let Expr::Dyadic {
        func, left, right, ..
    } = one("2+3\u{d7}4")
    else {
        panic!()
    };
    assert_eq!(func, Function::Prim('+'));
    assert_eq!(lit_ints(&left), [2]);
    assert!(matches!(
        *right,
        Expr::Dyadic {
            func: Function::Prim('\u{d7}'),
            ..
        }
    ));
    let Expr::Monadic {
        func: Function::Prim('-'),
        pos: 0,
        right,
        ..
    } = one("-(2+3)\u{d7}4")
    else {
        panic!()
    };
    let Expr::Dyadic { left, .. } = *right else {
        panic!()
    };
    assert!(matches!(
        *left,
        Expr::Dyadic {
            func: Function::Prim('+'),
            ..
        }
    ));
    let Expr::Dyadic { left, .. } = one("1 2+3") else {
        panic!()
    };
    assert_eq!(lit_ints(&left), [1, 2]);
}

#[test]
fn names_carry_positions_and_assignment_chains() {
    let Expr::Dyadic { right, .. } = one("1+ABC") else {
        panic!()
    };
    assert!(matches!(*right, Expr::Name(ref s, 2) if s == "ABC"));
    let Expr::Assign { name, value, .. } = one("A\u{2190}B\u{2190}1+2") else {
        panic!()
    };
    assert_eq!(name, "A");
    let Expr::Assign { name, value, .. } = *value else {
        panic!()
    };
    assert_eq!(name, "B");
    assert!(matches!(*value, Expr::Dyadic { .. }));
}

#[test]
fn reduce_and_scan_on_either_axis() {
    let Expr::Monadic {
        func,
        pos,
        axis,
        right,
    } = one("+/1 2 3")
    else {
        panic!()
    };
    assert_eq!(
        func,
        Function::Reduce {
            f: '+',
            first: false
        }
    );
    assert_eq!((pos, axis), (0, None));
    assert_eq!(lit_ints(&right), [1, 2, 3]);
    assert!(matches!(
        one("+\u{233f}M"),
        Expr::Monadic {
            func: Function::Reduce {
                f: '+',
                first: true
            },
            ..
        }
    ));
    assert!(matches!(
        one("\u{d7}\\M"),
        Expr::Monadic {
            func: Function::Scan {
                f: '\u{d7}',
                first: false
            },
            ..
        }
    ));
    assert!(matches!(
        one("+\u{2340}M"),
        Expr::Monadic {
            func: Function::Scan {
                f: '+',
                first: true
            },
            ..
        }
    ));
    // 2×+/1 2 3 : reduce binds first, then × is dyadic
    let Expr::Dyadic {
        func: Function::Prim('\u{d7}'),
        right,
        ..
    } = one("2\u{d7}+/1 2 3")
    else {
        panic!()
    };
    assert!(matches!(
        *right,
        Expr::Monadic {
            func: Function::Reduce { .. },
            ..
        }
    ));
}

#[test]
fn slash_with_an_operand_on_the_left_is_compress_and_reduce_takes_no_left() {
    assert!(matches!(
        one("1 0 1/1 2 3"),
        Expr::Dyadic {
            func: Function::Prim('/'),
            ..
        }
    ));
    assert!(matches!(
        one("A\u{233f}M"),
        Expr::Dyadic {
            func: Function::Prim('\u{233f}'),
            ..
        }
    ));
    assert!(matches!(
        one("1 0\\1 2"),
        Expr::Dyadic {
            func: Function::Prim('\\'),
            ..
        }
    ));
    syntax_at("2+/1 2 3", 1);
}

#[test]
fn inner_and_outer_products() {
    let Expr::Dyadic { func, pos, .. } = one("A+.\u{d7}B") else {
        panic!()
    };
    assert_eq!(
        func,
        Function::Inner {
            f: '+',
            g: '\u{d7}'
        }
    );
    assert_eq!(pos, 1);
    let Expr::Dyadic { func, .. } = one("1 2\u{2218}.\u{d7}3 4") else {
        panic!()
    };
    assert_eq!(func, Function::Outer { f: '\u{d7}' });
    syntax_at("+.\u{d7}B", 0);
    syntax_at("\u{2218}.\u{d7}B", 0);
}

#[test]
fn axis_brackets_follow_the_function() {
    let Expr::Monadic { func, axis, .. } = one("+/[1]M") else {
        panic!()
    };
    assert_eq!(
        func,
        Function::Reduce {
            f: '+',
            first: false
        }
    );
    assert_eq!(lit_ints(axis.as_deref().unwrap()), [1]);
    let Expr::Monadic {
        func: Function::Prim('\u{233d}'),
        axis,
        ..
    } = one("\u{233d}[1]M")
    else {
        panic!()
    };
    assert!(axis.is_some());
    let Expr::Dyadic {
        func: Function::Prim(','),
        axis,
        left,
        ..
    } = one("A,[1]B")
    else {
        panic!()
    };
    assert!(axis.is_some());
    assert!(matches!(*left, Expr::Name(ref n, 0) if n == "A"));
    syntax_at("[1]M", 0);
}

#[test]
fn bracket_indexing_with_semicolons_and_elided_axes() {
    let Expr::Index {
        array,
        pos,
        indexes,
    } = one("A[1]")
    else {
        panic!()
    };
    assert!(matches!(*array, Expr::Name(ref n, 0) if n == "A"));
    assert_eq!(pos, 1);
    assert_eq!(indexes.len(), 1);
    let Expr::Index { indexes, .. } = one("M[1 2;]") else {
        panic!()
    };
    assert_eq!(indexes.len(), 2);
    assert!(indexes[0].is_some() && indexes[1].is_none());
    let Expr::Index { indexes, .. } = one("M[;2]") else {
        panic!()
    };
    assert!(indexes[0].is_none() && indexes[1].is_some());
    let Expr::Index { indexes, .. } = one("A[]") else {
        panic!()
    };
    assert_eq!(indexes, vec![None]);
    let Expr::Index { array, .. } = one("A[B[1]][2]") else {
        panic!()
    };
    assert!(matches!(*array, Expr::Index { .. }));
    assert!(matches!(one("(\u{2373}5)[2]"), Expr::Index { .. }));
    assert!(matches!(one("'ABC'[2]"), Expr::Index { .. }));
    let Expr::Dyadic { left, .. } = one("A[1;2]+3") else {
        panic!()
    };
    assert!(matches!(*left, Expr::Index { .. }));
    syntax_at("A[1;2", 1);
    syntax_at("[1]", 0);
}

#[test]
fn indexed_assignment() {
    let Expr::IndexedAssign {
        name,
        indexes,
        value,
        ..
    } = one("A[2]\u{2190}9")
    else {
        panic!()
    };
    assert_eq!(name, "A");
    assert_eq!(indexes.len(), 1);
    assert_eq!(lit_ints(&value), [9]);
    let Expr::IndexedAssign { indexes, .. } = one("M[1;]\u{2190}0") else {
        panic!()
    };
    assert_eq!(indexes.len(), 2);
    syntax_at("(A)[1]\u{2190}9", 6);
    syntax_at("3\u{2190}4", 1);
}

#[test]
fn branch_only_at_the_start_of_a_statement() {
    let Expr::Branch { pos: 0, target } = one("\u{2192}3") else {
        panic!()
    };
    assert_eq!(lit_ints(target.as_deref().unwrap()), [3]);
    let Expr::Branch { target, .. } = one("\u{2192}L\u{d7}\u{2373}COND") else {
        panic!()
    };
    assert!(matches!(target.as_deref(), Some(Expr::Dyadic { .. })));
    assert!(matches!(
        one("\u{2192}"),
        Expr::Branch {
            pos: 0,
            target: None
        }
    ));
    syntax_at("1+\u{2192}2", 2);
}

#[test]
fn quad_forms_on_both_sides() {
    assert!(matches!(one("\u{2395}\u{2190}1+2"), Expr::QuadOut { .. }));
    assert!(matches!(
        one("\u{235e}\u{2190}'X'"),
        Expr::QuoteQuadOut { .. }
    ));
    assert!(matches!(one("\u{2395}"), Expr::QuadIn(0)));
    let Expr::Dyadic { right, .. } = one("1+\u{235e}") else {
        panic!()
    };
    assert!(matches!(*right, Expr::QuoteQuadIn(2)));
    syntax_at("\u{2395}IO\u{2190}0", 0);
}

#[test]
fn mixed_output_splits_on_top_level_semicolons_only() {
    let Expr::Mixed(parts) = one("'X IS ';X") else {
        panic!()
    };
    assert_eq!(parts.len(), 2);
    let Expr::Mixed(parts) = one("'A';1 2;A[1;2];'B'") else {
        panic!()
    };
    assert_eq!(parts.len(), 4);
    assert!(matches!(parts[2], Expr::Index { .. }));
    syntax_at(";X", 0);
    syntax_at("X;", 1);
}

#[test]
fn syntax_error_carets() {
    syntax_at("(1+2", 0);
    syntax_at("1+", 1);
    syntax_at("A 5", 0);
    syntax_at("+/", 1);
    syntax_at("L:1", 1);
    syntax_at("\u{2207}F", 0);
    syntax_at("1 2 3 4+", 7);
}
