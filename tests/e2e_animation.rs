use assert_cmd::Command;
use predicates::prelude::*;

fn termgfx() -> Command {
    Command::cargo_bin("termgfx").unwrap()
}

// ============================================================================
// ANIMATE COMMAND TESTS
// ============================================================================

#[test]
fn test_animate_help() {
    termgfx()
        .args(["animate", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::is_match("(?i)effect").unwrap())
        .stdout(predicate::str::is_match("(?i)duration").unwrap());
}

#[test]
fn test_animate_progress() {
    termgfx()
        .args(["animate", "-t", "progress", "-D", "0.1"])
        .assert()
        .success();
}

#[test]
fn test_animate_counter() {
    termgfx()
        .args(["animate", "-t", "counter", "--from", "0", "--to", "10", "-D", "0.1"])
        .assert()
        .success();
}

#[test]
fn test_animate_typewriter() {
    termgfx()
        .args(["animate", "-t", "typewriter", "--text", "Hello", "-D", "0.1"])
        .assert()
        .success();
}

#[test]
fn test_animate_chart_build() {
    termgfx()
        .args(["animate", "-t", "chart-build", "-d", "10,20,30", "-D", "0.1"])
        .assert()
        .success();
}

#[test]
fn test_animate_bars() {
    termgfx()
        .args(["animate", "-t", "bars", "-d", "A:10,B:20", "-D", "0.1"])
        .assert()
        .success();
}

#[test]
fn test_animate_with_prefix_suffix() {
    termgfx()
        .args(["animate", "-t", "counter", "--from", "0", "--to", "100", "--prefix", "$", "--suffix", "k", "-D", "0.1"])
        .assert()
        .success();
}

// ============================================================================
// DEMO COMMAND TESTS
// ============================================================================

#[test]
fn test_demo_help() {
    termgfx()
        .args(["demo", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::is_match("(?i)section").unwrap());
}

#[test]
fn test_demo_boxes_section() {
    termgfx()
        .args(["demo", "--section", "boxes"])
        .assert()
        .success();
}

#[test]
fn test_demo_charts_section() {
    termgfx()
        .args(["demo", "--section", "charts"])
        .assert()
        .success();
}

#[test]
fn test_demo_progress_section() {
    termgfx()
        .args(["demo", "--section", "progress"])
        .assert()
        .success();
}

#[test]
fn test_demo_animation_section() {
    termgfx()
        .args(["demo", "--section", "animation"])
        .assert()
        .success();
}

#[test]
fn test_animate_with_style() {
    termgfx()
        .args(["animate", "-t", "progress", "--style", "blocks", "-D", "0.1"])
        .assert()
        .success();
}

#[test]
fn test_animate_unknown_effect() {
    termgfx()
        .args(["animate", "-t", "unknown_effect", "-D", "0.1"])
        .assert()
        .success(); // Should handle gracefully
}

// ============================================================================
// SPINNER DURATION TESTS
// ============================================================================

#[test]
fn test_spinner_with_duration() {
    termgfx()
        .args(["spinner", "Loading...", "--duration", "1"])
        .assert()
        .success();
}

#[test]
fn test_spinner_duration_help() {
    termgfx()
        .args(["spinner", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::is_match("(?i)duration").unwrap());
}
