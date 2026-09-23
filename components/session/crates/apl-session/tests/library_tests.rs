//! The workspaces sw-apl ships in library 1. They are files in the
//! repository rather than anything the code builds, so these tests
//! read the real ones: a shipped workspace that does not load, or
//! whose DESCRIBE promises a name it does not hold, is a broken
//! thing to hand somebody.

use std::path::PathBuf;

use apl_session::{Files, Mode, Session};

/// What sw-apl ships: the name, the file, the mode it is read in, and
/// the names its DESCRIBE promises. A workspace that differs between
/// the modes has a file for each, named for its mode.
const SHIPPED: [(&str, &str, Mode, &[&str]); 6] = [
    (
        "LIFE",
        "LIFE",
        Mode::A,
        &["DESCRIBE", "HOWRUN", "GEN", "RUN", "GLIDER"],
    ),
    (
        "RACE",
        "RACE",
        Mode::A,
        &["DESCRIBE", "HOWRACE", "SHOW", "RACE"],
    ),
    (
        "EDIT",
        "EDIT",
        Mode::A,
        &["DESCRIBE", "HOWEDIT", "FACT", "MEAN"],
    ),
    (
        "BIRDS",
        "BIRDS.a-70",
        Mode::A,
        &[
            "DESCRIBE", "HOWBIRDS", "NOTHERE", "I", "K", "KI", "T", "B", "C", "W", "S", "APPLY",
            "DYAD", "FACT",
        ],
    ),
    (
        "BIRDS",
        "BIRDS.b-75",
        Mode::B,
        &[
            "DESCRIBE", "HOWBIRDS", "NOTHERE", "I", "K", "KI", "T", "B", "C", "W", "S", "M", "Y",
            "FSTEP", "SHOUT", "HYP", "FACT",
        ],
    ),
    (
        "TTTML",
        "TTTML.b-75",
        Mode::B,
        &[
            "DESCRIBE", "PLAY", "TRIAL", "TRAIN", "SHOW", "CHOOSE", "GAME", "LEARN",
        ],
    ),
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

fn in_mode(mode: Mode) -> Session {
    let mut session = Session::default();
    session.ws.mode = mode;
    session.respond(")CLEAR");
    session.ws.store = Box::new(Files(root()));
    session
}

fn loaded(name: &str, mode: Mode) -> Session {
    let mut session = in_mode(mode);
    let reply = out(&mut session, &format!(")LOAD 1 {name}"));
    assert_eq!(reply.len(), 1, "{name}: {reply:?}");
    assert!(reply[0].starts_with("SAVED "), "{name}: {reply:?}");
    session
}

#[test]
fn each_mode_lists_the_shipped_workspaces_that_run_in_it() {
    let mut a = in_mode(Mode::A);
    assert_eq!(out(&mut a, ")LIB 1"), ["BIRDS", "EDIT", "LIFE", "RACE"]);
    let mut b = in_mode(Mode::B);
    assert_eq!(
        out(&mut b, ")LIB 1"),
        ["BIRDS", "EDIT", "LIFE", "RACE", "TTTML"]
    );
}

#[test]
fn tttml_comes_trained_and_plays_to_win() {
    let mut b = loaded("TTTML", Mode::B);
    let known: usize = out(&mut b, "⍴KEYS")[0].parse().expect("a count");
    assert!(known > 700, "trained: {known} positions");
    // A win on the spot is taken, and a threat is blocked.
    assert_eq!(out(&mut b, "CHOOSE 1 ¯1 0 ¯1 1 0 0 0 0"), vec!["9"]);
    assert_eq!(out(&mut b, "CHOOSE 1 1 0 ¯1 0 0 0 0 0"), vec!["3"]);
    // Against a random player it does not lose, as X or as O.
    let rows = out(&mut b, "TRIAL 50");
    for row in &rows {
        let lost: i64 = row
            .split_whitespace()
            .nth(1)
            .expect("lost")
            .parse()
            .expect("n");
        assert_eq!(lost, 0, "{rows:?}");
    }
}

#[test]
fn each_shipped_workspace_loads_and_holds_what_it_promises() {
    for (name, file, mode, held) in SHIPPED {
        let mut session = loaded(name, mode);
        assert_eq!(out(&mut session, ")WSID"), vec![name]);
        for function in held {
            assert!(
                session.ws.is_function(function),
                "{file} does not hold {function}"
            );
        }
        // DESCRIBE is a niladic function, and saying something is
        // the whole of its job.
        let described = out(&mut session, "DESCRIBE");
        assert!(described.len() > 3, "{file}: {described:?}");
        assert!(described[0].starts_with(name), "{file}: {described:?}");
    }
}

#[test]
fn each_mode_loads_its_own_birds() {
    let mut a = loaded("BIRDS", Mode::A);
    assert!(a.ws.is_function("APPLY") && !a.ws.is_function("M"));
    let mut b = loaded("BIRDS", Mode::B);
    assert_eq!(out(&mut b, "M 'SHOUT'"), vec!["SHOUT!"], "the Mockingbird");
    assert_eq!(out(&mut b, "'FSTEP' Y 6"), vec!["720"], "the Sage");
    assert_eq!(out(&mut b, "7 KI 9"), vec!["9"]);
    assert_eq!(out(&mut a, "'+⌽' S ⍳5"), vec!["6 6 6 6 6"]);
    assert_eq!(out(&mut b, "'+ ⌽' S ⍳5"), vec!["6 6 6 6 6"]);
}

#[test]
fn a_shipped_workspace_is_exactly_what_save_writes() {
    let dir = std::env::temp_dir().join(format!("sw-apl-lib1-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    for (name, file, mode, _) in SHIPPED {
        let mut session = loaded(name, mode);
        session.ws.store = Box::new(Files(dir.clone()));
        out(&mut session, ")SAVE");
        let written = std::fs::read_dir(dir.join("work"))
            .expect("library 0")
            .filter_map(Result::ok)
            .find(|e| {
                e.file_name()
                    .to_string_lossy()
                    .starts_with(&format!("{name}."))
            })
            .map(|e| std::fs::read_to_string(e.path()));
        let shipped = std::fs::read_to_string(root().join(format!("ws/lib1/{file}.apl.ws")));
        // The moment differs, and the provenance line is added by
        // hand: `)SAVE` does not write it, because a mark a program
        // stamps on everything asserts nothing. The modes line
        // differs for a pair: saved alone, (A)'s BIRDS runs in both.
        assert_eq!(
            body(
                &written
                    .expect("the workspace was written")
                    .expect("readable")
            ),
            body(&shipped.expect("the shipped workspace")),
            "{file} does not round-trip"
        );
        std::fs::remove_dir_all(dir.join("work")).ok();
    }
    std::fs::remove_dir_all(&dir).ok();
}

/// A workspace file without the lines that cannot match: when it was
/// saved, and who wrote it.
fn body(text: &str) -> String {
    text.lines()
        .filter(|l| {
            !["\u{235d}!SAVED ", "\u{235d}!SOURCE ", "\u{235d}!MODES "]
                .iter()
                .any(|p| l.starts_with(p))
        })
        .collect::<Vec<&str>>()
        .join("\n")
}

#[test]
fn the_practice_function_is_wrong_in_the_way_its_describe_says() {
    let mut session = loaded("EDIT", Mode::A);
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
