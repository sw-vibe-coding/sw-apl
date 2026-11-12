//! (C) '80's reach into the machine, after the APL80 manual and the
//! owner's decisions (docs/mode-c.md): `)RAM` opens a modeled memory
//! to peek and poke, `u`, and to Call, dyadic `#`; `)DOS` takes DIR,
//! KILL and RETURN.

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

fn with_ram() -> Session {
    let mut c = in_mode(Mode::C);
    assert!(out(&mut c, ")RAM").is_empty());
    c
}

fn error(s: &mut Session, line: &str) -> String {
    let reply = s.respond(line);
    assert!(reply.error, "{line}: {:?}", reply.lines);
    reply.lines[0].clone()
}

#[test]
fn without_ram_peek_poke_and_call_are_errors() {
    let mut c = in_mode(Mode::C);
    assert_eq!(error(&mut c, "u 16384"), "NONCE ERROR");
    assert_eq!(error(&mut c, "65 u 16000"), "NONCE ERROR");
    assert_eq!(error(&mut c, "51 # 65"), "NONCE ERROR");
    assert_eq!(out(&mut c, "#65"), vec!["A"], "conversion needs no )RAM");
}

#[test]
fn ram_above_the_screen_is_plain_bytes() {
    let mut c = with_ram();
    assert_eq!(out(&mut c, "u 20000"), vec!["0"], "never poked");
    assert_eq!(out(&mut c, "7 u 16384"), vec!["7"], "poke gives its byte");
    assert_eq!(out(&mut c, "u 16384"), vec!["7"]);
    assert_eq!(out(&mut c, "1 2 3 u 30000 30001 30002"), vec!["1 2 3"]);
    assert_eq!(out(&mut c, "u 30000 30001 30002"), vec!["1 2 3"]);
    assert_eq!(out(&mut c, "u 65535"), vec!["0"], "the top of 48K");
}

#[test]
fn rom_devices_and_bad_bytes_are_domain_errors() {
    let mut c = with_ram();
    for line in [
        "u 1000",
        "u 14000",
        "u 65536",
        "u -1",
        "256 u 16384",
        "1.5 u 16384",
        "1 u 1000",
    ] {
        assert_eq!(error(&mut c, line), "DOMAIN ERROR", "{line}");
    }
    assert_eq!(error(&mut c, "1 2 u 16384 16385 16386"), "LENGTH ERROR");
}

#[test]
fn a_poke_on_the_screen_puts_the_character_there() {
    let mut c = with_ram();
    // 16000 is 640 into the screen: line 11, column 1, as ESC [ counts.
    assert_eq!(
        out(&mut c, "65 u 16000"),
        vec!["\x1b[s\x1b[11;1HA\x1b[u65"],
        "the cursor is saved, moved, and put back"
    );
    assert_eq!(out(&mut c, "u 16000"), vec!["65"]);
    assert_eq!(
        out(&mut c, "112 u 16001"),
        vec!["\x1b[s\x1b[11;2Hp\x1b[u112"]
    );
    assert_eq!(
        out(&mut c, "u 16001"),
        vec!["112"],
        "a shifted letter's code"
    );
    assert_eq!(error(&mut c, "13 u 16000"), "DOMAIN ERROR", "not printed");
}

#[test]
fn a_peek_reads_what_the_screen_shows() {
    let mut c = with_ram();
    assert_eq!(out(&mut c, "'HELLO'"), vec!["HELLO"]);
    // Line 1 holds the line typed, six blanks in; line 2 its answer.
    assert_eq!(out(&mut c, "#u 15360+6 7 8"), vec!["'HE"]);
    assert_eq!(out(&mut c, "#u 15423+i5"), vec!["HELLO"]);
}

#[test]
fn call_prints_a_character_or_reads_a_key() {
    let mut c = with_ram();
    assert_eq!(out(&mut c, "X]51 # 65"), vec!["A"], "51 prints it");
    assert_eq!(out(&mut c, "X"), vec!["65"], "and gives back its argument");
    assert_eq!(out(&mut c, "43 # 0"), vec!["0"], "no key is waiting");
    assert_eq!(error(&mut c, "44 # 0"), "DOMAIN ERROR", "no other routine");
    assert_eq!(error(&mut c, "51 # 256"), "DOMAIN ERROR");
}

/// A library directory of its own, removed when the test ends.
struct Dir(std::path::PathBuf);

impl Dir {
    fn new(tag: &str) -> Dir {
        let dir = std::env::temp_dir().join(format!("sw-apl-ram-{tag}-{}", std::process::id()));
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

#[test]
fn dos_lists_and_kills_workspaces_and_returns() {
    let dir = Dir::new("dos");
    let mut c = in_mode(Mode::C);
    c.ws.store = Box::new(Files(dir.0.clone()));
    out(&mut c, "N]5");
    out(&mut c, ")SAVE W");
    out(&mut c, ")SAVE V");
    assert_eq!(out(&mut c, ")DOS"), vec!["DOS READY"]);
    assert_eq!(out(&mut c, "DIR"), vec!["V", "W", "DOS READY"]);
    assert_eq!(out(&mut c, "N"), vec!["INCORRECT COMMAND", "DOS READY"]);
    assert_eq!(out(&mut c, "KILL W"), vec!["DOS READY"]);
    assert_eq!(out(&mut c, "dir"), vec!["V", "DOS READY"]);
    assert!(out(&mut c, "RETURN").is_empty());
    assert_eq!(out(&mut c, "N"), vec!["5"], "the workspace is intact");
}

#[test]
fn only_c_has_ram_and_dos() {
    for mode in [Mode::A, Mode::B] {
        let mut s = in_mode(mode);
        assert_eq!(out(&mut s, ")RAM"), vec!["INCORRECT COMMAND"]);
        assert_eq!(out(&mut s, ")DOS"), vec!["INCORRECT COMMAND"]);
    }
}
