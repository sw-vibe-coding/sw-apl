#![cfg(unix)]

use apl_wire::{Frame, receive, send};
use std::{
    fs::File,
    io::{BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    os::fd::FromRawFd,
    process::{Child, Command, Stdio},
    sync::mpsc::{self, Receiver},
    thread,
    time::{Duration, Instant},
};

struct Client(Child);
impl Drop for Client {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn launch(address: &str) -> (Client, File, Receiver<Vec<u8>>) {
    let (mut master_fd, mut slave_fd) = (-1, -1);
    let mut size = libc::winsize {
        ws_row: 24,
        ws_col: 100,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    // SAFETY: openpty receives valid out pointers and a valid window size.
    let result = unsafe {
        libc::openpty(
            &raw mut master_fd,
            &raw mut slave_fd,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &raw mut size,
        )
    };
    assert_eq!(result, 0, "{}", std::io::Error::last_os_error());
    // SAFETY: successful openpty returned two owned, distinct descriptors.
    let (master, slave) = unsafe { (File::from_raw_fd(master_fd), File::from_raw_fd(slave_fd)) };
    let child = Command::new(env!("CARGO_BIN_EXE_aplterm"))
        .args(["--connect", address])
        .env("TERM", "xterm-256color")
        .stdin(Stdio::from(slave.try_clone().unwrap()))
        .stdout(Stdio::from(slave.try_clone().unwrap()))
        .stderr(Stdio::from(slave))
        .spawn()
        .unwrap();
    let (sender, receiver) = mpsc::channel();
    let mut reader = master.try_clone().unwrap();
    thread::spawn(move || {
        let mut bytes = [0; 4096];
        while let Ok(n) = reader.read(&mut bytes) {
            if n == 0 || sender.send(bytes[..n].to_vec()).is_err() {
                break;
            }
        }
    });
    (Client(child), master, receiver)
}

fn wait_for(receiver: &Receiver<Vec<u8>>, transcript: &mut Vec<u8>, needle: &str) {
    let deadline = Instant::now() + Duration::from_secs(5);
    while !transcript
        .windows(needle.len())
        .any(|w| w == needle.as_bytes())
    {
        let chunk = receiver
            .recv_timeout(deadline.saturating_duration_since(Instant::now()))
            .unwrap_or_else(|error| {
                panic!(
                    "{error}: expected {needle:?}, saw {:?}",
                    String::from_utf8_lossy(transcript)
                )
            });
        transcript.extend(chunk);
    }
}

#[test]
fn actual_terminal_composes_before_enter_and_sends_unicode_only() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let (mut client, mut keyboard, output) = launch(&listener.local_addr().unwrap().to_string());
    let deadline = Instant::now() + Duration::from_secs(5);
    let (mut socket, _) = loop {
        if let Ok(connection) = listener.accept() {
            break connection;
        }
        assert!(Instant::now() < deadline, "client did not connect");
        thread::sleep(Duration::from_millis(10));
    };
    // A BSD accepts a connection with the listener's O_NONBLOCK on
    // it; Linux does not. Put it back either way, or every read here
    // is a WouldBlock rather than a wait.
    socket.set_nonblocking(false).unwrap();
    socket
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    let mut input = BufReader::new(socket.try_clone().unwrap());
    let cases = [
        ("2-3=4_5+6", "2+3×4-5÷6", "2+3×4-5÷6"),
        ("O\u{1d}_", "⊖", "⊖"),
        ("/\u{1d}_", "⌿", "⌿"),
        ("\\\u{1d}_", "⍀", "⍀"),
        ("L\u{1d}K", "⍞", "⍞"),
        ("a\u{1d}F{2", "A\u{332}←2", "Ⓐ←2"),
        ("lk", "LK", "LK"),
        ("O\u{1d}P", "⍟", "⍟"),
        ("\u{1b}[200~A\u{332}←2\u{1b}[201~", "A\u{332}←2", "Ⓐ←2"),
        ("2\u{1b}+2", "2+2", "2+2"),
        ("\u{1b}OSoff", ")OFF", ")OFF"),
        ("\"wsid", ")WSID", ")WSID"),
        (":a;1'\"", "(A[1])", "(A[1])"),
        ("90()K", "90∨∧'", "90∨∧'"),
    ];
    for (index, (keys, expected, visible)) in cases.into_iter().enumerate() {
        let prompt = format!("TEST{index}> ");
        let mut transcript = Vec::new();
        send(
            &mut socket,
            &Frame {
                lines: vec![],
                prompt: Some(prompt.clone()),
                off: false,
                mode: String::new(),
                more: false,
            },
        )
        .unwrap();
        wait_for(&output, &mut transcript, &prompt);
        keyboard.write_all(keys.as_bytes()).unwrap();
        wait_for(&output, &mut transcript, &format!("{prompt}{visible}"));
        keyboard.write_all(b"\r").unwrap();
        let received: String = receive(&mut input).unwrap().unwrap();
        assert_eq!(received, expected);
        assert!(!received.contains('\u{1d}'));
    }
    send(
        &mut socket,
        &Frame {
            lines: vec![],
            prompt: None,
            off: true,
            mode: String::new(),
            more: false,
        },
    )
    .unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        if let Some(status) = client.0.try_wait().unwrap() {
            assert!(status.success());
            break;
        }
        assert!(Instant::now() < deadline, "client did not exit");
        thread::sleep(Duration::from_millis(10));
    }
}

/// Accept aplterm's connection on a fake service, as the first test does.
fn accept(listener: &TcpListener) -> TcpStream {
    let deadline = Instant::now() + Duration::from_secs(5);
    let (socket, _) = loop {
        if let Ok(connection) = listener.accept() {
            break connection;
        }
        assert!(Instant::now() < deadline, "client did not connect");
        thread::sleep(Duration::from_millis(10));
    };
    socket.set_nonblocking(false).unwrap();
    socket
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    socket
}

#[test]
fn a_busy_service_is_sent_attention_by_escape_and_by_ctrl_c() {
    // The service takes a line and goes quiet, as it does while a loop
    // runs. aplterm is waiting for a frame, not reading a line, so
    // Escape is ATTN -- not the quote it is at a prompt -- and goes
    // straight to the service as the attention object. So does Ctrl-C,
    // the CLI's interrupt key.
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let (mut client, mut keyboard, output) = launch(&listener.local_addr().unwrap().to_string());
    let mut socket = accept(&listener);
    let mut input = BufReader::new(socket.try_clone().unwrap());
    for key in [&b"\x1b"[..], &b"\x03"[..]] {
        // Anything aplterm drew before this round is not this round's
        // prompt: drop it, so the prompt below is waited for afresh.
        thread::sleep(Duration::from_millis(200));
        while output.try_recv().is_ok() {}
        let mut transcript = Vec::new();
        let prompt = "BUSY> ".to_string();
        let frame = Frame {
            lines: vec![],
            prompt: Some(prompt.clone()),
            off: false,
            mode: String::new(),
            more: false,
        };
        send(&mut socket, &frame).unwrap();
        wait_for(&output, &mut transcript, &prompt);
        // Unshifted: a 2741 types capitals from the plain keys.
        keyboard.write_all(b"spin\r").unwrap();
        let typed: String = match receive(&mut input) {
            Ok(Some(t)) => t,
            other => {
                thread::sleep(Duration::from_millis(300));
                while let Ok(more) = output.try_recv() {
                    transcript.extend(more);
                }
                panic!(
                    "no line after {key:?}: {other:?}; screen: {:?}",
                    String::from_utf8_lossy(&transcript)
                );
            }
        };
        assert_eq!(typed, "SPIN");
        // The service says nothing now. Let aplterm settle into waiting.
        thread::sleep(Duration::from_millis(300));
        keyboard.write_all(key).unwrap();
        let line = apl_wire::read(&mut input).unwrap().unwrap();
        assert!(
            apl_wire::attention(&line),
            "{key:?} sent {line:?}, not ATTN"
        );
    }
    let bye = Frame {
        lines: vec![],
        prompt: None,
        off: true,
        mode: String::new(),
        more: false,
    };
    send(&mut socket, &bye).unwrap();
    let _ = client.0.wait();
}
