//! A (U) '74 batch run: a deck in, a printout out (docs/mode-u.md).

use apl_session::{Host, Memory, Mode, QUOTA, TimeSource};
use apl_u74_batch::run;

fn host() -> Host {
    Host {
        quota: QUOTA,
        store: Box::new(Memory::default()),
        mode: Mode::U,
        clock: TimeSource::None,
    }
}

fn printout(cards: &[&str]) -> Vec<String> {
    run(&cards.join("\n"), host())
}

/// Whether `lines` has `want` as consecutive lines.
fn shows(lines: &[String], want: &[&str]) -> bool {
    lines.windows(want.len()).any(|w| w == want)
}

const DOMINO: &[&str] = &[
    "@RUN   ALBANY,76104,CS301,1,20",
    "@APL,I",
    "$CM SOLVE A+.$MUX = B WITH DOMINO",
    "A $AL 2 2 $RH 2 1 1 3",
    "B $AL 5 10",
    "X $AL B $DM A",
    "$QD $AL X",
    "@FIN",
];

#[test]
fn the_first_deck_solves_its_system() {
    assert_eq!(
        printout(DOMINO),
        [
            "0001  @RUN   ALBANY,76104,CS301,1,20",
            "0002  @APL,I",
            "UNIVERSITY OF MARYLAND APL",
            "CLEAR WS",
            "0003  $CM SOLVE A+.$MUX = B WITH DOMINO",
            "      ⍝ SOLVE A+.×X = B WITH DOMINO",
            "0004  A $AL 2 2 $RH 2 1 1 3",
            "      A ← 2 2 ⍴ 2 1 1 3",
            "0005  B $AL 5 10",
            "      B ← 5 10",
            "0006  X $AL B $DM A",
            "      X ← B ⌹ A",
            "0007  $QD $AL X",
            "      ⎕ ← X",
            "1 3",
            "0008  @FIN",
            "END OF RUN",
        ]
    );
}

#[test]
fn cards_outside_the_apl_are_listed_and_not_run() {
    let out = printout(&["@RUN", "1+1", "@APL", "2+2", "@FIN", "3+3"]);
    assert!(!out.contains(&"2".to_string()), "before @APL: {out:?}");
    assert!(out.contains(&"4".to_string()));
    assert!(!out.contains(&"6".to_string()), "after @FIN");
    assert_eq!(out.last().map(String::as_str), Some("0006  3+3"));
}

#[test]
fn a_deck_without_its_control_cards_still_runs() {
    let out = printout(&["2 $MU 3"]);
    assert_eq!(
        out,
        [
            "NO @RUN CARD",
            "NO @APL CARD, APL STARTS AT CARD 1",
            "UNIVERSITY OF MARYLAND APL",
            "CLEAR WS",
            "0001  2 $MU 3",
            "      2 × 3",
            "6",
            "END OF DECK, NO @FIN CARD",
        ]
    );
}

#[test]
fn a_function_is_defined_by_del_cards() {
    let out = printout(&[
        "@RUN",
        "@APL",
        "$DL Z $AL SQ X",
        "Z $AL X $MU X",
        "$DL",
        "SQ 4",
        "@FIN",
    ]);
    assert!(shows(&out, &["0006  SQ 4", "      SQ 4", "16"]), "{out:?}");
    assert!(
        out.iter()
            .any(|l| l.starts_with("[1]") && l.ends_with("Z ← X × X"))
    );
}

#[test]
fn a_function_already_defined_is_not_reopened() {
    let out = printout(&[
        "@RUN",
        "@APL",
        "$DL Z $AL SQ X",
        "Z $AL X $MU X",
        "$DL",
        "$DL SQ",
        "Z $AL 0",
        "$DL",
        "SQ 4",
        "@FIN",
    ]);
    assert!(shows(&out, &["DEFN ERROR", "CARD 6"]), "{out:?}");
    assert!(shows(&out, &["      SQ 4", "16"]), "unchanged: {out:?}");
}

#[test]
fn an_error_names_its_card_and_column_and_the_run_goes_on() {
    let out = printout(&["@RUN", "@APL", "X $AL 1 $DI 0", "2+2", "@FIN"]);
    assert!(out.contains(&"DOMAIN ERROR".to_string()), "{out:?}");
    assert!(out.contains(&"CARD 3 COLUMN 9".to_string()), "{out:?}");
    assert!(out.contains(&"4".to_string()), "the run went on");
}

#[test]
fn an_error_in_a_function_leaves_it() {
    let out = printout(&[
        "@RUN", "@APL", "$DL BAD", "1 $DI 0", "$DL", "BAD", "2+2", "@FIN",
    ]);
    assert!(out.contains(&"CARD 6".to_string()), "{out:?}");
    assert!(shows(&out, &["      2+2", "4"]), "{out:?}");
}

#[test]
fn what_a_card_cannot_say_is_a_character_error() {
    let out = printout(&["@RUN", "@APL", "A $AL $XY 1", "B $AL [1]", "@FIN"]);
    assert!(shows(
        &out,
        &[
            "CHARACTER ERROR",
            "      A $AL $XY 1",
            "            ^",
            "CARD 3 COLUMN 7"
        ]
    ));
    assert!(
        shows(&out, &["CHARACTER ERROR", "      B $AL [1]"]),
        "{out:?}"
    );
    assert!(out.contains(&"CARD 4 COLUMN 7".to_string()));
}

#[test]
fn a_deck_has_no_system_commands() {
    let out = printout(&["@RUN", "@APL", ")CLEAR", "@FIN"]);
    assert!(
        shows(&out, &["0003  )CLEAR", "INCORRECT COMMAND", "CARD 3"]),
        "{out:?}"
    );
}

#[test]
fn quad_and_quote_quad_read_the_next_card() {
    let out = printout(&[
        "@RUN",
        "@APL",
        "T $AL $QQ",
        "HELLO, WORLD",
        "$RH T",
        "N $AL $QD",
        "2 $MU 3",
        "N+1",
        "@FIN",
    ]);
    assert!(
        shows(&out, &["      $RH T".replace("$RH", "⍴").as_str(), "12"]),
        "{out:?}"
    );
    assert!(shows(&out, &["0007  2 $MU 3", "      2 × 3"]), "{out:?}");
    assert!(shows(&out, &["      N+1", "7"]), "{out:?}");
}

#[test]
fn a_read_with_no_card_left_ends_the_run() {
    let out = printout(&["@RUN", "@APL", "N $AL $QD", "@FIN"]);
    assert!(out.contains(&"NO CARD LEFT TO READ".to_string()), "{out:?}");
    assert!(!out.contains(&"END OF RUN".to_string()));
    assert!(!out.contains(&"INTERRUPT".to_string()), "{out:?}");
}

#[test]
fn a_deck_that_ends_in_a_definition_says_so() {
    let out = printout(&["@RUN", "@APL", "$DL F", "1"]);
    assert!(shows(
        &out,
        &[
            "END OF DECK IN A FUNCTION DEFINITION",
            "END OF DECK, NO @FIN CARD"
        ]
    ));
}
