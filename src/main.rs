#[macro_use]
mod macros;
mod args;
mod destinations;
mod mail;
mod render;
mod send;
mod write;
use args::{Args, Command};

static mut VERBOSE: bool = false;

fn main() {
    match run() {
        Err(None) => std::process::exit(1),
        Err(Some(x)) => {
            eprintln!("{}", x);
            std::process::exit(1);
        }
        Ok(_) => std::process::exit(0),
    }
}

fn run() -> Result<(), Option<String>> {
    let args = Args::new();
    unsafe {
        VERBOSE = args.verbose;
    }
    printlnv!("Args are {:?}.", args);
    match args.command {
        Some(config) => match config {
            Command::Send(send) => send::send(send),
            Command::Write(write) => write::write(write),
        },
        _ => Err(None),
    }
}
