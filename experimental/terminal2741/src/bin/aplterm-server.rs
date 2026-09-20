use apl_2741_prototype::service;
const HELP: &str = "aplterm-server [--listen 127.0.0.1:2741] [--library PATH]
Prototype TCP server; one APL session per connection. Default library: checkout root.";
use std::{
    io,
    net::{TcpListener, TcpStream},
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    thread,
};

fn main() -> io::Result<()> {
    let Some((address, library)) = arguments()? else {
        return Ok(());
    };
    let listener = TcpListener::bind(&address)?;
    println!("APL server listening on {}", listener.local_addr()?);
    let active = Arc::new(AtomicUsize::new(0));
    for stream in listener.incoming() {
        let stream = stream?;
        if active.load(Ordering::Relaxed) >= 16 {
            drop(stream);
            continue;
        }
        spawn(stream, library.clone(), active.clone());
    }
    Ok(())
}

fn arguments() -> io::Result<Option<(String, PathBuf)>> {
    let mut args = std::env::args().skip(1);
    let mut address = "127.0.0.1:2741".to_string();
    let mut library = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => {
                println!("{HELP}");
                return Ok(None);
            }
            "--listen" | "--library" => {
                let value = args
                    .next()
                    .ok_or_else(|| io::Error::other(format!("missing value for {arg}")))?;
                if arg == "--listen" {
                    address = value;
                } else {
                    library = PathBuf::from(value);
                }
            }
            _ => return Err(io::Error::other(format!("unknown option: {arg}"))),
        }
    }
    Ok(Some((address, library)))
}

fn spawn(stream: TcpStream, library: PathBuf, active: Arc<AtomicUsize>) {
    active.fetch_add(1, Ordering::Relaxed);
    thread::spawn(move || {
        let result = std::panic::catch_unwind(|| service::serve(stream, library));
        active.fetch_sub(1, Ordering::Relaxed);
        if let Ok(Err(error)) = result {
            eprintln!("Connection ended: {error}");
        }
    });
}
