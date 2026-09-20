use crate::wire::{self, Frame};
use apl_session::{Console, INDENT, Session, Shown};
use std::{
    cell::RefCell,
    io::{self, BufReader},
    net::TcpStream,
    path::PathBuf,
    rc::Rc,
};

#[derive(Debug)]
struct Connection {
    reader: BufReader<TcpStream>,
    writer: TcpStream,
    closed: bool,
}

impl Connection {
    fn ask(&mut self, lines: Vec<String>, prompt: String) -> io::Result<Option<String>> {
        wire::send(
            &mut self.writer,
            &Frame {
                lines,
                prompt: Some(prompt),
                off: false,
            },
        )?;
        let line: Option<String> = wire::receive(&mut self.reader)?;
        if line
            .as_ref()
            .is_some_and(|s| s.chars().any(char::is_control))
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected a composed Unicode line",
            ));
        }
        Ok(line)
    }
}

#[derive(Debug)]
struct SocketConsole(Rc<RefCell<Connection>>);

impl Console for SocketConsole {
    fn read(&mut self, shown: &Shown, prompt: &str) -> Option<String> {
        let (finished, open) = shown.split();
        let mut lines = finished.to_vec();
        if !prompt.is_empty() {
            lines.push(prompt.to_string());
        }
        let mut connection = self.0.borrow_mut();
        let result = connection
            .ask(lines, open.unwrap_or(INDENT).to_string())
            .ok()
            .flatten();
        connection.closed = result.is_none();
        result
    }
}

/// A session is created inside its connection thread, never shared across clients.
pub fn serve(stream: TcpStream, library: PathBuf) -> io::Result<()> {
    stream.set_nodelay(true)?;
    let connection = Rc::new(RefCell::new(Connection {
        reader: BufReader::new(stream.try_clone()?),
        writer: stream,
        closed: false,
    }));
    let mut session = Session::attached(Box::new(SocketConsole(connection.clone())));
    session.ws.libraries = library;
    run(&mut session, &connection)
}

fn run(session: &mut Session, connection: &Rc<RefCell<Connection>>) -> io::Result<()> {
    let mut lines = Vec::new();
    let mut prompt = session.prompt();
    loop {
        let Some(line) = connection.borrow_mut().ask(lines, prompt)? else {
            return Ok(());
        };
        let reply = session.respond(&line);
        if connection.borrow().closed {
            return Ok(());
        }
        if reply.off {
            let frame = Frame {
                lines: reply.lines,
                prompt: None,
                off: true,
            };
            return wire::send(&mut connection.borrow_mut().writer, &frame);
        }
        let (finished, open) = reply.split();
        lines = finished.to_vec();
        prompt = open.map_or_else(|| session.prompt(), str::to_string);
    }
}
