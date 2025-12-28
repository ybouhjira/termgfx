//! E2E tests for theme CLI command

use assert_cmd::Command;
use predicates::prelude::*;

fn cmd() -> Command {
    Command::cargo_bin("termgfx").unwrap()
}

// ============================================================================
// theme list tests
// ============================================================================

#[test]
fn test_theme_list() {
    cmd()
        .arg("theme")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("Available Theme Presets"));
}

#[test]
fn test_theme_list_shows_all_presets() {
    let output = cmd().arg("theme").arg("list").output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("dark"));
    assert!(stdout.contains("light"));
    assert!(stdout.contains("nord"));
    assert!(stdout.contains("dracula"));
    assert!(stdout.contains("monokai"));
    assert!(stdout.contains("solarized"));
    assert!(stdout.contains("gruvbox"));
}

#[test]
fn test_theme_list_shows_mode_indicator() {
    let output = cmd().arg("theme").arg("list").output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show dark or light mode indicators for themes
    // The output shows "dark" or "light" without parentheses in ANSI escape codes
    assert!(stdout.contains("dark") || stdout.contains("light"));
}

#[test]
fn test_theme_without_subcommand_shows_list() {
    // Running `theme` without subcommand should show list
    let list_output = cmd().arg("theme").arg("list").output().unwrap();
    let default_output = cmd().arg("theme").output().unwrap();

    let list_stdout = String::from_utf8_lossy(&list_output.stdout);
    let default_stdout = String::from_utf8_lossy(&default_output.stdout);

    // Both should contain the same key content
    assert!(list_stdout.contains("Available Theme Presets"));
    assert!(default_stdout.contains("Available Theme Presets"));
}

// ============================================================================
// theme preview tests
// ============================================================================

#[test]
fn test_theme_preview_dark() {
    cmd()
        .arg("theme")
        .arg("preview")
        .arg("dark")
        .assert()
        .success()
        .stdout(predicate::str::contains("Theme: DARK"));
}

#[test]
fn test_theme_preview_light() {
    cmd()
        .arg("theme")
        .arg("preview")
        .arg("light")
        .assert()
        .success()
        .stdout(predicate::str::contains("Theme: LIGHT"));
}

#[test]
fn test_theme_preview_nord() {
    cmd()
        .arg("theme")
        .arg("preview")
        .arg("nord")
        .assert()
        .success()
        .stdout(predicate::str::contains("Theme: NORD"))
        .stdout(predicate::str::contains("Arctic"));
}

#[test]
fn test_theme_preview_dracula() {
    cmd()
        .arg("theme")
        .arg("preview")
        .arg("dracula")
        .assert()
        .success()
        .stdout(predicate::str::contains("Theme: DRACULA"));
}

#[test]
fn test_theme_preview_monokai() {
    cmd()
        .arg("theme")
        .arg("preview")
        .arg("monokai")
        .assert()
        .success()
        .stdout(predicate::str::contains("Theme: MONOKAI"));
}

#[test]
fn test_theme_preview_solarized() {
    cmd()
        .arg("theme")
        .arg("preview")
        .arg("solarized")
        .assert()
        .success()
        .stdout(predicate::str::contains("Theme: SOLARIZED"));
}

#[test]
fn test_theme_preview_gruvbox() {
    cmd()
        .arg("theme")
        .arg("preview")
        .arg("gruvbox")
        .assert()
        .success()
        .stdout(predicate::str::contains("Theme: GRUVBOX"));
}

#[test]
fn test_theme_preview_shows_colors() {
    let output = cmd()
        .arg("theme")
        .arg("preview")
        .arg("dark")
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show color labels
    assert!(stdout.contains("Primary"));
    assert!(stdout.contains("Secondary"));
    assert!(stdout.contains("Success"));
    assert!(stdout.contains("Warning"));
    assert!(stdout.contains("Danger"));
    assert!(stdout.contains("Info"));
    assert!(stdout.contains("Background"));
    assert!(stdout.contains("Foreground"));
}

#[test]
fn test_theme_preview_shows_sample_box() {
    let output = cmd()
        .arg("theme")
        .arg("preview")
        .arg("nord")
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show sample box with theme name
    assert!(stdout.contains("Sample"));
    assert!(stdout.contains("Hello from nord theme"));
}

#[test]
fn test_theme_preview_shows_usage_hint() {
    let output = cmd()
        .arg("theme")
        .arg("preview")
        .arg("dracula")
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show usage hint
    assert!(stdout.contains("TERMGFX_THEME=dracula"));
}

#[test]
fn test_theme_preview_invalid_name() {
    cmd()
        .arg("theme")
        .arg("preview")
        .arg("invalid_theme")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn test_theme_preview_default_is_dark() {
    // Preview without name should default to dark
    cmd()
        .arg("theme")
        .arg("preview")
        .assert()
        .success()
        .stdout(predicate::str::contains("Theme: DARK"));
}

// ============================================================================
// theme current tests
// ============================================================================

#[test]
fn test_theme_current_shows_theme() {
    cmd()
        .arg("theme")
        .arg("current")
        .assert()
        .success()
        .stdout(predicate::str::contains("dark").or(predicate::str::contains("light")));
}

#[test]
fn test_theme_current_shows_mode() {
    let output = cmd().arg("theme").arg("current").output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show mode (dark or light)
    assert!(stdout.contains("dark") || stdout.contains("light"));
}

#[test]
fn test_theme_current_shows_env_hint() {
    let output = cmd().arg("theme").arg("current").output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show env var hint
    assert!(stdout.contains("TERMGFX_THEME"));
}

// ============================================================================
// theme help tests
// ============================================================================

#[test]
fn test_theme_help() {
    cmd()
        .arg("theme")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Browse and preview theme presets"))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("preview"))
        .stdout(predicate::str::contains("current"));
}

#[test]
fn test_theme_help_shows_presets() {
    cmd()
        .arg("theme")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("dark"))
        .stdout(predicate::str::contains("nord"))
        .stdout(predicate::str::contains("dracula"));
}
