#![allow(unused_variables)]

use clap::Parser;


#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// Input file(s)
    #[arg(default_value="-")]
    files: Vec<String>,
}


fn main() {
    let args = Args::parse();
    println!("{args:?}");
}
