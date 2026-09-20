//! The workspaces sw-apl ships in library 1. They are files in the
//! repository rather than anything the code builds, so these tests
//! read the real ones: a shipped workspace that does not load, or
//! whose DESCRIBE promises a name it does not hold, is a broken
//! thing to hand somebody.

use std::path::PathBuf;

use apl_session::{Files, Session};

/// What sw-apl ships, and the names each DESCRIBE promises.
const SHIPPED: [(&str, &[&str]); 3] = [
    ("LIFE", &["DESCRIBE", "HOWRUN", "GEN", "RUN", "GLIDER"]),
    ("RACE", &["DESCRIBE", "HOWRACE", "SHOW", "RACE"]),
    ("EDIT", &["DESCRIBE", "HOWEDIT", "FACT", "MEAN"]),
];

/// The repository root, which is the default library directory: four
/// levels up from this crate.
fn root() -> PathBuf {
    let here = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    here.join("../../../..").canonicalize().expect("the root")
}

fn out(s: &mut Session, line: &str) -> Vec<String> {
    s.respond(line).lines
}

fn loaded(name: &str) -> Session {
    let mut session = Session::default();
    session.ws.store = Box::new(Files(root()));
    let reply = out(&mut session, &format!(")LOAD 1 {name}"));
    assert_eq!(reply.len(), 1, "{name}: {reply:?}");
    assert!(reply[0].starts_with("SAVED "), "{name}: {reply:?}");
    session
}

#[test]
fn the_shipped_workspaces_are_what_lib_1_lists() {
    let mut session = Session::default();
    session.ws.store = Box::new(Files(root()));
    let mut names: Vec<&str> = SHIPPED.iter().map(|(n, _)| *n).collect();
    names.sort_unstable();
    assert_eq!(out(&mut session, ")LIB 1"), names);
}

#[test]
fn each_shipped_workspace_loads_and_holds_what_it_promises() {
    for (name, held) in SHIPPED {
        let mut session = loaded(name);
        assert_eq!(out(&mut session, ")WSID"), vec![name]);
        for function in held {
            assert!(
                session.ws.is_function(function),
                "{name} does not hold {function}"
            );
        }
        // DESCRIBE is a niladic function, and saying something is
        // the whole of its job.
        let described = out(&mut session, "DESCRIBE");
        assert!(described.len() > 3, "{name}: {described:?}");
        assert!(described[0].starts_with(name), "{name}: {described:?}");
    }
}

#[test]
fn a_shipped_workspace_is_exactly_what_save_writes() {
    let dir = std::env::temp_dir().join(format!("sw-apl-lib1-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    for (name, _) in SHIPPED {
        let mut session = loaded(name);
        session.ws.store = Box::new(Files(dir.clone()));
        out(&mut session, ")SAVE");
        let written = std::fs::read_to_string(dir.join(format!("work/{name}.apl.ws")));
        let shipped = std::fs::read_to_string(root().join(format!("ws/lib1/{name}.apl.ws")));
        // The moment differs, and the provenance line is added by
        // hand: `)SAVE` does not write it, because a mark a program
        // stamps on everything asserts nothing.
        assert_eq!(
            body(&written.expect("the workspace was written")),
            body(&shipped.expect("the shipped workspace")),
            "{name} does not round-trip"
        );
    }
    std::fs::remove_dir_all(&dir).ok();
}

/// A workspace file without the lines that cannot match: when it was
/// saved, and who wrote it.
fn body(text: &str) -> String {
    text.lines()
        .filter(|l| !l.starts_with("\u{235d}!SAVED ") && !l.starts_with("\u{235d}!SOURCE "))
        .collect::<Vec<&str>>()
        .join("\n")
}

#[test]
fn the_practice_function_is_wrong_in_the_way_its_describe_says() {
    let mut session = loaded("EDIT");
    assert_eq!(out(&mut session, "FACT 5"), vec!["24"], "wrong by one");
    assert_eq!(
        out(&mut session, "MEAN 1 2 3 4"),
        vec!["2.5"],
        "this one is not"
    );
    // Fixing it is one line, which is the exercise.
    for line in [
        "\u{2207}FACT",
        "[5] \u{2192}LOOP\u{d7}\u{2373}I\u{2264}N",
        "\u{2207}",
    ] {
        out(&mut session, line);
    }
    assert_eq!(out(&mut session, "FACT 5"), vec!["120"]);
}
