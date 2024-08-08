#![allow(unused_imports)]

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};

use anyhow::{Error, Result};


pub mod config;


pub fn run(config: config::Config) -> Result<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("{:?}: {}", filename, err),
            Ok(mut file) => {
                let mut line = String::new();
                for _ in 0..config.lines {
                    let bytes = file.read_line(&mut line)?;
                    if bytes == 0 {
                        break;
                    }
                    print!("{}, {}", line.capacity(), line);
                    line.clear();
                }
            },
        }
    }

    Ok(())
}


/// Open filename, or stdin if "-" given.
pub fn open(filename: &PathBuf) -> Result<Box<dyn BufRead>> {
    let reader: Box<dyn BufRead> = if filename == Path::new("-") {
        Box::new(BufReader::new(io::stdin()))
    } else {
        Box::new(BufReader::new(File::open(filename)?))
    };

    Ok(reader)
}
