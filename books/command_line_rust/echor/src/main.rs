/// Command-Line Rust
/// Chapter 2
///
/// Process command-line arguments, deal with errors, stack, and heap memory.

use std::io::{self, Write};

use clap::Parser;


#[derive(Debug, Parser)]
#[command(about, version)]
struct Args {
    /// Do not output the trailing newline
    #[arg(short = 'n')]
    omit_newline: bool,

    /// Echo text to standard output
    text: Vec<String>,
}


fn main() {
    let args = Args::parse();
    let ending = if args.omit_newline { "" } else { "\n" };
    let buffer = format!("{}{}", args.text.join(" "), ending).into_bytes();
    let _ = io::stdout().write_all(&buffer);
}
