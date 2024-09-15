#![allow(unused_variables)]

use anyhow::Result;
use clap::Parser;

use huhu::{count_lines, open};


#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    path: std::path::PathBuf,
}


fn main() -> Result<()> {
    let args = Args::parse();
    let file = open(args.path.to_str().unwrap()).unwrap();
    let count = count_lines(file).unwrap();
    println!("There were {count} lines in given log file");
    Ok(())
}
