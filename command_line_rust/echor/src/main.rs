
/// Command-Line Rust
/// Chapter 2
///
/// Process command-line arguments, deal with errors, stack, and heap memory.


use clap::Parser;

#[derive(Debug, Parser)]
#[command(about, version)]
struct Args {
    /// Do not output the trailing newline
    #[arg(short='n')]
    omit_newline: bool,

    /// Echo text to standard output
    text: Vec<String>,
}


fn main() {
    let args = Args::parse();

    dbg!(&args);
}
