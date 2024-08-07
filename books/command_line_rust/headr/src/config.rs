
use std::env;
use std::ffi::OsString;
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
    pub fn from_command_line() -> Self {
        Self::from_args(env::args_os())
    }


    // Private /////////////////////////

    /// Build an array of Clap arguments
    fn build_args() -> Vec<Arg> {
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

    /// Parse given arguments
    ///
    /// Same signature as `clap::Parser::parse_from()` to enable
    /// unit-testing.
    fn from_args<I, T>(args: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone
    {
        let matches = command!()
            .args(Self::build_args())
            .get_matches_from(args);

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
}


#[cfg(test)]
mod tests {
    use super::*;

    use std::iter::empty;

    #[test]
    fn test_build_args() {
        let args = Config::build_args();
        assert_eq!(args.len(), 3);
    }

    #[test]
    fn test_parse_defaults() {
        let config = Config::from_args(empty::<OsString>());
        assert_eq!(config.files.len(), 0);
        assert_eq!(config.lines, 10);
        assert_eq!(config.bytes, None);
    }

    #[test]
    fn test_parse_lines() {
        let args = vec!["name", "-n", "123"];
        let config = Config::from_args(args.iter());
        assert_eq!(config.files.len(), 0);
        assert_eq!(config.lines, 123);
        assert_eq!(config.bytes, None);
    }

    #[test]
    fn test_parse_bytes() {
        let args = vec!["name", "-c", "321"];
        let config = Config::from_args(args.iter());
        assert_eq!(config.files.len(), 0);
        assert_eq!(config.lines, 10);
        assert_eq!(config.bytes, Some(321));
    }

    #[test]
    fn test_parse_lines_and_bytes() {
        let args = vec!["name", "-c", "321", "-n", "123"];
        let config = Config::from_args(args.iter());
        assert_eq!(config.files.len(), 0);
        assert_eq!(config.lines, 123);
        assert_eq!(config.bytes, Some(321));
    }

    #[test]
    fn test_parse_all() {
        let args = vec!["name", "-c", "321", "-n", "123", "a.txt", "b.txt"];
        let config = Config::from_args(args.iter());
        assert_eq!(config.lines, 123);
        assert_eq!(config.bytes, Some(321));
        assert_eq!(config.files.len(), 2);
        assert_eq!(
            config.files,
            vec![PathBuf::from("a.txt"), PathBuf::from("b.txt")],
        );
    }
}
