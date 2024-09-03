
use std::env;
use std::error::Error;
use std::fs;


/// Configuration for this run
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}


impl Config {

    /// Build config from command-line arguments and environment variables
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}


/// Search file for matches using the given `Config`
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}


/// Return lines matching `query` from multiline `contents` string.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line.trim());
        }
    }
    results
}


/// Same as `search()`, but allows for case-insensitive matching
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line.trim());
        }
    }
    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "
            Rust:
            safe, fast, productive.
            Pick three.";
        assert_eq!(search(query, contents), vec!["safe, fast, productive."]);
    }

    #[test]
    fn case_insensitive() {
        let query = "RUST";
        let contents = "
            Rust:
            safe, fast, productive.
            Pick three.";
        assert_eq!(search(query, contents).len(), 0);
        assert_eq!(search_case_insensitive(query, contents), vec!["Rust:"]);
    }
}
