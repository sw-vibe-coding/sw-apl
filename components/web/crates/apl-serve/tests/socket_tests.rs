//! A session over a real socket: the transport the CLI terminal and
//! `nc` use.

use apl_attn::{Flag, attend};
use apl_serve::serve;
use apl_session::{Files, Host, Mode, QUOTA};
use apl_wire::{ATTENTION, Frame, Socket, receive, send};
use std::{
    io::{BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn connect() -> (TcpStream, BufReader<TcpStream>, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let worker = thread::spawn(move || {
        let (socket, _) = listener.accept().unwrap();
        // As the service does: the session's flag on the session's thread.
        let attn = Flag::default();
        attend(Box::new(attn.clone()));
        let link = Socket::new(socket, attn).unwrap();
        serve(
            Box::new(link),
            Host {
                quota: QUOTA,
                store: Box::new(Files(std::env::temp_dir())),
                mode: Mode::A,
            },
        )
        .unwrap();
    });
    let stream = TcpStream::connect(address).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    let reader = BufReader::new(stream.try_clone().unwrap());
    (stream, reader, worker)
}

fn read(reader: &mut BufReader<TcpStream>) -> Frame {
    receive(reader).unwrap().unwrap()
}

#[test]
fn session_supports_quad_quote_quad_definitions_and_underscored_names() {
    let (mut socket, mut reader, worker) = connect();
    assert_eq!(read(&mut reader).prompt.as_deref(), Some("      "));
    send(&mut socket, &"A←⎕").unwrap();
    assert!(read(&mut reader).lines.iter().any(|s| s.contains('⎕')));
    send(&mut socket, &"6×7").unwrap();
    read(&mut reader);
    send(&mut socket, &"A").unwrap();
    assert_eq!(read(&mut reader).lines, ["42"]);
    send(&mut socket, &"⍞←'NAME: '").unwrap();
    assert_eq!(read(&mut reader).prompt.as_deref(), Some("NAME: "));
    send(&mut socket, &"B←⍞").unwrap();
    read(&mut reader);
    send(&mut socket, &"HELLO").unwrap();
    read(&mut reader);
    send(&mut socket, &"B").unwrap();
    assert_eq!(read(&mut reader).lines, ["HELLO"]);
    for line in ["A\u{332}←9", "∇R←DOUBLE X", "R←X+X", "∇"] {
        send(&mut socket, &line).unwrap();
        read(&mut reader);
    }
    send(&mut socket, &"DOUBLE A\u{332}").unwrap();
    assert_eq!(read(&mut reader).lines, ["18"]);
    send(&mut socket, &")OFF").unwrap();
    assert!(read(&mut reader).off);
    worker.join().unwrap();
}

#[test]
fn disconnect_during_quad_read_releases_session() {
    let (mut socket, mut reader, worker) = connect();
    read(&mut reader);
    send(&mut socket, &"⎕").unwrap();
    read(&mut reader);
    socket.shutdown(std::net::Shutdown::Both).unwrap();
    drop(socket);
    drop(reader);
    worker.join().unwrap();
}

#[test]
fn a_line_that_is_not_json_is_taken_verbatim() {
    // What `nc host port` sends: the operator types APL and presses
    // return, with nothing to quote it. The transcript comes back
    // readable, which is what makes nc the emergency client.
    let (mut socket, mut reader, worker) = connect();
    read(&mut reader);
    socket.write_all("2+2\n".as_bytes()).unwrap();
    assert_eq!(read(&mut reader).lines, ["4"]);
    socket.write_all(")OFF\n".as_bytes()).unwrap();
    assert!(read(&mut reader).off);
    worker.join().unwrap();
}

/// Define a loop that never ends, and start it.
fn spin(socket: &mut TcpStream, reader: &mut BufReader<TcpStream>) {
    for line in ["∇SPIN", "X←1", "→1", "∇"] {
        send(socket, &line).unwrap();
        read(reader);
    }
    send(socket, &"SPIN").unwrap();
}

#[test]
fn attention_over_the_socket_stops_a_loop() {
    // The loop is running, so the session is reading nothing; only the
    // connection's own reading thread can see this line.
    let (mut socket, mut reader, worker) = connect();
    read(&mut reader);
    spin(&mut socket, &mut reader);
    thread::sleep(Duration::from_millis(200));
    socket
        .write_all(format!("{ATTENTION}\n").as_bytes())
        .unwrap();
    let stopped = read(&mut reader);
    assert!(
        stopped.lines.iter().any(|l| l.contains("INTERRUPT")),
        "{:?}",
        stopped.lines
    );
    // And the session carries on.
    send(&mut socket, &"2+2").unwrap();
    assert_eq!(read(&mut reader).lines, ["4"]);
    send(&mut socket, &")OFF").unwrap();
    read(&mut reader);
    worker.join().unwrap();
}

#[test]
fn attention_stops_one_session_on_a_server_and_not_another() {
    // Two terminals, two sessions, both looping. Only one is sent ATTN,
    // and the other must still be running afterwards -- which is the
    // reason the flag stopped being one global.
    let (mut one, mut one_reader, one_worker) = connect();
    let (mut two, mut two_reader, two_worker) = connect();
    read(&mut one_reader);
    read(&mut two_reader);
    spin(&mut one, &mut one_reader);
    spin(&mut two, &mut two_reader);
    thread::sleep(Duration::from_millis(200));
    one.write_all(format!("{ATTENTION}\n").as_bytes()).unwrap();
    let stopped = read(&mut one_reader);
    assert!(
        stopped.lines.iter().any(|l| l.contains("INTERRUPT")),
        "{:?}",
        stopped.lines
    );
    two_reader
        .get_ref()
        .set_read_timeout(Some(Duration::from_millis(500)))
        .unwrap();
    assert!(
        receive::<Frame>(&mut two_reader).is_err(),
        "the session not asked stopped too"
    );
    two_reader
        .get_ref()
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    two.write_all(format!("{ATTENTION}\n").as_bytes()).unwrap();
    assert!(
        read(&mut two_reader)
            .lines
            .iter()
            .any(|l| l.contains("INTERRUPT"))
    );
    for (socket, reader) in [(&mut one, &mut one_reader), (&mut two, &mut two_reader)] {
        send(socket, &")OFF").unwrap();
        read(reader);
    }
    one_worker.join().unwrap();
    two_worker.join().unwrap();
}

#[test]
fn the_service_hangs_up_after_off() {
    // The connection is read on a thread that keeps its own handle, so
    // the service has to close it on purpose; otherwise nc would wait
    // forever after )OFF.
    let (mut socket, mut reader, worker) = connect();
    read(&mut reader);
    socket.write_all(")OFF\n".as_bytes()).unwrap();
    assert!(read(&mut reader).off);
    worker.join().unwrap();
    assert!(
        receive::<Frame>(&mut reader).unwrap().is_none(),
        "the connection stayed open"
    );
}
