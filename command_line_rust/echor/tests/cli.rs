
use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;


/// Running with zero arguments should fail, printing usage string to stderr
/// (Actual GNU coreutils echo doesn't do this, but that's pedagogy for you!)
#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}


/// Most basic example
#[test]
fn run() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("Hello").assert().success().stdout("Hello\n");
}


/// Compare with output from `echo "Hello there"`
#[test]
fn hello1() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    let expected = fs::read_to_string("tests/expected/hello1.txt").unwrap();
    cmd.arg("Hello there").assert().success().stdout(expected);
}
