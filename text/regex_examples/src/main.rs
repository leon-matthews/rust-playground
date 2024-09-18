
use lazy_static::lazy_static;
use regex::Regex;


lazy_static! {
    // Official Semantic Versioning 2.0.0 Regex
    // https://semver.org/
    static ref SEMVER_REGEX: Regex = Regex::new(concat!(
        r"^(?P<major>0|[1-9]\d*)",
        r"\.",
        r"(?P<minor>0|[1-9]\d*)",
        r"\.",
        r"(?P<patch>0|[1-9]\d*)",
        r"(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?",
        r"(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$",
    )).expect("Error parsing regex");
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

    // Extract captures
    extract_captures("2.4.31");

    // Capture AND parse
    capture_and_parse("2.4.31");
}


/// Does the given string contain a version number?
fn is_version(haystack: &str) -> bool {
    // Compiled on the fly
    let pattern = Regex::new(r"\d+\.\d+\.\d+").unwrap();
    pattern.is_match(haystack)
}


/// As `is_version()`, but regex contructed on first use
fn is_version_static(haystack: &str) -> bool {
    SEMVER_REGEX.is_match(haystack)
}


fn extract_captures(version: &str) {
    let Some(captures) = SEMVER_REGEX.captures(version) else {
        eprintln!("Semver regex failed to  match version {:?}", version);
        return;
    };
    println!(
        "Major:{:?} Minor:{:?} Patch:{:?}",
        &captures["major"],
        &captures["minor"],
        &captures["patch"],
    );
}


/// Parse integers out of regex with decent error handling
fn capture_and_parse(version: &str) {
    let Some(captures) = SEMVER_REGEX.captures(version) else {
        eprintln!("Semver regex failed to  match version {:?}", version);
        return;
    };

    let parse_usize = |key| captures.name(key)
        .map_or("", |m| m.as_str())
        .parse()
        .unwrap_or_default();

    let major: usize = parse_usize("major");
    let minor: usize = parse_usize("minor");
    let patch: usize = parse_usize("patch");

    println!("Major:{major:?} Minor:{minor:?} Patch:{patch:?}");
}
