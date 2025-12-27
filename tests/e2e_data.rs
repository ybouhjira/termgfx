use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

fn termgfx() -> Command {
    Command::cargo_bin("termgfx").unwrap()
}

// ============================================================================
// DIFF COMMAND TESTS
// ============================================================================

#[test]
fn test_diff_identical_files() {
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();

    writeln!(file1, "line 1\nline 2\nline 3").unwrap();
    writeln!(file2, "line 1\nline 2\nline 3").unwrap();

    termgfx()
        .args([
            "diff",
            file1.path().to_str().unwrap(),
            file2.path().to_str().unwrap(),
        ])
        .assert()
        .success();
}

#[test]
fn test_diff_different_files() {
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();

    writeln!(file1, "line 1\nline 2\nline 3").unwrap();
    writeln!(file2, "line 1\nmodified\nline 3").unwrap();

    termgfx()
        .args([
            "diff",
            file1.path().to_str().unwrap(),
            file2.path().to_str().unwrap(),
        ])
        .assert()
        .success();
}

#[test]
fn test_diff_unified_format() {
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();

    writeln!(file1, "a\nb\nc").unwrap();
    writeln!(file2, "a\nx\nc").unwrap();

    termgfx()
        .args([
            "diff",
            file1.path().to_str().unwrap(),
            file2.path().to_str().unwrap(),
            "--unified",
        ])
        .assert()
        .success();
}

#[test]
fn test_diff_with_context() {
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();

    writeln!(file1, "1\n2\n3\n4\n5").unwrap();
    writeln!(file2, "1\n2\nX\n4\n5").unwrap();

    termgfx()
        .args([
            "diff",
            file1.path().to_str().unwrap(),
            file2.path().to_str().unwrap(),
            "--unified",
            "--context",
            "2",
        ])
        .assert()
        .success();
}

#[test]
fn test_diff_nonexistent_file() {
    let file1 = NamedTempFile::new().unwrap();

    termgfx()
        .args([
            "diff",
            file1.path().to_str().unwrap(),
            "/nonexistent/file.txt",
        ])
        .assert()
        .failure();
}

// ============================================================================
// TABLE COMMAND TESTS
// ============================================================================

#[test]
fn test_table_inline_data() {
    termgfx()
        .args([
            "table",
            "--headers",
            "Name,Age,City",
            "--rows",
            "Alice,30,NYC|Bob,25,LA",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Name"))
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"));
}

#[test]
fn test_table_single_row() {
    termgfx()
        .args(["table", "--headers", "A,B,C", "--rows", "1,2,3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("A"))
        .stdout(predicate::str::contains("1"));
}

#[test]
fn test_table_border_single() {
    termgfx()
        .args([
            "table",
            "--headers",
            "X,Y",
            "--rows",
            "1,2",
            "--border",
            "single",
        ])
        .assert()
        .success();
}

#[test]
fn test_table_border_double() {
    termgfx()
        .args([
            "table",
            "--headers",
            "X,Y",
            "--rows",
            "1,2",
            "--border",
            "double",
        ])
        .assert()
        .success();
}

#[test]
fn test_table_border_rounded() {
    termgfx()
        .args([
            "table",
            "--headers",
            "X,Y",
            "--rows",
            "1,2",
            "--border",
            "rounded",
        ])
        .assert()
        .success();
}

#[test]
fn test_table_alignment_left() {
    termgfx()
        .args([
            "table",
            "--headers",
            "Col",
            "--rows",
            "Data",
            "--alignment",
            "left",
        ])
        .assert()
        .success();
}

#[test]
fn test_table_alignment_center() {
    termgfx()
        .args([
            "table",
            "--headers",
            "Col",
            "--rows",
            "Data",
            "--alignment",
            "center",
        ])
        .assert()
        .success();
}

#[test]
fn test_table_alignment_right() {
    termgfx()
        .args([
            "table",
            "--headers",
            "Col",
            "--rows",
            "Data",
            "--alignment",
            "right",
        ])
        .assert()
        .success();
}

#[test]
fn test_table_from_csv_file() {
    let mut csv_file = NamedTempFile::with_suffix(".csv").unwrap();
    writeln!(csv_file, "Name,Score\nAlice,95\nBob,87").unwrap();

    termgfx()
        .args(["table", "--file", csv_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Name"))
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn test_table_many_columns() {
    termgfx()
        .args([
            "table",
            "--headers",
            "A,B,C,D,E",
            "--rows",
            "1,2,3,4,5|6,7,8,9,10",
        ])
        .assert()
        .success();
}

// ============================================================================
// TREE COMMAND TESTS
// ============================================================================

#[test]
fn test_tree_inline_data() {
    termgfx()
        .args(["tree", "root>child1,child2"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_tree_nested() {
    termgfx()
        .args(["tree", "root>parent>child"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_tree_from_json_inline() {
    // Tree command uses inline data format, not JSON files
    // JSON file support would require walkdir feature
    termgfx()
        .args(["tree", "project>src>main.rs,lib.rs"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_tree_simple_structure() {
    termgfx()
        .args(["tree", "project>src,tests,docs"])
        .assert()
        .success();
}
