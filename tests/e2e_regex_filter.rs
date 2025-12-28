//! E2E tests for regex-filter CLI command

use assert_cmd::Command;
use predicates::prelude::*;

#[allow(deprecated)]
fn cmd() -> Command {
    Command::cargo_bin("termgfx").unwrap()
}

// ============================================================================
// regex-filter help tests
// ============================================================================

#[test]
fn test_regex_filter_help() {
    cmd()
        .arg("regex-filter")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Filter entries using regex patterns",
        ))
        .stdout(predicate::str::contains("--pattern"))
        .stdout(predicate::str::contains("--items"));
}

#[test]
fn test_regex_filter_help_shows_examples() {
    cmd()
        .arg("regex-filter")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(".log$"))
        .stdout(predicate::str::contains("error|warn"));
}

// ============================================================================
// regex-filter basic tests
// ============================================================================

#[test]
fn test_regex_filter_basic() {
    cmd()
        .arg("regex-filter")
        .arg("--pattern")
        .arg(r"\.log$")
        .arg("--items")
        .arg("app.log,config.json")
        .assert()
        .success()
        .stdout(predicate::str::contains("app.log"))
        .stdout(predicate::str::contains("☑"));
}

#[test]
fn test_regex_filter_shows_pattern() {
    cmd()
        .arg("regex-filter")
        .arg("--pattern")
        .arg("test")
        .arg("--items")
        .arg("test.txt")
        .assert()
        .success()
        .stdout(predicate::str::contains("Pattern: test"));
}

#[test]
fn test_regex_filter_shows_match_count() {
    cmd()
        .arg("regex-filter")
        .arg("--pattern")
        .arg(r"\.txt$")
        .arg("--items")
        .arg("a.txt,b.txt,c.log")
        .assert()
        .success()
        .stdout(predicate::str::contains("2/3"));
}

#[test]
fn test_regex_filter_shows_emoji() {
    cmd()
        .arg("regex-filter")
        .arg("--pattern")
        .arg("x")
        .arg("--items")
        .arg("x")
        .assert()
        .success()
        .stdout(predicate::str::contains("🔍"));
}

// ============================================================================
// regex-filter matching tests
// ============================================================================

#[test]
fn test_regex_filter_matches_end() {
    cmd()
        .arg("regex-filter")
        .arg("--pattern")
        .arg(r"\.log$")
        .arg("--items")
        .arg("app.log,app.log.bak,config.json")
        .assert()
        .success()
        .stdout(predicate::str::contains("☑ app.log"))
        .stdout(predicate::str::contains("☐ app.log.bak (no match)"));
}

#[test]
fn test_regex_filter_matches_start() {
    cmd()
        .arg("regex-filter")
        .arg("--pattern")
        .arg("^test_")
        .arg("--items")
        .arg("test_file.txt,mytest_file.txt")
        .assert()
        .success()
        .stdout(predicate::str::contains("☑ test_file.txt"))
        .stdout(predicate::str::contains("☐ mytest_file.txt (no match)"));
}

#[test]
fn test_regex_filter_alternation() {
    cmd()
        .arg("regex-filter")
        .arg("--pattern")
        .arg("error|warning")
        .arg("--items")
        .arg("error.log,warning.log,info.log")
        .assert()
        .success()
        .stdout(predicate::str::contains("2/3"));
}

// ============================================================================
// regex-filter case sensitivity tests
// ============================================================================

#[test]
fn test_regex_filter_case_sensitive_by_default() {
    cmd()
        .arg("regex-filter")
        .arg("--pattern")
        .arg("error")
        .arg("--items")
        .arg("ERROR.log,error.log")
        .assert()
        .success()
        .stdout(predicate::str::contains("1/2"));
}

#[test]
fn test_regex_filter_case_insensitive() {
    cmd()
        .arg("regex-filter")
        .arg("--pattern")
        .arg("error")
        .arg("-I")
        .arg("--items")
        .arg("ERROR.log,error.log")
        .assert()
        .success()
        .stdout(predicate::str::contains("2/2"));
}

