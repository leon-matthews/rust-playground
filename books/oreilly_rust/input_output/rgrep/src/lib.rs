#![allow(dead_code)]
#![allow(unused_variables)]

use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::path::PathBuf;


pub fn command_line() -> Result<(), Box<dyn Error>> {
    // Get command lines arguments...
    let mut args = std::env::args().skip(1);
    let target = match args.next() {
        Some(s) => s,
        None => Err("usage: rgrep PATTERN FILE [FILES] ...")?,
    };
    let files: Vec<PathBuf> = args.map(PathBuf::from).collect();

    // No files? Search stdin instead.
    if files.is_empty() {
        let stdin = io::stdin();
        grep(&target, stdin.lock())?;
    } else {
        for file in files {
            let f = File::open(file)?;
            grep(&target, BufReader::new(f))?;
        }
    }
    Ok(())
}


/// Print lines in `reader` that contain the `target` string.
fn grep<R>(target: &str, reader: R) -> io::Result<()> where R: BufRead
{
    for result in reader.lines() {
        let line = result?;
        if line.contains(target) {
            println!("{}", line);
        }
    }
    Ok(())
}
