//! (C) '80's commands and operators, after the APL80 manual: functions
//! defined by `)DEF` and changed by `)EDIT` and the line commands, with
//! no del; its workspaces its own, written as APL80 would type them;
//! `)TRON )TROFF )PS )PD )AUTO )EXEC`; the format statement; status,
//! `⌶` (APL80's `s`); and `#`, character and number (docs/mode-c.md).

use apl_session::{Files, Mode, Session};

fn in_mode(mode: Mode) -> Session {
    let mut s = Session::default();
    s.ws.mode = mode;
    s.respond(")CLEAR");
    s
}

fn out(s: &mut Session, line: &str) -> Vec<String> {
    s.respond(line).lines
}

/// Type each line, and give back what the last one showed.
fn typed(s: &mut Session, lines: &[&str]) -> Vec<String> {
    lines.iter().map(|l| out(s, l)).last().unwrap_or_default()
}

fn square(s: &mut Session) {
    typed(s, &[")DEF Z←SQ X", "Z←X×X", ""]);
}

#[test]
fn def_opens_a_function_and_an_empty_line_closes_it() {
    let mut c = in_mode(Mode::C);
    assert!(out(&mut c, ")DEF Z←SQ X").is_empty());
    assert_eq!(c.prompt(), "[1]   ", "the first line to type");
    assert!(out(&mut c, "Z←X×X").is_empty());
    assert_eq!(c.prompt(), "[2]   ");
    assert!(out(&mut c, "").is_empty(), "BREAK, typed as an empty line");
    assert_eq!(c.prompt(), "      ");
    assert_eq!(out(&mut c, "SQ 7"), vec!["49"]);
}

#[test]
fn def_will_not_take_a_name_in_use() {
    let mut c = in_mode(Mode::C);
    square(&mut c);
    assert_eq!(out(&mut c, ")DEF Z←SQ Y")[0], "DEFN ERROR");
    out(&mut c, "V←1");
    assert_eq!(out(&mut c, ")DEF V")[0], "DEFN ERROR");
    assert_eq!(c.prompt(), "      ", "and no definition is open");
}

#[test]
fn c_has_no_del() {
    let mut c = in_mode(Mode::C);
    assert_eq!(out(&mut c, "∇Z←F X")[0], "CHARACTER ERROR: U+2207");
    assert_eq!(c.prompt(), "      ");
    assert!(!c.ws.is_function("F"));
}

#[test]
fn edit_reopens_after_the_last_line_and_shows_with_a_query() {
    let mut c = in_mode(Mode::C);
    square(&mut c);
    assert!(out(&mut c, ")EDIT SQ").is_empty());
    assert_eq!(c.prompt(), "[2]   ");
    assert_eq!(out(&mut c, ")?"), vec!["[0]   Z←SQ X", "[1]   Z←X×X"]);
    out(&mut c, "Z←Z+1");
    out(&mut c, "");
    assert_eq!(out(&mut c, "SQ 3"), vec!["10"]);
    assert_eq!(out(&mut c, ")EDIT NOSUCH")[0], "DEFN ERROR");
}

#[test]
fn a_line_number_replaces_that_line_and_renumbers_at_once() {
    let mut c = in_mode(Mode::C);
    typed(&mut c, &[")DEF Z←F X", "Z←X", "Z←Z+1", "Z←Z×2", ""]);
    out(&mut c, ")EDIT F");
    assert!(
        out(&mut c, ")2").is_empty(),
        "line 2 is gone, and asked for again"
    );
    assert_eq!(c.prompt(), "[2]   ");
    out(&mut c, "Z←Z+10");
    assert_eq!(c.prompt(), "[4]   ", "back to the first unused line");
    out(&mut c, ")3");
    assert_eq!(
        out(&mut c, ")?"),
        vec!["[0]   Z←F X", "[1]   Z←X", "[2]   Z←Z+10"],
        "left without a replacement, line 3 is deleted and the rest renumbered"
    );
    out(&mut c, "");
    assert_eq!(out(&mut c, "F 1"), vec!["11"]);
}

