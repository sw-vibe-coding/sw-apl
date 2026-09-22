//! The group commands, which only '70 has, against a workspace built
//! by hand: the group rules, and `)ERASE` of a group.

use apl_a70_commands::grouping;
use apl_eval::{Defn, Workspace};
use apl_inquiry::command;
use apl_value::{Array, Number};

/// Run a command the way `system_command` hands it over -- the name
/// without its parenthesis, and the words after it -- to the group
/// commands first and the inquiry commands after, as it does in (A).
fn run(ws: &mut Workspace, line: &str) -> Vec<String> {
    let mut words = line.trim_start_matches(')').split_whitespace();
    let name = words.next().unwrap_or("").to_string();
    let rest: Vec<&str> = words.collect();
    grouping(&mut ws.saved, &name, &rest)
        .or_else(|| command(ws, &name, &rest))
        .expect("a group or inquiry command")
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
fn a_group_is_a_name_standing_for_names() {
    let mut ws = with_names(&["A"], &["F"]);
    assert_eq!(run(&mut ws, ")GROUP G A F MISSING"), Vec::<String>::new());
    assert_eq!(run(&mut ws, ")GRPS"), vec!["G"]);
    assert_eq!(run(&mut ws, ")GRP G"), vec!["A F MISSING"]);
    // A member need not exist: a group holds names, not referents.
    assert!(ws.get("MISSING").is_none());
}

#[test]
fn a_group_name_already_in_use_is_refused() {
    let mut ws = with_names(&["A"], &["F"]);
    assert_eq!(run(&mut ws, ")GROUP A B"), vec!["NOT GROUPED, NAME IN USE"]);
    assert_eq!(run(&mut ws, ")GROUP F B"), vec!["NOT GROUPED, NAME IN USE"]);
    assert_eq!(run(&mut ws, ")GRPS"), Vec::<String>::new());
}

#[test]
fn naming_a_group_again_supersedes_it_and_naming_it_alone_disperses() {
    let mut ws = with_names(&["A", "B"], &[]);
    run(&mut ws, ")GROUP G A");
    run(&mut ws, ")GROUP G B");
    assert_eq!(run(&mut ws, ")GRP G"), vec!["B"], "superseded, not added");
    // The group name used twice adds to it instead.
    run(&mut ws, ")GROUP G G A");
    assert_eq!(run(&mut ws, ")GRP G"), vec!["B A"]);
    // One name alone disperses the group and leaves its members.
    assert_eq!(run(&mut ws, ")GROUP G"), Vec::<String>::new());
    assert_eq!(run(&mut ws, ")GRPS"), Vec::<String>::new());
    assert!(ws.get("A").is_some(), "the members are untouched");
    assert!(ws.get("B").is_some());
}

#[test]
fn erasing_a_group_takes_its_members_too() {
    let mut ws = with_names(&["A", "B"], &["F"]);
    run(&mut ws, ")GROUP G A F");
    assert_eq!(run(&mut ws, ")ERASE G"), Vec::<String>::new());
    assert!(ws.get("A").is_none(), "the member went with the group");
    assert!(!ws.is_function("F"));
    assert_eq!(run(&mut ws, ")GRPS"), Vec::<String>::new(), "and the group");
    assert!(ws.get("B").is_some());
}

#[test]
fn a_group_lists_its_members_in_the_order_they_were_gathered() {
    let mut ws = with_names(&[], &[]);
    run(&mut ws, ")GROUP G ZED ALPHA MID");
    assert_eq!(
        run(&mut ws, ")GRP G"),
        vec!["ZED ALPHA MID"],
        "as written; only )GRPS and the name listings sort"
    );
}

#[test]
fn an_empty_workspace_lists_no_groups() {
    let mut ws = Workspace::default();
    assert_eq!(run(&mut ws, ")GRPS"), Vec::<String>::new());
}

#[test]
fn a_group_command_given_what_it_does_not_take_is_incorrect() {
    let mut ws = with_names(&["A"], &[]);
    for bad in [")GRP", ")GRP A B", ")GROUP"] {
        assert_eq!(run(&mut ws, bad), vec!["INCORRECT COMMAND"], "{bad}");
    }
}
