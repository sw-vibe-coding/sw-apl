//! The libraries as each mode sees them: listing, loading, saving
//! and dropping, and two workspaces sharing a name. A key says the
//! modes its workspace runs in: `NAME` for both, `NAME.a-70` for (A)
//! only, `NAME.b-75` for (B) only (owner, 2026-09-22).

use apl_modes::{Mode, Modes, modes};
use apl_shelves::{forget, keep, list, read};
use apl_store::{Memory, Store};

/// A workspace file with the given modes line, or none.
fn ws(line: Option<&str>, body: &str) -> String {
    let mut text = "⍝ sw-apl workspace. Re-executable APL: loading it runs it.\n".to_string();
    if let Some(line) = line {
        text = text + "⍝!MODES " + line + "\n";
    }
    text + body + "\n"
}

fn store(entries: &[(&str, &str)]) -> Memory {
    let mut m = Memory::default();
    for (key, text) in entries {
        m.work.insert((*key).to_string(), (*text).to_string());
    }
    m
}

#[test]
fn each_mode_lists_and_reads_only_what_runs_in_it() {
    let s = store(&[
        ("OLD", &ws(None, "X←1")),
        ("BOTH", &ws(Some("(A)(B)"), "X←2")),
        ("NEW", &ws(Some("(B)"), "X←3")),
    ]);
    assert_eq!(list(&s, Mode::A, 0), vec!["BOTH", "OLD"]);
    assert_eq!(list(&s, Mode::B, 0), vec!["BOTH", "NEW"]);
    assert!(read(&s, Mode::A, 0, "NEW").is_none());
    assert!(read(&s, Mode::B, 0, "OLD").is_none());
    assert!(read(&s, Mode::B, 0, "BOTH").is_some());
}

#[test]
fn two_workspaces_may_share_a_name_when_their_modes_do_not_overlap() {
    let s = store(&[
        ("BIRDS.a-70", &ws(Some("(A)"), "X←68")),
        ("BIRDS.b-75", &ws(Some("(B)"), "X←75")),
    ]);
    assert_eq!(list(&s, Mode::A, 0), vec!["BIRDS"]);
    assert_eq!(list(&s, Mode::B, 0), vec!["BIRDS"]);
    assert!(read(&s, Mode::A, 0, "BIRDS").unwrap().contains("X←68"));
    assert!(read(&s, Mode::B, 0, "BIRDS").unwrap().contains("X←75"));
}

#[test]
fn saving_in_68_over_a_68_workspace_keeps_its_key() {
    let mut s = store(&[("OLD", &ws(None, "X←1"))]);
    keep(&mut s, Mode::A, 0, "OLD", &ws(Some("(A)(B)"), "X←9")).unwrap();
    assert_eq!(s.list(0), vec!["OLD"], "one file, under the name");
    assert_eq!(s.read(0, "OLD").unwrap(), ws(Some("(A)(B)"), "X←9"));
}

#[test]
fn saving_in_one_mode_leaves_the_other_modes_copy_alone() {
    let mut s = store(&[("W", &ws(Some("(A)(B)"), "X←1"))]);
    keep(&mut s, Mode::B, 0, "W", &ws(Some("(B)"), "X←⍎'2'")).unwrap();
    let a = read(&s, Mode::A, 0, "W").unwrap();
    assert!(a.contains("X←1"), "(A) still has its own W");
    assert_eq!(modes(&a), Modes::only(Mode::A), "and only (A) now lists it");
    assert!(read(&s, Mode::B, 0, "W").unwrap().contains("X←⍎'2'"));
    assert_eq!(
        s.list(0),
        vec!["W.a-70", "W.b-75"],
        "each named for its mode"
    );
}

