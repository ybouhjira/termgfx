use assert_cmd::Command;
use predicates::prelude::*;

fn termgfx() -> Command {
    Command::cargo_bin("termgfx").unwrap()
}

// ============================================================================
// PLAYGROUND COMMAND TESTS
// Interactive playground for exploring termgfx components
// ============================================================================

#[test]
fn playground_help_works() {
    termgfx()
        .args(["playground", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Interactive playground/showcase"))
        .stdout(predicate::str::contains("Navigate:"));
}

#[test]
fn playground_exists_in_help() {
    termgfx()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("playground"));
}

#[test]
fn playground_help_shows_navigation() {
    termgfx()
        .args(["playground", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("h/l"))
        .stdout(predicate::str::contains("k/j"));
}

#[test]
fn playground_help_shows_edit_instructions() {
    termgfx()
        .args(["playground", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Enter to edit"));
}

#[test]
fn playground_help_shows_quit_instructions() {
    termgfx()
        .args(["playground", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("q/Esc to quit"));
}
