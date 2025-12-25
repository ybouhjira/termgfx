use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

fn termgfx() -> Command {
    Command::cargo_bin("termgfx").unwrap()
}

// ============================================================================
// HEATMAP COMMAND TESTS
// ============================================================================

#[test]
fn test_heatmap_help() {
    termgfx()
        .args(["heatmap", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("2D heatmap visualization"));
}

#[test]
fn test_heatmap_basic() {
    // Grid data format: "1,2,3;4,5,6;7,8,9" (semicolons separate rows)
    termgfx()
        .args(["heatmap", "--data", "1,2,3;4,5,6;7,8,9"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_heatmap_with_labels() {
    termgfx()
        .args([
            "heatmap",
            "--data", "10,20,30;40,50,60",
            "--x-labels", "A,B,C",
            "--y-labels", "Row1,Row2",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("A"))
        .stdout(predicate::str::contains("Row1"));
}

#[test]
fn test_heatmap_custom_colors() {
    termgfx()
        .args([
            "heatmap",
            "--data", "1,2,3;4,5,6",
            "--colors", "green-red",
        ])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_heatmap_from_file() {
    let mut csv_file = NamedTempFile::with_suffix(".csv").unwrap();
    writeln!(csv_file, "1,2,3\n4,5,6\n7,8,9").unwrap();

    termgfx()
        .args(["heatmap", "--file", csv_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_heatmap_styles() {
    // Test different color schemes
    for scheme in ["blue-red", "green-red", "viridis", "magma"] {
        termgfx()
            .args([
                "heatmap",
                "--data", "1,5,9;2,6,10;3,7,11",
                "--colors", scheme,
            ])
            .assert()
            .success()
            .stdout(predicate::str::is_empty().not());
    }
}

#[test]
fn test_heatmap_animated() {
    // In non-TTY mode, animation should just show final result
    termgfx()
        .args([
            "heatmap",
            "--data", "1,2;3,4",
            "--animate",
        ])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_heatmap_missing_data() {
    // No data and no file should fail
    termgfx()
        .args(["heatmap"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_heatmap_single_row() {
    termgfx()
        .args(["heatmap", "--data", "1,2,3,4,5"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_heatmap_single_column() {
    termgfx()
        .args(["heatmap", "--data", "1;2;3;4;5"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_heatmap_with_title() {
    termgfx()
        .args([
            "heatmap",
            "--data", "1,2,3;4,5,6",
            "--title", "Temperature Map",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Temperature Map"));
}

#[test]
fn test_heatmap_empty_data() {
    termgfx()
        .args(["heatmap", "--data", ""])
        .assert()
        .failure();
}

#[test]
fn test_heatmap_irregular_grid() {
    // Test with rows of different lengths - should handle gracefully
    termgfx()
        .args(["heatmap", "--data", "1,2,3;4,5;6,7,8,9"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}
