//! What the on-screen board offers, run for real.
//!
//! The board in the browser has a Commands mode and an Idioms mode,
//! read from `data/board.json`. An idiom on it that is a SYNTAX ERROR
//! would teach the wrong thing, and a command on it that the session
//! does not have would answer INCORRECT COMMAND to a reader who only
//! tapped a button -- and nothing but a test would ever notice. So
//! every entry is run here, through a real session.

use std::fs;
use std::path::PathBuf;

use apl_session::{Files, Session};

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
fn session(tag: &str) -> Session {
    let dir = std::env::temp_dir().join(format!("apl-board-{tag}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    let lib1 = dir.join("ws/lib1");
    fs::create_dir_all(&lib1).expect("a scratch library");
    for entry in fs::read_dir(root().join("ws/lib1")).expect("ws/lib1") {
        let from = entry.expect("an entry").path();
        fs::copy(&from, lib1.join(from.file_name().expect("a name"))).expect("a copy");
    }
    let mut s = Session::default();
    s.ws.store = Box::new(Files(dir));
    s
}

fn entries(key: &str) -> Vec<serde_json::Value> {
    board()[key].as_array().expect("a list").clone()
}

#[test]
fn every_idiom_on_the_board_runs() {
    let mut s = session("idioms");
    for idiom in entries("idioms") {
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
    let mut s = session("commands");
    for command in entries("commands") {
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
