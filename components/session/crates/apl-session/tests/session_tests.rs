//! The session turns one input line into transcript lines.

use apl_session::{Reply, Session};

fn out(s: &mut Session, line: &str) -> Vec<String> {
    match s.respond(line) {
        Reply::Output(v) => v,
        Reply::Off => panic!("unexpected )OFF"),
    }
}

#[test]
fn evaluates_and_displays() {
    let mut s = Session::default();
    assert_eq!(out(&mut s, "3 + 4"), vec!["7"]);
    assert_eq!(out(&mut s, "1 2 3 + 10 20 30"), vec!["11 22 33"]);
    assert_eq!(out(&mut s, "20 \u{f7} 8"), vec!["2.5"]);
}

#[test]
fn assignment_is_silent_and_remembered() {
    let mut s = Session::default();
    assert_eq!(out(&mut s, "A \u{2190} 5"), Vec::<String>::new());
    assert_eq!(out(&mut s, "A + 3"), vec!["8"]);
}

#[test]
fn blank_and_comment_lines_print_nothing() {
    let mut s = Session::default();
    assert_eq!(out(&mut s, ""), Vec::<String>::new());
    assert_eq!(out(&mut s, "\u{235d} hello"), Vec::<String>::new());
}

#[test]
fn errors_print_name_statement_and_caret() {
    let mut s = Session::default();
    assert_eq!(
        out(&mut s, "2 3+4 5 6"),
        vec!["LENGTH ERROR", "      2 3+4 5 6", "         ^"]
    );
    assert_eq!(
        out(&mut s, "1+XYZ"),
        vec!["VALUE ERROR", "      1+XYZ", "        ^"]
    );
    assert_eq!(
        out(&mut s, "\u{3c1}5"),
        vec![
            "CHARACTER ERROR: U+03C1 (use \u{2374} U+2374)",
            "      \u{3c1}5",
            "      ^"
        ]
    );
}

#[test]
fn system_commands() {
    let mut s = Session::default();
    assert_eq!(s.respond(")OFF"), Reply::Off);
    assert_eq!(s.respond("  )off"), Reply::Off);
    assert_eq!(out(&mut s, ")FOO"), vec!["INCORRECT COMMAND"]);
}

#[test]
fn mvp_transcript() {
    let mut s = Session::default();
    assert_eq!(out(&mut s, "+/\u{2373}10"), vec!["55"]);
    assert_eq!(out(&mut s, "2 3\u{2374}\u{2373}6"), vec!["1 2 3", "4 5 6"]);
    assert_eq!(out(&mut s, "\u{2374}42"), vec![""]);
    assert_eq!(out(&mut s, "\u{2395}\u{2190}\u{2373}3"), vec!["1 2 3"]);
    assert_eq!(out(&mut s, ")ORIGIN 0"), vec!["WAS 1"]);
    assert_eq!(out(&mut s, "\u{2373}3"), vec!["0 1 2"]);
    assert_eq!(out(&mut s, ")origin 1"), vec!["WAS 0"]);
    assert_eq!(out(&mut s, "1+\u{2395}\u{2190}5"), vec!["5", "6"]);
}

#[test]
fn quad_output_before_an_error_still_prints() {
    let mut s = Session::default();
    let lines = out(&mut s, "XYZ+\u{2395}\u{2190}5");
    assert_eq!(lines[0], "5");
    assert_eq!(lines[1], "VALUE ERROR");
}

#[test]
fn settings_commands_reply_was_and_validate() {
    let mut s = Session::default();
    assert_eq!(out(&mut s, ")DIGITS 3"), vec!["WAS 10"]);
    assert_eq!(out(&mut s, "\u{f7}3"), vec!["0.333"]);
    assert_eq!(out(&mut s, ")DIGITS 10"), vec!["WAS 3"]);
    assert_eq!(out(&mut s, ")WIDTH 80"), vec!["WAS 120"]);
    for bad in [
        ")ORIGIN 2",
        ")ORIGIN",
        ")DIGITS 0",
        ")DIGITS 17",
        ")WIDTH 10",
        ")OFF NOW",
    ] {
        assert_eq!(out(&mut s, bad), vec!["INCORRECT COMMAND"], "{bad}");
    }
}

#[test]
fn quad_names_are_syntax_errors() {
    let mut s = Session::default();
    assert_eq!(
        out(&mut s, "\u{2395}IO\u{2190}0"),
        vec!["SYNTAX ERROR", "      \u{2395}IO\u{2190}0", "      ^"]
    );
}

#[test]
fn character_data_end_to_end() {
    let mut s = Session::default();
    assert_eq!(out(&mut s, "'HELLO'"), vec!["HELLO"]);
    assert_eq!(out(&mut s, "'it''s'"), vec!["it's"]);
    assert_eq!(out(&mut s, "\u{2374}'HELLO'"), vec!["5"]);
    assert_eq!(out(&mut s, "'AB','CD'"), vec!["ABCD"]);
    assert_eq!(out(&mut s, "3\u{2374}'ab'"), vec!["aba"]);
    assert_eq!(out(&mut s, "''"), vec![""]);
    assert_eq!(out(&mut s, "'A'+1")[0], "DOMAIN ERROR");
    assert_eq!(out(&mut s, "'AB',1")[0], "DOMAIN ERROR");
    assert_eq!(out(&mut s, "\u{2395}\u{2190}'hi'"), vec!["hi"]);
}

#[test]
fn lookalike_glyph_error_carries_the_hint() {
    let mut s = Session::default();
    assert_eq!(
        out(&mut s, "\u{3c1}1 2")[0],
        "CHARACTER ERROR: U+03C1 (use \u{2374} U+2374)"
    );
}

