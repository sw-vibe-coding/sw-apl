//! The inquiry commands against a workspace built by hand, so that
//! the listing and what `)ERASE` refuses are pinned without going
//! through a session. The group commands' tests are in
//! apl-a68-commands.

use apl_eval::{Defn, Workspace};
use apl_inquiry::command;
use apl_value::{Array, Number};

/// Run a command the way `system_command` hands it over: the name
/// without its parenthesis, and the words after it.
fn run(ws: &mut Workspace, line: &str) -> Vec<String> {
    let mut words = line.trim_start_matches(')').split_whitespace();
    let name = words.next().unwrap_or("").to_string();
    let rest: Vec<&str> = words.collect();
    command(ws, &name, &rest).expect("an inquiry command")
}

fn with_names(names: &[&str], funcs: &[&str]) -> Workspace {
    let mut ws = Workspace::default();
    for n in names {
        ws.set(n, Array::scalar(Number::Int(1))).unwrap();
    }
    for n in funcs {
        ws.define(Defn {
            name: (*n).to_string(),
            ..Defn::default()
        })
        .unwrap();
    }
    ws
}

#[test]
fn names_are_listed_alphabetically_on_one_line() {
    let mut ws = with_names(&["ZED", "ALPHA", "MID"], &["PLOT", "AND", "VS"]);
    assert_eq!(run(&mut ws, ")VARS"), vec!["ALPHA MID ZED"]);
    assert_eq!(run(&mut ws, ")FNS"), vec!["AND PLOT VS"]);
}

#[test]
fn a_letter_starts_the_listing_there() {
    let mut ws = with_names(&["A", "M", "Z"], &[]);
    assert_eq!(run(&mut ws, ")VARS M"), vec!["M Z"]);
    assert_eq!(run(&mut ws, ")VARS A"), vec!["A M Z"]);
    // Past the end of the alphabet is an empty listing, not an error.
    assert_eq!(run(&mut ws, ")VARS ZZ"), Vec::<String>::new());
}

#[test]
fn an_empty_workspace_lists_nothing() {
    let mut ws = Workspace::default();
    assert_eq!(run(&mut ws, ")FNS"), Vec::<String>::new());
    assert_eq!(run(&mut ws, ")VARS"), Vec::<String>::new());
}

#[test]
fn a_listing_wraps_at_the_print_width() {
    let mut ws = with_names(&[], &[]);
    for n in ["AAAAAAAAAA", "BBBBBBBBBB", "CCCCCCCCCC", "DDDDDDDDDD"] {
        ws.set(n, Array::scalar(Number::Int(1))).unwrap();
    }
    ws.saved.print.width = 30;
    assert_eq!(
        run(&mut ws, ")VARS"),
        vec!["AAAAAAAAAA BBBBBBBBBB", "CCCCCCCCCC DDDDDDDDDD"],
        "two names to a line at width 30"
    );
}

#[test]
fn vars_lists_globals_not_the_locals_shadowing_them() {
    let mut ws = with_names(&["GLOBAL", "SHADOWED"], &[]);
    ws.enter("F", &["SHADOWED".to_string(), "LOCAL".to_string()])
        .unwrap();
    ws.set("SHADOWED", Array::scalar(Number::Int(2))).unwrap();
    ws.set("LOCAL", Array::scalar(Number::Int(3))).unwrap();
    // SHADOWED is still a global, displaced by the call. LOCAL is
    // not a global at all: it exists only for the length of the call.
    assert_eq!(run(&mut ws, ")VARS"), vec!["GLOBAL SHADOWED"]);
}

#[test]
fn erasing_takes_variables_functions_and_groups() {
    let mut ws = with_names(&["A", "B"], &["F"]);
    assert_eq!(run(&mut ws, ")ERASE A F"), Vec::<String>::new());
    assert!(ws.get("A").is_none());
    assert!(!ws.is_function("F"));
    assert!(ws.get("B").is_some(), "only what was named");
    // A name that holds nothing is ignored.
    assert_eq!(run(&mut ws, ")ERASE NOSUCH"), Vec::<String>::new());
}

#[test]
fn a_function_on_the_state_indicator_is_not_erased() {
    let mut ws = with_names(&["A"], &["RUNNING", "IDLE"]);
    ws.enter("RUNNING", &[]).unwrap();
    assert_eq!(
        run(&mut ws, ")ERASE RUNNING IDLE A"),
        vec!["NOT ERASED: RUNNING"]
    );
    assert!(ws.is_function("RUNNING"), "still there to take up again");
    assert!(!ws.is_function("IDLE"), "the rest went");
    assert!(ws.get("A").is_none());
}

#[test]
fn symbols_reports_what_is_held_and_cannot_be_set() {
    let mut ws = with_names(&["A", "B"], &["F"]);
    let reply = run(&mut ws, ")SYMBOLS");
    assert_eq!(reply.len(), 1);
    assert!(reply[0].starts_with("IS "), "{reply:?}");
    assert!(reply[0].ends_with(", USED 3"), "{reply:?}");
    assert_eq!(run(&mut ws, ")SYMBOLS 500"), vec!["INCORRECT COMMAND"]);
}

#[test]
fn a_command_given_what_it_does_not_take_is_incorrect() {
    let mut ws = with_names(&["A"], &[]);
    for bad in [")ERASE", ")VARS A B"] {
        assert_eq!(run(&mut ws, bad), vec!["INCORRECT COMMAND"], "{bad}");
    }
}

#[test]
fn a_command_this_crate_does_not_answer_is_left_alone() {
    let mut ws = Workspace::default();
    assert!(command(&mut ws, "SAVE", &[]).is_none());
    assert!(command(&mut ws, "WSID", &[]).is_none());
}

#[test]
fn a_nested_call_does_not_invent_a_global() {
    let mut ws = with_names(&[], &[]);
    // OUTER localizes X, which no global held; INNER localizes it
    // again and so displaces OUTER's local. Only the outermost call
    // to localize a name can say whether a global is behind it.
    ws.enter("OUTER", &["X".to_string()]).unwrap();
    ws.set("X", Array::scalar(Number::Int(1))).unwrap();
    ws.enter("INNER", &["X".to_string()]).unwrap();
    ws.set("X", Array::scalar(Number::Int(2))).unwrap();
    assert_eq!(run(&mut ws, ")VARS"), Vec::<String>::new());
    // Leaving both calls puts nothing back, so there is still none.
    ws.leave();
    ws.leave();
    assert_eq!(run(&mut ws, ")VARS"), Vec::<String>::new());
}
