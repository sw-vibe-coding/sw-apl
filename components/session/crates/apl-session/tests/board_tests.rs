//! What the on-screen board offers, run for real.
//!
//! The board in the browser has a Commands mode and an Idioms mode,
//! and in (B) a CMD mode, read from `data/board.json`. An entry with
//! `modes` is offered only in those modes (`"A"`, `"B"`); one without
//! is offered in both. An idiom on it that is a SYNTAX ERROR
//! would teach the wrong thing, and a command on it that the session
//! does not have would answer INCORRECT COMMAND to a reader who only
//! tapped a button -- and nothing but a test would ever notice. So
//! every entry is run here, through a real session.

use std::fs;
use std::path::PathBuf;

use apl_session::{Files, Mode, Session};

/// The repository root: four levels up from this crate.
fn root() -> PathBuf {
    let here = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    here.join("../../../..").canonicalize().expect("the root")
}

/// The board's data, as the page reads it.
fn board() -> serde_json::Value {
    let text = fs::read_to_string(root().join("data/board.json")).expect("data/board.json");
    serde_json::from_str(&text).expect("data/board.json is JSON")
}

/// A session whose library 1 is the shipped one and whose library 0
/// is a scratch directory, so `)SAVE` and `)DROP` do not touch the
/// reader's own work.
fn session(tag: &str, mode: Mode) -> Session {
    let dir = std::env::temp_dir().join(format!("apl-board-{tag}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    let lib1 = dir.join("ws/lib1");
    fs::create_dir_all(&lib1).expect("a scratch library");
    for entry in fs::read_dir(root().join("ws/lib1")).expect("ws/lib1") {
        let from = entry.expect("an entry").path();
        fs::copy(&from, lib1.join(from.file_name().expect("a name"))).expect("a copy");
    }
    let mut s = Session::default();
    s.ws.mode = mode;
    s.respond(")CLEAR");
    s.ws.store = Box::new(Files(dir));
    s
}

fn entries(key: &str) -> Vec<serde_json::Value> {
    board()[key].as_array().expect("a list").clone()
}

/// The entries of `key` the board offers in `mode`.
fn offered(key: &str, mode: Mode) -> Vec<serde_json::Value> {
    let letter = if mode == Mode::A { "A" } else { "B" };
    let here = |e: &serde_json::Value| e["modes"].as_str().is_none_or(|m| m.contains(letter));
    entries(key).into_iter().filter(here).collect()
}

#[test]
fn every_idiom_on_the_board_runs() {
    for mode in [Mode::A, Mode::B] {
        idioms_run(mode);
    }
}

fn idioms_run(mode: Mode) {
    let mut s = session(&format!("idioms-{mode:?}"), mode);
    for idiom in offered("idioms", mode) {
        let line = idiom["insert"].as_str().expect("an idiom's text");
        let reply = s.respond(line).lines.join("\n");
        assert!(
            !reply.contains("ERROR"),
            "idiom {line:?} answered:\n{reply}"
        );
        assert!(!reply.trim().is_empty(), "idiom {line:?} printed nothing");
    }
}

#[test]
fn every_idiom_has_a_label() {
    for idiom in entries("idioms") {
        let label = idiom["label"].as_str().unwrap_or_default();
        assert!(!label.is_empty(), "an idiom has no label: {idiom}");
    }
}

#[test]
fn every_command_on_the_board_is_a_command_the_session_has() {
    for mode in [Mode::A, Mode::B] {
        commands_run(mode);
    }
}

fn commands_run(mode: Mode) {
    let mut s = session(&format!("commands-{mode:?}"), mode);
    for command in offered("commands", mode) {
        let insert = command["insert"].as_str().expect("a command's text");
        // One that takes an argument is inserted without it, so it is
        // tried with one; the rest are tried exactly as inserted.
        let tried = command["try"].as_str().unwrap_or(insert);
        let reply = s.respond(tried).lines.join("\n");
        assert!(
            !reply.contains("INCORRECT COMMAND"),
            "{tried:?} answered:\n{reply}"
        );
    }
}

#[test]
fn a_command_that_takes_an_argument_leaves_room_for_it() {
    // The board inserts `)LOAD ` and leaves the carriage after the
    // space; it never sends a command without the argument it needs.
    for command in entries("commands") {
        let insert = command["insert"].as_str().expect("a command's text");
        if command.get("try").is_some() {
            assert!(
                insert.ends_with(' '),
                "{insert:?} takes an argument but leaves no room for it"
            );
        }
    }
}

#[test]
fn every_cmd_key_on_the_75_board_runs() {
    // The IBM 5110's CMD key, with the letter keys, types what is
    // engraved on their fronts (5110 APL Reference Manual, Figure 3).
    let mut s = session("cmd", Mode::B);
    let cmd = entries("cmd");
    assert!(cmd.len() > 10, "{cmd:?}");
    for key in cmd {
        let label = key["label"].as_str().unwrap_or_default();
        assert!(label.starts_with("CMD "), "{key}");
        let tried = key["try"]
            .as_str()
            .or(key["insert"].as_str())
            .expect("a line");
        let reply = s.respond(tried).lines.join("\n");
        assert!(
            !reply.contains("ERROR"),
            "{label}: {tried:?} answered:\n{reply}"
        );
    }
}

#[test]
fn a_mode_is_offered_only_what_it_has() {
    let a: Vec<String> = offered("commands", Mode::A)
        .iter()
        .map(|e| e["insert"].to_string())
        .collect();
    let b: Vec<String> = offered("commands", Mode::B)
        .iter()
        .map(|e| e["insert"].to_string())
        .collect();
    assert!(a.iter().any(|c| c.contains(")ORIGIN")) && !b.iter().any(|c| c.contains(")ORIGIN")));
    assert!(b.iter().any(|c| c.contains(")LIBS")) && a.iter().any(|c| c.contains(")LIBS")));
    // One )LOAD that leaves room for a library and a name, not a
    // button for each workspace.
    assert!(!a.iter().chain(&b).any(|c| c.contains(")LOAD 1 ")));
}
