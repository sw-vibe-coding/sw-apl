//! Stopping a running body from outside it.
//!
//! Its own file, and so its own process: the stop flag is global, as
//! it must be to be set from a signal handler, and cargo runs the
//! tests within one file on parallel threads. A test that sets the
//! flag beside a test that calls a function would take its stop.

use apl_ast::Defn;
use apl_call::{call, clear, interrupt, suspend};
use apl_value::{AplError, AplResult, Array, ErrorKind, Number};
use apl_workspace::{Output, Workspace};

/// A stand-in for `eval_line`, as in `call_tests.rs`.
fn run(_ws: &mut Workspace, line: &str) -> AplResult<Output> {
    match line.trim() {
        "fail" => Err(AplError::new(ErrorKind::Domain)),
        "show" => Ok(Output::Value(Array::scalar(Number::Int(1)))),
        _ => Ok(Output::Nothing),
    }
}

fn define(ws: &mut Workspace, name: &str, body: &[&str]) {
    ws.define(Defn {
        name: name.to_string(),
        body: body.iter().map(|l| (*l).to_string()).collect(),
        ..Defn::default()
    });
}

#[test]
fn a_stop_asked_for_from_outside_interrupts_the_body() {
    let mut ws = Workspace::default();
    define(&mut ws, "LOOP", &["quiet", "quiet", "quiet"]);
    // Nothing asked for, so the body runs to the end.
    assert!(call(&mut ws, "LOOP", None, None, run).is_ok());
    assert!(ws.si().is_empty());
    // Asked for, and the body stops between lines with INTERRUPT.
    interrupt();
    let err = call(&mut ws, "LOOP", None, None, run).unwrap_err();
    assert_eq!(err.kind, ErrorKind::Interrupt);
    let context = err.context.expect("it names where it stopped");
    assert_eq!((context.function.as_str(), context.line), ("LOOP", 1));
    // It suspends like any other failure, so the state indicator has
    // it and a branch could take it up again.
    assert_eq!(ws.si().len(), 1);
    suspend(&mut ws);
    assert!(ws.si()[0].suspended);
    clear(&mut ws);
    // Reading the flag cleared it, so the next call is not stopped.
    assert!(call(&mut ws, "LOOP", None, None, run).is_ok());
    assert!(ws.si().is_empty());
}
