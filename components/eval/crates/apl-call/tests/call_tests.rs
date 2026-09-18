//! Calling a defined function, driven by a stand-in evaluator rather
//! than the real one. `Run` is a function pointer precisely so this
//! machinery can be pinned on its own: what suspends, what unwinds,
//! and which activation the state indicator stars.

use apl_ast::Defn;
use apl_call::{branch_target, call, clear, resume, suspend, value};
use apl_value::{AplError, AplResult, Array, ErrorKind, Number};
use apl_workspace::{Output, Workspace};

/// A stand-in for `eval_line`: it reads a handful of words so the
/// call machinery can be exercised without the interpreter.
fn run(ws: &mut Workspace, line: &str) -> AplResult<Output> {
    let (word, rest) = line.trim().split_once(' ').unwrap_or((line.trim(), ""));
    match word {
        "fail" => Err(AplError::new(ErrorKind::Domain)),
        "show" => Ok(Output::Value(num(1))),
        "go" => Ok(Output::Branch(rest.trim().parse().ok())),
        "set" => {
            ws.set(rest.trim(), num(42));
            Ok(Output::Nothing)
        }
        _ => Ok(Output::Nothing),
    }
}

fn num(i: i64) -> Array {
    Array::scalar(Number::Int(i))
}

fn define(ws: &mut Workspace, name: &str, result: Option<&str>, body: &[&str]) {
    ws.define(Defn {
        name: name.to_string(),
        result: result.map(str::to_string),
        body: body.iter().map(|l| (*l).to_string()).collect(),
        ..Defn::default()
    });
}

#[test]
fn a_call_written_as_a_statement_suspends_where_it_failed() {
    let mut ws = Workspace::default();
    define(&mut ws, "F", None, &["quiet", "fail", "quiet"]);
    let err = call(&mut ws, "F", None, None, run).unwrap_err();
    assert_eq!(err.kind, ErrorKind::Domain);
    let context = err.context.expect("the failing line travels with it");
    assert_eq!((context.function.as_str(), context.line), ("F", 2));
    assert_eq!(context.statement, "fail");
    assert_eq!(ws.si().len(), 1, "the activation stayed on the stack");
    assert_eq!(ws.si()[0].line, 2);
}

#[test]
fn a_call_written_inside_an_expression_unwinds_instead() {
    let mut ws = Workspace::default();
    define(&mut ws, "F", Some("R"), &["fail"]);
    let before = ws.si().len();
    let err = value(&mut ws, "F", 7, (None, None), run).unwrap_err();
    assert_eq!(err.kind, ErrorKind::Domain);
    assert_eq!(err.caret, Some(7), "the caret falls back to the call");
    assert_eq!(
        ws.si().len(),
        before,
        "sw-apl cannot resume half an expression, so it does not suspend"
    );
}

#[test]
fn suspend_stars_the_innermost_and_survives_an_unwind() {
    let mut ws = Workspace::default();
    define(&mut ws, "INNER", None, &["fail"]);
    define(&mut ws, "OUTER", None, &["quiet", "fail"]);
    ws.enter("OUTER", &[]).unwrap();
    ws.stop(0, 2, false);
    ws.enter("INNER", &[]).unwrap();
    ws.stop(1, 1, false);
    assert!(
        ws.si().iter().all(|a| !a.suspended),
        "nothing is starred until the error reaches the terminal"
    );
    suspend(&mut ws);
    assert!(!ws.si()[0].suspended, "the caller is pendent");
    assert!(ws.si()[1].suspended, "the innermost is the suspended one");
}

