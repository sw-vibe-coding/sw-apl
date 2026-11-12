//! The commands sw-apl adds, which no historical system had: they are
//! system commands, never quad names, so no program or saved
//! workspace can come to depend on them.

use apl_session::{Added, Files, Mode, Session, Source};

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
    assert_eq!(out(&mut in_mode(Mode::C), ")DIALECT"), vec!["(C) '80"]);
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
const SHARED: [&str; 19] = [
    "CLEAR", "WSID", "COPY", "PCOPY", "ERASE", "SAVE", "LOAD", "DROP", "LIB", "FNS", "VARS", "SI",
    "SIV", "SYMBOLS", "OFF", "CONTINUE", "DIALECT", "HELP", "LIBS",
];
const ONLY_A: [&str; 6] = ["ORIGIN", "DIGITS", "WIDTH", "GROUP", "GRP", "GRPS"];
const ONLY_C: [&str; 10] = [
    "DEF", "EDIT", "AUTO", "EXEC", "PS", "PD", "TRON", "TROFF", "RAM", "DOS",
];

fn commands(mode: Mode) -> Vec<&'static str> {
    let mut all = SHARED.to_vec();
    match mode {
        Mode::A => all.extend(ONLY_A),
        Mode::B => {}
        Mode::C => all.extend(ONLY_C),
    }
    all
}

#[test]
fn help_lists_the_commands_the_mode_has() {
    for mode in [Mode::A, Mode::B, Mode::C] {
        let shown = out(&mut in_mode(mode), ")HELP").join("\n");
        for name in commands(mode) {
            assert!(
                shown.contains(&format!("){name}")),
                "{mode:?}: )HELP lacks ){name}\n{shown}"
            );
        }
        if mode != Mode::A {
            assert!(
                !shown.contains(")ORIGIN"),
                "{mode:?} has no )ORIGIN\n{shown}"
            );
        }
    }
}

