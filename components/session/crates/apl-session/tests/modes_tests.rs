//! Modes in a session: the settings and the random link coming back
//! from their directives, and each mode listing, loading and dropping
//! only the workspaces that run in it.
//!
//! Two sessions share one library directory here, one in each mode,
//! which is how one person switching modes sees the same disc.

use apl_session::{Files, Mode, Session};

fn out(s: &mut Session, line: &str) -> Vec<String> {
    s.respond(line).lines
}

fn session(dir: &std::path::Path, mode: Mode) -> Session {
    let mut s = Session::default();
    s.ws.store = Box::new(Files(dir.to_path_buf()));
    s.ws.mode = mode;
    s
}

/// A library directory of its own, removed when the test ends.
struct Dir(std::path::PathBuf);

impl Dir {
    fn new(tag: &str) -> Dir {
        let dir = std::env::temp_dir().join(format!("sw-apl-modes-{tag}-{}", std::process::id()));
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(&dir).expect("mkdir");
        Dir(dir)
    }
}

impl Drop for Dir {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).ok();
    }
}

#[test]
fn a_loaded_workspace_takes_its_settings_from_the_directives() {
    let dir = Dir::new("settings");
    let mut a = session(&dir.0, Mode::A);
    for line in [")ORIGIN 0", ")DIGITS 3", ")WIDTH 80", ")SAVE W"] {
        out(&mut a, line);
    }
    let mut b = session(&dir.0, Mode::A);
    assert_eq!(out(&mut b, ")LOAD W").len(), 1, "only the SAVED line");
    assert_eq!(b.ws.saved.env.io, 0);
    assert_eq!(b.ws.saved.print.digits, 3);
    assert_eq!(b.ws.saved.print.width, 80);
}

#[test]
fn a_loaded_workspace_carries_on_its_random_sequence() {
    let dir = Dir::new("link");
    let mut a = session(&dir.0, Mode::A);
    out(&mut a, "?1000 1000 1000");
    out(&mut a, ")SAVE W");
    let next = out(&mut a, "?1000 1000 1000");
    let mut b = session(&dir.0, Mode::A);
    out(&mut b, ")LOAD W");
    assert_eq!(
        out(&mut b, "?1000 1000 1000"),
        next,
        "the link was restored"
    );
}

#[test]
fn a_directive_out_of_range_is_ignored() {
    let mut s = Session::default();
    assert!(out(&mut s, "⍝!ORIGIN 7").is_empty());
    assert!(out(&mut s, "⍝!WIDTH 5").is_empty());
    assert_eq!(s.ws.saved.env.io, 1);
    assert_eq!(s.ws.saved.print.width, 120);
}

#[test]
fn a_workspace_using_nothing_mode_specific_is_listed_in_both() {
    let dir = Dir::new("both");
    let mut b = session(&dir.0, Mode::B);
    out(&mut b, "X←42");
    out(&mut b, ")SAVE SHARED");
    let mut a = session(&dir.0, Mode::A);
    assert_eq!(out(&mut a, ")LIB"), vec!["SHARED"]);
    out(&mut a, ")LOAD SHARED");
    assert_eq!(out(&mut a, "X"), vec!["42"]);
}

#[test]
fn a_workspace_using_an_i_beam_is_68_only() {
    let dir = Dir::new("ibeam");
    let mut a = session(&dir.0, Mode::A);
    for line in ["∇R←T", "R←⌶20", "∇", ")SAVE CLOCK"] {
        out(&mut a, line);
    }
    let mut b = session(&dir.0, Mode::B);
    assert!(out(&mut b, ")LIB").is_empty());
    assert_eq!(out(&mut b, ")LOAD CLOCK"), vec!["WS NOT FOUND"]);
    assert_eq!(out(&mut a, ")LIB"), vec!["CLOCK"]);
}

#[test]
fn a_workspace_saved_before_modes_is_68_only() {
    let dir = Dir::new("old");
    let work = dir.0.join("work");
    std::fs::create_dir_all(&work).unwrap();
    let old = "⍝ sw-apl workspace. Re-executable APL: loading it runs it.\n\
               ⍝!SAVED 1.00.00 09/17/26\n⍝!LINK 16807\n)WSID OLD\n)ORIGIN 0\n\
               )DIGITS 10\n)WIDTH 120\nX←7\n";
    std::fs::write(work.join("OLD.apl.ws"), old).unwrap();
    let mut a = session(&dir.0, Mode::A);
    out(&mut a, ")LOAD OLD");
    assert_eq!(a.ws.saved.env.io, 0, "its )ORIGIN line still works in '70");
    assert_eq!(out(&mut a, "X"), vec!["7"]);
    let mut b = session(&dir.0, Mode::B);
    assert!(out(&mut b, ")LIB").is_empty());
}

