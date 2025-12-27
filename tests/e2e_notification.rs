#![allow(deprecated)]
use assert_cmd::Command;
use predicates::prelude::*;

fn termgfx() -> Command {
    Command::cargo_bin("termgfx").unwrap()
}

// ============================================================================
// NOTIFICATION COMMAND TESTS
// ============================================================================

#[test]
fn test_notification_help() {
    termgfx()
        .args(["notification", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Desktop + terminal alerts"));
}

#[test]
fn test_notification_basic() {
    termgfx()
        .args(["notification", "Test message"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Test message"));
}

#[test]
fn test_notification_with_title() {
    termgfx()
        .args(["notification", "Important update", "--title", "System"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Important update"));
}

#[test]
fn test_notification_info_style() {
    termgfx()
        .args(["notification", "Info message", "--style", "info"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Info message"));
}

#[test]
fn test_notification_success_style() {
    termgfx()
        .args(["notification", "Success!", "--style", "success"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Success!"));
}

#[test]
fn test_notification_warning_style() {
    termgfx()
        .args(["notification", "Warning!", "--style", "warning"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Warning!"));
}

#[test]
fn test_notification_error_style() {
    termgfx()
        .args(["notification", "Error!", "--style", "error"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Error!"));
}

#[test]
fn test_notification_with_sound() {
    termgfx()
        .args(["notification", "Alert with sound", "--sound"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alert with sound"));
}

#[test]
fn test_notification_terminal_only() {
    termgfx()
        .args(["notification", "Terminal only", "--terminal-only"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Terminal only"));
}

#[test]
fn test_notification_desktop_only() {
    termgfx()
        .args(["notification", "Desktop only", "--desktop-only"])
        .assert()
        .success();
    // Desktop-only shows no terminal output
}

#[test]
fn test_notification_missing_message() {
    termgfx()
        .args(["notification"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}
