
use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};


/// Options for line numbering behaviour
pub enum LineNumbers {
    DontShow,
    NonBlank,
    ShowAll,
}


/// Concatenate given file to standard output, optionally adding line numbers
pub fn concatenate(file: impl BufRead, line_numbers: &LineNumbers) -> Result<()> {
    let mut line_num: usize = 0;
    for line in file.lines() {
        let line = line?;
        match line_numbers {
            LineNumbers::DontShow => println!("{line}"),
            LineNumbers::NonBlank => {
                if line.trim().is_empty() {
                    println!();
                } else {
                    line_num += 1;
                    println!("{:6}\t{}", line_num, line);
                }
            },
            LineNumbers::ShowAll => {
                line_num += 1;
                println!("{:6}\t{}", line_num, line);
            },
        }
    }
    Ok(())
}


/// Open filename, or stdin if "-" given.
pub fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
