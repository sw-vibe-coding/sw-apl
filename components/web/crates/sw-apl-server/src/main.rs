//! `sw-apl-server`: the APL\360 service a 2741 dials into.
//!
//! A local process holding one session per connection. Two listeners
//! carry one protocol: a TCP port for `aplterm` (and for `nc`), and
//! an HTTP port that hands a browser the terminal page and then talks
//! the same protocol over a WebSocket.
//!
//! Local by default, and deliberately. A service holding `)SAVE` and
//! `)LOAD` is a file-writing primitive for whoever can reach it, and
//! APL\360's own answer to that -- sign-on numbers and passwords --
//! is out of scope here. Both listeners bind to the loopback address
//! unless told otherwise.

mod http;
mod listen;
mod socket;

use std::net::TcpListener;
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::thread;

use clap::Parser;

use listen::{Dialled, Service, accept};

/// Full `-V` / `--version` block: name, copyright, license,
/// repository, then build information from `build.rs`.
pub const VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    "\nCopyright (c) 2026 Michael A Wright\n",
    "License: MIT\n",
    "Repository: https://github.com/sw-vibe-coding/sw-apl\n\n",
    "Build Information:\n  Host: ",
    env!("BUILD_HOST"),
    "\n  Commit: ",
    env!("GIT_HASH"),
    "\n  Timestamp: ",
    env!("BUILD_TIMESTAMP"),
);

/// The APL\360 service: one session per connection, over a socket or
/// a browser.
#[derive(Parser, Debug)]
#[command(
    name = "sw-apl-server",
    version = VERSION,
    about,
    long_about = "A local APL\\360 service: one session per connection, over a \\
                  socket for aplterm and over a WebSocket for a browser. The \\
                  session blocks while a statement waits for a line, which is \\
                  what makes quad, quote-quad and the del editor work.",
    after_long_help = include_str!("cli_help.txt")
)]
pub struct Args {
    /// Where terminals speaking the line protocol connect: aplterm,
    /// or nc.
    #[arg(long, value_name = "ADDR", default_value = "127.0.0.1:2741")]
    pub listen: String,

    /// Where a browser fetches the terminal page and opens its
    /// WebSocket.
    #[arg(long, value_name = "ADDR", default_value = "127.0.0.1:8360")]
    pub http: String,

    /// Where the workspace libraries are: the directory holding
    /// work/ (library 0) and ws/ (the shipped libraries). Every
    /// session saves and loads under it.
    #[arg(long, value_name = "DIR", default_value = ".")]
    pub library: PathBuf,

    /// Workspace size in bytes, for every session.
    #[arg(long = "ws-size", value_name = "BYTES", default_value_t = apl_session::QUOTA)]
    pub ws_size: usize,

    /// How many sessions may be held at once. Each is a thread.
    #[arg(long, value_name = "N", default_value_t = 16)]
    pub sessions: usize,
}

fn main() -> ExitCode {
    let args = Args::parse();
    match start(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("sw-apl-server: {err}");
            ExitCode::FAILURE
        }
    }
}

/// Bind both listeners before announcing either, so a port already
/// taken is an error rather than half a service. The browser
/// listener is run on this thread and the line listener on another.
fn start(args: &Args) -> std::io::Result<()> {
    let (line, web) = (
        TcpListener::bind(&args.listen)?,
        TcpListener::bind(&args.http)?,
    );
    let service = Service {
        ws: (args.ws_size, args.library.clone()),
        held: Arc::new(AtomicUsize::new(0)),
        limit: args.sessions,
    };
    println!("sw-apl-server: terminals on {}", line.local_addr()?);
    println!("sw-apl-server: browser at http://{}/", web.local_addr()?);
    let terminals = service.clone();
    thread::spawn(move || accept(&line, &terminals, Dialled::Line));
    accept(&web, &service, Dialled::Browser)
}
