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
    assert_eq!(
        out(&mut s, "\u{2349}[1]M")[0],
        "SYNTAX ERROR",
        "transpose takes no axis"
    );
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

#[test]
fn grade_radix_and_deal_end_to_end() {
    let mut s = Session::default();
    assert_eq!(out(&mut s, "\u{234b}30 10 20"), vec!["2 3 1"]);
    assert_eq!(out(&mut s, "\u{2352}30 10 20"), vec!["1 3 2"]);
    assert_eq!(out(&mut s, "2 2 2\u{22a4}5"), vec!["1 0 1"]);
    assert_eq!(out(&mut s, "2 2 2\u{22a5}1 0 1"), vec!["5"]);
    assert_eq!(
        out(&mut s, "24 60 60\u{22a5}24 60 60\u{22a4}3661"),
        vec!["3661"]
    );
    let a = out(&mut s, "5?5");
    let mut b = Session::default();
    assert_eq!(a, out(&mut b, "5?5"));
    assert_eq!(out(&mut s, "6?5")[0], "DOMAIN ERROR");
}

#[test]
fn reduce_and_scan_on_either_axis_end_to_end() {
    let mut s = Session::default();
    assert_eq!(
        out(&mut s, "M\u{2190}2 3\u{2374}\u{2373}6"),
        Vec::<String>::new()
    );
    assert_eq!(out(&mut s, "+/M"), vec!["6 15"]);
    assert_eq!(out(&mut s, "+\u{233f}M"), vec!["5 7 9"]);
    assert_eq!(out(&mut s, "+/[1]M"), vec!["5 7 9"]);
    assert_eq!(out(&mut s, "+\\1 2 3 4"), vec!["1 3 6 10"]);
    assert_eq!(out(&mut s, "+\u{2340}M"), vec!["1 2 3", "5 7 9"]);
    assert_eq!(out(&mut s, "+\\[1]M"), vec!["1 2 3", "5 7 9"]);
    assert_eq!(out(&mut s, "\u{2228}/0 0 1"), vec!["1"]);
    assert_eq!(
        out(&mut s, "\u{2308}/\u{2373}0"),
        vec!["\u{af}1.797693135E308"]
    );
    assert_eq!(out(&mut s, "\u{25cb}/\u{2373}0")[0], "DOMAIN ERROR");
    assert_eq!(out(&mut s, "+/[3]M")[0], "INDEX ERROR");
}

#[test]
fn inner_and_outer_products_end_to_end() {
    let mut s = Session::default();
    assert_eq!(out(&mut s, "1 2 3+.\u{d7}4 5 6"), vec!["32"]);
    assert_eq!(
        out(&mut s, "(2 2\u{2374}1 2 3 4)+.\u{d7}2 2\u{2374}5 6 7 8"),
        vec!["19 22", "43 50"]
    );
    assert_eq!(
        out(&mut s, "1 2 3\u{2218}.+10 20"),
        vec!["11 21", "12 22", "13 23"]
    );
    assert_eq!(
        out(&mut s, "1 2 3\u{2218}.=1 2 3"),
        vec!["1 0 0", "0 1 0", "0 0 1"]
    );
    assert_eq!(
        out(&mut s, "1 2\u{2218}.\u{d7}[1]1 2")[0],
        "SYNTAX ERROR",
        "products take no axis"
    );
}

#[test]
fn indexing_end_to_end() {
    let mut s = Session::default();
    assert_eq!(
        out(&mut s, "M\u{2190}3 3\u{2374}\u{2373}9"),
        Vec::<String>::new()
    );
    assert_eq!(out(&mut s, "M[2;3]"), vec!["6"]);
    assert_eq!(out(&mut s, "M[1 3;]"), vec!["1 2 3", "7 8 9"]);
    assert_eq!(out(&mut s, "M[;2]"), vec!["2 5 8"]);
    assert_eq!(out(&mut s, "'ABCDE'[3 1]"), vec!["CA"]);
    assert_eq!(out(&mut s, "M[2;]\u{2190}0"), Vec::<String>::new());
    assert_eq!(out(&mut s, "M"), vec!["1 2 3", "0 0 0", "7 8 9"]);
    assert_eq!(
        out(&mut s, "M[4;1]"),
        vec!["INDEX ERROR", "      M[4;1]", "       ^"]
    );
}

