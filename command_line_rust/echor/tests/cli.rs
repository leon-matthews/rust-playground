
use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

/// Test our command against files of expected output
fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

/// Running with zero arguments should fail, printing usage string to stderr
/// (Actual GNU coreutils echo doesn't do this, but that's pedagogy for you!)
#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
    Ok(())
}

/// Compare with `echo "Hello there"`
#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], &"tests/expected/hello1.txt")
}

/// Compare with `echo -n "Hello there"`
#[test]
fn hello1_no_newline() -> TestResult {
    run(&["-n", "Hello there"], &"tests/expected/hello1.n.txt")
}

/// Compare against 'echo "Hello" "There"'
#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], &"tests/expected/hello2.txt")
}

/// Compare against 'echo -n "Hello" "There"'
#[test]
fn hello2_no_newline() -> TestResult {
    run(&["-n", "Hello", "there"], &"tests/expected/hello2.n.txt")
}
