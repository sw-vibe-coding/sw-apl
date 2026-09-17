//! Evaluator over the AST with a workspace of variables.

use apl_eval::{Output, Workspace, eval_line};
use apl_value::{Data, ErrorKind, Number};

fn nums(ws: &mut Workspace, line: &str) -> Vec<Number> {
    match eval_line(ws, line).unwrap().value().unwrap().data {
        Data::Num(v) => v,
        Data::Char(_) => panic!("chars"),
    }
}

#[test]
fn arithmetic_right_to_left() {
    let mut ws = Workspace::default();
    assert_eq!(nums(&mut ws, "2+3\u{d7}4"), [Number::Int(14)]);
    assert_eq!(nums(&mut ws, "(2+3)\u{d7}4"), [Number::Int(20)]);
    assert_eq!(nums(&mut ws, "\u{af}3+10"), [Number::Int(7)]);
    assert_eq!(nums(&mut ws, "-3+10"), [Number::Int(-13)]);
}

#[test]
fn assignment_is_silent_and_variables_persist() {
    let mut ws = Workspace::default();
    assert_eq!(eval_line(&mut ws, "A\u{2190}5").unwrap(), Output::Nothing);
    assert_eq!(nums(&mut ws, "A+3"), [Number::Int(8)]);
    assert_eq!(
        eval_line(&mut ws, "B\u{2190}A\u{d7}2").unwrap(),
        Output::Nothing
    );
    assert_eq!(nums(&mut ws, "A+B"), [Number::Int(15)]);
}

#[test]
fn assignment_inside_an_expression_yields_its_value() {
    let mut ws = Workspace::default();
    assert_eq!(nums(&mut ws, "1+A\u{2190}5"), [Number::Int(6)]);
}

#[test]
fn value_error_carries_the_name_position() {
    let mut ws = Workspace::default();
    let e = eval_line(&mut ws, "1+XYZ").unwrap_err();
    assert_eq!(e.kind, ErrorKind::Value);
    assert_eq!(e.caret, Some(2));
}

#[test]
fn primitive_errors_point_at_the_glyph() {
    let mut ws = Workspace::default();
    let e = eval_line(&mut ws, "1 2+3 4 5").unwrap_err();
    assert_eq!(e.kind, ErrorKind::Length);
    assert_eq!(e.caret, Some(3));
    let e = eval_line(&mut ws, "5\u{f7}0").unwrap_err();
    assert_eq!(e.kind, ErrorKind::Domain);
    assert_eq!(e.caret, Some(1));
}

#[test]
fn monadic_rho_gives_the_shape() {
    let mut ws = Workspace::default();
    assert_eq!(nums(&mut ws, "\u{2374}1 2 3 4 5"), [Number::Int(5)]);
    let a = eval_line(&mut ws, "\u{2374}7").unwrap().value().unwrap();
    assert_eq!(a.shape, vec![0]);
}

#[test]
fn empty_line_evaluates_to_nothing() {
    let mut ws = Workspace::default();
    assert_eq!(eval_line(&mut ws, "").unwrap(), Output::Nothing);
}

#[test]
fn iota_rho_reduce_end_to_end() {
    let mut ws = Workspace::default();
    assert_eq!(nums(&mut ws, "+/\u{2373}10"), [Number::Int(55)]);
    let m = eval_line(&mut ws, "2 3\u{2374}\u{2373}6")
        .unwrap()
        .value()
        .unwrap();
    assert_eq!(m.shape, vec![2, 3]);
    assert_eq!(nums(&mut ws, ",2 2\u{2374}7"), [Number::Int(7); 4]);
    assert_eq!(
        nums(&mut ws, "1 2,3"),
        [Number::Int(1), Number::Int(2), Number::Int(3)]
    );
}

#[test]
fn the_index_origin_is_workspace_state_not_a_quad_name() {
    let mut ws = Workspace::default();
    ws.env.io = 0;
    assert_eq!(
        nums(&mut ws, "\u{2373}3"),
        [Number::Int(0), Number::Int(1), Number::Int(2)]
    );
    let e = eval_line(&mut ws, "\u{2395}IO\u{2190}1").unwrap_err();
    assert_eq!(e.kind, ErrorKind::Syntax);
}

#[test]
fn quad_output_is_buffered_and_silent_at_top_level() {
    let mut ws = Workspace::default();
    assert_eq!(
        eval_line(&mut ws, "\u{2395}\u{2190}1 2").unwrap(),
        Output::Nothing
    );
    assert_eq!(ws.output.len(), 1);
    ws.output.clear();
    assert_eq!(nums(&mut ws, "1+\u{2395}\u{2190}5"), [Number::Int(6)]);
    assert_eq!(ws.output.len(), 1);
    assert_eq!(
        eval_line(&mut ws, "\u{2395}").unwrap_err().kind,
        ErrorKind::NotImplemented
    );
}

#[test]
fn mixed_output_yields_every_part() {
    let mut ws = Workspace::default();
    let Output::Mixed(parts) = eval_line(&mut ws, "'X IS ';2+3").unwrap() else {
        panic!()
    };
    assert_eq!(parts.len(), 2);
    assert_eq!(parts[1], apl_value::Array::scalar(Number::Int(5)));
}

#[test]
fn indexing_branch_and_derived_forms_are_not_implemented_yet() {
    let mut ws = Workspace::default();
    ws.set("A", apl_value::Array::vector(vec![Number::Int(1)]));
    for line in [
        "A[1]",
        "A[1]\u{2190}2",
        "\u{2192}3",
        "A+.\u{d7}A",
        "A\u{2218}.\u{d7}A",
    ] {
        let e = eval_line(&mut ws, line).unwrap_err();
        assert_eq!(e.kind, ErrorKind::NotImplemented, "{line}");
        assert!(e.caret.is_some(), "{line}");
    }
}