#[test]
fn only_the_axis_taking_glyphs_accept_a_bracket() {
    let mut s = Session::default();
    assert_eq!(
        out(&mut s, "M\u{2190}2 3\u{2374}\u{2373}6"),
        Vec::<String>::new()
    );
    // The seven APL\360 forms that take an axis.
    for line in [
        "\u{233d}[1]M",
        "\u{2296}[2]M",
        "1 1/[1]M",
        "1 1 1\u{233f}[2]M",
        "1 1 1\\[2]M",
        "1 1\u{2340}[1]M",
        "M,[1]M",
        "+/[1]M",
        "+\\[2]M",
    ] {
        assert_ne!(out(&mut s, line)[0], "SYNTAX ERROR", "{line} takes an axis");
    }
    // Everything else: APL\360 has no axis form, so it is a syntax error.
    for line in [
        "2 2\u{2191}[1]M",
        "1 1\u{2193}[1]M",
        "\u{2349}[1]M",
        "2\u{2374}[1]M",
        "M\u{220a}[1]M",
        "1 2 3\u{2373}[1]1",
        "2\u{22a5}[1]M",
        "2\u{22a4}[1]5",
        "\u{2373}[1]3",
        "\u{234b}[1]1 2",
        "M+[1]M",
        "M+.\u{d7}[1]M",
    ] {
        assert_eq!(out(&mut s, line)[0], "SYNTAX ERROR", "{line} takes no axis");
    }
    // Monadic ravel with an axis is APL2, not APL\360 syntax we reject
    // outright: the glyph does take an axis, dyadically.
    assert_eq!(out(&mut s, ",[1]M")[0], "NOT IMPLEMENTED");
}

#[test]
fn structural_functions_on_scalars_and_empties() {
    let mut s = Session::default();
    let cases = [
        ("\u{2374}5", vec![""]),
        (",5", vec!["5"]),
        ("\u{233d}5", vec!["5"]),
        ("\u{2296}5", vec!["5"]),
        ("\u{2349}5", vec!["5"]),
        ("1\u{2191}5", vec!["5"]),
        ("1\u{2193}5", vec![""]),
        ("1/5", vec!["5"]),
        ("5\u{220a}5", vec!["1"]),
        ("2\u{22a4}5", vec!["1"]),
        ("\u{2374}\u{2373}0", vec!["0"]),
        (",\u{2373}0", vec![""]),
        ("\u{233d}\u{2373}0", vec![""]),
        ("\u{234b}\u{2373}0", vec![""]),
        ("\u{2374}0 0\u{2374}0", vec!["0 0"]),
        ("+/0 3\u{2374}0", vec![""]),
        ("+\u{233f}0 3\u{2374}0", vec!["0 0 0"]),
        ("\u{2374}\u{2349}2 3\u{2374}\u{2373}6", vec!["3 2"]),
    ];
    for (line, want) in cases {
        assert_eq!(out(&mut s, line), want, "{line}");
    }
    assert_eq!(
        out(&mut s, "\u{234b}5")[0],
        "RANK ERROR",
        "grade needs a vector"
    );
    assert_eq!(
        out(&mut s, "5\u{2373}5")[0],
        "RANK ERROR",
        "index-of needs a vector left"
    );
}

/// Type a del header, the body lines, and the closing del.
fn define(s: &mut Session, header: &str, body: &[&str]) {
    assert_eq!(out(s, header), Vec::<String>::new());
    for line in body {
        assert_eq!(out(s, line), Vec::<String>::new());
    }
    assert_eq!(out(s, "\u{2207}"), Vec::<String>::new());
}

#[test]
fn definition_mode_prompts_with_the_line_number() {
    let mut s = Session::default();
    assert_eq!(s.prompt(), "      ");
    out(&mut s, "\u{2207}R\u{2190}DOUBLE N");
    assert_eq!(s.prompt(), "[1]   ");
    out(&mut s, "R\u{2190}N+N");
    assert_eq!(s.prompt(), "[2]   ");
    out(&mut s, "\u{2207}");
    assert_eq!(s.prompt(), "      ");
    assert_eq!(out(&mut s, "DOUBLE 4"), vec!["8"]);
}

