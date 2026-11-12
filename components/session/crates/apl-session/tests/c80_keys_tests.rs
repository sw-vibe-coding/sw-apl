//! (C) '80 typed and shown in APL80's characters (docs/mode-c.md, The
//! keys): what a session takes and what it shows back.

use std::collections::VecDeque;

use apl_session::{Console, Files, Mode, Session, Shown};

fn in_c() -> Session {
    let mut s = Session::default();
    s.ws.mode = Mode::C;
    s.respond(")CLEAR");
    s
}

fn out(s: &mut Session, line: &str) -> Vec<String> {
    s.respond(line).lines
}

#[test]
fn arithmetic_with_the_substitutes_and_the_minus_signs() {
    let mut c = in_c();
    assert_eq!(out(&mut c, "6%4"), vec!["1.5"]);
    assert_eq!(
        out(&mut c, "3_5"),
        vec!["-2"],
        "underscore subtracts; minus is the sign"
    );
    assert_eq!(out(&mut c, "-3+1"), vec!["-2"]);
    assert_eq!(out(&mut c, "2x3"), vec!["6"], "the shifted X times");
    assert_eq!(out(&mut c, "i5"), vec!["1 2 3 4 5"]);
    assert_eq!(out(&mut c, "2 3pi6"), vec!["1 2 3", "4 5 6"]);
    assert_eq!(out(&mut c, "3$4"), vec!["1"]);
    assert_eq!(out(&mut c, "1 2\".+10 20"), vec!["11 21", "12 22"]);
}

#[test]
fn names_round_brackets_and_arrows() {
    let mut c = in_c();
    assert!(
        out(&mut c, "M-N]10 20 30").is_empty(),
        "a name with a minus, and the left arrow's code"
    );
    assert_eq!(out(&mut c, "M-N(2)"), vec!["20"], "round brackets index");
    assert!(out(&mut c, "M-N(1)←5").is_empty());
    assert_eq!(out(&mut c, "M-N"), vec!["5 20 30"]);
    assert_eq!(
        out(&mut c, "↑3 1 2"),
        vec!["2 3 1"],
        "monadic up arrow grades"
    );
    assert_eq!(out(&mut c, "2↑3 1 2"), vec!["3 1"], "dyadic takes");
    assert_eq!(out(&mut c, "A.B←1"), Vec::<String>::new());
    assert_eq!(
        out(&mut c, ")VARS"),
        vec!["A.B M-N"],
        "names are shown as typed"
    );
    out(&mut c, ")ERASE A.B");
    assert_eq!(out(&mut c, ")VARS"), vec!["M-N"], "and found as typed");
}

#[test]
fn the_axis_is_chosen_with_d() {
    let mut c = in_c();
    out(&mut c, "M←2 3pi6");
    assert_eq!(out(&mut c, "+/1dM"), vec!["5 7 9"]);
    assert_eq!(out(&mut c, "+/M"), vec!["6 15"]);
}

#[test]
fn what_is_shown_is_in_apl80s_characters() {
    let mut c = in_c();
    assert_eq!(out(&mut c, "-1.5 2"), vec!["-1.5 2"]);
    assert_eq!(
        out(&mut c, "'A%Bp'"),
        vec!["A%Bp"],
        "a string comes back as typed"
    );
    assert_eq!(
        out(&mut c, "#'p'"),
        vec!["112"],
        "and holds the shifted letter's code"
    );
    assert_eq!(out(&mut c, "1E-9"), vec!["1E-9"]);
}

#[test]
fn an_apl_glyph_is_not_one_of_apl80s_characters() {
    let mut c = in_c();
    let shown = out(&mut c, "2+⍴3");
    assert_eq!(shown[0], "CHARACTER ERROR: U+2374");
    assert_eq!(shown[1], "      2+⍴3", "the line as typed");
    assert_eq!(shown[2], "        ^", "and the caret under the glyph");
}

#[test]
fn an_error_points_at_what_was_typed() {
    let mut c = in_c();
    let shown = out(&mut c, "+/1dM");
    assert_eq!(shown[0], "VALUE ERROR");
    assert_eq!(shown[1], "      +/1dM");
    assert_eq!(shown[2], "          ^", "under M, past the axis");
}

#[test]
fn a_function_is_called_with_round_brackets_and_listed_as_typed() {
    let mut c = in_c();
    for line in [")DEF Z]SQ X", "Z]XxX", ""] {
        out(&mut c, line);
    }
    assert_eq!(
        out(&mut c, "SQ(3)"),
        vec!["9"],
        "a function's argument, not an index"
    );
    out(&mut c, ")EDIT SQ");
    assert_eq!(out(&mut c, ")?"), vec!["[0]   Z←SQ X", "[1]   Z]XxX"]);
    out(&mut c, "");
}

