
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use anyhow::{Result};
use clap::Parser;


/// Command-line entry point
pub fn run() -> Result<()> {
    let args = Args::parse();
    for filename in &args.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                if let Ok(info) = count(file) {
                    println!(
                        "{:>8}{:>8}{:>8} {}",
                        info.num_lines,
                        info.num_words,
                        info.num_bytes,
                        filename,
                    );
                }
            },
        }
    }

    Ok(())
}


/// Command-line arguments
#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Show line count
    #[arg(short, long)]
    lines: bool,

    /// Show word count
    #[arg(short, long)]
    words: bool,

    /// Show byte count
    #[arg(short('c'), long)]
    bytes: bool,

    /// Show character count
    #[arg(short('m'), long, conflicts_with("bytes"))]
    chars: bool,
}


/// Counts for one file
#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}


pub fn count(mut file: impl BufRead) -> Result<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    let mut line = String::new();
    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}


/// Open file or stdin
fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}


#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_count() {
        let lines = "I don't want the world.\nI just want your half.";
        let buffer = Cursor::new(lines);
        let info = count(buffer);
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 2,
            num_words: 10,
            num_chars: 46,
            num_bytes: 46,
        };
        assert_eq!(info.unwrap(), expected);
    }
}