#[test]
fn a_bad_header_is_defn_error_and_leaves_immediate_execution() {
    let mut s = Session::default();
    assert_eq!(
        out(&mut s, "\u{2207}A B C D"),
        vec!["DEFN ERROR", "      \u{2207}A B C D", "      ^"]
    );
    assert_eq!(s.prompt(), "      ");
    assert_eq!(out(&mut s, "2+2"), vec!["4"]);
}

#[test]
fn a_function_prints_what_its_lines_display_then_its_result() {
    let mut s = Session::default();
    define(
        &mut s,
        "\u{2207}R\u{2190}REPORT N;T",
        &[
            "'COUNTING TO ';N",
            "T\u{2190}\u{2373}N",
            "T",
            "R\u{2190}+/T",
        ],
    );
    assert_eq!(
        out(&mut s, "REPORT 4"),
        vec!["COUNTING TO 4", "1 2 3 4", "10"]
    );
}

#[test]
fn a_function_with_no_result_is_a_statement_not_a_value() {
    let mut s = Session::default();
    define(&mut s, "\u{2207}GREET", &["'HELLO'"]);
    assert_eq!(out(&mut s, "GREET"), vec!["HELLO"]);
    assert_eq!(
        out(&mut s, "1+GREET"),
        vec!["HELLO", "VALUE ERROR", "      1+GREET", "        ^"]
    );
}

#[test]
fn the_wrong_valence_is_syntax_error() {
    let mut s = Session::default();
    define(&mut s, "\u{2207}R\u{2190}DOUBLE N", &["R\u{2190}N+N"]);
    assert_eq!(
        out(&mut s, "2 DOUBLE 3"),
        vec!["SYNTAX ERROR", "      2 DOUBLE 3", "        ^"]
    );
}

#[test]
fn a_function_name_is_not_a_variable() {
    let mut s = Session::default();
    define(&mut s, "\u{2207}R\u{2190}TEN", &["R\u{2190}10"]);
    assert_eq!(out(&mut s, "TEN+TEN"), vec!["20"]);
    define(&mut s, "\u{2207}R\u{2190}F B", &["R\u{2190}B"]);
    assert_eq!(out(&mut s, "F F 3"), vec!["3"]);
    assert_eq!(out(&mut s, "F \u{2373}3"), vec!["1 2 3"]);
}

#[test]
fn runaway_recursion_reports_depth_error() {
    let mut s = Session::default();
    define(&mut s, "\u{2207}R\u{2190}DOWN N", &["R\u{2190}DOWN N-1"]);
    assert_eq!(
        out(&mut s, "DOWN 1"),
        vec!["DEPTH ERROR", "      DOWN 1", "      ^"]
    );
    assert_eq!(out(&mut s, "2+2"), vec!["4"]);
}

#[test]
fn a_loop_runs_and_prints_each_round() {
    let mut s = Session::default();
    define(
        &mut s,
        "\u{2207}COUNT N;I",
        &[
            "I\u{2190}0",
            "LOOP:I\u{2190}I+1",
            "'ROUND ';I",
            "\u{2192}LOOP\u{d7}\u{2373}I<N",
            "'DONE'",
        ],
    );
    assert_eq!(
        out(&mut s, "COUNT 3"),
        vec!["ROUND 1", "ROUND 2", "ROUND 3", "DONE"]
    );
}

#[test]
fn recursion_with_a_branch_terminates() {
    let mut s = Session::default();
    define(
        &mut s,
        "\u{2207}R\u{2190}FAC N",
        &[
            "R\u{2190}1",
            "\u{2192}0\u{d7}\u{2373}N\u{2264}1",
            "R\u{2190}N\u{d7}FAC N-1",
        ],
    );
    assert_eq!(out(&mut s, "FAC 6"), vec!["720"]);
    assert_eq!(out(&mut s, "FAC \u{af}1"), vec!["1"]);
}

#[test]
fn a_label_in_immediate_execution_is_a_syntax_error() {
    let mut s = Session::default();
    assert_eq!(
        out(&mut s, "L:2+2"),
        vec!["SYNTAX ERROR", "      L:2+2", "       ^"]
    );
}

#[test]
fn a_branch_in_immediate_execution_displays_nothing() {
    let mut s = Session::default();
    assert_eq!(out(&mut s, "\u{2192}3"), Vec::<String>::new());
    assert_eq!(out(&mut s, "\u{2192}"), Vec::<String>::new());
    assert_eq!(out(&mut s, "2+2"), vec!["4"]);
}
