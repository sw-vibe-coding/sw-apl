//! (C) '80's library 1: LIFE, written as APL80 types it, with its
//! functions as `)DEF` blocks and a latent greeting set by `)AUTO`.
//! The real file in `ws/lib1/` is what these read.

use std::path::PathBuf;

use apl_session::{Files, Mode, Session};

/// The repository root, which is the default library directory: four
/// levels up from this crate.
fn root() -> PathBuf {
    let here = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    here.join("../../../..").canonicalize().expect("the root")
}

fn out(s: &mut Session, line: &str) -> Vec<String> {
    s.respond(line).lines
}

fn in_c(library: PathBuf) -> Session {
    let mut session = Session::default();
    session.ws.mode = Mode::C;
    session.respond(")CLEAR");
    session.ws.store = Box::new(Files(library));
    session
}

#[test]
fn c_lists_its_own_life_in_library_1() {
    let mut c = in_c(root());
    assert_eq!(out(&mut c, ")LIB 1"), ["LIFE"]);
}

#[test]
fn life_loads_with_a_greeting_and_holds_what_it_describes() {
    let mut c = in_c(root());
    let reply = out(&mut c, ")LOAD 1 LIFE");
    assert!(reply[0].starts_with("SAVED "), "{reply:?}");
    let greeting = reply.last().expect("the greeting");
    assert!(greeting.contains("RUN 4"), "{reply:?}");
    assert!(greeting.chars().count() <= 64, "{greeting}");
    assert_eq!(out(&mut c, ")WSID"), ["LIFE"]);
    for name in ["DESCRIBE", "ROWS", "SUM", "GEN", "GLIDER", "RUN"] {
        assert!(c.ws.is_function(name), "LIFE does not hold {name}");
    }
    let described = out(&mut c, "DESCRIBE");
    assert!(described[0].starts_with("LIFE"), "{described:?}");
    for line in &described {
        assert!(line.chars().count() <= 64, "{line}");
    }
}

/// One generation moves the glider as Conway's rules say: five cells
/// still, the top row emptied, and the shape one step on.
#[test]
fn life_runs_a_generation_of_its_glider() {
    let mut c = in_c(root());
    out(&mut c, ")LOAD 1 LIFE");
    assert_eq!(out(&mut c, "+/+/G"), ["5"]);
    assert!(out(&mut c, "G←GEN G").is_empty());
    assert_eq!(out(&mut c, "+/+/G"), ["5"]);
    let top = out(&mut c, "4 3↑G");
    assert_eq!(top, ["0 0 0", "1 0 1", "0 1 1", "0 1 0"]);
    out(&mut c, "GLIDER");
    let shown = out(&mut c, "RUN 1");
    assert_eq!(shown.len(), 8, "{shown:?}");
    assert_eq!(shown[0].trim_end(), " *", "{shown:?}");
}

#[test]
fn life_is_exactly_what_save_writes() {
    let dir = std::env::temp_dir().join(format!("sw-apl-c80-lib1-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    let mut c = in_c(root());
    out(&mut c, ")LOAD 1 LIFE");
    c.ws.store = Box::new(Files(dir.clone()));
    out(&mut c, ")SAVE");
    let written = std::fs::read_to_string(dir.join("work/LIFE.c.apl.ws"));
    let shipped = std::fs::read_to_string(root().join("ws/lib1/LIFE.c.apl.ws"));
    std::fs::remove_dir_all(&dir).ok();
    let body = |text: String| -> Vec<String> {
        text.lines()
            .filter(|l| !l.starts_with("\u{235d}!SAVED ") && !l.starts_with("\u{235d}!SOURCE "))
            .map(String::from)
            .collect()
    };
    assert_eq!(
        body(written.expect("written")),
        body(shipped.expect("shipped"))
    );
}
