#[allow(unused_variables)]

use assert_cmd::Command;


/// Check output of our primary `hello` binary.
#[test]
fn hello_runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}


/// Our `false` binary should always have a non-zero exit-code.
#[test]
fn false_fails() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}


/// Our `true` binary should always have the exit-code zero.
#[test]
fn true_succeeds() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}
