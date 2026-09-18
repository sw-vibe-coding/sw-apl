//! A workspace holding a locked function is not written as plain
//! text. This is obscuring, not encryption: rot-13 keeps a locked
//! body from being read by accident or by curiosity, which is what
//! APL\360's binary workspaces stopped, and it keeps nothing from
//! anyone who means to read it.

use std::rc::Rc;

use apl_ast::Defn;
use apl_workspace::Saved;
use apl_wsfile::{PREAMBLE, plain, write};

fn defn(name: &str, body: &str, locked: bool) -> Defn {
    Defn {
        name: name.to_string(),
        result: Some("R".to_string()),
        body: vec![body.to_string()],
        locked,
        ..Defn::default()
    }
}

fn workspace(locked: bool) -> Saved {
    let mut saved = Saved {
        id: Some("W".to_string()),
        ..Saved::default()
    };
    let f = defn("SECRET", "R\u{2190}42", locked);
    saved.funcs.insert(f.name.clone(), Rc::new(f));
    saved
}

#[test]
fn a_workspace_with_nothing_locked_stays_plain_text() {
    let text = write(&workspace(false), "1.00.00 01/01/70");
    assert!(text.starts_with('\u{235d}'), "{text}");
    assert!(!text.starts_with(PREAMBLE[0]), "nothing here needs hiding");
    assert!(text.contains("R\u{2190}42"), "and it reads as APL");
    assert_eq!(plain(&text), text, "revealing it changes nothing");
}

#[test]
fn a_workspace_with_a_locked_function_is_obscured() {
    let text = write(&workspace(true), "1.00.00 01/01/70");
    assert!(text.starts_with(PREAMBLE[0]), "{text}");
    assert!(
        !text.contains("SECRET"),
        "the name is not there to read: {text}"
    );
    assert!(!text.contains(")WSID W"), "nor the commands");
}

#[test]
fn revealing_an_obscured_workspace_gives_the_plain_one_back() {
    let plain_text = write(&workspace(false), "1.00.00 01/01/70");
    let locked = write(&workspace(true), "1.00.00 01/01/70");
    // The two differ only by the lock, so revealing one gives the
    // other with a del-tilde where the del was.
    let revealed = plain(&locked);
    assert_eq!(revealed, plain_text.replace("\n\u{2207}\n", "\n\u{236b}\n"));
    assert!(revealed.contains("R\u{2190}42"));
}

#[test]
fn rot_thirteen_is_its_own_inverse_so_one_helper_does_both_ways() {
    let once = write(&workspace(true), "1.00.00 01/01/70");
    // Writing the revealed workspace back out obscures it again, and
    // the file is the same file.
    let twice = plain(&plain(&once));
    assert_eq!(twice, plain(&once), "revealing a plain file is a no-op");
}

#[test]
fn an_obscured_file_says_what_it_is_before_anything_scrambled() {
    let text = write(&workspace(true), "1.00.00 01/01/70");
    let head: Vec<&str> = text.lines().take(3).collect();
    assert_eq!(head[0], PREAMBLE[0], "{head:?}");
    // Run as a program it prints one line and signs off, rather than
    // a screen of CHARACTER ERRORs.
    assert!(head[1].starts_with('\''), "{head:?}");
    assert_eq!(head[2], ")OFF", "{head:?}");
}
