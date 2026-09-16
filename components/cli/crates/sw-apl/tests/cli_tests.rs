//! End-to-end checks of the `sw-apl` binary's shell behaviour.

use std::io::Write;
use std::process::{Command, Stdio};

fn sw_apl() -> Command {
    Command::new(env!("CARGO_BIN_EXE_sw-apl"))
}

fn run_stdin(args: &[&str], input: &str) -> (String, i32) {
    let mut child = sw_apl()
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn sw-apl");
    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(input.as_bytes())
        .expect("write stdin");
    let output = child.wait_with_output().expect("wait");
    let code = output.status.code().unwrap_or(-1);
    (String::from_utf8_lossy(&output.stdout).into_owned(), code)
}

#[test]
fn version_names_the_binary_with_build_block() {
    let out = sw_apl().arg("--version").output().expect("run");
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.starts_with("sw-apl 0."), "got: {text}");
    for field in [
        "Copyright",
        "License: MIT",
        "Repository:",
        "Host:",
        "Commit:",
        "Timestamp:",
    ] {
        assert!(text.contains(field), "missing {field} in: {text}");
    }
    let short = sw_apl().arg("-V").output().expect("run");
    assert_eq!(short.stdout, out.stdout, "-V and --version must match");
}

#[test]
fn help_describes_apl_and_flags() {
    let out = sw_apl().arg("--help").output().expect("run");
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.contains("APL"), "got: {text}");
    assert!(text.contains("--file"), "got: {text}");
    assert!(text.contains("--no-echo"), "got: {text}");
    assert!(text.contains("AI CODING AGENT INSTRUCTIONS"), "got: {text}");
    let short = sw_apl().arg("-h").output().expect("run");
    assert!(short.stdout.len() < out.stdout.len(), "-h must be shorter");
}

#[test]
fn stdin_batch_echoes_with_indent_and_stops_at_off() {
    let (text, code) = run_stdin(&[], "2+2\n)OFF\n3+3\n");
    assert_eq!(code, 0);
    assert_eq!(text, "      2+2\n4\n      )OFF\n");
}

#[test]
fn no_echo_suppresses_the_input_lines() {
    let (text, _) = run_stdin(&["--no-echo"], "2+2\n");
    assert_eq!(text, "4\n");
}

#[test]
fn file_batch_runs_a_sample() {
    let dir = std::env::temp_dir().join(format!("sw-apl-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("t.apl");
    std::fs::write(&path, "\u{235D} comment\n1 2 3\n)OFF\n").expect("write");
    let out = sw_apl().arg("-f").arg(&path).output().expect("run");
    let text = String::from_utf8_lossy(&out.stdout);
    assert_eq!(
        text,
        "      \u{235D} comment\n      1 2 3\n1 2 3\n      )OFF\n"
    );
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn missing_file_fails_with_message() {
    let out = sw_apl()
        .arg("-f")
        .arg("/nonexistent/x.apl")
        .output()
        .expect("run");
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).starts_with("sw-apl: "));
}
