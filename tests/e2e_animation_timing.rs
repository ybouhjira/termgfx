#![allow(deprecated)]
use assert_cmd::Command;
use std::time::{Duration, Instant};

fn termgfx() -> Command {
    Command::cargo_bin("termgfx").unwrap()
}

const MAX_ANIMATION_TIME: Duration = Duration::from_secs(3);

// ============================================================================
// BOX ANIMATION TIMING TESTS
// ============================================================================

#[test]
#[ignore] // Timing-based tests are flaky under coverage instrumentation
fn test_box_animation_completes_in_time() {
    let start = Instant::now();

    termgfx()
        .args(["box", "This is a test message for timing", "--animate"])
        .timeout(MAX_ANIMATION_TIME)
        .assert()
        .success();

    let elapsed = start.elapsed();
    assert!(
        elapsed < MAX_ANIMATION_TIME,
        "Box animation took {:?}, expected less than {:?}",
        elapsed,
        MAX_ANIMATION_TIME
    );
}

#[test]
#[ignore] // Timing-based tests are flaky under coverage instrumentation
fn test_box_long_message_animation_timing() {
    let start = Instant::now();

    termgfx()
        .args(["box", "A much longer message that should still animate quickly without taking forever to complete", "--animate"])
        .timeout(MAX_ANIMATION_TIME)
        .assert()
        .success();

    let elapsed = start.elapsed();
    assert!(
        elapsed < MAX_ANIMATION_TIME,
        "Box long message animation took {:?}, expected less than {:?}",
        elapsed,
        MAX_ANIMATION_TIME
    );
}

// ============================================================================
// BANNER ANIMATION TIMING TESTS
// ============================================================================

#[test]
#[ignore] // Timing-based tests are flaky under coverage instrumentation
fn test_banner_animation_completes_in_time() {
    let start = Instant::now();

    termgfx()
        .args(["banner", "TIMING TEST", "--animate"])
        .timeout(MAX_ANIMATION_TIME)
        .assert()
        .success();

    let elapsed = start.elapsed();
    assert!(
        elapsed < MAX_ANIMATION_TIME,
        "Banner animation took {:?}, expected less than {:?}",
        elapsed,
        MAX_ANIMATION_TIME
    );
}

#[test]
#[ignore] // Timing-based tests are flaky under coverage instrumentation
fn test_banner_with_gradient_animation_timing() {
    let start = Instant::now();

    termgfx()
        .args([
            "banner",
            "GRADIENT",
            "--gradient",
            "cyan-purple",
            "--animate",
        ])
        .timeout(MAX_ANIMATION_TIME)
        .assert()
        .success();

    let elapsed = start.elapsed();
    assert!(
        elapsed < MAX_ANIMATION_TIME,
        "Banner gradient animation took {:?}, expected less than {:?}",
        elapsed,
        MAX_ANIMATION_TIME
    );
}

// ============================================================================
// SPARKLINE ANIMATION TIMING TESTS
// ============================================================================

#[test]
#[ignore] // Timing-based tests are flaky under coverage instrumentation
fn test_sparkline_animation_completes_in_time() {
    let start = Instant::now();

    termgfx()
        .args([
            "sparkline",
            "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20",
            "--animate",
        ])
        .timeout(MAX_ANIMATION_TIME)
        .assert()
        .success();

    let elapsed = start.elapsed();
    assert!(
        elapsed < MAX_ANIMATION_TIME,
        "Sparkline animation took {:?}, expected less than {:?}",
        elapsed,
        MAX_ANIMATION_TIME
    );
}

// ============================================================================
// BAR CHART ANIMATION TIMING TESTS
// ============================================================================

#[test]
#[ignore] // Timing-based tests are flaky under coverage instrumentation
fn test_bar_chart_animation_completes_in_time() {
    let start = Instant::now();

    termgfx()
        .args([
            "chart",
            "bar",
            "--data",
            "A:10,B:20,C:30,D:40,E:50",
            "--animate",
        ])
        .timeout(MAX_ANIMATION_TIME)
        .assert()
        .success();

    let elapsed = start.elapsed();
    assert!(
        elapsed < MAX_ANIMATION_TIME,
        "Bar chart animation took {:?}, expected less than {:?}",
        elapsed,
        MAX_ANIMATION_TIME
    );
}

// ============================================================================
// TABLE ANIMATION TIMING TESTS
// ============================================================================

