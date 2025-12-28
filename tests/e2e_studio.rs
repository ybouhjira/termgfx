//! E2E tests for termgfx studio command

use assert_cmd::Command;
use predicates::prelude::*;

#[allow(deprecated)]
fn cmd() -> Command {
    Command::cargo_bin("termgfx").unwrap()
}

// ============================================================================
// studio help tests
// ============================================================================

#[test]
fn test_studio_help() {
    cmd()
        .arg("studio")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("TermGFX Studio"))
        .stdout(predicate::str::contains("fullscreen IDE-like"));
}

#[test]
fn test_studio_help_shows_navigation() {
    cmd()
        .arg("studio")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Tab"))
        .stdout(predicate::str::contains("Cycle panels"))
        .stdout(predicate::str::contains("Enter"))
        .stdout(predicate::str::contains("Edit parameter"));
}

#[test]
fn test_studio_help_shows_panels() {
    cmd()
        .arg("studio")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Sidebar"))
        .stdout(predicate::str::contains("Params"))
        .stdout(predicate::str::contains("Preview"))
        .stdout(predicate::str::contains("Command"));
}

#[test]
fn test_studio_help_shows_keybindings() {
    cmd()
        .arg("studio")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("q/Esc"))
        .stdout(predicate::str::contains("Quit"))
        .stdout(predicate::str::contains("Copy command"));
}

#[test]
fn test_studio_help_shows_keyboard_shortcuts() {
    // Issue #109: Verify keyboard shortcuts are documented
    cmd()
        .arg("studio")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("?"))
        .stdout(predicate::str::contains("Help"))
        .stdout(predicate::str::contains("r"))
        .stdout(predicate::str::contains("Reset"));
}

#[test]
fn test_studio_help_shows_panel_jump_keys() {
    // Issue #109: Verify 1/2/3 panel jump keys are documented
    cmd()
        .arg("studio")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("1/2/3"))
        .stdout(predicate::str::contains("Jump to panel"));
}

// ============================================================================
// studio interactive tests (require TTY - skipped in CI)
// ============================================================================

// Note: The studio command requires an interactive terminal (TTY).
// These tests verify the help text and non-interactive behavior.
// Full interactive testing would require rexpect or similar PTY library.

#[test]
fn test_studio_no_tty_error() {
    // When run without a TTY, studio should fail with a helpful message
    cmd()
        .arg("studio")
        .assert()
        .failure()
        .stderr(predicate::str::contains("interactive terminal").or(predicate::str::contains("TTY")));
}

// ============================================================================
// Unit tests for registry module are in registry.rs
// ============================================================================
