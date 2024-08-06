
use std::path::PathBuf;

use clap::{Arg, ArgAction, command};


#[derive(Debug)]
pub struct Config {
    pub files: Vec<PathBuf>,
    pub lines: usize,                   // Number of lines to print
    pub bytes: Option<usize>,           // Number of bytes to print
}


impl Config {
    /// Parse command-line arguments
    pub fn from_args() -> Self {
        // Parse
        let args = Self::create_args();
        let matches = command!()
            .args(args)
            .get_matches();

        let bytes = matches.get_one("bytes");
        let lines = matches.get_one("lines")
            .expect("Lines argument should be required");
        let files = matches.get_many::<PathBuf>("files")
            .unwrap_or_default()
            .cloned()
            .collect::<Vec<_>>();
        Self {
            bytes: bytes.copied(),
            lines: *lines,
            files,
        }
    }

    /// Build an array of Clap arguments
    fn create_args() -> Vec<Arg> {
        let mut args = vec![];

        // Bytes
        args.push(
            Arg::new("bytes").short('c').long("bytes")
            .value_parser(clap::value_parser!(usize))
            .help("Print the first NUM bytes of each file")
        );

        // Lines
        args.push(
            Arg::new("lines").short('n').long("lines")
            .value_parser(clap::value_parser!(usize))
            .default_value("10")
            .help("Print the first NUM lines of each file")
        );

        // Files
        args.push(
            Arg::new("files").action(ArgAction::Append)
            .value_parser(clap::value_parser!(PathBuf))
        );

        args
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_args() {
        let args = Config::create_args();
        assert_eq!(args.len(), 3);
    }
}