#[test]
#[ignore] // Timing-based tests are flaky under coverage instrumentation
fn test_table_animation_completes_in_time() {
    let start = Instant::now();

    termgfx()
        .args([
            "table",
            "--headers",
            "A,B,C,D",
            "--rows",
            "1,2,3,4|5,6,7,8|9,10,11,12|13,14,15,16",
            "--animate",
        ])
        .timeout(MAX_ANIMATION_TIME)
        .assert()
        .success();

    let elapsed = start.elapsed();
    assert!(
        elapsed < MAX_ANIMATION_TIME,
        "Table animation took {:?}, expected less than {:?}",
        elapsed,
        MAX_ANIMATION_TIME
    );
}

// ============================================================================
// TREE ANIMATION TIMING TESTS
// ============================================================================

#[test]
#[ignore] // Timing-based tests are flaky under coverage instrumentation
fn test_tree_animation_completes_in_time() {
    let start = Instant::now();

    termgfx()
        .args(["tree", "root>a,b,c,d,e>f,g,h,i,j", "--animate"])
        .timeout(MAX_ANIMATION_TIME)
        .assert()
        .success();

    let elapsed = start.elapsed();
    assert!(
        elapsed < MAX_ANIMATION_TIME,
        "Tree animation took {:?}, expected less than {:?}",
        elapsed,
        MAX_ANIMATION_TIME
    );
}

// ============================================================================
// GAUGE ANIMATION TIMING TESTS
// ============================================================================

#[test]
#[ignore] // Timing-based tests are flaky under coverage instrumentation
fn test_gauge_animation_completes_in_time() {
    let start = Instant::now();

    termgfx()
        .args(["gauge", "75", "--label", "Test", "--animate"])
        .timeout(MAX_ANIMATION_TIME)
        .assert()
        .success();

    let elapsed = start.elapsed();
    assert!(
        elapsed < MAX_ANIMATION_TIME,
        "Gauge animation took {:?}, expected less than {:?}",
        elapsed,
        MAX_ANIMATION_TIME
    );
}

// ============================================================================
// TIMELINE ANIMATION TIMING TESTS
// ============================================================================

#[test]
#[ignore] // Timing-based tests are flaky under coverage instrumentation
fn test_timeline_animation_completes_in_time() {
    let start = Instant::now();

    termgfx()
        .args([
            "timeline",
            "--events",
            "Start,Middle,End,Final",
            "--animate",
        ])
        .timeout(MAX_ANIMATION_TIME)
        .assert()
        .success();

    let elapsed = start.elapsed();
    assert!(
        elapsed < MAX_ANIMATION_TIME,
        "Timeline animation took {:?}, expected less than {:?}",
        elapsed,
        MAX_ANIMATION_TIME
    );
}

// ============================================================================
// HEATMAP ANIMATION TIMING TESTS
// ============================================================================

#[test]
#[ignore] // Timing-based tests are flaky under coverage instrumentation
fn test_heatmap_animation_completes_in_time() {
    let start = Instant::now();

    termgfx()
        .args(["heatmap", "--data", "1,2,3;4,5,6;7,8,9", "--animate"])
        .timeout(MAX_ANIMATION_TIME)
        .assert()
        .success();

    let elapsed = start.elapsed();
    assert!(
        elapsed < MAX_ANIMATION_TIME,
        "Heatmap animation took {:?}, expected less than {:?}",
        elapsed,
        MAX_ANIMATION_TIME
    );
}

// ============================================================================
// COMBINED TIMING TEST - ALL ANIMATIONS UNDER 2 SECONDS
// ============================================================================

#[test]
#[ignore] // Timing-based tests are flaky under coverage instrumentation
fn test_all_animations_are_fast() {
    let fast_limit = Duration::from_secs(2);

    // Box
    let start = Instant::now();
    termgfx()
        .args(["box", "Quick test", "--animate"])
        .assert()
        .success();
    assert!(start.elapsed() < fast_limit, "Box too slow");

    // Banner
    let start = Instant::now();
    termgfx()
        .args(["banner", "TEST", "--animate"])
        .assert()
        .success();
    assert!(start.elapsed() < fast_limit, "Banner too slow");

    // Sparkline
    let start = Instant::now();
    termgfx()
        .args(["sparkline", "1,2,3,4,5", "--animate"])
        .assert()
        .success();
    assert!(start.elapsed() < fast_limit, "Sparkline too slow");

    // Table
    let start = Instant::now();
    termgfx()
        .args([
            "table",
            "--headers",
            "A,B",
            "--rows",
            "1,2|3,4",
            "--animate",
        ])
        .assert()
        .success();
    assert!(start.elapsed() < fast_limit, "Table too slow");

    // Tree
    let start = Instant::now();
    termgfx()
        .args(["tree", "root>a,b", "--animate"])
        .assert()
        .success();
    assert!(start.elapsed() < fast_limit, "Tree too slow");
}
