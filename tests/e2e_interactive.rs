use assert_cmd::Command;
use predicates::prelude::*;

fn termgfx() -> Command {
    Command::cargo_bin("termgfx").unwrap()
}

// ============================================================================
// INPUT COMMAND TESTS
// Note: Interactive commands need special handling in non-TTY mode
// These tests verify the command exists and accepts arguments
// ============================================================================

#[test]
fn test_input_help() {
    termgfx()
        .args(["input", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("prompt"));
}

#[test]
fn test_input_with_placeholder() {
    termgfx()
        .args(["input", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("placeholder"));
}

#[test]
fn test_input_password_flag_exists() {
    termgfx()
        .args(["input", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("password"));
}

// ============================================================================
// SELECT COMMAND TESTS
// ============================================================================

#[test]
fn test_select_help() {
    termgfx()
        .args(["select", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("prompt"))
        .stdout(predicate::str::contains("options"));
}

// ============================================================================
// CHOOSE COMMAND TESTS
// ============================================================================

#[test]
fn test_choose_help() {
    termgfx()
        .args(["choose", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("prompt"))
        .stdout(predicate::str::contains("options"));
}

#[test]
fn test_choose_multi_flag_exists() {
    termgfx()
        .args(["choose", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("multi"));
}

// ============================================================================
// CONFIRM COMMAND TESTS
// ============================================================================

#[test]
fn test_confirm_help() {
    termgfx()
        .args(["confirm", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("prompt"));
}

#[test]
fn test_confirm_default_flag() {
    termgfx()
        .args(["confirm", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("default"));
}

#[test]
fn test_confirm_style_flag() {
    termgfx()
        .args(["confirm", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("style"));
}
