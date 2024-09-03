
use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;


#[test]
/// Fails with usage string if no arguments given
fn fail_no_arguments() -> Result<()> {
    let message = "Not enough arguments";
    Command::cargo_bin("minigrep")?
        .assert()
        .failure()
        .stderr(predicate::str::contains(message));
    Ok(())
}


#[test]
/// Fails with error message if file to search does not exist
fn fail_no_such_file() -> Result<()> {
    let message = "No such file or directory";
    Command::cargo_bin("minigrep")?
        .args(["needle", "haystack.txt"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(message));
    Ok(())
}
