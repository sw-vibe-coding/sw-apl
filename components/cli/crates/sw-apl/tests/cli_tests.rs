//! End-to-end checks of the `sw-apl` binary's shell behaviour.

use std::io::Write;
use std::process::{Command, Stdio};

fn sw_apl() -> Command {
    Command::new(env!("CARGO_BIN_EXE_sw-apl"))
}

/// The transcript up to the sign-off `)OFF` prints, which carries the
/// clock and so cannot be compared. `samples/59-sign-off.apl` pins
/// the sign-off itself, through the reg-rs filter.
fn before_sign_off(text: &str) -> String {
    match text.find(")OFF\n") {
        Some(at) => text[..at + 5].to_string(),
        None => text.to_string(),
    }
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
    assert_eq!(before_sign_off(&text), "      2+2\n4\n      )OFF\n");
    assert!(!text.contains("3+3"), ")OFF stopped the run: {text}");
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
        before_sign_off(&text),
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

#[test]
fn invalid_utf8_lines_report_a_character_error_and_continue() {
    let dir = std::env::temp_dir().join(format!("sw-apl-utf8-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("bad.apl");
    let mut bytes = b"1 2\n".to_vec();
    bytes.extend_from_slice(b"3 \xff\xfe 4\n");
    bytes.extend_from_slice(b"5\n)OFF\n");
    std::fs::write(&path, bytes).expect("write");
    let out = sw_apl().arg("-f").arg(&path).output().expect("run");
    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    assert_eq!(
        before_sign_off(&text),
        "      1 2\n1 2\n      3 \u{fffd}\u{fffd} 4\nCHARACTER ERROR: invalid UTF-8 at byte 2\n      5\n5\n      )OFF\n"
    );
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn batch_echoes_definition_lines_behind_the_bracketed_prompt() {
    let input = "\u{2207}R\u{2190}DOUBLE N\nR\u{2190}N+N\n\u{2207}\nDOUBLE 4\n)OFF\n";
    let (text, code) = run_stdin(&[], input);
    assert_eq!(code, 0);
    let want = [
        "      \u{2207}R\u{2190}DOUBLE N",
        "[1]   R\u{2190}N+N",
        "[2]   \u{2207}",
        "      DOUBLE 4",
        "8",
        "      )OFF",
        "",
    ];
    assert_eq!(before_sign_off(&text), want.join("\n"));
}

#[test]
fn batch_feeds_a_read_from_the_script_and_carries_on_after_it() {
    // The line the statement reads is consumed by the read, not run
    // again by the loop: the queue is shared between the two.
    let input = "X\u{2190}\u{2395}\n2 3 4\nX\u{d7}2\n)OFF\n";
    let (text, code) = run_stdin(&[], input);
    assert_eq!(code, 0);
    let want = [
        "      X\u{2190}\u{2395}",
        "\u{2395}:",
        "      2 3 4",
        "      X\u{d7}2",
        "4 6 8",
        "      )OFF",
        "",
    ];
    assert_eq!(before_sign_off(&text), want.join("\n"));
}

#[test]
fn a_quote_quad_prompt_and_its_answer_share_a_line() {
    // The prompt and the read are two lines of one function, so they
    // are one statement and the answer lands on the prompt's line.
    let input = concat!(
        "\u{2207}R\u{2190}GREET;WHO\n",
        "\u{235e}\u{2190}'NAME: '\n",
        "WHO\u{2190}\u{235e}\n",
        "R\u{2190}'HELLO ',WHO\n",
        "\u{2207}\n",
        "GREET\n",
        "MIKE\n",
        ")OFF\n"
    );
    let (text, code) = run_stdin(&[], input);
    assert_eq!(code, 0);
    assert!(text.contains("NAME: MIKE\n"), "got: {text}");
    assert!(text.contains("HELLO MIKE\n"), "got: {text}");
}

#[test]
fn a_statement_ends_the_line_quote_quad_left_open() {
    // Two statements in immediate execution, so the first one's line
    // is finished before the second reads. See docs/parity.md.
    let input = "\u{235e}\u{2190}'NAME: '\nWHO\u{2190}\u{235e}\nMIKE\nWHO\n)OFF\n";
    let (text, _) = run_stdin(&[], input);
    assert!(text.contains("NAME: \n"), "got: {text}");
    assert!(text.contains("\nMIKE\n"), "got: {text}");
}

#[test]
fn a_read_with_nothing_left_in_the_script_is_an_interrupt() {
    let (text, code) = run_stdin(&[], "1+\u{2395}\n");
    assert_eq!(code, 0);
    assert!(text.contains("INTERRUPT"), "got: {text}");
}
