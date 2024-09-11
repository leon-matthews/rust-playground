
use lazy_static::lazy_static;
use regex::Regex;


lazy_static! {
    static ref VERSION_REGEX: Regex = Regex::new(
        r"(\d+)\.(\d+)\.(\d+)",
    ).expect("Error parsing regex");
}


fn main() {
    // Simple match, regex compiled on the fly
    let version = "1.11.1";
    println!(
        "It is {} that {:?} contains a version number.",
        is_version(&version),
        version,
    );

    // Simple match, regex compiled the first time its used
    let version = "2.0.31";
    println!(
        "It is {} that {:?} contains a version number.",
        is_version_static(&version),
        version,
    );
}


/// Does the given string contain a version number?
fn is_version(haystack: &str) -> bool {
    // Compiled on the fly
    let pattern = Regex::new(r"\d+\.\d+\.\d+").unwrap();
    pattern.is_match(haystack)
}


/// As `is_version()`, but regex contructed on first use
fn is_version_static(haystack: &str) -> bool {
    VERSION_REGEX.is_match(haystack)
}
