//! `[N⎕M]` through a session: the manual's example on p. A.5 of the
//! APL\360-OS and APL\360-DOS User's Manual, typed line by line.

use apl_session::Session;

fn out(s: &mut Session, line: &str) -> Vec<String> {
    s.respond(line).lines
}

#[test]
fn the_manuals_example_with_its_fill_begun_by_a_parenthesis() {
    let mut s = Session::default();
    out(&mut s, "∇Z←ABC X");
    out(&mut s, "Z←(33×Q+(R×5)-6");
    assert_eq!(out(&mut s, "[1⎕9]"), ["[1]   Z←(33×Q+(R×5)-6"]);
    assert_eq!(s.prompt(), " ".repeat(9));
    assert_eq!(out(&mut s, "/   1 /1"), ["[1]   Z←(3×Q +( ×5)-6"]);
    assert_eq!(s.prompt(), "[1]   Z←(3×Q");
    assert!(
        out(&mut s, ")  T").is_empty(),
        "a fill, not a system command"
    );
    assert_eq!(s.prompt(), "[2]   ");
    out(&mut s, "∇");
    assert_eq!(
        out(&mut s, "∇ABC[⎕]∇"),
        ["      ∇Z←ABC X", "[1]   Z←(3×Q)+(T×5)-6", "      ∇"]
    );
}

#[test]
fn an_edit_may_be_asked_for_as_the_definition_opens() {
    let mut s = Session::default();
    out(&mut s, "∇F");
    out(&mut s, "X←1");
    out(&mut s, "∇");
    assert_eq!(out(&mut s, "∇F[1⎕8]"), ["[1]   X←1"]);
    assert_eq!(out(&mut s, "2"), ["[1]   X←  1"]);
    assert!(out(&mut s, "+2").is_empty());
    out(&mut s, "∇");
    assert_eq!(out(&mut s, "∇F[⎕]∇")[1], "[1]   X←+21");
}
