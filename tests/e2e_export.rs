//! E2E tests for export CLI command

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::NamedTempFile;

#[allow(deprecated)]
fn cmd() -> Command {
    Command::cargo_bin("termgfx").unwrap()
}

// ============================================================================
// export help tests
// ============================================================================

#[test]
fn test_export_help() {
    cmd()
        .arg("export")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Export terminal graphics"))
        .stdout(predicate::str::contains("box"))
        .stdout(predicate::str::contains("progress"))
        .stdout(predicate::str::contains("bar-chart"));
}

#[test]
fn test_export_box_help() {
    cmd()
        .arg("export")
        .arg("box")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Export a styled box to SVG"))
        .stdout(predicate::str::contains("--style"))
        .stdout(predicate::str::contains("--output"));
}

// ============================================================================
// export box tests
// ============================================================================

#[test]
fn test_export_box_to_stdout() {
    cmd()
        .arg("export")
        .arg("box")
        .arg("Test Message")
        .assert()
        .success()
        .stdout(predicate::str::contains("<?xml"))
        .stdout(predicate::str::contains("<svg"))
        .stdout(predicate::str::contains("Test Message"))
        .stdout(predicate::str::contains("</svg>"));
}

#[test]
fn test_export_box_info_style() {
    cmd()
        .arg("export")
        .arg("box")
        .arg("Info Box")
        .arg("--style")
        .arg("info")
        .assert()
        .success()
        .stdout(predicate::str::contains("#3b82f6")); // Info color
}

#[test]
fn test_export_box_success_style() {
    cmd()
        .arg("export")
        .arg("box")
        .arg("Success Box")
        .arg("--style")
        .arg("success")
        .assert()
        .success()
        .stdout(predicate::str::contains("#10b981")); // Success color
}

#[test]
fn test_export_box_warning_style() {
    cmd()
        .arg("export")
        .arg("box")
        .arg("Warning Box")
        .arg("--style")
        .arg("warning")
        .assert()
        .success()
        .stdout(predicate::str::contains("#f59e0b")); // Warning color
}

#[test]
fn test_export_box_danger_style() {
    cmd()
        .arg("export")
        .arg("box")
        .arg("Danger Box")
        .arg("--style")
        .arg("danger")
        .assert()
        .success()
        .stdout(predicate::str::contains("#ef4444")); // Danger color
}

#[test]
fn test_export_box_to_file() {
    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path().to_str().unwrap();

    cmd()
        .arg("export")
        .arg("box")
        .arg("File Export Test")
        .arg("-o")
        .arg(path)
        .assert()
        .success()
        .stderr(predicate::str::contains("Exported to:"));

    // Verify file contains valid SVG
    let content = fs::read_to_string(path).unwrap();
    assert!(content.contains("<?xml"));
    assert!(content.contains("<svg"));
    assert!(content.contains("File Export Test"));
}

#[test]
fn test_export_box_custom_dimensions() {
    cmd()
        .arg("export")
        .arg("box")
        .arg("Custom Size")
        .arg("--width")
        .arg("600")
        .arg("--height")
        .arg("200")
        .assert()
        .success()
        .stdout(predicate::str::contains("width=\"600\""))
        .stdout(predicate::str::contains("height=\"200\""));
}

#[test]
fn test_export_box_custom_background() {
    cmd()
        .arg("export")
        .arg("box")
        .arg("Custom BG")
        .arg("--background")
        .arg("#ff0000")
        .assert()
        .success()
        .stdout(predicate::str::contains("fill=\"#ff0000\""));
}

// ============================================================================
// export progress tests
// ============================================================================

#[test]
fn test_export_progress_to_stdout() {
    cmd()
        .arg("export")
        .arg("progress")
        .arg("75")
        .assert()
        .success()
        .stdout(predicate::str::contains("<?xml"))
        .stdout(predicate::str::contains("<svg"))
        .stdout(predicate::str::contains("75%"));
}

#[test]
fn test_export_progress_success_style() {
    cmd()
        .arg("export")
        .arg("progress")
        .arg("50")
        .arg("--style")
        .arg("success")
        .assert()
        .success()
        .stdout(predicate::str::contains("#10b981"));
}

#[test]
fn test_export_progress_to_file() {
    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path().to_str().unwrap();

    cmd()
        .arg("export")
        .arg("progress")
        .arg("100")
        .arg("-o")
        .arg(path)
        .assert()
        .success();

    let content = fs::read_to_string(path).unwrap();
    assert!(content.contains("100%"));
}

#[test]
fn test_export_progress_clamps_value() {
    // Values over 100 should be clamped
    cmd()
        .arg("export")
        .arg("progress")
        .arg("150")
        .assert()
        .success()
        .stdout(predicate::str::contains("100%"));
}

// ============================================================================
// export bar-chart tests
// ============================================================================

#[test]
fn test_export_bar_chart_to_stdout() {
    cmd()
        .arg("export")
        .arg("bar-chart")
        .arg("--data")
        .arg("A:10,B:20,C:15")
        .assert()
        .success()
        .stdout(predicate::str::contains("<?xml"))
        .stdout(predicate::str::contains("<svg"))
        .stdout(predicate::str::contains("rect")); // Bars are rectangles
}

#[test]
fn test_export_bar_chart_shows_labels() {
    cmd()
        .arg("export")
        .arg("bar-chart")
        .arg("--data")
        .arg("Sales:100,Costs:60")
        .assert()
        .success()
        .stdout(predicate::str::contains("Sales"))
        .stdout(predicate::str::contains("Costs"));
}

#[test]
fn test_export_bar_chart_to_file() {
    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path().to_str().unwrap();

    cmd()
        .arg("export")
        .arg("bar-chart")
        .arg("--data")
        .arg("X:5,Y:10")
        .arg("-o")
        .arg(path)
        .assert()
        .success();

    let content = fs::read_to_string(path).unwrap();
    assert!(content.contains("<svg"));
    assert!(content.contains("X"));
    assert!(content.contains("Y"));
}

#[test]
fn test_export_bar_chart_custom_dimensions() {
    cmd()
        .arg("export")
        .arg("bar-chart")
        .arg("--data")
        .arg("A:1")
        .arg("--width")
        .arg("800")
        .arg("--height")
        .arg("500")
        .assert()
        .success()
        .stdout(predicate::str::contains("width=\"800\""))
        .stdout(predicate::str::contains("height=\"500\""));
}

#[test]
fn test_export_bar_chart_invalid_data() {
    cmd()
        .arg("export")
        .arg("bar-chart")
        .arg("--data")
        .arg("invalid-data-format")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No valid data points"));
}

#[test]
fn test_export_bar_chart_empty_data() {
    cmd()
        .arg("export")
        .arg("bar-chart")
        .arg("--data")
        .arg("")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No valid data points"));
}

// ============================================================================
// SVG validation tests
// ============================================================================

#[test]
fn test_svg_has_correct_xml_declaration() {
    cmd()
        .arg("export")
        .arg("box")
        .arg("Test")
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>",
        ));
}

#[test]
fn test_svg_has_proper_namespace() {
    cmd()
        .arg("export")
        .arg("box")
        .arg("Test")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "xmlns=\"http://www.w3.org/2000/svg\"",
        ));
}

#[test]
fn test_svg_escapes_special_characters() {
    cmd()
        .arg("export")
        .arg("box")
        .arg("Test & <special> \"characters\"")
        .assert()
        .success()
        .stdout(predicate::str::contains("&amp;"))
        .stdout(predicate::str::contains("&lt;"))
        .stdout(predicate::str::contains("&gt;"))
        .stdout(predicate::str::contains("&quot;"));
}
