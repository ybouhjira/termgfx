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
// Widget tests (Issue #106)
// ============================================================================

#[test]
fn test_studio_help_shows_widget_controls() {
    // Issue #106: Verify widget controls are documented
    cmd()
        .arg("studio")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Space"))
        .stdout(predicate::str::contains("Toggle bool"));
}

#[test]
fn test_studio_help_shows_slider_control() {
    // Issue #106: Verify slider arrow key controls documented
    cmd()
        .arg("studio")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("h/←"))
        .stdout(predicate::str::contains("l/→"));
}

// ============================================================================
// Mouse support tests (Issue #107)
// ============================================================================

#[test]
fn test_studio_help_shows_mouse_support() {
    // Issue #107: Verify mouse support is documented
    cmd()
        .arg("studio")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Mouse"))
        .stdout(predicate::str::contains("Click"))
        .stdout(predicate::str::contains("Scroll"));
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
    cmd().arg("studio").assert().failure().stderr(
        predicate::str::contains("interactive terminal").or(predicate::str::contains("TTY")),
    );
}

// ============================================================================
// Resizable panes tests (Issue #108)
// ============================================================================

#[test]
fn test_studio_help_shows_resize_controls() {
    // Issue #108: Verify resize controls are documented
    cmd()
        .arg("studio")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Resizing"))
        .stdout(predicate::str::contains("Ctrl"))
        .stdout(predicate::str::contains("Drag"));
}

#[test]
fn test_studio_help_shows_layout_reset() {
    // Issue #108: Verify layout reset is documented
    cmd()
        .arg("studio")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Reset layout"));
}

#[test]
fn test_studio_help_shows_divider_drag() {
    // Issue #108: Verify divider drag is documented
    cmd()
        .arg("studio")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Drag divider"))
        .stdout(predicate::str::contains("Resize panels"));
}

// ============================================================================
// Unit tests for registry module are in registry.rs
// ============================================================================