#[test]
fn saving_in_75_under_a_68_only_name_makes_a_second_workspace() {
    let dir = Dir::new("twins");
    let mut a = session(&dir.0, Mode::A);
    for line in ["∇R←T", "R←⌶20", "∇", ")SAVE TWIN"] {
        out(&mut a, line);
    }
    let mut b = session(&dir.0, Mode::B);
    out(&mut b, "Y←75");
    let saved = out(&mut b, ")SAVE TWIN");
    assert!(saved[0].ends_with(" TWIN"), "{saved:?}");
    let mut a2 = session(&dir.0, Mode::A);
    out(&mut a2, ")LOAD TWIN");
    assert!(a2.ws.is_function("T"), "(A) still has its own TWIN");
    assert_eq!(out(&mut a2, ")LIB"), vec!["TWIN"]);
    let mut b2 = session(&dir.0, Mode::B);
    out(&mut b2, ")LOAD TWIN");
    assert_eq!(out(&mut b2, "Y"), vec!["75"]);
}

#[test]
fn dropping_a_shared_workspace_in_one_mode_leaves_the_other_its_copy() {
    let dir = Dir::new("drop");
    let mut a = session(&dir.0, Mode::A);
    out(&mut a, "X←1");
    out(&mut a, ")SAVE W");
    let mut b = session(&dir.0, Mode::B);
    assert_eq!(out(&mut b, ")DROP W").len(), 1, "the moment");
    assert!(out(&mut b, ")LIB").is_empty());
    assert_eq!(out(&mut a, ")LIB"), vec!["W"]);
    assert_eq!(out(&mut b, ")DROP W"), vec!["WS NOT FOUND"]);
}

#[test]
fn the_host_sets_the_mode_with_the_quota_and_the_store() {
    #[derive(Debug)]
    struct Mute;
    impl apl_session::Console for Mute {
        fn read(&mut self, _: &apl_session::Shown, _: &str) -> Option<String> {
            None
        }
    }
    let dir = Dir::new("host");
    let host = apl_session::Host {
        quota: 12_345,
        store: Box::new(Files(dir.0.clone())),
        mode: Mode::B,
    };
    let s = Session::attached(Box::new(Mute), host);
    assert_eq!((s.ws.quota, s.ws.mode), (12_345, Mode::B));
}

fn in_mode(mode: Mode) -> Session {
    let mut s = Session::default();
    s.ws.mode = mode;
    s
}

#[test]
fn an_i_beam_is_a_nonce_error_in_75() {
    let mut b = in_mode(Mode::B);
    let reply = b.respond("⌶25");
    assert!(reply.error, "{:?}", reply.lines);
    assert_eq!(reply.lines[0], "NONCE ERROR", "{:?}", reply.lines);
    let mut a = in_mode(Mode::A);
    assert!(!a.respond("⌶25").error, "(A) still has the I-beams");
}

#[test]
fn the_settings_commands_are_68_only() {
    let mut b = in_mode(Mode::B);
    for line in [")ORIGIN 0", ")DIGITS 3", ")WIDTH 80"] {
        assert_eq!(out(&mut b, line), vec!["INCORRECT COMMAND"], "{line}");
    }
    assert_eq!((b.ws.saved.env.io, b.ws.saved.print.digits), (1, 10));
    let mut a = in_mode(Mode::A);
    assert_eq!(out(&mut a, ")ORIGIN 0"), vec!["WAS 1"]);
}

#[test]
fn the_settings_directives_work_in_both_modes() {
    let mut b = in_mode(Mode::B);
    out(&mut b, "⍝!ORIGIN 0");
    assert_eq!(b.ws.saved.env.io, 0);
}

#[test]
fn the_group_commands_are_68_only() {
    let mut b = in_mode(Mode::B);
    out(&mut b, "A←1");
    for line in [")GROUP G A", ")GRP G", ")GRPS"] {
        assert_eq!(out(&mut b, line), vec!["INCORRECT COMMAND"], "{line}");
    }
    assert!(b.ws.saved.groups.is_empty());
    let mut a = in_mode(Mode::A);
    out(&mut a, "A←1");
    assert!(out(&mut a, ")GROUP G A").is_empty());
    assert_eq!(out(&mut a, ")GRP G"), vec!["A"]);
}
