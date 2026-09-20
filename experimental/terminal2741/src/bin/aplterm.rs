use apl_2741_prototype::{
    keyboard::{Keyboard, display},
    terminal,
    wire::{self, Frame},
};
use std::{
    io::{self, BufReader, IsTerminal},
    net::TcpStream,
};

const HELP: &str = "aplterm [--connect 127.0.0.1:2741] [--keymap FILE] [--literal]
2741 key translation; Ctrl-] overstrikes, Shift-F underbars.
F2 toggles literal Unicode input; Esc quotes the next key; F4 types ).
Backspace deletes, arrows edit, Up/Down recall history. Ctrl-C clears input.
Ctrl-D on empty input disconnects; )OFF signs off. Paste is literal Unicode.";

fn main() -> io::Result<()> {
    let mut keyboard = Keyboard::default();
    let Some(address) = arguments(&mut keyboard)? else {
        return Ok(());
    };
    if !io::stdin().is_terminal() {
        return Err(io::Error::other("client requires an interactive terminal"));
    }
    let socket = TcpStream::connect(&address)?;
    socket.set_nodelay(true)?;
    println!(
        "2741 connected to {address}. Ctrl-] overstrike; F2 literal mode; F4 command ); Ctrl-D disconnect."
    );
    run(socket, keyboard)
}

fn arguments(keyboard: &mut Keyboard) -> io::Result<Option<String>> {
    let mut address = "127.0.0.1:2741".to_string();
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--literal" => keyboard.literal = true,
            "--help" | "-h" => {
                println!("{HELP}");
                return Ok(None);
            }
            "--connect" | "--keymap" => {
                let value = args
                    .next()
                    .ok_or_else(|| io::Error::other(format!("missing value for {arg}")))?;
                option(&arg, value, &mut address, keyboard)?;
            }
            _ => return Err(io::Error::other(format!("unknown option: {arg}"))),
        }
    }
    Ok(Some(address))
}

fn option(
    arg: &str,
    value: String,
    address: &mut String,
    keyboard: &mut Keyboard,
) -> io::Result<()> {
    if arg == "--connect" {
        *address = value;
    } else {
        keyboard.map =
            serde_json::from_str(&std::fs::read_to_string(value)?).map_err(io::Error::other)?;
    }
    Ok(())
}

fn run(mut socket: TcpStream, mut keyboard: Keyboard) -> io::Result<()> {
    let mut reader = BufReader::new(socket.try_clone()?);
    let mut history = Vec::new();
    while let Some(frame) = wire::receive::<Frame>(&mut reader)? {
        for line in &frame.lines {
            println!("{}", display(line));
        }
        if frame.off {
            break;
        }
        if let Some(prompt) = frame.prompt {
            let Some(line) = terminal::read_line(&prompt, &mut keyboard, &mut history)? else {
                break;
            };
            wire::send(&mut socket, &line)?;
        }
    }
    Ok(())
}
