#![allow(dead_code)]

use std::fs::File;
use std::io::{self, BufRead, BufReader};

use anyhow::Result;


/// Open file or stdin
fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
