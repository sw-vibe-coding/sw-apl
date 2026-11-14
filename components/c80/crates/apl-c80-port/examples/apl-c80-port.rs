//! `apl-c80-port FILE` prints FILE, a workspace in glyphs, as (C) '80
//! would save it; `apl-c80-port --sample FILE` prints FILE, a sample,
//! with what it types in APL80's characters.

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (convert, path): (fn(&str) -> String, _) = match args.as_slice() {
        [flag, path] if flag == "--sample" => (apl_c80_port::sample, path),
        [path] => (apl_c80_port::port, path),
        _ => {
            eprintln!("usage: apl-c80-port [--sample] FILE");
            return ExitCode::FAILURE;
        }
    };
    match std::fs::read_to_string(path) {
        Ok(text) => {
            print!("{}", convert(&text));
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("apl-c80-port: {path}: {e}");
            ExitCode::FAILURE
        }
    }
}
