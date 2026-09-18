//! The numeric boundaries as a user meets them: where counting
//! accepts a computed length, where a value stops fitting, and what
//! survives a trip through a saved workspace.

use apl_session::Session;

fn out(s: &mut Session, line: &str) -> Vec<String> {
    s.respond(line).lines
}

fn first(s: &mut Session, line: &str) -> String {
    out(s, line).first().cloned().unwrap_or_default()
}

#[test]
fn counting_accepts_a_length_that_is_whole_within_the_fuzz() {
    let mut s = Session::default();
    // (0.1+0.2)×10 is 3.0000000000000004. It prints as 3, floors to
    // 3 and compares equal to 3, so counting to it must work too --
    // the manual's fuzz exists for exactly this anomaly.
    assert_eq!(first(&mut s, "(0.1+0.2)\u{d7}10"), "3");
    assert_eq!(first(&mut s, "\u{230a}(0.1+0.2)\u{d7}10"), "3");
    assert_eq!(first(&mut s, "((0.1+0.2)\u{d7}10)=3"), "1");
    assert_eq!(first(&mut s, "\u{2373}(0.1+0.2)\u{d7}10"), "1 2 3");
    // And so must the mixed functions that take a count.
    assert_eq!(first(&mut s, "((0.1+0.2)\u{d7}10)\u{2374}7"), "7 7 7");
    assert_eq!(
        first(&mut s, "((0.1+0.2)\u{d7}10)\u{2191}\u{2373}9"),
        "1 2 3"
    );
}

#[test]
fn a_length_that_is_not_whole_is_still_a_domain_error() {
    let mut s = Session::default();
    // The fuzz is narrow: half is not nearly a whole number.
    for bad in [
        "\u{2373}2.5",
        "\u{2373}\u{af}1",
        "2.5\u{2374}7",
        "\u{2373}3.001",
    ] {
        assert_eq!(first(&mut s, bad), "DOMAIN ERROR", "{bad}");
    }
}

/// Where a number stops being exact.
///
/// Everything is a double, as it was on a 360: the manual says
/// `)DIGITS` "has no effect on the precision of internal
/// calculations, which is approximately 16 decimal digits". So the
/// boundary is 2*53, the last integer a double holds exactly, and
/// not the width of the machine's integer type -- a nineteen-digit
/// literal is a double from the moment it is typed.
#[test]
fn a_number_is_exact_up_to_two_to_the_fifty_three() {
    let mut s = Session::default();
    out(&mut s, ")DIGITS 16");
    assert_eq!(first(&mut s, "(2*53)-1"), "9007199254740991");
    assert_eq!(first(&mut s, "((2*53)-1)=(2*53)-2"), "0", "still telling");
    // At 2*53 and above, consecutive integers are the same double.
    assert_eq!(first(&mut s, "(2*53)=(2*53)+1"), "1", "no longer telling");
    let huge = "9223372036854775807";
    assert_eq!(first(&mut s, &format!("{huge}=1+{huge}")), "1");
    // What it must not do is wrap round to a negative.
    assert!(!first(&mut s, &format!("{huge}+1")).starts_with('\u{af}'));
}

/// `)DIGITS` caps every number, not only the fractions. The manual:
/// "Subsequent output of numbers will show no greater number of
/// significant digits than indicated." So an exact integer wider
/// than the setting prints in exponential form, which surprises
/// anyone who has met a later APL -- there, print precision governs
/// floats and integers print in full. This is the boundary, and it
/// is the manual's reading rather than ours.
#[test]
fn digits_caps_integers_too() {
    let mut s = Session::default();
    assert_eq!(first(&mut s, ")DIGITS"), "INCORRECT COMMAND");
    assert_eq!(first(&mut s, "9999999999"), "9999999999", "ten digits");
    assert_eq!(first(&mut s, "10000000000"), "1E10", "eleven");
    assert_eq!(first(&mut s, "+/\u{2373}200000"), "2.00001E10");
    out(&mut s, ")DIGITS 16");
    assert_eq!(first(&mut s, "10000000000"), "10000000000");
    assert_eq!(first(&mut s, "+/\u{2373}200000"), "20000100000");
}

#[test]
fn counting_something_enormous_is_ws_full_only_when_it_is_kept() {
    let mut s = Session::default();
    // The workspace bounds what it holds, not what an expression
    // builds on the way to a result.
    assert_eq!(first(&mut s, "\u{2374}\u{2373}200000"), "200000");
    assert_eq!(first(&mut s, "+/\u{2373}200000"), "2.00001E10");
    assert_eq!(first(&mut s, "BIG\u{2190}\u{2373}200000"), "WS FULL");
    assert_eq!(first(&mut s, "BIG"), "VALUE ERROR");
}

#[test]
fn exponent_notation_reads_in_and_prints_back() {
    let mut s = Session::default();
    for (typed, shown) in [
        ("1E10", "1E10"),
        ("1E\u{af}10", "1E\u{af}10"),
        ("1.5E300", "1.5E300"),
        ("1E\u{af}300", "1E\u{af}300"),
        // Exponential form only where it is needed: a short number
        // with a small exponent still prints in full.
        ("\u{af}1.5E\u{af}5", "\u{af}0.000015"),
    ] {
        assert_eq!(first(&mut s, typed), shown, "{typed}");
    }
    // The exponent is a magnitude, not a count of digits: these are
    // the same number written two ways.
    assert_eq!(first(&mut s, "1E3=1000"), "1");
}

#[test]
fn digits_bounds_what_is_shown_and_not_what_is_held() {
    let mut s = Session::default();
    out(&mut s, "A\u{2190}1\u{f7}3");
    assert_eq!(first(&mut s, ")DIGITS 1"), "WAS 10");
    assert_eq!(first(&mut s, "A"), "0.3");
    out(&mut s, ")DIGITS 16");
    assert_eq!(first(&mut s, "A"), "0.3333333333333333");
    // The value did not change with the setting: three times it is
    // still one whichever way it was being shown.
    assert_eq!(first(&mut s, "3\u{d7}A"), "1");
    // Outside 1 to 16 the command is refused and the setting stands.
    assert_eq!(first(&mut s, ")DIGITS 0"), "INCORRECT COMMAND");
    assert_eq!(first(&mut s, ")DIGITS 17"), "INCORRECT COMMAND");
    assert_eq!(first(&mut s, "A"), "0.3333333333333333");
}

#[test]
fn the_edges_survive_a_saved_workspace() {
    let dir = std::env::temp_dir().join(format!("sw-apl-numeric-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    let mut s = Session::default();
    s.ws.libraries.clone_from(&dir);
    // A workspace file holds literals, so the numbers that are hard
    // to print are the ones that have to survive being written.
    for line in [
        ")WSID EDGES",
        "SMALL\u{2190}1E\u{af}300",
        "LARGE\u{2190}1.5E300",
        "EXACT\u{2190}(2*53)-1",
        "THIRD\u{2190}1\u{f7}3",
        ")DIGITS 16",
        ")SAVE",
        ")CLEAR",
    ] {
        out(&mut s, line);
    }
    out(&mut s, ")LOAD EDGES");
    assert_eq!(first(&mut s, "SMALL=1E\u{af}300"), "1");
    assert_eq!(first(&mut s, "LARGE=1.5E300"), "1");
    assert_eq!(first(&mut s, "EXACT"), "9007199254740991");
    // A third cannot be written exactly in decimal, so what comes
    // back is what sixteen digits can say -- and sixteen digits is
    // enough to name the same double.
    assert_eq!(first(&mut s, "THIRD=1\u{f7}3"), "1");
    std::fs::remove_dir_all(&dir).ok();
}