#[test]
fn a_save_never_replaces_another_modes_own_workspace() {
    let mut s = store(&[
        ("W.a-70", &ws(Some("(A)"), "X←1")),
        ("W.b-75", &ws(Some("(B)"), "X←2")),
    ]);
    keep(&mut s, Mode::A, 0, "W", &ws(Some("(A)(B)"), "X←3")).unwrap();
    assert!(read(&s, Mode::A, 0, "W").unwrap().contains("X←3"));
    assert!(
        read(&s, Mode::B, 0, "W").unwrap().contains("X←2"),
        "(B) keeps its W"
    );
    assert_eq!(
        modes(&read(&s, Mode::A, 0, "W").unwrap()),
        Modes::only(Mode::A)
    );
    assert_eq!(s.list(0), vec!["W.a-70", "W.b-75"]);
}

#[test]
fn a_save_that_runs_in_both_is_listed_where_the_name_is_free() {
    let mut s = store(&[]);
    keep(&mut s, Mode::B, 0, "W", &ws(Some("(A)(B)"), "X←1")).unwrap();
    assert_eq!(s.list(0), vec!["W"], "kept once");
    assert!(read(&s, Mode::A, 0, "W").is_some(), "and listed in (A) too");
}

#[test]
fn a_save_in_75_leaves_a_68_only_namesake_alone() {
    let mut s = store(&[("W", &ws(None, "X←68"))]);
    keep(&mut s, Mode::B, 0, "W", &ws(Some("(A)(B)"), "X←75")).unwrap();
    assert!(read(&s, Mode::A, 0, "W").unwrap().contains("X←68"));
    assert!(read(&s, Mode::B, 0, "W").unwrap().contains("X←75"));
    assert_eq!(s.list(0), vec!["W", "W.b-75"]);
}

#[test]
fn a_save_that_runs_in_one_mode_says_so_in_its_key() {
    let mut s = store(&[]);
    keep(&mut s, Mode::B, 0, "T", &ws(Some("(B)"), "X←⍎'1'")).unwrap();
    assert_eq!(s.list(0), vec!["T.b-75"]);
    assert_eq!(list(&s, Mode::B, 0), vec!["T"], "listed by its name");
    keep(&mut s, Mode::A, 0, "U", &ws(Some("(A)"), "X←⌶20")).unwrap();
    assert!(s.list(0).contains(&"U.a-70".to_string()));
}

#[test]
fn a_workspace_narrowed_to_one_mode_is_renamed_for_it() {
    let mut s = store(&[("W", &ws(Some("(A)(B)"), "X←1"))]);
    forget(&mut s, Mode::B, 0, "W").unwrap();
    assert_eq!(s.list(0), vec!["W.a-70"]);
    assert!(read(&s, Mode::A, 0, "W").is_some());
}

#[test]
fn a_key_named_the_older_way_is_still_found() {
    let s = store(&[
        ("W", &ws(Some("(A)"), "X←1")),
        ("W@B", &ws(Some("(B)"), "X←2")),
    ]);
    assert_eq!(list(&s, Mode::B, 0), vec!["W"]);
    assert!(read(&s, Mode::B, 0, "W").unwrap().contains("X←2"));
    assert!(read(&s, Mode::A, 0, "W").unwrap().contains("X←1"));
}

#[test]
fn a_save_is_always_listed_in_the_mode_it_was_made_in() {
    let mut s = store(&[]);
    keep(&mut s, Mode::A, 0, "W", &ws(Some("(B)"), "X←1")).unwrap();
    assert!(read(&s, Mode::A, 0, "W").is_some());
}

#[test]
fn dropping_in_one_mode_leaves_the_other_its_copy() {
    let mut s = store(&[("W", &ws(Some("(A)(B)"), "X←1"))]);
    forget(&mut s, Mode::B, 0, "W").unwrap();
    assert!(read(&s, Mode::B, 0, "W").is_none());
    assert!(read(&s, Mode::A, 0, "W").is_some());
    forget(&mut s, Mode::A, 0, "W").unwrap();
    assert!(s.list(0).is_empty(), "gone once no mode has it");
    assert!(forget(&mut s, Mode::A, 0, "W").is_err());
}

#[test]
fn a_68_workspace_cannot_be_dropped_from_75() {
    let mut s = store(&[("OLD", &ws(None, "X←1"))]);
    assert!(forget(&mut s, Mode::B, 0, "OLD").is_err());
    assert_eq!(s.list(0), vec!["OLD"]);
}
