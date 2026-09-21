//! The browser's half of the service, end to end: the page, the
//! upgrade, and the same protocol over a WebSocket.
//!
//! The real binary is run, on ports the kernel picks, so what is
//! checked is what a reader gets -- including that both listeners
//! carry one protocol and a session on each is its own.

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::process::{Child, Command, Stdio};

use apl_wire::Frame;
use tungstenite::{Message, WebSocket, connect, stream::MaybeTlsStream};

/// The running service, stopped when the test ends however it ends.
struct Service(Child);

impl Drop for Service {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

/// Start the service on ports the kernel picks, and read back where
/// it put them. Library 0 is a scratch directory: a test must never
/// write into the checkout's `work/`.
fn start() -> (Service, String, String) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_sw-apl-server"))
        .args(["--listen", "127.0.0.1:0", "--http", "127.0.0.1:0"])
        .args(["--library".as_ref(), std::env::temp_dir().as_os_str()])
        .stdout(Stdio::piped())
        .spawn()
        .expect("the service starts");
    let mut said = BufReader::new(child.stdout.take().expect("stdout is piped"));
    let (mut line, mut browser) = (String::new(), String::new());
    said.read_line(&mut line).unwrap();
    said.read_line(&mut browser).unwrap();
    let at = |said: &str| said.rsplit(' ').next().unwrap().trim().to_string();
    (Service(child), at(&line), at(&browser))
}

/// Fetch a page over HTTP/1.1, the whole of it.
fn fetch(url: &str) -> String {
    let host = url.trim_start_matches("http://").trim_end_matches('/');
    let mut socket = TcpStream::connect(host).unwrap();
    write!(
        socket,
        "GET / HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n"
    )
    .unwrap();
    let mut page = String::new();
    socket.read_to_string(&mut page).unwrap();
    page
}

/// One frame from the terminal's end.
fn frame(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) -> Frame {
    loop {
        if let Message::Text(text) = socket.read().unwrap() {
            return serde_json::from_str(&text).unwrap();
        }
    }
}

/// Type one line at the browser terminal.
fn typed(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>, line: &str) {
    let json = serde_json::to_string(line).unwrap();
    socket.send(Message::text(json)).unwrap();
}

#[test]
fn the_service_hands_a_browser_a_terminal_and_then_a_session() {
    let (_service, _terminals, browser) = start();
    let page = fetch(&browser);
    assert!(
        page.starts_with("HTTP/1.1 200 OK"),
        "the page is served: {page:.40}"
    );
    assert!(
        page.contains("<title>sw-apl</title>"),
        "and it is the terminal"
    );
    assert!(
        page.contains("Nothing typed here leaves it"),
        "and it says where the interpreter is running"
    );

    let host = browser.trim_start_matches("http://").trim_end_matches('/');
    let (mut socket, _) = connect(format!("ws://{host}/")).expect("the upgrade is accepted");
    assert_eq!(frame(&mut socket).prompt.as_deref(), Some("      "));
    typed(&mut socket, "2+2");
    assert_eq!(frame(&mut socket).lines, ["4"]);

    // The blocking read, over a WebSocket: the statement asks, and
    // the answer is the line typed after it.
    typed(&mut socket, "X←⎕");
    assert_eq!(frame(&mut socket).lines, ["⎕:"]);
    typed(&mut socket, "6×7");
    frame(&mut socket);
    typed(&mut socket, "X");
    assert_eq!(frame(&mut socket).lines, ["42"]);

    typed(&mut socket, ")OFF");
    assert!(frame(&mut socket).off, "and the session can be signed off");
}

#[test]
fn a_terminal_on_the_socket_and_one_in_a_browser_are_separate_sessions() {
    let (_service, terminals, browser) = start();
    let host = browser.trim_start_matches("http://").trim_end_matches('/');
    let (mut web, _) = connect(format!("ws://{host}/")).unwrap();
    let mut line = TcpStream::connect(&terminals).unwrap();
    let mut said = BufReader::new(line.try_clone().unwrap());
    let mut answer = || {
        let mut text = String::new();
        said.read_line(&mut text).unwrap();
        serde_json::from_str::<Frame>(&text).unwrap()
    };
    answer();
    frame(&mut web);

    typed(&mut web, "X←1");
    frame(&mut web);
    writeln!(line, "X").unwrap();
    assert_eq!(
        answer().lines,
        ["VALUE ERROR", "      X", "      ^"],
        "each connection has a workspace of its own"
    );
}

#[test]
fn attention_over_the_websocket_stops_a_loop() {
    // The same ATTN as on a raw socket, sent from the browser's end
    // while a loop is running and the session is reading nothing.
    let (_service, _line, browser) = start();
    let url = browser.replace("http://", "ws://");
    let (mut socket, _) = connect(url.as_str()).expect("the upgrade");
    frame(&mut socket);
    for line in ["∇SPIN", "X←1", "→1", "∇"] {
        typed(&mut socket, line);
        frame(&mut socket);
    }
    typed(&mut socket, "SPIN");
    std::thread::sleep(std::time::Duration::from_millis(200));
    socket.send(Message::text(apl_wire::ATTENTION)).unwrap();
    let stopped = frame(&mut socket);
    assert!(
        stopped.lines.iter().any(|l| l.contains("INTERRUPT")),
        "{:?}",
        stopped.lines
    );
    typed(&mut socket, "2+2");
    assert_eq!(frame(&mut socket).lines, ["4"]);
}

#[test]
fn the_page_sends_attention_on_escape() {
    // The served page is the client; it has to send the object.
    let (_service, _line, browser) = start();
    let page = fetch(&browser);
    assert!(page.contains("attn: true"), "the page never sends ATTN");
    assert!(page.contains("\"Escape\""), "and not on Escape");
}
