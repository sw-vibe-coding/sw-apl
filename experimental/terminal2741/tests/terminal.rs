#![cfg(unix)]

use apl_2741_prototype::wire::{self, Frame};
use std::{
    fs::File,
    io::{BufReader, Read, Write},
    net::TcpListener,
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
    let size = libc::winsize {
        ws_row: 24,
        ws_col: 100,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    // SAFETY: openpty receives valid out pointers and a valid window size.
    let result = unsafe {
        libc::openpty(
            &mut master_fd,
            &mut slave_fd,
            std::ptr::null_mut(),
            std::ptr::null(),
            &size,
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
        wire::send(
            &mut socket,
            &Frame {
                lines: vec![],
                prompt: Some(prompt.clone()),
                off: false,
            },
        )
        .unwrap();
        wait_for(&output, &mut transcript, &prompt);
        keyboard.write_all(keys.as_bytes()).unwrap();
        wait_for(&output, &mut transcript, &format!("{prompt}{visible}"));
        keyboard.write_all(b"\r").unwrap();
        let received: String = wire::receive(&mut input).unwrap().unwrap();
        assert_eq!(received, expected);
        assert!(!received.contains('\u{1d}'));
    }
    wire::send(
        &mut socket,
        &Frame {
            lines: vec![],
            prompt: None,
            off: true,
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
