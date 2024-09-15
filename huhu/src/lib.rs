#![allow(dead_code)]

use std::fs::File;
use std::io::{self, BufRead, BufReader};

use anyhow::Result;


pub fn count_lines(file: impl BufRead) -> Result<usize> {
    let mut count: usize = 0;
    for _ in file.lines() {
        count += 1;
    }
    Ok(count)
}


/// Open file or stdin
pub fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