#[test]
fn line_0_replaces_the_header_but_not_the_name() {
    let mut c = in_mode(Mode::C);
    square(&mut c);
    out(&mut c, ")EDIT SQ");
    out(&mut c, ")0");
    assert_eq!(c.prompt(), "[0]   ");
    assert_eq!(out(&mut c, "Z←CUBE X")[0], "DEFN ERROR", "the name stays");
    out(&mut c, ")0");
    out(&mut c, "Z←A SQ X");
    out(&mut c, "Z←A×X");
    out(&mut c, ")1");
    out(&mut c, "");
    assert_eq!(
        out(&mut c, "2 SQ 5"),
        vec!["10"],
        "line 1 deleted: line 2 is line 1"
    );
}

#[test]
fn p_shows_fourteen_lines_from_one() {
    let mut c = in_mode(Mode::C);
    out(&mut c, ")DEF F");
    for n in 1..=20 {
        out(&mut c, &format!("{n}"));
    }
    let page = out(&mut c, ")P3");
    assert_eq!(page.len(), 14);
    assert_eq!(page[0], "[3]   3");
    assert_eq!(page[13], "[16]  16");
    assert_eq!(out(&mut c, ")P 19").len(), 2, "the space is optional");
}

#[test]
fn e_offers_a_line_to_type_over() {
    let mut c = in_mode(Mode::C);
    square(&mut c);
    out(&mut c, ")EDIT SQ");
    assert!(out(&mut c, ")E1").is_empty());
    assert_eq!(c.prompt(), "[1]   ");
    assert_eq!(c.offer().as_deref(), Some("Z←X×X"));
    out(&mut c, "Z←X×X×X");
    assert_eq!(c.offer(), None);
    out(&mut c, "");
    assert_eq!(out(&mut c, "SQ 2"), vec!["8"]);
}

#[test]
fn a_system_command_runs_while_a_function_is_open() {
    let mut c = in_mode(Mode::C);
    square(&mut c);
    out(&mut c, ")DEF G");
    assert_eq!(out(&mut c, ")FNS"), vec!["SQ"]);
    assert_eq!(out(&mut c, ")SAVE W"), vec!["NOT WITH OPEN DEFINITION"]);
    out(&mut c, "");
}

#[test]
fn def_and_the_c_commands_are_cs_alone() {
    let mut a = in_mode(Mode::A);
    for command in [")DEF Z←F X", ")EDIT F", ")TRON", ")PS", ")AUTO 1", ")EXEC"] {
        assert_eq!(out(&mut a, command), vec!["INCORRECT COMMAND"], "{command}");
    }
    assert_eq!(out(&mut a, "#'A'")[0], "CHARACTER ERROR: U+0023");
}

/// A library directory of its own, removed when the test ends.
struct Dir(std::path::PathBuf);