#[test]
fn mixed_output_prints_parts_side_by_side() {
    let mut s = Session::default();
    assert_eq!(out(&mut s, "'X IS ';2+3"), vec!["X IS 5"]);
    assert_eq!(out(&mut s, "'A';1 2 3;'B'"), vec!["A1 2 3B"]);
    assert_eq!(out(&mut s, "'ROUND ';3;' ---'"), vec!["ROUND 3 ---"]);
    assert_eq!(
        out(&mut s, "'M';2 2\u{2374}\u{2373}4"),
        vec!["M", "1 2", "3 4"]
    );
}

#[test]
fn roll_is_reproducible_from_a_clear_workspace() {
    let mut a = Session::default();
    let mut b = Session::default();
    let x = out(&mut a, "?6 6 6 6 6 6 6 6");
    assert_eq!(x, out(&mut b, "?6 6 6 6 6 6 6 6"));
    assert_ne!(x, out(&mut a, "?6 6 6 6 6 6 6 6"), "the link advances");
}

#[test]
fn catenate_with_axis_and_laminate_end_to_end() {
    let mut s = Session::default();
    assert_eq!(
        out(&mut s, "A\u{2190}2 2\u{2374}\u{2373}4"),
        Vec::<String>::new()
    );
    assert_eq!(out(&mut s, "A,A"), vec!["1 2 1 2", "3 4 3 4"]);
    assert_eq!(out(&mut s, "A,[1]A"), vec!["1 2", "3 4", "1 2", "3 4"]);
    assert_eq!(out(&mut s, "A,9 8"), vec!["1 2 9", "3 4 8"]);
    assert_eq!(out(&mut s, "1 2 3,[0.5]4 5 6"), vec!["1 2 3", "4 5 6"]);
    assert_eq!(out(&mut s, "1 2 3,[1.5]4 5 6"), vec!["1 4", "2 5", "3 6"]);
    assert_eq!(out(&mut s, ")ORIGIN 0"), vec!["WAS 1"]);
    assert_eq!(out(&mut s, "A,[0]A"), vec!["1 2", "3 4", "1 2", "3 4"]);
    assert_eq!(out(&mut s, "A,[2]A")[0], "INDEX ERROR");
    assert_eq!(out(&mut s, "\u{233d}[1]A"), vec!["2 1", "4 3"]);
}

#[test]
fn select_functions_end_to_end() {
    let mut s = Session::default();
    assert_eq!(
        out(&mut s, "M\u{2190}3 3\u{2374}\u{2373}9"),
        Vec::<String>::new()
    );
    assert_eq!(out(&mut s, "2 2\u{2191}M"), vec!["1 2", "4 5"]);
    assert_eq!(out(&mut s, "\u{af}1 \u{af}2\u{2193}M"), vec!["1", "4"]);
    assert_eq!(out(&mut s, "3\u{2191}5"), vec!["5 0 0"]);
    assert_eq!(out(&mut s, "\u{233d}M"), vec!["3 2 1", "6 5 4", "9 8 7"]);
    assert_eq!(out(&mut s, "\u{2296}M"), vec!["7 8 9", "4 5 6", "1 2 3"]);
    assert_eq!(out(&mut s, "\u{233d}[1]M"), vec!["7 8 9", "4 5 6", "1 2 3"]);
    assert_eq!(out(&mut s, "1\u{2296}M"), vec!["4 5 6", "7 8 9", "1 2 3"]);
    assert_eq!(
        out(&mut s, "0 1 2\u{233d}M"),
        vec!["1 2 3", "5 6 4", "9 7 8"]
    );
    assert_eq!(out(&mut s, "\u{2349}M"), vec!["1 4 7", "2 5 8", "3 6 9"]);
    assert_eq!(out(&mut s, "1 1\u{2349}M"), vec!["1 5 9"]);
    assert_eq!(out(&mut s, "\u{233d}[3]M")[0], "INDEX ERROR");
    assert_eq!(out(&mut s, "\u{2349}[1]M")[0], "NOT IMPLEMENTED");
}

#[test]
fn compress_expand_membership_indexof_end_to_end() {
    let mut s = Session::default();
    assert_eq!(out(&mut s, "1 0 1/10 20 30"), vec!["10 30"]);
    assert_eq!(out(&mut s, "(3=\u{2373}5)/\u{2373}5"), vec!["3"]);
    assert_eq!(
        out(&mut s, "M\u{2190}3 3\u{2374}\u{2373}9"),
        Vec::<String>::new()
    );
    assert_eq!(out(&mut s, "1 0 1/M"), vec!["1 3", "4 6", "7 9"]);
    assert_eq!(out(&mut s, "1 0 1\u{233f}M"), vec!["1 2 3", "7 8 9"]);
    assert_eq!(out(&mut s, "0 1 1/[1]M"), vec!["4 5 6", "7 8 9"]);
    assert_eq!(out(&mut s, "1 0 1\\1 2"), vec!["1 0 2"]);
    assert_eq!(
        out(&mut s, "1 0 1\u{2340}2 2\u{2374}\u{2373}4"),
        vec!["1 2", "0 0", "3 4"]
    );
    assert_eq!(out(&mut s, "1 2 3 4\u{220a}2 4 6"), vec!["0 1 0 1"]);
    assert_eq!(out(&mut s, "10 20 30\u{2373}20 40"), vec!["2 4"]);
    assert_eq!(out(&mut s, "'hello'\u{220a}'aeiou'"), vec!["0 1 0 0 1"]);
}
