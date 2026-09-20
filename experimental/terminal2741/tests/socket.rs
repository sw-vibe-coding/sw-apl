use apl_2741_prototype::{
    service,
    wire::{self, Frame},
};
use std::{
    io::BufReader,
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn connect() -> (TcpStream, BufReader<TcpStream>, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let worker = thread::spawn(move || {
        let (socket, _) = listener.accept().unwrap();
        service::serve(socket, std::env::temp_dir()).unwrap();
    });
    let stream = TcpStream::connect(address).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    let reader = BufReader::new(stream.try_clone().unwrap());
    (stream, reader, worker)
}

fn read(reader: &mut BufReader<TcpStream>) -> Frame {
    wire::receive(reader).unwrap().unwrap()
}

#[test]
fn session_supports_quad_quote_quad_definitions_and_underscored_names() {
    let (mut socket, mut reader, worker) = connect();
    assert_eq!(read(&mut reader).prompt.as_deref(), Some("      "));
    wire::send(&mut socket, &"A←⎕").unwrap();
    assert!(read(&mut reader).lines.iter().any(|s| s.contains('⎕')));
    wire::send(&mut socket, &"6×7").unwrap();
    read(&mut reader);
    wire::send(&mut socket, &"A").unwrap();
    assert_eq!(read(&mut reader).lines, ["42"]);
    wire::send(&mut socket, &"⍞←'NAME: '").unwrap();
    assert_eq!(read(&mut reader).prompt.as_deref(), Some("NAME: "));
    wire::send(&mut socket, &"B←⍞").unwrap();
    read(&mut reader);
    wire::send(&mut socket, &"HELLO").unwrap();
    read(&mut reader);
    wire::send(&mut socket, &"B").unwrap();
    assert_eq!(read(&mut reader).lines, ["HELLO"]);
    for line in ["A\u{332}←9", "∇R←DOUBLE X", "R←X+X", "∇"] {
        wire::send(&mut socket, &line).unwrap();
        read(&mut reader);
    }
    wire::send(&mut socket, &"DOUBLE A\u{332}").unwrap();
    assert_eq!(read(&mut reader).lines, ["18"]);
    wire::send(&mut socket, &")OFF").unwrap();
    assert!(read(&mut reader).off);
    worker.join().unwrap();
}

#[test]
fn disconnect_during_quad_read_releases_session() {
    let (mut socket, mut reader, worker) = connect();
    read(&mut reader);
    wire::send(&mut socket, &"⎕").unwrap();
    read(&mut reader);
    socket.shutdown(std::net::Shutdown::Both).unwrap();
    drop(socket);
    drop(reader);
    worker.join().unwrap();
}
