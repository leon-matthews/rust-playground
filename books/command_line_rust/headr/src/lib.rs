
use std::path::PathBuf;

use anyhow::Result;
use clap::{Arg, ArgAction, ArgMatches, command};


pub fn run(config: Config) -> Result<()> {
    println!("-n {}", config.lines);
    println!("-c {:?}", config.bytes);
    println!("{:#?}", config.files);
    Ok(())
}


#[derive(Debug)]
pub struct Config {
    files: Vec<PathBuf>,
    lines: usize,               // Number of lines to print
    bytes: Option<usize>,       // Number of bytes to print
}


impl Config {
    /// Parse command-line arguments
    pub fn parse_args() -> Self {
        // Bytes
        let bytes = Arg::new("bytes").short('c').long("bytes")
            .value_parser(clap::value_parser!(usize))
            .help("Print the first NUM bytes of each file");

        // Lines
        let lines = Arg::new("lines").short('n').long("lines")
            .value_parser(clap::value_parser!(usize))
            .default_value("10")
            .help("Print the first NUM lines of each file");

        // Files
        let files = Arg::new("files").action(ArgAction::Append)
            .value_parser(clap::value_parser!(PathBuf));

        // Parse
        let matches = command!()
            .args([bytes, lines, files])
            .get_matches();

        // Build config.
        Self::from_matches(matches)
    }

    /// Build config instance from clap's matches
    fn from_matches(matches: ArgMatches) -> Self {
        let bytes = matches.get_one::<usize>("bytes");
        dbg!(&bytes);

        let lines = matches.get_one::<usize>("lines")
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
}
