//! Each setting's bounds, which the directives, the '70 commands and
//! the '75 system variables share, and each mode's clear workspace.
//! The commands' own tests are in apl-a70-commands.

use apl_settings::setting;
use apl_workspace::Saved;

#[test]
fn each_setting_takes_its_range_and_gives_back_what_it_was() {
    let mut s = Saved::default();
    assert_eq!(setting(&mut s, "ORIGIN", "0").as_deref(), Some("1"));
    assert_eq!(setting(&mut s, "DIGITS", "16").as_deref(), Some("10"));
    assert_eq!(setting(&mut s, "WIDTH", "30").as_deref(), Some("120"));
    assert_eq!((s.env.io, s.print.digits, s.print.width), (0, 16, 30));
    assert_eq!(setting(&mut s, "LINK", "5").as_deref(), Some("16807"));
    assert_eq!(s.env.link, 5);
}

#[test]
fn a_value_out_of_range_changes_nothing() {
    let mut s = Saved::default();
    for (name, value) in [
        ("ORIGIN", "2"),
        ("DIGITS", "0"),
        ("DIGITS", "17"),
        ("WIDTH", "29"),
        ("WIDTH", "255"),
        ("WIDTH", "X"),
        ("LINK", "0"),
        ("LINK", "2147483647"),
        ("LINK", "¯5"),
    ] {
        assert_eq!(setting(&mut s, name, value), None, "{name} {value}");
    }
    assert_eq!((s.env.io, s.print.digits, s.print.width), (1, 10, 120));
    assert_eq!(s.env.link, 16807);
}

#[test]
fn each_mode_clears_to_its_own_settings() {
    use apl_modes::Mode;
    let a = apl_settings::clear(Mode::A);
    assert_eq!((a.print.digits, a.print.width, a.print.whole), (10, 120, 0));
    let b = apl_settings::clear(Mode::B);
    assert_eq!((b.print.digits, b.print.width, b.print.whole), (5, 64, 10));
    for s in [a, b] {
        assert_eq!((s.env.io, s.env.link), (1, 16807));
    }
}
