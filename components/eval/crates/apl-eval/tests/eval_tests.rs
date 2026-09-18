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
fn a_branch_names_the_line_to_run_next() {
    let mut ws = Workspace::default();
    assert_eq!(
        eval_line(&mut ws, "\u{2192}3").unwrap(),
        Output::Branch(Some(3))
    );
    assert_eq!(
        eval_line(&mut ws, "\u{2192}0").unwrap(),
        Output::Branch(Some(0))
    );
    // An empty vector, and a bare arrow, fall through to the next line.
    assert_eq!(
        eval_line(&mut ws, "\u{2192}9\u{d7}\u{2373}0").unwrap(),
        Output::Nothing
    );
    assert_eq!(
        eval_line(&mut ws, "\u{2192}").unwrap(),
        Output::Branch(None),
        "a bare arrow is not a blank line: it clears the state indicator"
    );
    // The first element selects; the rest are ignored.
    assert_eq!(
        eval_line(&mut ws, "\u{2192}2 7 7").unwrap(),
        Output::Branch(Some(2))
    );
    assert_eq!(
        eval_line(&mut ws, "\u{2192}'A'").unwrap_err().kind,
        ErrorKind::Domain
    );
    assert_eq!(
        eval_line(&mut ws, "\u{2192}2 2\u{2374}1 2 3 4")
            .unwrap_err()
            .kind,
        ErrorKind::Rank
    );
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
    // A name the header localizes starts the call undefined. The
    // failure suspends H, so T stays shadowed until the state
    // indicator is cleared with a bare branch.
    define(&mut ws, "R\u{2190}H;T", &["R\u{2190}T"]);
    assert_eq!(eval_line(&mut ws, "H").unwrap_err().kind, ErrorKind::Value);
    assert_eq!(eval_line(&mut ws, "T").unwrap_err().kind, ErrorKind::Value);
    assert_eq!(
        eval_line(&mut ws, "\u{2192}").unwrap(),
        Output::Branch(None)
    );
    apl_call::clear(&mut ws);
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

/// Also a guard on `MAX_DEPTH` itself: the limit has to fire before
/// the machine stack runs out, so if a change makes each level cost
/// more, this overflows rather than reporting DEPTH ERROR.
#[test]
fn recursion_is_bounded_by_depth_error() {
    let mut ws = Workspace::default();
    define(&mut ws, "R\u{2190}DOWN N", &["R\u{2190}DOWN N-1"]);
    let e = eval_line(&mut ws, "DOWN 3").unwrap_err();
    assert_eq!(e.kind, ErrorKind::Depth);
    // Only the call written as a whole statement suspends. The
    // recursive ones sit inside an expression, which sw-apl cannot
    // take up again, so they unwound on the way out.
    assert_eq!(ws.si().len(), 1);
    assert_eq!(ws.si()[0].name, "DOWN");
    // Which activation is starred is settled where the error reaches
    // the terminal, which is the session's job, not the evaluator's.
    apl_call::suspend(&mut ws);
    assert!(ws.si()[0].suspended);
    apl_call::clear(&mut ws);
    assert_eq!(eval_line(&mut ws, "N").unwrap_err().kind, ErrorKind::Value);
}

#[test]
fn an_error_inside_a_body_names_the_function_and_the_line() {
    let mut ws = Workspace::default();
    define(
        &mut ws,
        "R\u{2190}BAD N",
        &["R\u{2190}1", "R\u{2190}2 3+4 5 6"],
    );
    let e = eval_line(&mut ws, "BAD 0").unwrap_err();
    assert_eq!(e.kind, ErrorKind::Length);
    let context = e.context.expect("the failing line travels with the error");
    assert_eq!(context.function, "BAD");
    assert_eq!(context.line, 2);
    assert_eq!(context.statement, "R\u{2190}2 3+4 5 6");
    // The caret points into that line, not into the calling statement.
    assert_eq!(e.caret, Some(5));
}

#[test]
fn a_label_is_a_local_constant_holding_its_line_number() {
    let mut ws = Workspace::default();
    define(
        &mut ws,
        "R\u{2190}WHERE",
        &[
            "R\u{2190}0",
            "TOP:R\u{2190}R+1",
            "R\u{2190}R,TOP,BOT",
            "BOT:R\u{2190}R",
        ],
    );
    assert_eq!(
        nums(&mut ws, "WHERE"),
        [Number::Int(1), Number::Int(2), Number::Int(4)]
    );
    // The label is gone once the call returns.
    assert_eq!(
        eval_line(&mut ws, "TOP").unwrap_err().kind,
        ErrorKind::Value
    );
}

#[test]
fn a_branch_moves_the_line_counter_and_a_loop_terminates() {
    let mut ws = Workspace::default();
    define(
        &mut ws,
        "R\u{2190}SUMTO N;I",
        &[
            "R\u{2190}0",
            "I\u{2190}0",
            "LOOP:I\u{2190}I+1",
            "R\u{2190}R+I",
            "\u{2192}LOOP\u{d7}\u{2373}I<N",
        ],
    );
    assert_eq!(nums(&mut ws, "SUMTO 4"), [Number::Int(10)]);
    assert_eq!(nums(&mut ws, "SUMTO 1"), [Number::Int(1)]);
    assert_eq!(nums(&mut ws, "SUMTO 100"), [Number::Int(5050)]);
}

#[test]
fn a_branch_out_of_the_function_returns_the_result_so_far() {
    let mut ws = Workspace::default();
    define(
        &mut ws,
        "R\u{2190}EARLY N",
        &["R\u{2190}N", "\u{2192}0", "R\u{2190}999"],
    );
    assert_eq!(nums(&mut ws, "EARLY 7"), [Number::Int(7)]);
    // Any line number the function does not have leaves it too.
    define(
        &mut ws,
        "R\u{2190}OFFEND N",
        &["R\u{2190}N", "\u{2192}99", "R\u{2190}999"],
    );
    assert_eq!(nums(&mut ws, "OFFEND 7"), [Number::Int(7)]);
    // Running off the last line leaves it as well.
    define(&mut ws, "R\u{2190}FALLOFF N", &["R\u{2190}N"]);
    assert_eq!(nums(&mut ws, "FALLOFF 7"), [Number::Int(7)]);
}

#[test]
fn recursion_terminates_when_a_branch_stops_it() {
    let mut ws = Workspace::default();
    define(
        &mut ws,
        "R\u{2190}FAC N",
        &[
            "R\u{2190}1",
            "\u{2192}0\u{d7}\u{2373}N\u{2264}1",
            "R\u{2190}N\u{d7}FAC N-1",
        ],
    );
    assert_eq!(nums(&mut ws, "FAC 5"), [Number::Int(120)]);
    assert_eq!(nums(&mut ws, "FAC 1"), [Number::Int(1)]);
    assert_eq!(nums(&mut ws, "FAC 10"), [Number::Int(3_628_800)]);
}

#[test]
fn a_label_does_not_change_what_its_line_does() {
    let mut ws = Workspace::default();
    define(&mut ws, "R\u{2190}SHOUT", &["L:'HI'", "R\u{2190}L"]);
    assert_eq!(nums(&mut ws, "SHOUT"), [Number::Int(1)]);
    assert_eq!(ws.output.len(), 1, "the labelled line still displays");
}
