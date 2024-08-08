#![allow(unused_imports)]

use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::{Path, PathBuf};

use anyhow::{Error, Result};


pub mod config;


pub fn run(config: config::Config) -> Result<()> {
    let num_files = config.files.len();
    for (file_num, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename.display(), err),
            Ok(mut file) => {
                // Print header between files, if needed
                if num_files > 1 {
                    print_header(&filename, file_num);
                }

                // Print first part of file
                if let Some(num_bytes) = config.bytes {
                    print_bytes(&mut file, num_bytes)?;
                } else {
                    print_lines(&mut file, config.lines)?;
                }
            },
        }
    }

    Ok(())
}


/// Print first `num_bytes` from open file
fn print_bytes(file: &mut Box<dyn BufRead>, num_bytes: u64) -> Result<()> {
    // Print bytes, as is
    let mut buffer = vec![0; num_bytes as usize];
    let bytes_read = file.read(&mut buffer)?;
    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
    Ok(())
}


/// Print file's name
fn print_header(filename: &PathBuf, file_num: usize) {
    println!("{}==> {} <==", if file_num > 0 { "\n" } else { "" }, filename.display());
}


/// Print `num_lines` from open file, preserving existing newlines
fn print_lines(file: &mut Box<dyn BufRead>, num_lines: u64) -> Result<()> {
    let mut line = String::new();
    for _ in 0..num_lines {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        print!("{line}");
        line.clear();
    }
    Ok(())
}


/// Open filename, or stdin if "-" given.
fn open(filename: &PathBuf) -> Result<Box<dyn BufRead>> {
    let reader: Box<dyn BufRead> = if filename == Path::new("-") {
        Box::new(BufReader::new(io::stdin()))
    } else {
        Box::new(BufReader::new(File::open(filename)?))
    };

    Ok(reader)
}
