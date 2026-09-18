//! The symbol table and the activation stack. These are the pieces
//! the rest of the interpreter trusts without checking, so they are
//! pinned here rather than only through session transcripts.

use apl_ast::Defn;
use apl_value::{Array, ErrorKind, Number};
use apl_workspace::Workspace;

fn n(i: i64) -> Array {
    Array::scalar(Number::Int(i))
}

fn names(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| (*s).to_string()).collect()
}

#[test]
fn a_call_shadows_its_names_and_puts_them_back() {
    let mut ws = Workspace::default();
    ws.set("A", n(1));
    ws.set("B", n(2));
    ws.enter("F", &names(&["A", "C"])).unwrap();
    assert_eq!(ws.get("A"), None, "a localized name starts undefined");
    assert_eq!(ws.get("B"), Some(&n(2)), "a global the call did not name");
    ws.set("A", n(10));
    ws.set("C", n(30));
    ws.leave();
    assert_eq!(ws.get("A"), Some(&n(1)), "the displaced value came back");
    assert_eq!(ws.get("C"), None, "a name the call invented is gone");
    assert_eq!(ws.get("B"), Some(&n(2)));
}

#[test]
fn nested_calls_unwind_one_layer_at_a_time() {
    let mut ws = Workspace::default();
    ws.set("X", n(1));
    ws.enter("OUTER", &names(&["X"])).unwrap();
    ws.set("X", n(2));
    ws.enter("INNER", &names(&["X"])).unwrap();
    ws.set("X", n(3));
    assert_eq!(ws.si().len(), 2);
    ws.leave();
    assert_eq!(ws.get("X"), Some(&n(2)), "back to the outer call's X");
    ws.leave();
    assert_eq!(ws.get("X"), Some(&n(1)), "back to the global X");
    assert!(ws.si().is_empty());
}

#[test]
fn the_stack_reads_outermost_first_and_carries_what_si_reports() {
    let mut ws = Workspace::default();
    ws.enter("OUTER", &names(&["P"])).unwrap();
    ws.enter("INNER", &names(&["Q", "R"])).unwrap();
    assert_eq!(ws.si()[0].name, "OUTER");
    assert_eq!(ws.si()[1].name, "INNER");
    assert_eq!(ws.si()[1].locals, names(&["Q", "R"]));
    assert!(ws.si().iter().all(|a| a.line == 1 && !a.suspended));
}

#[test]
fn stop_marks_one_activation_and_leaves_the_others_alone() {
    let mut ws = Workspace::default();
    let outer = ws.enter("OUTER", &[]).unwrap();
    let inner = ws.enter("INNER", &[]).unwrap();
    assert_eq!((outer, inner), (0, 1));
    ws.stop(inner, 7, true);
    ws.stop(outer, 3, false);
    assert_eq!((ws.si()[0].line, ws.si()[0].suspended), (3, false));
    assert_eq!((ws.si()[1].line, ws.si()[1].suspended), (7, true));
    // Stopping an activation that is not there changes nothing.
    ws.stop(99, 1, true);
    assert_eq!(ws.si().len(), 2);
    assert_eq!((ws.si()[1].line, ws.si()[1].suspended), (7, true));
}

#[test]
fn calls_may_not_nest_past_the_depth_guard() {
    let mut ws = Workspace::default();
    let mut deep = 0;
    while ws.enter("F", &[]).is_ok() {
        deep += 1;
        assert!(deep < 10_000, "the guard never fired");
    }
    assert_eq!(ws.si().len(), deep, "the failed entry pushed nothing");
    let err = ws.enter("F", &[]).unwrap_err();
    assert_eq!(err.kind, ErrorKind::Depth);
    // Unwinding the lot makes room again, so the guard is a depth
    // limit and not a budget that runs out over a session.
    for _ in 0..deep {
        ws.leave();
    }
    assert!(ws.si().is_empty());
    assert!(ws.enter("F", &[]).is_ok());
}

#[test]
fn leaving_an_empty_stack_is_harmless() {
    let mut ws = Workspace::default();
    ws.leave();
    ws.leave();
    assert!(ws.si().is_empty());
}

#[test]
fn a_name_holds_a_function_or_a_variable_never_both() {
    let mut ws = Workspace::default();
    ws.set("F", n(1));
    let defn = Defn {
        name: "F".to_string(),
        ..Defn::default()
    };
    ws.define(defn);
    assert!(ws.is_function("F"));
    assert_eq!(ws.get("F"), None, "defining F displaced the variable");
    ws.erase("F");
    assert!(!ws.is_function("F"));
    assert!(ws.function("F").is_none());
    // Erasing a name that holds nothing is allowed: the del editor
    // does it after a rename that had nothing to replace.
    ws.erase("NOTHING");
}