// ============================================================================
// regex-filter invert tests
// ============================================================================

#[test]
fn test_regex_filter_invert() {
    cmd()
        .arg("regex-filter")
        .arg("--pattern")
        .arg(r"\.log$")
        .arg("-v")
        .arg("--items")
        .arg("app.log,config.json")
        .assert()
        .success()
        .stdout(predicate::str::contains("☑ config.json"))
        .stdout(predicate::str::contains("☐ app.log (no match)"));
}

// ============================================================================
// regex-filter quiet mode tests
// ============================================================================

#[test]
fn test_regex_filter_quiet_mode() {
    cmd()
        .arg("regex-filter")
        .arg("--pattern")
        .arg(r"\.log$")
        .arg("--items")
        .arg("app.log,config.json,error.log")
        .arg("--quiet")
        .assert()
        .success()
        .stdout(predicate::str::contains("app.log"))
        .stdout(predicate::str::contains("error.log"))
        .stdout(predicate::str::contains("config.json").not());
}

#[test]
fn test_regex_filter_quiet_no_pane() {
    cmd()
        .arg("regex-filter")
        .arg("--pattern")
        .arg("test")
        .arg("--items")
        .arg("test.txt")
        .arg("--quiet")
        .assert()
        .success()
        .stdout(predicate::str::contains("╭").not())
        .stdout(predicate::str::contains("test.txt"));
}

// ============================================================================
// regex-filter action button tests
// ============================================================================

#[test]
fn test_regex_filter_custom_action() {
    cmd()
        .arg("regex-filter")
        .arg("--pattern")
        .arg("x")
        .arg("--items")
        .arg("x")
        .arg("--action")
        .arg("Delete")
        .assert()
        .success()
        .stdout(predicate::str::contains("[Delete]"));
}

#[test]
fn test_regex_filter_custom_cancel() {
    cmd()
        .arg("regex-filter")
        .arg("--pattern")
        .arg("x")
        .arg("--items")
        .arg("x")
        .arg("--cancel")
        .arg("Abort")
        .assert()
        .success()
        .stdout(predicate::str::contains("[Abort]"));
}

// ============================================================================
// regex-filter hide non-matches tests
// ============================================================================

#[test]
fn test_regex_filter_hide_non_matches() {
    cmd()
        .arg("regex-filter")
        .arg("--pattern")
        .arg(r"\.log$")
        .arg("--items")
        .arg("app.log,config.json")
        .arg("--hide-non-matches")
        .assert()
        .success()
        .stdout(predicate::str::contains("☑ app.log"))
        .stdout(predicate::str::contains("config.json").not());
}

// ============================================================================
// regex-filter border tests
// ============================================================================

#[test]
fn test_regex_filter_rounded_border() {
    cmd()
        .arg("regex-filter")
        .arg("--pattern")
        .arg("x")
        .arg("--items")
        .arg("x")
        .arg("--border")
        .arg("rounded")
        .assert()
        .success()
        .stdout(predicate::str::contains("╭"));
}

#[test]
fn test_regex_filter_double_border() {
    cmd()
        .arg("regex-filter")
        .arg("--pattern")
        .arg("x")
        .arg("--items")
        .arg("x")
        .arg("--border")
        .arg("double")
        .assert()
        .success()
        .stdout(predicate::str::contains("╔"));
}

// ============================================================================
// regex-filter error handling tests
// ============================================================================

#[test]
fn test_regex_filter_invalid_regex() {
    cmd()
        .arg("regex-filter")
        .arg("--pattern")
        .arg("[invalid")
        .arg("--items")
        .arg("test")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid regex"));
}

#[test]
fn test_regex_filter_empty_pattern_matches_all() {
    cmd()
        .arg("regex-filter")
        .arg("--pattern")
        .arg("")
        .arg("--items")
        .arg("a,b,c")
        .assert()
        .success()
        .stdout(predicate::str::contains("3/3"));
}