#[test]
fn every_command_has_a_page_that_names_it() {
    for mode in [Mode::A, Mode::B, Mode::C] {
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
    for mode in [Mode::A, Mode::B, Mode::C] {
        let mut s = in_mode(mode);
        let mut topics = out(&mut s, ")HELP TOPICS");
        topics.extend(out(&mut s, ")HELP"));
        for name in commands(mode).into_iter().chain([
            "TOPICS",
            "MODES",
            "EXTENSIONS",
            "RESTRICTIONS",
            "EDITOR",
        ]) {
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
        for (letter, mode) in [('A', Mode::A), ('B', Mode::B), ('C', Mode::C)] {
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

/// A session whose library 2 is the fixture standing in for a
/// repository of workspaces kept elsewhere.
fn with_extended(mode: Mode) -> Session {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../..");
    let dir = root.join("tests/libs/extended");
    let mut s = in_mode(mode);
    let place = dir.to_string_lossy().to_string();
    let store = Added::new(Box::new(Files(root.clone())));
    s.ws.store = Box::new(store.with(2, "EXTENDED", &place, Source::Dir(dir)));
    s
}

#[test]
fn libs_lists_the_libraries_a_session_reaches() {
    assert_eq!(
        out(&mut in_mode(Mode::A), ")LIBS"),
        vec!["0 USER", "1 CORE"]
    );
    let mut s = with_extended(Mode::B);
    assert_eq!(out(&mut s, ")LIBS"), vec!["0 USER", "1 CORE", "2 EXTENDED"]);
    assert_eq!(out(&mut s, ")LIBS 2"), vec!["INCORRECT COMMAND"]);
}

#[test]
fn a_configured_library_is_listed_and_loaded_by_mode() {
    let mut a = with_extended(Mode::A);
    assert_eq!(
        out(&mut a, ")LIB 2"),
        vec!["GREET"],
        "(B)'s own is not listed"
    );
    let mut b = with_extended(Mode::B);
    assert_eq!(out(&mut b, ")LIB 2"), vec!["GREET", "NEWER"]);
    assert!(out(&mut b, ")LOAD 2 GREET")[0].starts_with("SAVED "));
    assert_eq!(out(&mut b, "HELLO"), vec!["HELLO FROM LIBRARY 2"]);
    assert!(out(&mut b, ")LOAD 2 NEWER")[0].starts_with("SAVED "));
    assert_eq!(out(&mut b, "RUN"), vec!["5"]);
    assert_eq!(out(&mut a, ")LOAD 2 NEWER"), vec!["WS NOT FOUND"]);
}

#[test]
fn a_configured_library_is_read_only_and_others_are_not_there() {
    let mut s = with_extended(Mode::A);
    out(&mut s, ")LOAD 2 GREET");
    assert_eq!(out(&mut s, ")DROP 2 GREET"), vec!["INCORRECT COMMAND"]);
    let copied = out(&mut with_extended(Mode::A), ")COPY 2 GREET HELLO");
    assert!(copied[0].starts_with("SAVED "), "{copied:?}");
    assert_eq!(out(&mut s, ")LIB 3"), vec!["IMPROPER LIBRARY REFERENCE"]);
    assert_eq!(
        out(&mut s, ")LOAD 3 GREET"),
        vec!["IMPROPER LIBRARY REFERENCE"]
    );
}

#[test]
fn the_editor_has_a_page_in_each_mode() {
    for mode in [Mode::A, Mode::B, Mode::C] {
        let mut s = in_mode(mode);
        assert!(
            out(&mut s, ")HELP TOPICS")
                .iter()
                .any(|l| l.starts_with("EDITOR")),
            "{mode:?}: TOPICS lists EDITOR"
        );
        let page = out(&mut s, ")HELP EDITOR").join("\n");
        // (C) has APL80's line commands and no del.
        let forms = match mode {
            Mode::C => vec![")DEF", ")EDIT", ")?", ")P N", ")0", ")E N"],
            _ => vec!["[⎕]", "[⎕N]", "[N⎕]", "[∆N]", "[0]"],
        };
        for form in forms {
            assert!(page.contains(form), "{mode:?}: no {form}\n{page}");
        }
        // The slash line is APL\360's; the 5110 edits in place.
        assert_eq!(
            page.contains("CHANGE PART OF LINE N"),
            mode == Mode::A,
            "{mode:?}\n{page}"
        );
    }
}

/// `docs/modes.md` names every command a mode has, in its System
/// commands table, and says yes in a mode's column exactly when that
/// mode has a page for it: the comparison cannot drift from the code.
#[test]
fn the_modes_table_agrees_with_the_commands() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../..");
    let pages = std::fs::read_to_string(root.join("data/help.txt")).expect("data/help.txt");
    let table = std::fs::read_to_string(root.join("docs/modes.md")).expect("docs/modes.md");
    let section = table
        .split("## System commands")
        .nth(1)
        .expect("the section");
    let rows: Vec<&str> = section
        .split("\n## ")
        .next()
        .unwrap_or("")
        .lines()
        .collect();
    for head in pages.lines().filter_map(|l| l.strip_prefix("== ")) {
        let words: Vec<&str> = head.split_whitespace().collect();
        if words[2] == "TOPIC" {
            continue;
        }
        let name = format!("){}", words[0]);
        let row = rows
            .iter()
            .find(|r| r.split('`').any(|code| code.split(' ').any(|w| w == name)))
            .unwrap_or_else(|| panic!("{name} is not in the table"));
        let cells: Vec<&str> = row.split('|').map(str::trim).collect();
        for (i, letter) in ['A', 'B', 'C'].iter().enumerate() {
            let has = words[1].contains(*letter);
            assert_eq!(cells[2 + i] == "yes", has, "{name} in ({letter}): {row}");
        }
    }
}

/// A group the overview shows is a page of its own: its commands in
/// this mode, each with the line that says how it is typed.
#[test]
fn a_group_name_lists_its_commands() {
    for mode in [Mode::A, Mode::B, Mode::C] {
        let mut s = in_mode(mode);
        let overview = out(&mut s, ")HELP");
        for (group, heading) in [
            ("WORKSPACE", "WORKSPACE"),
            ("LIBRARY", "LIBRARY"),
            ("INQUIRY", "INQUIRY"),
            ("TERMINAL", "TERMINAL"),
            ("SW-APL", "SW-APL'S OWN"),
        ] {
            let page = out(&mut s, &format!(")HELP {group}"));
            assert_eq!(
                page[0],
                format!("{heading} COMMANDS IN {}", out(&mut s, ")DIALECT")[0])
            );
            // The group's line in the overview, and the lines it wraps
            // onto, which are indented under it.
            let at = overview
                .iter()
                .position(|l| l.starts_with(heading))
                .expect(heading);
            let wrapped = overview[at + 1..].iter().take_while(|l| l.starts_with(' '));
            let listed = std::iter::once(&overview[at]).chain(wrapped);
            let names: Vec<String> = listed
                .flat_map(|l| {
                    l.chars()
                        .skip(13)
                        .collect::<String>()
                        .split_whitespace()
                        .map(String::from)
                        .collect::<Vec<_>>()
                })
                .collect();
            assert!(!names.is_empty(), "{mode:?} {group}");
            for name in names {
                let usage = out(&mut s, &format!(")HELP {name}"))[0].clone();
                assert!(
                    page.contains(&usage),
                    "{mode:?} {group}: no {usage}\n{page:?}"
                );
            }
            assert!(page.iter().all(|l| l.chars().count() <= 64));
        }
        assert_eq!(
            out(&mut s, ")HELP workspace")[0],
            out(&mut s, ")HELP WORKSPACE")[0]
        );
    }
}
