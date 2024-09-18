#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::fs::File;
use std::io::{self, BufRead, BufReader};

use anyhow::Result;
use regex::Regex;


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


#[derive(Debug)]
pub struct Record {
    hostname: String,
    microseconds: usize,
}


impl Record {
    pub fn parse(line: &str) -> Self {
        let pattern = r"^([^\s]+).+ (\d+)$";
        let pattern = Regex::new(pattern).expect("Error compiling regex");
        let captures = pattern.captures(line).expect("No match found");
        let hostname = String::from(captures.get(1).unwrap().as_str());
        let microseconds = captures.get(2).map_or("", |m| m.as_str()).parse().unwrap_or_default();
        Record { hostname, microseconds }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_line() {
        let line = concat!(
            "contemporano.com 194.233.82.92 - - ",
            "[12/Aug/2024:00:00:50 +1200] ",
            "\"GET /vendor/ HTTP/1.1\" ",
            "301 322 \"-\" ",
            "\"Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 ",
            "(KHTML, like Gecko) Chrome/72.0.3626.121 Safari/537.36\" ",
            "111"
        );

        let record = Record::parse(line);
        assert_eq!(record.hostname, "contemporano.com");
        assert_eq!(record.microseconds, 111);
    }
}
