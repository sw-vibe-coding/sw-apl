//! Evaluator over the AST with a workspace of variables.

use apl_eval::{Output, Workspace, eval_line};
use apl_parse::parse_header;
use apl_value::{Array, Data, ErrorKind, Number};

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
fn branch_is_not_implemented_yet() {
    let mut ws = Workspace::default();
    let e = eval_line(&mut ws, "\u{2192}3").unwrap_err();
    assert_eq!(e.kind, ErrorKind::NotImplemented);
    assert_eq!(e.caret, Some(0));
}

#[test]
fn indexing_and_indexed_assignment_evaluate() {
    let mut ws = Workspace::default();
    assert_eq!(
        eval_line(&mut ws, "V\u{2190}10 20 30").unwrap(),
        Output::Nothing
    );
    assert_eq!(nums(&mut ws, "V[2]"), [Number::Int(20)]);
    assert_eq!(nums(&mut ws, "V[3 1]"), [Number::Int(30), Number::Int(10)]);
    assert_eq!(
        eval_line(&mut ws, "V[2]\u{2190}99").unwrap(),
        Output::Nothing
    );
    assert_eq!(
        nums(&mut ws, "V"),
        [Number::Int(10), Number::Int(99), Number::Int(30)]
    );
    assert_eq!(
        nums(&mut ws, "1+V[1]\u{2190}5"),
        [Number::Int(6)],
        "indexed assignment yields the value"
    );
    let e = eval_line(&mut ws, "V[4]").unwrap_err();
    assert_eq!((e.kind, e.caret), (ErrorKind::Index, Some(1)));
    let e = eval_line(&mut ws, "V[1;1]").unwrap_err();
    assert_eq!((e.kind, e.caret), (ErrorKind::Rank, Some(1)));
    let e = eval_line(&mut ws, "W[1]\u{2190}1").unwrap_err();
    assert_eq!((e.kind, e.caret), (ErrorKind::Value, Some(0)));
    assert_eq!(
        nums(&mut ws, "(2 2\u{2374}\u{2373}4)[2;1]"),
        [Number::Int(3)]
    );
}

/// Define a function from a del header and its body lines.
fn define(ws: &mut Workspace, header: &str, body: &[&str]) {
    let mut defn = parse_header(header).unwrap();
    defn.body = body.iter().map(|&l| l.to_string()).collect();
    ws.define(defn);
}

#[test]
fn a_defined_function_binds_its_arguments_and_yields_its_result() {
    let mut ws = Workspace::default();
    define(&mut ws, "R\u{2190}DOUBLE N", &["R\u{2190}N+N"]);
    define(&mut ws, "R\u{2190}A HYP B", &["R\u{2190}((A*2)+B*2)*0.5"]);
    define(&mut ws, "R\u{2190}TEN", &["R\u{2190}10"]);
    assert_eq!(nums(&mut ws, "DOUBLE 21"), [Number::Int(42)]);
    assert_eq!(nums(&mut ws, "3 HYP 4"), [Number::Int(5)]);
    assert_eq!(nums(&mut ws, "TEN"), [Number::Int(10)]);
    assert_eq!(nums(&mut ws, "1+TEN"), [Number::Int(11)]);
    assert_eq!(nums(&mut ws, "DOUBLE DOUBLE 1+2"), [Number::Int(12)]);
}

#[test]
fn locals_shadow_globals_and_are_restored() {
    let mut ws = Workspace::default();
    eval_line(&mut ws, "T\u{2190}99").unwrap();
    eval_line(&mut ws, "G\u{2190}7").unwrap();
    define(
        &mut ws,
        "R\u{2190}F N;T",
        &["T\u{2190}N\u{d7}2", "R\u{2190}T+G"],
    );
    assert_eq!(nums(&mut ws, "F 5"), [Number::Int(17)]);
    assert_eq!(nums(&mut ws, "T"), [Number::Int(99)], "T is restored");
    // A name the header localizes starts the call undefined.
    define(&mut ws, "R\u{2190}H;T", &["R\u{2190}T"]);
    assert_eq!(eval_line(&mut ws, "H").unwrap_err().kind, ErrorKind::Value);
    assert_eq!(nums(&mut ws, "T"), [Number::Int(99)]);
}

#[test]
fn a_function_with_no_result_displays_nothing_but_has_no_value() {
    let mut ws = Workspace::default();
    define(&mut ws, "SETUP", &["Z\u{2190}5"]);
    assert_eq!(eval_line(&mut ws, "SETUP").unwrap(), Output::Nothing);
    assert_eq!(nums(&mut ws, "Z"), [Number::Int(5)]);
    let e = eval_line(&mut ws, "1+SETUP").unwrap_err();
    assert_eq!((e.kind, e.caret), (ErrorKind::Value, Some(2)));
    // A header with a result that is never assigned has none either.
    define(&mut ws, "R\u{2190}EMPTY", &["Z\u{2190}1"]);
    assert_eq!(eval_line(&mut ws, "EMPTY").unwrap(), Output::Nothing);
    assert_eq!(
        eval_line(&mut ws, "1+EMPTY").unwrap_err().kind,
        ErrorKind::Value
    );
}

#[test]
fn the_valence_written_must_be_the_valence_declared() {
    let mut ws = Workspace::default();
    define(&mut ws, "R\u{2190}DOUBLE N", &["R\u{2190}N+N"]);
    define(&mut ws, "R\u{2190}TEN", &["R\u{2190}10"]);
    let e = eval_line(&mut ws, "2 DOUBLE 3").unwrap_err();
    assert_eq!((e.kind, e.caret), (ErrorKind::Syntax, Some(2)));
    assert_eq!(
        eval_line(&mut ws, "DOUBLE").unwrap_err().kind,
        ErrorKind::Syntax
    );
    assert_eq!(
        eval_line(&mut ws, "TEN 1").unwrap_err().kind,
        ErrorKind::Syntax
    );
}

#[test]
fn a_body_line_that_displays_reaches_the_pending_output() {
    let mut ws = Workspace::default();
    define(&mut ws, "R\u{2190}NOISY N", &["N+1", "'HI'", "R\u{2190}N"]);
    assert_eq!(nums(&mut ws, "NOISY 4"), [Number::Int(4)]);
    assert_eq!(ws.output.len(), 2);
    assert_eq!(ws.output[0], Output::Value(Array::scalar(Number::Int(5))));
}

#[test]
fn recursion_is_bounded_by_depth_error() {
    let mut ws = Workspace::default();
    define(&mut ws, "R\u{2190}DOWN N", &["R\u{2190}DOWN N-1"]);
    let e = eval_line(&mut ws, "DOWN 3").unwrap_err();
    assert_eq!((e.kind, e.caret), (ErrorKind::Depth, Some(0)));
    // The frames all unwound: nothing of the call is left behind.
    assert_eq!(eval_line(&mut ws, "N").unwrap_err().kind, ErrorKind::Value);
}

#[test]
fn an_error_inside_a_body_points_at_the_call() {
    let mut ws = Workspace::default();
    define(&mut ws, "R\u{2190}BAD N", &["R\u{2190}2 3+4 5 6"]);
    let e = eval_line(&mut ws, "1+BAD 0").unwrap_err();
    assert_eq!((e.kind, e.caret), (ErrorKind::Length, Some(2)));
}
