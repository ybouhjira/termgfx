use assert_cmd::Command;
use predicates::prelude::*;

fn termgfx() -> Command {
    Command::cargo_bin("termgfx").unwrap()
}

// ============================================================================
// GAUGE COMMAND TESTS
// ============================================================================

#[test]
fn test_gauge_help() {
    termgfx()
        .args(["gauge", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("radial"))
        .stdout(predicate::str::contains("value"));
}

#[test]
fn test_gauge_basic() {
    termgfx()
        .args(["gauge", "50"])
        .assert()
        .success()
        .stdout(predicate::str::contains("50"));
}

#[test]
fn test_gauge_zero() {
    termgfx()
        .args(["gauge", "0"])
        .assert()
        .success();
}

#[test]
fn test_gauge_full() {
    termgfx()
        .args(["gauge", "100"])
        .assert()
        .success()
        .stdout(predicate::str::contains("100"));
}

#[test]
fn test_gauge_with_label() {
    termgfx()
        .args(["gauge", "75", "--label", "CPU Usage"])
        .assert()
        .success()
        .stdout(predicate::str::contains("CPU Usage"))
        .stdout(predicate::str::contains("75"));
}

#[test]
fn test_gauge_with_min_max() {
    termgfx()
        .args(["gauge", "150", "--min", "0", "--max", "200"])
        .assert()
        .success()
        .stdout(predicate::str::contains("150"));
}

#[test]
fn test_gauge_style_semicircle() {
    termgfx()
        .args(["gauge", "60", "--style", "semicircle"])
        .assert()
        .success();
}

#[test]
fn test_gauge_style_full() {
    termgfx()
        .args(["gauge", "60", "--style", "full"])
        .assert()
        .success();
}

#[test]
fn test_gauge_style_minimal() {
    termgfx()
        .args(["gauge", "60", "--style", "minimal"])
        .assert()
        .success();
}

#[test]
fn test_gauge_with_color() {
    termgfx()
        .args(["gauge", "80", "--color", "green"])
        .assert()
        .success();
}

#[test]
fn test_gauge_animated() {
    termgfx()
        .args(["gauge", "90", "--animate"])
        .assert()
        .success();
}

#[test]
fn test_gauge_missing_value() {
    termgfx()
        .args(["gauge"])
        .assert()
        .failure();
}

#[test]
fn test_gauge_custom_range() {
    termgfx()
        .args(["gauge", "5", "--min", "0", "--max", "10"])
        .assert()
        .success()
        .stdout(predicate::str::contains("5"));
}

#[test]
fn test_gauge_label_and_color() {
    termgfx()
        .args(["gauge", "45", "--label", "Memory", "--color", "blue"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Memory"));
}