impl Dir {
    fn new(tag: &str) -> Dir {
        let dir = std::env::temp_dir().join(format!("sw-apl-c80-{tag}-{}", std::process::id()));
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

fn session(dir: &std::path::Path, mode: Mode) -> Session {
    let mut s = in_mode(mode);
    s.ws.store = Box::new(Files(dir.to_path_buf()));
    s
}

#[test]
fn a_c_workspace_is_written_as_apl80_types_it_and_is_cs_alone() {
    let dir = Dir::new("save");
    let mut c = session(&dir.0, Mode::C);
    square(&mut c);
    typed(&mut c, &["N←5", ")AUTO SQ N"]);
    out(&mut c, ")SAVE W");
    let text = std::fs::read_to_string(dir.0.join("work/W.c.apl.ws")).expect("W.c.apl.ws");
    assert!(text.contains("⍝!MODES (C)\n"), "{text}");
    assert!(text.contains("\n)DEF Z←SQ X\nZ←X×X\n\n"), "{text}");
    assert!(text.contains("\n)AUTO SQ N\n"), "{text}");
    assert!(!text.contains('∇') && !text.contains("⎕LX"), "{text}");
    let mut again = session(&dir.0, Mode::C);
    let loaded = out(&mut again, ")LOAD W");
    assert_eq!(
        loaded.last().map(String::as_str),
        Some("25"),
        "the latent ran"
    );
    assert!(again.ws.is_function("SQ"));
    let mut a = session(&dir.0, Mode::A);
    assert!(out(&mut a, ")LIB").is_empty(), "(A) does not list it");
}

#[test]
fn auto_sets_the_latent_expression_and_exec_runs_it() {
    let mut c = in_mode(Mode::C);
    assert!(out(&mut c, ")AUTO 'HELLO'").is_empty());
    assert_eq!(out(&mut c, ")EXEC"), vec!["HELLO"]);
    assert!(out(&mut c, ")AUTO").is_empty(), "none");
    assert!(out(&mut c, ")EXEC").is_empty());
}

#[test]
fn tron_shows_each_line_before_it_runs() {
    let mut c = in_mode(Mode::C);
    typed(&mut c, &[")DEF Z←F X", "Z←X+1", "Z←Z×2", ""]);
    assert!(out(&mut c, ")TRON").is_empty());
    assert_eq!(out(&mut c, "F 1"), vec!["F[1]", "F[2]", "4"]);
    out(&mut c, ")TROFF");
    assert_eq!(out(&mut c, "F 1"), vec!["4"]);
}

#[test]
fn ps_and_pd_print_six_digits_or_fifteen() {
    let mut c = in_mode(Mode::C);
    assert_eq!(
        out(&mut c, "÷3"),
        vec!["0.333333"],
        "a clear workspace prints single"
    );
    assert!(out(&mut c, ")PD").is_empty());
    assert_eq!(out(&mut c, "÷3"), vec!["0.333333333333333"]);
    out(&mut c, ")PS");
    assert_eq!(out(&mut c, "÷3"), vec!["0.333333"]);
}

#[test]
fn the_format_statement_holds_until_characters_an_error_or_a_command() {
    let mut c = in_mode(Mode::C);
    assert!(out(&mut c, "⍕6 2").is_empty());
    assert_eq!(out(&mut c, "1.5 2"), vec!["  1.50  2.00"]);
    assert_eq!(
        out(&mut c, "÷4"),
        vec!["   .25"],
        "it holds, laid out as (B)'s format lays it"
    );
    out(&mut c, "'A'");
    assert_eq!(out(&mut c, "1.5"), vec!["1.5"], "character output ends it");
    out(&mut c, "⍕5 1");
    out(&mut c, "÷0");
    assert_eq!(out(&mut c, "1.5"), vec!["1.5"], "so does an error");
    out(&mut c, "⍕5 1");
    out(&mut c, ")FNS");
    assert_eq!(out(&mut c, "1.5"), vec!["1.5"], "and a command");
    assert_eq!(
        out(&mut c, "⍕6")[0],
        "LENGTH ERROR",
        "pairs of width and decimals"
    );
}

#[test]
fn status_reads_the_session() {
    let mut c = in_mode(Mode::C);
    assert_eq!(out(&mut c, "⌶1"), vec!["0"], "the read pointer: no files");
    assert_eq!(out(&mut c, "⌶4"), vec!["0"], "no function running");
    typed(&mut c, &[")DEF Z←F", "Z←0", "Z←⌶4", ""]);
    assert_eq!(out(&mut c, "F"), vec!["2"], "the line running");
    let free: f64 = out(&mut c, "⌶5")[0]
        .parse()
        .expect("a number, in six digits");
    assert!(free > 0.0);
    assert_eq!(out(&mut c, "⌶9")[0], "DOMAIN ERROR");
}

#[test]
fn hash_converts_through_apl80s_character_codes() {
    let mut c = in_mode(Mode::C);
    assert_eq!(out(&mut c, "#'ABC'"), vec!["65 66 67"]);
    assert_eq!(out(&mut c, "#65 66 67"), vec!["ABC"]);
    assert_eq!(
        out(&mut c, "#'⍴÷'"),
        vec!["112 37"],
        "the shifted P, and percent"
    );
    assert_eq!(out(&mut c, "#112 37"), vec!["⍴÷"]);
    assert_eq!(
        out(&mut c, "#'⍎'")[0],
        "DOMAIN ERROR",
        "APL80 has no such character"
    );
}
