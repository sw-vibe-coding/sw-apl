//! A locked function through a save and a load. The lock is the
//! del-tilde that closed the definition, and the file it is written
//! into must not hand the body to whoever opens it.

use apl_session::Session;

fn out(s: &mut Session, line: &str) -> Vec<String> {
    s.respond(line).lines
}

fn in_own_dir(name: &str) -> (Session, std::path::PathBuf, Guard) {
    let dir = std::env::temp_dir().join(format!("sw-apl-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    let mut session = Session::default();
    session.ws.libraries.clone_from(&dir);
    (session, dir.join("work"), Guard(dir))
}

struct Guard(std::path::PathBuf);

impl Drop for Guard {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).ok();
    }
}

/// Define SECRET locked, and PLAIN not.
fn define(s: &mut Session) {
    for line in [
        "\u{2207}R\u{2190}SECRET",
        "R\u{2190}42",
        "\u{236b}",
        "\u{2207}R\u{2190}PLAIN",
        "R\u{2190}7",
        "\u{2207}",
    ] {
        out(s, line);
    }
}

#[test]
fn a_workspace_with_a_locked_function_is_not_saved_as_readable_apl() {
    let (mut s, work, _g) = in_own_dir("locked");
    out(&mut s, ")WSID VAULT");
    define(&mut s);
    out(&mut s, ")SAVE");
    let text = std::fs::read_to_string(work.join("VAULT.apl.ws")).expect("saved");
    assert!(text.starts_with("\u{235d}!OBSCURED"), "{text}");
    assert!(
        !text.contains("R\u{2190}42"),
        "the locked body is not there"
    );
    assert!(!text.contains("SECRET"), "nor the name");
    // Nor is the unlocked one, because the whole file is obscured:
    // one locked function is enough.
    assert!(!text.contains("R\u{2190}7"));
}

#[test]
fn an_obscured_workspace_loads_and_the_lock_comes_back() {
    let (mut s, _work, _g) = in_own_dir("locked-load");
    out(&mut s, ")WSID VAULT");
    define(&mut s);
    out(&mut s, ")SAVE");
    out(&mut s, ")CLEAR");
    let loaded = out(&mut s, ")LOAD VAULT");
    assert_eq!(loaded.len(), 1);
    assert!(loaded[0].starts_with("SAVED "), "{loaded:?}");
    assert_eq!(out(&mut s, "SECRET"), vec!["42"], "it still runs");
    assert_eq!(out(&mut s, "PLAIN"), vec!["7"]);
    // Still locked: it cannot be reopened or displayed.
    assert_eq!(out(&mut s, "\u{2207}SECRET")[0], "DEFN ERROR");
    assert_eq!(
        out(&mut s, "\u{2207}SECRET[\u{2395}]\u{2207}")[0],
        "DEFN ERROR"
    );
    // The unlocked one is not locked by having travelled with it.
    let shown = out(&mut s, "\u{2207}PLAIN[\u{2395}]\u{2207}");
    assert!(shown[0].ends_with("\u{2207}R\u{2190}PLAIN"), "{shown:?}");
}

#[test]
fn a_workspace_that_gains_a_lock_changes_form_when_saved_again() {
    let (mut s, work, _g) = in_own_dir("locked-again");
    out(&mut s, ")WSID VAULT");
    for line in ["\u{2207}R\u{2190}PLAIN", "R\u{2190}7", "\u{2207}"] {
        out(&mut s, line);
    }
    out(&mut s, ")SAVE");
    let file = work.join("VAULT.apl.ws");
    let before = std::fs::read_to_string(&file).expect("saved");
    assert!(before.contains("R\u{2190}7"), "plain text so far");
    for line in ["\u{2207}R\u{2190}SECRET", "R\u{2190}42", "\u{236b}"] {
        out(&mut s, line);
    }
    out(&mut s, ")SAVE");
    let after = std::fs::read_to_string(&file).expect("saved again");
    assert!(after.starts_with("\u{235d}!OBSCURED"), "{after}");
    assert!(
        !after.contains("R\u{2190}7"),
        "and the old body went with it"
    );
}

#[test]
fn copying_out_of_an_obscured_workspace_works_and_keeps_the_lock() {
    let (mut s, _work, _g) = in_own_dir("locked-copy");
    out(&mut s, ")WSID VAULT");
    define(&mut s);
    out(&mut s, ")SAVE");
    out(&mut s, ")CLEAR");
    out(&mut s, ")COPY VAULT SECRET");
    assert_eq!(out(&mut s, "SECRET"), vec!["42"]);
    assert_eq!(
        out(&mut s, "\u{2207}SECRET")[0],
        "DEFN ERROR",
        "still locked"
    );
    assert_eq!(
        out(&mut s, ")FNS"),
        vec!["SECRET"],
        "and only what was asked"
    );
}
