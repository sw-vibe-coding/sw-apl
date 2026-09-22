//! The settings commands, which only '68 has.

use apl_a70_commands::settings_command;
use apl_eval::Saved;

#[test]
fn the_commands_reply_as_apl_360_did() {
    let mut s = Saved::default();
    assert_eq!(
        settings_command(&mut s, "ORIGIN", &["0"]).as_deref(),
        Some("WAS 1")
    );
    assert_eq!(
        settings_command(&mut s, "WIDTH", &[]).as_deref(),
        Some("INCORRECT COMMAND")
    );
    assert_eq!(
        settings_command(&mut s, "WIDTH", &["80", "90"]).as_deref(),
        Some("INCORRECT COMMAND")
    );
    assert_eq!(settings_command(&mut s, "WSID", &[]), None, "not a setting");
}
