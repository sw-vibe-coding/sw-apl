//! A port already taken, as when an earlier service is still running.
//! The service says which address, which flag sets it, and what to
//! do, and fails without starting half of itself.

use std::net::TcpListener;
use std::process::Command;

fn taken() -> (TcpListener, String) {
    let held = TcpListener::bind("127.0.0.1:0").expect("a port");
    let addr = held.local_addr().expect("its address").to_string();
    (held, addr)
}

fn run(args: &[&str]) -> (bool, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_sw-apl-server"))
        .args(args)
        .output()
        .expect("the service runs");
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

#[test]
fn a_taken_terminal_port_names_itself_and_the_flag() {
    let (_held, addr) = taken();
    let (ok, err) = run(&["--listen", &addr, "--http", "127.0.0.1:0"]);
    assert!(!ok, "{err}");
    assert!(err.contains(&addr) && err.contains("in use"), "{err}");
    assert!(err.contains("--listen"), "{err}");
}

#[test]
fn a_taken_browser_port_names_itself_and_the_flag() {
    let (_held, addr) = taken();
    let (ok, err) = run(&["--listen", "127.0.0.1:0", "--http", &addr]);
    assert!(!ok, "{err}");
    assert!(err.contains(&addr) && err.contains("in use"), "{err}");
    assert!(err.contains("--http"), "{err}");
}
