use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_join_horizontal() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.arg("join")
        .arg("Column A\nLine 2")
        .arg("Column B\nLine 2")
        .arg("--gap")
        .arg("3");

    cmd.assert().success();
}

#[test]
fn test_join_vertical() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.arg("join")
        .arg("Block 1")
        .arg("Block 2")
        .arg("--vertical")
        .arg("--gap")
        .arg("2");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Block 1"))
        .stdout(predicate::str::contains("Block 2"));
}

#[test]
fn test_join_align_center() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.arg("join")
        .arg("Short")
        .arg("Much longer text")
        .arg("--align")
        .arg("center");

    cmd.assert().success();
}

#[test]
fn test_join_align_right() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.arg("join")
        .arg("A")
        .arg("B")
        .arg("--align")
        .arg("right");

    cmd.assert().success();
}

#[test]
fn test_columns_basic() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.arg("columns")
        .arg("--widths")
        .arg("10,15,10")
        .arg("--gap")
        .arg("2")
        .write_stdin("Line1\nLine2\nLine3\nLine4\nLine5\nLine6");

    cmd.assert().success();
}

#[test]
fn test_columns_invalid_widths() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.arg("columns")
        .arg("--widths")
        .arg("abc,def")
        .write_stdin("content");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid widths format"));
}

#[test]
fn test_stack_left_aligned() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.arg("stack")
        .arg("Header")
        .arg("Content line")
        .arg("Footer")
        .arg("--align")
        .arg("left");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Header"))
        .stdout(predicate::str::contains("Content line"))
        .stdout(predicate::str::contains("Footer"));
}

#[test]
fn test_stack_center_aligned() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.arg("stack")
        .arg("Short")
        .arg("Much longer content here")
        .arg("--align")
        .arg("center")
        .arg("--gap")
        .arg("2");

    cmd.assert().success();
}

#[test]
fn test_stack_right_aligned() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.arg("stack")
        .arg("A")
        .arg("B")
        .arg("C")
        .arg("--align")
        .arg("right");

    cmd.assert().success();
}

#[test]
fn test_join_no_input() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.arg("join");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No input provided"));
}

#[test]
fn test_stack_no_input() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.arg("stack");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No input provided"));
}

#[test]
fn test_columns_no_widths() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.arg("columns").write_stdin("content");

    // This should fail because --widths is required
    cmd.assert().failure();
}

#[test]
fn test_join_with_ansi_colors() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    // Use ANSI escape codes for colored text
    cmd.arg("join")
        .arg("\x1b[31mRed\x1b[0m")
        .arg("\x1b[32mGreen\x1b[0m");

    cmd.assert().success();
}

#[test]
fn test_stack_multiline() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.arg("stack")
        .arg("Line 1A\nLine 1B")
        .arg("Line 2A\nLine 2B\nLine 2C")
        .arg("--gap")
        .arg("1");

    cmd.assert().success();
}

#[test]
fn test_join_single_input() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.arg("join").arg("Single column");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Single column"));
}

#[test]
fn test_columns_with_gaps() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.arg("columns")
        .arg("--widths")
        .arg("5,5,5")
        .arg("--gap")
        .arg("5")
        .write_stdin("A\nB\nC\nD\nE\nF");

    cmd.assert().success();
}
