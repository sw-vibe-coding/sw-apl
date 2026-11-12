//! DOS, which `)DOS` hands the keyboard to (docs/mode-c.md): DIR, KILL
//! and RETURN, within reason, and any other command an error.

use apl_c80_commands::READY;
use apl_commands::system_command;
use apl_eval::Workspace;
use apl_reply::Reply;

/// What DOS says to a command it has not got, or a name it cannot take.
const INCORRECT: &str = "INCORRECT COMMAND";

/// One line typed to DOS, which waits again after it: DIR lists the
/// saved workspaces, as `)LIB` does, and KILL drops one, as `)DROP`
/// does; RETURN goes back to APL with the workspace as it was.
pub(crate) fn dos(ws: &mut Workspace, line: &str) -> Reply {
    let words: Vec<String> = line
        .split_whitespace()
        .map(str::to_ascii_uppercase)
        .collect();
    let words: Vec<&str> = words.iter().map(String::as_str).collect();
    let mut lines = match words.as_slice() {
        ["RETURN"] => {
            ws.dos = false;
            return Reply::default();
        }
        ["DIR"] => system_command(ws, "LIB").lines,
        ["KILL", name] => kill(ws, name),
        _ => vec![INCORRECT.to_string()],
    };
    lines.push(READY.to_string());
    Reply::from(lines)
}

/// KILL `name`: the workspace dropped, and nothing to say; or why not.
fn kill(ws: &mut Workspace, name: &str) -> Vec<String> {
    let Ok(found) = apl_library::named(&[name]) else {
        return vec![INCORRECT.to_string()];
    };
    match apl_shelves::forget(&mut *ws.store, ws.mode, 0, &found.name) {
        Ok(()) => Vec::new(),
        Err(_) => vec!["WS NOT FOUND".to_string()],
    }
}
