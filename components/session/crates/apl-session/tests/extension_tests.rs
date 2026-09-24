//! The commands sw-apl adds, which no historical system had: they are
//! system commands, never quad names, so no program or saved
//! workspace can come to depend on them.

use apl_session::{Mode, Session};

fn in_mode(mode: Mode) -> Session {
    let mut s = Session::default();
    s.ws.mode = mode;
    s.respond(")CLEAR");
    s
}

fn out(s: &mut Session, line: &str) -> Vec<String> {
    s.respond(line).lines
}

#[test]
fn dialect_names_the_mode() {
    assert_eq!(out(&mut in_mode(Mode::A), ")DIALECT"), vec!["(A) '70"]);
    assert_eq!(out(&mut in_mode(Mode::B), ")DIALECT"), vec!["(B) '75"]);
}

#[test]
fn dialect_is_cut_short_as_any_long_command_is() {
    assert_eq!(out(&mut in_mode(Mode::B), ")DIAL"), vec!["(B) '75"]);
    assert_eq!(out(&mut in_mode(Mode::B), ")DIALOGUE"), vec!["(B) '75"]);
}

#[test]
fn dialect_does_not_switch() {
    let mut b = in_mode(Mode::B);
    assert_eq!(out(&mut b, ")DIALECT 70"), vec!["INCORRECT COMMAND"]);
    assert_eq!(out(&mut b, ")DIALECT"), vec!["(B) '75"], "still (B)");
}

/// Every command each mode has, historical and sw-apl's own.
const SHARED: [&str; 18] = [
    "CLEAR", "WSID", "COPY", "PCOPY", "ERASE", "SAVE", "LOAD", "DROP", "LIB", "FNS", "VARS", "SI",
    "SIV", "SYMBOLS", "OFF", "CONTINUE", "DIALECT", "HELP",
];
const ONLY_A: [&str; 6] = ["ORIGIN", "DIGITS", "WIDTH", "GROUP", "GRP", "GRPS"];

fn commands(mode: Mode) -> Vec<&'static str> {
    let mut all = SHARED.to_vec();
    if mode == Mode::A {
        all.extend(ONLY_A);
    }
    all
}

#[test]
fn help_lists_the_commands_the_mode_has() {
    for mode in [Mode::A, Mode::B] {
        let shown = out(&mut in_mode(mode), ")HELP").join("\n");
        for name in commands(mode) {
            assert!(
                shown.contains(&format!("){name}")),
                "{mode:?}: )HELP lacks ){name}\n{shown}"
            );
        }
        if mode == Mode::B {
            assert!(!shown.contains(")ORIGIN"), "(B) has no )ORIGIN\n{shown}");
        }
    }
}

#[test]
fn every_command_has_a_page_that_names_it() {
    for mode in [Mode::A, Mode::B] {
        let mut s = in_mode(mode);
        for name in commands(mode) {
            let page = out(&mut s, &format!(")HELP {name}"));
            assert!(
                page[0].starts_with(&format!("){name}")),
                "{mode:?} {name}: {page:?}"
            );
        }
    }
}

#[test]
fn every_page_fits_the_75_screen() {
    for mode in [Mode::A, Mode::B] {
        let mut s = in_mode(mode);
        let mut topics = out(&mut s, ")HELP TOPICS");
        topics.extend(out(&mut s, ")HELP"));
        for name in
            commands(mode)
                .into_iter()
                .chain(["TOPICS", "MODES", "EXTENSIONS", "RESTRICTIONS"])
        {
            topics.extend(out(&mut s, &format!(")HELP {name}")));
        }
        for line in topics {
            assert!(line.chars().count() <= 64, "{mode:?}: {line:?}");
        }
    }
}

#[test]
fn help_says_what_a_mode_has_not() {
    let mut b = in_mode(Mode::B);
    let page = out(&mut b, ")HELP ORIGIN");
    assert!(page[0].contains("(B) '75 HAS NO )ORIGIN"), "{page:?}");
    let page = out(&mut b, ")HELP NOSUCH");
    assert!(page[0].starts_with("NO HELP FOR NOSUCH"), "{page:?}");
    assert!(page.iter().any(|l| l.contains("TOPICS")), "{page:?}");
}

#[test]
fn a_page_may_be_asked_for_with_its_parenthesis() {
    let mut a = in_mode(Mode::A);
    assert_eq!(out(&mut a, ")HELP )LOAD"), out(&mut a, ")HELP LOAD"));
}

#[test]
fn every_help_page_tries_a_command_its_modes_have() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../..");
    let pages = std::fs::read_to_string(root.join("data/help.txt")).expect("data/help.txt");
    for head in pages.lines().filter_map(|l| l.strip_prefix("== ")) {
        let words: Vec<&str> = head.split_whitespace().collect();
        let (modes, group) = (words[1], words[2]);
        let tried = words[3..].join(" ");
        if group == "TOPIC" || tried.is_empty() {
            continue;
        }
        for (letter, mode) in [('A', Mode::A), ('B', Mode::B)] {
            if modes.contains(letter) {
                let reply = out(&mut in_mode(mode), &tried).join("\n");
                assert!(
                    !reply.contains("INCORRECT COMMAND"),
                    "{mode:?} {head}: {reply}"
                );
            }
        }
    }
}
