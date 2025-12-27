use assert_cmd::Command;
use predicates::prelude::*;

fn termgfx() -> Command {
    Command::cargo_bin("termgfx").unwrap()
}

// ============================================================================
// LINE CHART TESTS
// ============================================================================

#[test]
fn test_chart_line_basic() {
    termgfx()
        .args(["chart", "line", "--data", "10,20,30,40,50"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_chart_line_with_title() {
    termgfx()
        .args([
            "chart",
            "line",
            "--data",
            "5,10,15,20",
            "--title",
            "Sales Data",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Sales Data"));
}

#[test]
fn test_chart_line_single_point() {
    termgfx()
        .args(["chart", "line", "--data", "50"])
        .assert()
        .success();
}

#[test]
fn test_chart_line_many_points() {
    termgfx()
        .args(["chart", "line", "--data", "1,2,3,4,5,6,7,8,9,10,11,12"])
        .assert()
        .success();
}

#[test]
fn test_chart_line_varying_values() {
    termgfx()
        .args(["chart", "line", "--data", "10,50,20,80,30,90"])
        .assert()
        .success();
}

// ============================================================================
// BAR CHART TESTS
// ============================================================================

#[test]
fn test_chart_bar_basic() {
    termgfx()
        .args(["chart", "bar", "--data", "A:10,B:20,C:30"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_chart_bar_single_bar() {
    termgfx()
        .args(["chart", "bar", "--data", "Sales:100"])
        .assert()
        .success();
}

#[test]
fn test_chart_bar_many_bars() {
    termgfx()
        .args([
            "chart",
            "bar",
            "--data",
            "Jan:10,Feb:20,Mar:15,Apr:30,May:25",
        ])
        .assert()
        .success();
}

#[test]
fn test_chart_bar_labels_with_spaces() {
    termgfx()
        .args(["chart", "bar", "--data", "Item A:50,Item B:75"])
        .assert()
        .success();
}

// ============================================================================
// PIE CHART TESTS
// ============================================================================

#[test]
fn test_chart_pie_basic() {
    termgfx()
        .args(["chart", "pie", "--data", "Red:30,Blue:40,Green:30"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_chart_pie_two_slices() {
    termgfx()
        .args(["chart", "pie", "--data", "Yes:70,No:30"])
        .assert()
        .success();
}

#[test]
fn test_chart_pie_many_slices() {
    termgfx()
        .args(["chart", "pie", "--data", "A:20,B:20,C:20,D:20,E:20"])
        .assert()
        .success();
}

#[test]
fn test_chart_pie_unequal_distribution() {
    termgfx()
        .args(["chart", "pie", "--data", "Major:80,Minor:15,Tiny:5"])
        .assert()
        .success();
}
