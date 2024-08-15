
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use anyhow::{Result};
use clap::Parser;


/// Command-line arguments
#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Input file, or stdin
    #[arg(value_name = "IN_FILE", default_value = "-")]
    in_file: String,

    /// Output file, or stdout
    #[arg(value_name = "OUT_FILE")]
    out_file: Option<String>,

    /// Show counts
    #[arg(short, long)]
    count: bool,
}


/// Open file or stdin
fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

/// Command-line entry point
pub fn run() -> Result<()> {
    let args = Args::parse();
    dbg!(&args);
    Ok(())
}
