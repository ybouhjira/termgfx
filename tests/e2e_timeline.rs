#![allow(deprecated)]
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_timeline_help() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.arg("timeline").arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Events"));
}

#[test]
fn test_timeline_basic() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.arg("timeline").arg("--events").arg("Start,Middle,End");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Start"))
        .stdout(predicate::str::contains("Middle"))
        .stdout(predicate::str::contains("End"));
}

#[test]
fn test_timeline_with_dates() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.arg("timeline")
        .arg("--events")
        .arg("2024-01:Start,2024-06:Middle,2024-12:End");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("2024-01"))
        .stdout(predicate::str::contains("Start"));
}

#[test]
fn test_timeline_with_colors() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.arg("timeline")
        .arg("--events")
        .arg("Start,Middle,End")
        .arg("--color")
        .arg("blue");
    cmd.assert().success();
}

#[test]
fn test_timeline_styles() {
    // Test arrow style
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.arg("timeline")
        .arg("--events")
        .arg("A,B,C")
        .arg("--style")
        .arg("arrow");
    cmd.assert().success();

    // Test line style
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.arg("timeline")
        .arg("--events")
        .arg("A,B,C")
        .arg("--style")
        .arg("line");
    cmd.assert().success();

    // Test dots style
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.arg("timeline")
        .arg("--events")
        .arg("A,B,C")
        .arg("--style")
        .arg("dots");
    cmd.assert().success();
}

#[test]
fn test_timeline_animated() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.arg("timeline")
        .arg("--events")
        .arg("Start,End")
        .arg("--animate");
    cmd.assert().success();
}

#[test]
fn test_timeline_vertical() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.arg("timeline")
        .arg("--events")
        .arg("Start,Middle,End")
        .arg("--vertical");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Start"));
}

#[test]
fn test_timeline_missing_events() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.arg("timeline");
    cmd.assert().failure();
}
