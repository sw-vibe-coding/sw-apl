//! Each setting's bounds, which the directives and the '68 commands
//! share. The commands' own tests are in apl-a70-commands.

use apl_eval::Saved;
use apl_settings::setting;

#[test]
fn each_setting_takes_its_range_and_gives_back_what_it_was() {
    let mut s = Saved::default();
    assert_eq!(setting(&mut s, "ORIGIN", "0").as_deref(), Some("1"));
    assert_eq!(setting(&mut s, "DIGITS", "16").as_deref(), Some("10"));
    assert_eq!(setting(&mut s, "WIDTH", "30").as_deref(), Some("120"));
    assert_eq!((s.env.io, s.print.digits, s.print.width), (0, 16, 30));
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
        ("LINK", "5"),
    ] {
        assert_eq!(setting(&mut s, name, value), None, "{name} {value}");
    }
    assert_eq!((s.env.io, s.print.digits, s.print.width), (1, 10, 120));
}