/// A console answering reads from lines typed in advance, and keeping
/// everything it was shown, prompts and answers included.
#[derive(Debug, Default)]
struct Typist {
    typed: VecDeque<String>,
    shown: Vec<String>,
}

impl Console for Typist {
    fn read(&mut self, shown: &Shown, prompt: &str) -> Option<String> {
        self.shown.extend(shown.lines.iter().cloned());
        if !prompt.is_empty() {
            self.shown.push(prompt.to_string());
        }
        let typed = self.typed.pop_front()?;
        self.shown.push(format!("      {typed}"));
        Some(typed)
    }

    fn take(&mut self) -> Vec<String> {
        std::mem::take(&mut self.shown)
    }
}

#[test]
fn quote_quad_takes_apl80s_characters() {
    let mut c = in_c();
    c.ws.console = Box::new(Typist {
        typed: ["p%".to_string()].into(),
        ..Typist::default()
    });
    assert_eq!(out(&mut c, "#m"), vec!["      p%", "112 37"]);
}

#[test]
fn quad_prompts_with_the_shifted_q() {
    let mut c = in_c();
    c.ws.console = Box::new(Typist {
        typed: ["2x3".to_string()].into(),
        ..Typist::default()
    });
    assert_eq!(out(&mut c, "1+q"), vec!["q:", "      2x3", "7"]);
}

/// A library directory of its own, removed when the test ends.
struct Dir(std::path::PathBuf);

impl Drop for Dir {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).ok();
    }
}

#[test]
fn a_workspace_file_is_in_apl80s_characters() {
    let dir = std::env::temp_dir().join(format!("sw-apl-c80-keys-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("mkdir");
    let dir = Dir(dir);
    let mut c = in_c();
    c.ws.store = Box::new(Files(dir.0.clone()));
    for line in [
        "M-N]-1.5 2",
        "S]'p%'",
        ")DEF Z]A.B X",
        "Z]X(1)_1",
        "",
        ")SAVE W",
    ] {
        out(&mut c, line);
    }
    let text = std::fs::read_to_string(dir.0.join("work/W.c.apl.ws")).expect("saved");
    assert!(text.contains("\nM-N←-1.5 2\n"), "{text}");
    assert!(text.contains("\nS←'p%'\n"), "{text}");
    assert!(text.contains("\n)DEF Z←A.B X\nZ]X(1)_1\n\n"), "{text}");
    let mut again = in_c();
    again.ws.store = Box::new(Files(dir.0.clone()));
    out(&mut again, ")LOAD W");
    assert_eq!(out(&mut again, "A.B M-N"), vec!["-2.5"]);
    assert_eq!(out(&mut again, "S"), vec!["p%"]);
}

#[test]
fn hash_sends_the_screen_codes_as_terminal_codes() {
    let mut c = in_c();
    assert_eq!(
        out(&mut c, "#28 31"),
        vec!["\x1b[H\x1b[J"],
        "home, and clear the frame"
    );
    assert_eq!(
        out(&mut c, "(#-130),'HI'"),
        vec!["\x1b[3;3HHI"],
        "PRINT @ 130"
    );
    assert_eq!(
        out(&mut c, "#129 191"),
        vec!["\u{1fb00}█"],
        "the graphics blocks"
    );
    assert_eq!(
        out(&mut c, "#'A',#129"),
        vec!["65 129"],
        "and back to their codes"
    );
    assert_eq!(
        out(&mut c, "p,#200"),
        vec!["1"],
        "space compression is one character"
    );
}

#[test]
fn hash_obeys_the_codes_apl80_gives_actions() {
    let mut c = in_c();
    assert_eq!(
        out(&mut c, "p#5 9"),
        vec!["0"],
        "no printer: nothing, and no character"
    );
    let link = c.ws.saved.env.link;
    out(&mut c, "?100");
    assert_ne!(c.ws.saved.env.link, link);
    out(&mut c, "#11");
    assert_eq!(c.ws.saved.env.link, link, "11 starts the random link again");
    assert_eq!(out(&mut c, "#3")[0], "NONCE ERROR", "zone printing");
    assert_eq!(out(&mut c, "#16")[0], "NONCE ERROR", "stopping the clock");
    assert_eq!(out(&mut c, "#300")[0], "DOMAIN ERROR");
}
