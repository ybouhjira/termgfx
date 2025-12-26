use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_watch_help() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.arg("watch").arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Watch mode"))
        .stdout(predicate::str::contains("--interval"))
        .stdout(predicate::str::contains("--differences"))
        .stdout(predicate::str::contains("--no-title"))
        .stdout(predicate::str::contains("--exit-on-error"));
}

#[test]
fn test_watch_requires_command() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.arg("watch");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_watch_invalid_interval() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.arg("watch")
        .arg("echo test")
        .arg("--interval")
        .arg("invalid");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid"));
}
