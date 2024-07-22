
use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;


type TestResult = Result<(), Box<dyn std::error::Error>>;


/// Test our command against files of expected output
fn compare_file(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    compare_output(args, &expected)
}


/// Test our compare_outputcommand against given string
fn compare_output(args: &[&str], expected: &str) -> TestResult {
    // Shadow `expected` argument with String object to silence:
    // error[E0521]: borrowed data escapes outside of function
    let expected = expected.to_string();

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
fn dies_bad_args() -> TestResult {
    Command::cargo_bin("echor")?
        .args(["-z"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
    Ok(())
}


/// No arguments should print just a newline
#[test]
fn test_empty() -> TestResult {
    compare_output(&[], "\n")
}


/// No positional args and no newline should emit an empty string
#[test]
fn test_empty_no_newline() -> TestResult {
    compare_output(&["-n"], "")
}


/// Compare with `echo "Hello there"`
#[test]
fn hello1() -> TestResult {
    compare_file(&["Hello there"], &"tests/expected/hello1.txt")
}


/// Compare with `echo -n "Hello there"`
#[test]
fn hello1_no_newline() -> TestResult {
    compare_file(&["-n", "Hello there"], &"tests/expected/hello1.n.txt")
}


/// Compare against 'echo "Hello" "There"'
#[test]
fn hello2() -> TestResult {
    compare_file(&["Hello", "there"], &"tests/expected/hello2.txt")
}


/// Compare against 'echo -n "Hello" "There"'
#[test]
fn hello2_no_newline() -> TestResult {
    compare_file(&["-n", "Hello", "there"], &"tests/expected/hello2.n.txt")
}