#[test]
fn clearing_takes_the_pendent_callers_but_not_a_second_suspension() {
    let mut ws = Workspace::default();
    // Outermost first: a suspension, a caller waiting on it, and a
    // second suspension on top.
    ws.enter("A", &[]).unwrap();
    ws.stop(0, 1, true);
    ws.enter("B", &[]).unwrap();
    ws.stop(1, 2, false);
    ws.enter("C", &[]).unwrap();
    ws.stop(2, 3, true);
    clear(&mut ws);
    assert_eq!(ws.si().len(), 1, "C and the pendent B went together");
    assert_eq!(ws.si()[0].name, "A");
    clear(&mut ws);
    assert!(ws.si().is_empty());
    // Clearing nothing is harmless.
    clear(&mut ws);
}

#[test]
fn a_body_follows_its_branches_and_reports_what_it_displays() {
    let mut ws = Workspace::default();
    define(&mut ws, "F", None, &["go 3", "show", "show"]);
    assert_eq!(call(&mut ws, "F", None, None, run).unwrap(), None);
    assert_eq!(ws.output.len(), 1, "line 2 was jumped over");
    ws.output.clear();
    // A branch to a line the function does not have returns.
    define(&mut ws, "G", None, &["go 0", "show"]);
    assert_eq!(call(&mut ws, "G", None, None, run).unwrap(), None);
    assert!(ws.output.is_empty());
    // A bare arrow falls through rather than returning.
    define(&mut ws, "H", None, &["go", "show"]);
    assert_eq!(call(&mut ws, "H", None, None, run).unwrap(), None);
    assert_eq!(ws.output.len(), 1);
}

#[test]
fn resuming_finishes_the_function_and_returns_through_its_caller() {
    let mut ws = Workspace::default();
    define(&mut ws, "INNER", Some("R"), &["fail", "set R"]);
    define(&mut ws, "OUTER", None, &["quiet", "quiet", "show"]);
    // Stand the stack up as a failure inside OUTER's line 2 would.
    ws.enter("OUTER", &["R".to_string()]).unwrap();
    ws.stop(0, 2, false);
    ws.enter("INNER", &["R".to_string()]).unwrap();
    ws.stop(1, 1, false);
    suspend(&mut ws);
    // Take INNER up at line 2: it finishes, its value is shown where
    // the call stood, and OUTER carries on at line 3.
    let shown = resume(&mut ws, 2, run).unwrap();
    assert_eq!(shown, Output::Nothing, "OUTER declares no result");
    assert!(ws.si().is_empty(), "both activations returned");
    assert_eq!(ws.output.len(), 2, "INNER's value, then OUTER's line 3");
    assert_eq!(ws.output[0], Output::Value(num(42)));
}

#[test]
fn resuming_with_nothing_suspended_is_a_syntax_error() {
    let mut ws = Workspace::default();
    assert_eq!(resume(&mut ws, 1, run).unwrap_err().kind, ErrorKind::Syntax);
    ws.enter("F", &[]).unwrap();
    assert_eq!(
        resume(&mut ws, -1, run).unwrap_err().kind,
        ErrorKind::Syntax,
        "a line number below one is not a line"
    );
}

#[test]
fn the_valence_written_must_be_the_one_declared() {
    let mut ws = Workspace::default();
    define(&mut ws, "F", None, &["quiet"]);
    for args in [
        (Some(num(1)), None),
        (None, Some(num(1))),
        (Some(num(1)), Some(num(1))),
    ] {
        let err = call(&mut ws, "F", args.0, args.1, run).unwrap_err();
        assert_eq!(err.kind, ErrorKind::Syntax);
        assert!(ws.si().is_empty(), "a refused call enters nothing");
    }
}

#[test]
fn a_branch_reads_the_first_element_of_its_value() {
    assert_eq!(branch_target(&num(3)).unwrap(), Some(3));
    assert_eq!(
        branch_target(&Array::vector(vec![Number::Int(2), Number::Int(9)])).unwrap(),
        Some(2)
    );
    assert_eq!(
        branch_target(&Array::vector(Vec::new())).unwrap(),
        None,
        "an empty vector falls through"
    );
    assert_eq!(
        branch_target(&Array::scalar(Number::Float(1.5)))
            .unwrap_err()
            .kind,
        ErrorKind::Domain
    );
}
