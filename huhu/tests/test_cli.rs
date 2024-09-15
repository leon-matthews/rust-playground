#![allow(unused_imports)]

use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::{assert_eq, assert_ne};

const PROGRAM: &str = "huhu";


#[test]
fn usage() -> Result<()> {
    for flag in &["-h", "--help"] {
        Command::cargo_bin(PROGRAM)?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("Usage"));
    }
    Ok(())
}
