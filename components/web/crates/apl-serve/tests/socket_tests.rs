//! A session over a real socket: the transport the CLI terminal and
//! `nc` use.

use apl_serve::serve;
use apl_session::QUOTA;
use apl_wire::{Frame, Socket, receive, send};
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
        let link = Socket::new(socket).unwrap();
        serve(Box::new(link), (QUOTA, std::env::temp_dir())).unwrap();
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
