use assert_cmd::Command;
use predicates::prelude::*;

fn termgfx() -> Command {
    Command::cargo_bin("termgfx").unwrap()
}

// ============================================================================
// GENERAL CLI TESTS
// ============================================================================

#[test]
fn test_help() {
    termgfx()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("termgfx"))
        .stdout(predicate::str::contains("COMMAND"));
}

#[test]
fn test_version() {
    termgfx()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("termgfx"));
}

#[test]
fn test_no_args_shows_help() {
    termgfx()
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn test_unknown_command() {
    termgfx()
        .arg("unknowncommand")
        .assert()
        .failure();
}

// ============================================================================
// SUBCOMMAND HELP TESTS
// ============================================================================

#[test]
fn test_box_help() {
    termgfx()
        .args(["box", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("message"))
        .stdout(predicate::str::contains("style"))
        .stdout(predicate::str::contains("border"));
}

#[test]
fn test_banner_help() {
    termgfx()
        .args(["banner", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("title"))
        .stdout(predicate::str::contains("gradient"));
}

#[test]
fn test_progress_help() {
    termgfx()
        .args(["progress", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("percent"))
        .stdout(predicate::str::contains("style"))
        .stdout(predicate::str::contains("animate"));
}

#[test]
fn test_spinner_help() {
    termgfx()
        .args(["spinner", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("message"))
        .stdout(predicate::str::contains("style"));
}

#[test]
fn test_typewriter_help() {
    termgfx()
        .args(["typewriter", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::is_match("(?i)message").unwrap())
        .stdout(predicate::str::is_match("(?i)speed").unwrap());
}

#[test]
fn test_sparkline_help() {
    termgfx()
        .args(["sparkline", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::is_match("(?i)data").unwrap());
}

#[test]
fn test_diff_help() {
    termgfx()
        .args(["diff", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::is_match("(?i)file1").unwrap())
        .stdout(predicate::str::is_match("(?i)file2").unwrap())
        .stdout(predicate::str::is_match("(?i)unified").unwrap());
}

#[test]
fn test_table_help() {
    termgfx()
        .args(["table", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("headers"))
        .stdout(predicate::str::contains("rows"))
        .stdout(predicate::str::contains("file"));
}

#[test]
fn test_tree_help() {
    termgfx()
        .args(["tree", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("data"))
        .stdout(predicate::str::contains("path"));
}

#[test]
fn test_chart_help() {
    termgfx()
        .args(["chart", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("line"))
        .stdout(predicate::str::contains("bar"))
        .stdout(predicate::str::contains("pie"));
}

#[test]
fn test_chart_line_help() {
    termgfx()
        .args(["chart", "line", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("data"))
        .stdout(predicate::str::contains("title"));
}

#[test]
fn test_chart_bar_help() {
    termgfx()
        .args(["chart", "bar", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("data"));
}

#[test]
fn test_chart_pie_help() {
    termgfx()
        .args(["chart", "pie", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("data"));
}

// ============================================================================
// ERROR HANDLING TESTS
// ============================================================================

#[test]
fn test_box_missing_message() {
    termgfx()
        .args(["box"])
        .assert()
        .failure();
}

#[test]
fn test_banner_missing_title() {
    termgfx()
        .args(["banner"])
        .assert()
        .failure();
}

#[test]
fn test_progress_missing_percent() {
    termgfx()
        .args(["progress"])
        .assert()
        .failure();
}

#[test]
fn test_progress_invalid_percent() {
    termgfx()
        .args(["progress", "not_a_number"])
        .assert()
        .failure();
}

#[test]
fn test_spinner_missing_message() {
    termgfx()
        .args(["spinner"])
        .assert()
        .failure();
}

#[test]
fn test_typewriter_missing_message() {
    termgfx()
        .args(["typewriter"])
        .assert()
        .failure();
}

#[test]
fn test_sparkline_missing_data() {
    termgfx()
        .args(["sparkline"])
        .assert()
        .failure();
}

#[test]
fn test_diff_missing_files() {
    termgfx()
        .args(["diff"])
        .assert()
        .failure();
}

#[test]
fn test_diff_missing_second_file() {
    termgfx()
        .args(["diff", "file1.txt"])
        .assert()
        .failure();
}

#[test]
fn test_chart_line_missing_data() {
    termgfx()
        .args(["chart", "line"])
        .assert()
        .failure();
}

#[test]
fn test_chart_bar_missing_data() {
    termgfx()
        .args(["chart", "bar"])
        .assert()
        .failure();
}

#[test]
fn test_chart_pie_missing_data() {
    termgfx()
        .args(["chart", "pie"])
        .assert()
        .failure();
}
