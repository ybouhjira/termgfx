//! E2E tests for preview CLI command

use assert_cmd::Command;
use predicates::prelude::*;

#[allow(deprecated)]
fn cmd() -> Command {
    Command::cargo_bin("termgfx").unwrap()
}

// ============================================================================
// preview help tests
// ============================================================================

#[test]
fn test_preview_help() {
    cmd()
        .arg("preview")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Preview data before performing actions"))
        .stdout(predicate::str::contains("--title"))
        .stdout(predicate::str::contains("--items"))
        .stdout(predicate::str::contains("--action"));
}

// ============================================================================
// preview basic tests
// ============================================================================

#[test]
fn test_preview_basic() {
    cmd()
        .arg("preview")
        .arg("--items")
        .arg("file1.txt,file2.txt")
        .assert()
        .success()
        .stdout(predicate::str::contains("Preview"))
        .stdout(predicate::str::contains("file1.txt"))
        .stdout(predicate::str::contains("file2.txt"));
}

#[test]
fn test_preview_with_title() {
    cmd()
        .arg("preview")
        .arg("--title")
        .arg("Files to delete")
        .arg("--items")
        .arg("a.txt,b.txt")
        .assert()
        .success()
        .stdout(predicate::str::contains("Files to delete"));
}

#[test]
fn test_preview_with_action() {
    cmd()
        .arg("preview")
        .arg("--items")
        .arg("test.log")
        .arg("--action")
        .arg("Delete")
        .assert()
        .success()
        .stdout(predicate::str::contains("[Delete]"));
}

#[test]
fn test_preview_with_cancel_label() {
    cmd()
        .arg("preview")
        .arg("--items")
        .arg("test.log")
        .arg("--cancel")
        .arg("Abort")
        .assert()
        .success()
        .stdout(predicate::str::contains("[Abort]"));
}

#[test]
fn test_preview_shows_item_count() {
    cmd()
        .arg("preview")
        .arg("--items")
        .arg("a,b,c")
        .assert()
        .success()
        .stdout(predicate::str::contains("3"));
}

#[test]
fn test_preview_single_item() {
    cmd()
        .arg("preview")
        .arg("--items")
        .arg("only_one.txt")
        .assert()
        .success()
        .stdout(predicate::str::contains("1 (item)"));
}

// ============================================================================
// preview style tests
// ============================================================================

#[test]
fn test_preview_info_style() {
    cmd()
        .arg("preview")
        .arg("--items")
        .arg("test")
        .arg("--style")
        .arg("info")
        .assert()
        .success();
}

#[test]
fn test_preview_danger_style() {
    cmd()
        .arg("preview")
        .arg("--items")
        .arg("important.db")
        .arg("--style")
        .arg("danger")
        .arg("--action")
        .arg("Delete")
        .assert()
        .success()
        .stdout(predicate::str::contains("[Delete]"));
}

#[test]
fn test_preview_success_style() {
    cmd()
        .arg("preview")
        .arg("--items")
        .arg("approved.txt")
        .arg("--style")
        .arg("success")
        .arg("--action")
        .arg("Apply")
        .assert()
        .success();
}

#[test]
fn test_preview_warning_style() {
    cmd()
        .arg("preview")
        .arg("--items")
        .arg("warning.log")
        .arg("--style")
        .arg("warning")
        .assert()
        .success();
}

// ============================================================================
// preview border tests
// ============================================================================

#[test]
fn test_preview_rounded_border() {
    cmd()
        .arg("preview")
        .arg("--items")
        .arg("test")
        .arg("--border")
        .arg("rounded")
        .assert()
        .success()
        .stdout(predicate::str::contains("╭"));
}

#[test]
fn test_preview_double_border() {
    cmd()
        .arg("preview")
        .arg("--items")
        .arg("test")
        .arg("--border")
        .arg("double")
        .assert()
        .success()
        .stdout(predicate::str::contains("╔"));
}

#[test]
fn test_preview_single_border() {
    cmd()
        .arg("preview")
        .arg("--items")
        .arg("test")
        .arg("--border")
        .arg("single")
        .assert()
        .success()
        .stdout(predicate::str::contains("┌"));
}

#[test]
fn test_preview_ascii_border() {
    cmd()
        .arg("preview")
        .arg("--items")
        .arg("test")
        .arg("--border")
        .arg("ascii")
        .assert()
        .success()
        .stdout(predicate::str::contains("+"));
}

// ============================================================================
// preview numbering tests
// ============================================================================

#[test]
fn test_preview_with_numbers() {
    cmd()
        .arg("preview")
        .arg("--items")
        .arg("first,second,third")
        .assert()
        .success()
        .stdout(predicate::str::contains("1. first"))
        .stdout(predicate::str::contains("2. second"))
        .stdout(predicate::str::contains("3. third"));
}

#[test]
fn test_preview_no_numbers() {
    cmd()
        .arg("preview")
        .arg("--items")
        .arg("first,second")
        .arg("--no-numbers")
        .assert()
        .success()
        .stdout(predicate::str::contains("first"))
        .stdout(predicate::str::contains("second"))
        .stdout(predicate::str::contains("1. first").not());
}

// ============================================================================
// preview truncation tests
// ============================================================================

#[test]
fn test_preview_truncation() {
    let items = (1..=30).map(|i| format!("item{}", i)).collect::<Vec<_>>().join(",");
    cmd()
        .arg("preview")
        .arg("--items")
        .arg(&items)
        .arg("--max-items")
        .arg("5")
        .assert()
        .success()
        .stdout(predicate::str::contains("... and 25 more"));
}

#[test]
fn test_preview_custom_max_items() {
    let items = (1..=10).map(|i| format!("f{}.txt", i)).collect::<Vec<_>>().join(",");
    cmd()
        .arg("preview")
        .arg("--items")
        .arg(&items)
        .arg("--max-items")
        .arg("3")
        .assert()
        .success()
        .stdout(predicate::str::contains("... and 7 more"));
}

// ============================================================================
// preview columns tests
// ============================================================================

#[test]
fn test_preview_with_columns() {
    cmd()
        .arg("preview")
        .arg("--items")
        .arg("file1.txt|10KB,file2.txt|20KB")
        .arg("--columns")
        .arg("Name,Size")
        .assert()
        .success()
        .stdout(predicate::str::contains("Name"))
        .stdout(predicate::str::contains("Size"))
        .stdout(predicate::str::contains("file1.txt"))
        .stdout(predicate::str::contains("10KB"));
}

#[test]
fn test_preview_columns_three_cols() {
    cmd()
        .arg("preview")
        .arg("--items")
        .arg("access.log|15KB|2024-01-15,error.log|3KB|2024-01-14")
        .arg("--columns")
        .arg("Name,Size,Date")
        .assert()
        .success()
        .stdout(predicate::str::contains("Name"))
        .stdout(predicate::str::contains("Size"))
        .stdout(predicate::str::contains("Date"))
        .stdout(predicate::str::contains("access.log"))
        .stdout(predicate::str::contains("2024-01-15"));
}

// ============================================================================
// preview emoji tests
// ============================================================================

#[test]
fn test_preview_has_clipboard_emoji() {
    cmd()
        .arg("preview")
        .arg("--items")
        .arg("test")
        .assert()
        .success()
        .stdout(predicate::str::contains("📋"));
}
