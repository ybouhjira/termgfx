use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::NamedTempFile;

fn termgfx() -> Command {
    Command::cargo_bin("termgfx").unwrap()
}

// ============================================================================
// DASHBOARD HELP TESTS
// ============================================================================

#[test]
fn test_dashboard_help() {
    termgfx()
        .args(["dashboard", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("sparkline:1;2;3"));
}

// ============================================================================
// BASIC DASHBOARD TESTS
// ============================================================================

#[test]
fn test_dashboard_basic_single_panel() {
    termgfx()
        .args([
            "dashboard",
            "--layout", "1x1",
            "--panels", "box:Hello Dashboard"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello Dashboard"));
}

#[test]
fn test_dashboard_grid_layout_2x2() {
    termgfx()
        .args([
            "dashboard",
            "--layout", "2x2",
            "--panels", "box:Panel 1,box:Panel 2,box:Panel 3,box:Panel 4"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Panel 1"))
        .stdout(predicate::str::contains("Panel 2"))
        .stdout(predicate::str::contains("Panel 3"))
        .stdout(predicate::str::contains("Panel 4"));
}

#[test]
fn test_dashboard_grid_layout_3x1() {
    termgfx()
        .args([
            "dashboard",
            "--layout", "3x1",
            "--panels", "box:Top,box:Middle,box:Bottom"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Top"))
        .stdout(predicate::str::contains("Middle"))
        .stdout(predicate::str::contains("Bottom"));
}

#[test]
fn test_dashboard_with_title() {
    termgfx()
        .args([
            "dashboard",
            "--title", "System Dashboard",
            "--layout", "2x1",
            "--panels", "box:CPU,box:Memory"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("System Dashboard"))
        .stdout(predicate::str::contains("CPU"))
        .stdout(predicate::str::contains("Memory"));
}

// ============================================================================
// PANEL TYPE TESTS
// ============================================================================

#[test]
fn test_dashboard_progress_panel() {
    termgfx()
        .args([
            "dashboard",
            "--layout", "1x1",
            "--panels", "progress:75"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("75%"));
}

#[test]
fn test_dashboard_sparkline_panel() {
    termgfx()
        .args([
            "dashboard",
            "--layout", "1x1",
            "--panels", "sparkline:1,2,3,4,5"
        ])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_dashboard_sparkline_semicolon_delimiter() {
    // Test semicolon delimiter for sparkline values (fixes #56)
    termgfx()
        .args([
            "dashboard",
            "--layout", "1x3",
            "--panels", "box:Status,sparkline:10;20;30;40;50,progress:75"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status"))
        .stdout(predicate::str::contains("75%"));
}

#[test]
fn test_dashboard_gauge_panel() {
    termgfx()
        .args([
            "dashboard",
            "--layout", "1x1",
            "--panels", "gauge:50"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("50%"));
}

#[test]
fn test_dashboard_text_panel() {
    termgfx()
        .args([
            "dashboard",
            "--layout", "1x1",
            "--panels", "text:Simple text content"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Simple text content"));
}

#[test]
fn test_dashboard_mixed_panels() {
    termgfx()
        .args([
            "dashboard",
            "--layout", "2x2",
            "--panels", "box:Status,progress:75,sparkline:1,2,3,gauge:50"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status"))
        .stdout(predicate::str::contains("75%"))
        .stdout(predicate::str::contains("50%"));
}

// ============================================================================
// BORDER STYLE TESTS
// ============================================================================

#[test]
fn test_dashboard_border_single() {
    termgfx()
        .args([
            "dashboard",
            "--layout", "1x1",
            "--panels", "box:Test",
            "--border", "single"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Test"));
}

#[test]
fn test_dashboard_border_double() {
    termgfx()
        .args([
            "dashboard",
            "--layout", "1x1",
            "--panels", "box:Test",
            "--border", "double"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Test"));
}

#[test]
fn test_dashboard_border_rounded() {
    termgfx()
        .args([
            "dashboard",
            "--layout", "1x1",
            "--panels", "box:Test",
            "--border", "rounded"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Test"));
}

// ============================================================================
// CONFIG FILE TESTS
// ============================================================================

#[test]
fn test_dashboard_from_config_file() {
    let config_content = r#"{
  "layout": "2x2",
  "title": "Config Dashboard",
  "panels": [
    {"type": "box", "content": "Panel 1"},
    {"type": "progress", "content": "75"},
    {"type": "sparkline", "content": "1,2,3,4,5"},
    {"type": "gauge", "content": "50"}
  ]
}"#;

    let mut config_file = NamedTempFile::new().unwrap();
    fs::write(config_file.path(), config_content).unwrap();

    termgfx()
        .args([
            "dashboard",
            "--config", config_file.path().to_str().unwrap()
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Config Dashboard"))
        .stdout(predicate::str::contains("Panel 1"))
        .stdout(predicate::str::contains("75%"))
        .stdout(predicate::str::contains("50%"));
}

#[test]
fn test_dashboard_config_override_panels() {
    let config_content = r#"{
  "layout": "1x1",
  "panels": [
    {"type": "box", "content": "From Config"}
  ]
}"#;

    let mut config_file = NamedTempFile::new().unwrap();
    fs::write(config_file.path(), config_content).unwrap();

    // CLI panels should override config panels
    termgfx()
        .args([
            "dashboard",
            "--config", config_file.path().to_str().unwrap(),
            "--panels", "box:From CLI"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("From CLI"));
}

// ============================================================================
// ERROR HANDLING TESTS
// ============================================================================

#[test]
fn test_dashboard_missing_panels() {
    // 2x2 requires 4 panels but only 2 provided
    termgfx()
        .args([
            "dashboard",
            "--layout", "2x2",
            "--panels", "box:Panel 1,box:Panel 2"
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Expected 4 panels"));
}

#[test]
fn test_dashboard_invalid_layout() {
    termgfx()
        .args([
            "dashboard",
            "--layout", "invalid",
            "--panels", "box:Test"
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid layout format"));
}

#[test]
fn test_dashboard_invalid_panel_type() {
    termgfx()
        .args([
            "dashboard",
            "--layout", "1x1",
            "--panels", "invalid:Test"
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown panel type"));
}

#[test]
fn test_dashboard_missing_config_file() {
    termgfx()
        .args([
            "dashboard",
            "--config", "/nonexistent/file.json"
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read config"));
}

#[test]
fn test_dashboard_invalid_json_config() {
    let mut config_file = NamedTempFile::new().unwrap();
    fs::write(config_file.path(), "{ invalid json }").unwrap();

    termgfx()
        .args([
            "dashboard",
            "--config", config_file.path().to_str().unwrap()
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to parse config"));
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[test]
fn test_dashboard_empty_panel_content() {
    termgfx()
        .args([
            "dashboard",
            "--layout", "1x1",
            "--panels", "box:"
        ])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_dashboard_large_grid() {
    termgfx()
        .args([
            "dashboard",
            "--layout", "4x4",
            "--panels", "box:1,box:2,box:3,box:4,box:5,box:6,box:7,box:8,box:9,box:10,box:11,box:12,box:13,box:14,box:15,box:16"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("1"));
}

#[test]
fn test_dashboard_default_layout() {
    // Should use default 2x2 layout
    termgfx()
        .args([
            "dashboard",
            "--panels", "box:1,box:2,box:3,box:4"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("1"))
        .stdout(predicate::str::contains("4"));
}
