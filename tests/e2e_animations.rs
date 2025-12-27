use assert_cmd::Command;
use predicates::prelude::*;

fn termgfx() -> Command {
    Command::cargo_bin("termgfx").unwrap()
}

// ============================================================================
// BOX ANIMATION TESTS
// ============================================================================

#[test]
fn test_box_animate_flag() {
    termgfx()
        .args(["box", "Test Message", "--animate"])
        .assert()
        .success();
}

#[test]
fn test_box_animate_with_style() {
    termgfx()
        .args(["box", "Success!", "--style", "success", "--animate"])
        .assert()
        .success();
}

// ============================================================================
// BANNER ANIMATION TESTS
// ============================================================================

#[test]
fn test_banner_animate_flag() {
    termgfx()
        .args(["banner", "Animated Banner", "--animate"])
        .assert()
        .success();
}

#[test]
fn test_banner_animate_with_gradient() {
    termgfx()
        .args([
            "banner",
            "Gradient",
            "--gradient",
            "cyan-purple",
            "--animate",
        ])
        .assert()
        .success();
}

// ============================================================================
// SPARKLINE ANIMATION TESTS
// ============================================================================

#[test]
fn test_sparkline_animate_flag() {
    termgfx()
        .args(["sparkline", "1,2,3,4,5", "--animate"])
        .assert()
        .success();
}

#[test]
fn test_sparkline_animate_many_values() {
    termgfx()
        .args(["sparkline", "10,20,30,40,50,60,70,80,90,100", "--animate"])
        .assert()
        .success();
}

// ============================================================================
// BAR CHART ANIMATION TESTS
// ============================================================================

#[test]
fn test_bar_chart_animate_flag() {
    termgfx()
        .args(["chart", "bar", "--data", "A:10,B:20,C:30", "--animate"])
        .assert()
        .success();
}

#[test]
fn test_bar_chart_animate_multiple_bars() {
    termgfx()
        .args([
            "chart",
            "bar",
            "--data",
            "Sales:100,Costs:80,Profit:20",
            "--animate",
        ])
        .assert()
        .success();
}

// ============================================================================
// TABLE ANIMATION TESTS
// ============================================================================

#[test]
fn test_table_animate_flag() {
    termgfx()
        .args([
            "table",
            "--headers",
            "A,B,C",
            "--rows",
            "1,2,3|4,5,6",
            "--animate",
        ])
        .assert()
        .success();
}

#[test]
fn test_table_animate_with_border() {
    termgfx()
        .args([
            "table",
            "--headers",
            "Name,Age",
            "--rows",
            "Alice,30|Bob,25",
            "--border",
            "double",
            "--animate",
        ])
        .assert()
        .success();
}

// ============================================================================
// TREE ANIMATION TESTS
// ============================================================================

#[test]
fn test_tree_animate_flag() {
    termgfx()
        .args(["tree", "root>child1,child2", "--animate"])
        .assert()
        .success();
}

#[test]
fn test_tree_animate_deep() {
    termgfx()
        .args(["tree", "project>src,docs>main.rs,README", "--animate"])
        .assert()
        .success();
}

// ============================================================================
// COMBINED ANIMATION TESTS
// ============================================================================

#[test]
fn test_multiple_animation_options() {
    // Ensure all animate flags work
    termgfx()
        .args([
            "box",
            "Test",
            "--style",
            "info",
            "--border",
            "rounded",
            "--animate",
        ])
        .assert()
        .success();

    termgfx()
        .args(["banner", "Title", "--gradient", "green-cyan", "--animate"])
        .assert()
        .success();
}

#[test]
fn test_help_shows_animate_flag() {
    termgfx()
        .args(["box", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("animate"));

    termgfx()
        .args(["banner", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("animate"));

    termgfx()
        .args(["sparkline", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("animate"));

    termgfx()
        .args(["table", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("animate"));

    termgfx()
        .args(["tree", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("animate"));

    termgfx()
        .args(["chart", "bar", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("animate"));
}
