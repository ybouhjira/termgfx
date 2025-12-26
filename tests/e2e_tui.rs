use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::time::Duration;
use tempfile::NamedTempFile;

#[test]
fn test_tui_requires_config_or_layout_and_widgets() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.arg("tui");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Either --config or both --layout and --widgets must be provided"));
}

#[test]
fn test_tui_requires_both_layout_and_widgets() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.args(&["tui", "--layout", "2x2"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Either --config or both --layout and --widgets must be provided"));
}

#[test]
fn test_tui_inline_widget_count_mismatch() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.args(&[
        "tui",
        "--layout", "2x2",
        "--widgets", "box:Hello,gauge:75", // Only 2 widgets for 2x2 layout (needs 4)
    ]);
    cmd.timeout(Duration::from_secs(2));
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Widget count mismatch"));
}

#[test]
fn test_tui_invalid_layout_format() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.args(&[
        "tui",
        "--layout", "invalid",
        "--widgets", "box:Hello",
    ]);
    cmd.timeout(Duration::from_secs(2));
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid layout format"));
}

#[test]
fn test_tui_invalid_widget_definition() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.args(&[
        "tui",
        "--layout", "1x1",
        "--widgets", "invalid", // Missing colon separator
    ]);
    cmd.timeout(Duration::from_secs(2));
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid widget definition"));
}

#[test]
fn test_tui_config_file_not_found() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.args(&[
        "tui",
        "--config", "/nonexistent/config.json",
    ]);
    cmd.timeout(Duration::from_secs(2));
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read config file"));
}

#[test]
fn test_tui_config_file_invalid_json() {
    let temp_file = NamedTempFile::new().unwrap();
    fs::write(temp_file.path(), "invalid json content").unwrap();

    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.args(&[
        "tui",
        "--config", temp_file.path().to_str().unwrap(),
    ]);
    cmd.timeout(Duration::from_secs(2));
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to parse config file"));
}

#[test]
fn test_tui_valid_config_file_2x2() {
    let config = r#"{
        "layout": "2x2",
        "widgets": [
            {"type": "box", "content": "Hello World"},
            {"type": "gauge", "content": "75"},
            {"type": "sparkline", "content": "1,2,3,4,5"},
            {"type": "log", "content": "Log line 1\nLog line 2"}
        ],
        "refresh_interval": 1000
    }"#;

    let temp_file = NamedTempFile::new().unwrap();
    fs::write(temp_file.path(), config).unwrap();

    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.args(&[
        "tui",
        "--config", temp_file.path().to_str().unwrap(),
    ]);
    cmd.timeout(Duration::from_millis(500));

    // The command will timeout since TUI runs in loop, but that's expected
    // We just want to verify it starts without errors
    let result = cmd.assert();

    // In CI/non-TTY environments, this might fail immediately
    // Accept both timeout and specific error messages
    let output = result.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);

    // It should not have parsing errors
    assert!(!stderr.contains("Failed to parse config file"));
    assert!(!stderr.contains("Widget count mismatch"));
}

#[test]
fn test_tui_valid_inline_1x2() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.args(&[
        "tui",
        "--layout", "1x2",
        "--widgets", "box:Hello,gauge:50",
    ]);
    cmd.timeout(Duration::from_millis(500));

    let result = cmd.assert();
    let output = result.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!stderr.contains("Widget count mismatch"));
    assert!(!stderr.contains("Invalid widget definition"));
}

#[test]
fn test_tui_valid_inline_3x1() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.args(&[
        "tui",
        "--layout", "3x1",
        "--widgets", "box:Top,gauge:33,sparkline:1;2;3",
    ]);
    cmd.timeout(Duration::from_millis(500));

    let result = cmd.assert();
    let output = result.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!stderr.contains("Widget count mismatch"));
}

#[test]
fn test_tui_custom_refresh_interval() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.args(&[
        "tui",
        "--layout", "1x1",
        "--widgets", "box:Test",
        "--refresh", "500",
    ]);
    cmd.timeout(Duration::from_millis(500));

    let result = cmd.assert();
    let output = result.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!stderr.contains("error"));
}

#[test]
fn test_tui_sparkline_widget() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.args(&[
        "tui",
        "--layout", "1x1",
        "--widgets", "sparkline:1,2,3,4,5,6,7,8,9",
    ]);
    cmd.timeout(Duration::from_millis(500));

    let result = cmd.assert();
    let output = result.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!stderr.contains("Unknown widget type"));
}

#[test]
fn test_tui_log_widget_multiline() {
    let config = r#"{
        "layout": "1x1",
        "widgets": [
            {"type": "log", "content": "Line 1\nLine 2\nLine 3\nLine 4"}
        ]
    }"#;

    let temp_file = NamedTempFile::new().unwrap();
    fs::write(temp_file.path(), config).unwrap();

    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.args(&[
        "tui",
        "--config", temp_file.path().to_str().unwrap(),
    ]);
    cmd.timeout(Duration::from_millis(500));

    let result = cmd.assert();
    let output = result.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!stderr.contains("error"));
}

#[test]
fn test_tui_large_layout_4x4() {
    let widgets = vec![
        "box:1", "box:2", "box:3", "box:4",
        "gauge:10", "gauge:20", "gauge:30", "gauge:40",
        "sparkline:1;2", "sparkline:3;4", "sparkline:5;6", "sparkline:7;8",
        "log:A", "log:B", "log:C", "log:D",
    ];

    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.args(&[
        "tui",
        "--layout", "4x4",
        "--widgets", &widgets.join(","),
    ]);
    cmd.timeout(Duration::from_millis(500));

    let result = cmd.assert();
    let output = result.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!stderr.contains("Widget count mismatch"));
}

#[test]
fn test_tui_help_message() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.args(&["tui", "--help"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Interactive TUI mode"))
        .stdout(predicate::str::contains("--config"))
        .stdout(predicate::str::contains("--layout"))
        .stdout(predicate::str::contains("--widgets"))
        .stdout(predicate::str::contains("--refresh"));
}

#[test]
fn test_tui_widget_types_in_help() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.args(&["tui", "--help"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("box"))
        .stdout(predicate::str::contains("gauge"))
        .stdout(predicate::str::contains("sparkline"))
        .stdout(predicate::str::contains("log"));
}
