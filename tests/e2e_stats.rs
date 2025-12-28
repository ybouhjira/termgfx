#![allow(deprecated)]
use assert_cmd::Command;
use predicates::prelude::*;

fn termgfx() -> Command {
    Command::cargo_bin("termgfx").unwrap()
}

// ============================================================================
// STATS BAR COMMAND TESTS
// ============================================================================

#[test]
fn test_stats_basic() {
    termgfx()
        .args(["stats", "entries:500,size:2.3 MB"])
        .assert()
        .success()
        .stdout(predicate::str::contains("entries"))
        .stdout(predicate::str::contains("500"))
        .stdout(predicate::str::contains("size"))
        .stdout(predicate::str::contains("2.3 MB"));
}

#[test]
fn test_stats_with_emoji() {
    termgfx()
        .args(["stats", "count:100", "--emoji", "📊"])
        .assert()
        .success()
        .stdout(predicate::str::contains("📊"))
        .stdout(predicate::str::contains("100"));
}

#[test]
fn test_stats_pipe_separator() {
    termgfx()
        .args(["stats", "a:1,b:2", "--separator", "pipe"])
        .assert()
        .success()
        .stdout(predicate::str::contains("│"));
}

#[test]
fn test_stats_dot_separator() {
    termgfx()
        .args(["stats", "a:1,b:2", "--separator", "dot"])
        .assert()
        .success()
        .stdout(predicate::str::contains("•"));
}

#[test]
fn test_stats_slash_separator() {
    termgfx()
        .args(["stats", "a:1,b:2", "--separator", "slash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("/"));
}

#[test]
fn test_stats_diamond_separator() {
    termgfx()
        .args(["stats", "a:1,b:2", "--separator", "diamond"])
        .assert()
        .success()
        .stdout(predicate::str::contains("◆"));
}

#[test]
fn test_stats_arrow_separator() {
    termgfx()
        .args(["stats", "a:1,b:2", "--separator", "arrow"])
        .assert()
        .success()
        .stdout(predicate::str::contains("→"));
}

#[test]
fn test_stats_items_flag() {
    termgfx()
        .args([
            "stats",
            "--items",
            "Files:1,234",
            "--items",
            "Size:45 MB",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Files"))
        .stdout(predicate::str::contains("1,234"))
        .stdout(predicate::str::contains("Size"))
        .stdout(predicate::str::contains("45 MB"));
}

#[test]
fn test_stats_json_output() {
    termgfx()
        .args(["stats", "count:100,size:50 MB", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"label\": \"count\""))
        .stdout(predicate::str::contains("\"value\": \"100\""))
        .stdout(predicate::str::contains("\"label\": \"size\""));
}

#[test]
fn test_stats_no_color() {
    // With --no-color, output should not have ANSI codes for values
    // This is a basic check - without color codes, the raw text should be present
    termgfx()
        .args(["stats", "status:ok", "--no-color"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ok"));
}

#[test]
fn test_stats_percentage_coloring() {
    // Low percentage should be green (32m)
    termgfx()
        .args(["stats", "usage:30%"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\u{1b}[32m")); // Green

    // High percentage should be red (31m)
    termgfx()
        .args(["stats", "usage:95%"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\u{1b}[31m")); // Red
}

#[test]
fn test_stats_status_coloring() {
    // "ok" should be green
    termgfx()
        .args(["stats", "status:ok"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\u{1b}[32m")); // Green

    // "error" should be red
    termgfx()
        .args(["stats", "status:error"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\u{1b}[31m")); // Red
}

#[test]
fn test_stats_size_coloring() {
    // Size values should be yellow
    termgfx()
        .args(["stats", "size:100 MB"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\u{1b}[33m")); // Yellow
}

#[test]
fn test_stats_time_coloring() {
    // Time values should be cyan
    termgfx()
        .args(["stats", "modified:5m ago"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\u{1b}[36m")); // Cyan
}

#[test]
fn test_stats_help() {
    termgfx()
        .args(["stats", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("stats"))
        .stdout(predicate::str::contains("--separator"))
        .stdout(predicate::str::contains("--emoji"))
        .stdout(predicate::str::contains("--items"));
}

// ============================================================================
// TTY BEHAVIOR TESTS (using rexpect for real PTY)
// ============================================================================

#[cfg(feature = "cli")]
mod tty_tests {
    use rexpect::spawn_bash;

    #[test]
    #[ignore = "Requires TTY - run with: cargo test tty_tests -- --ignored"]
    fn test_stats_in_real_tty() {
        let mut p = spawn_bash(Some(10_000)).expect("Failed to spawn bash");

        p.send_line("./target/debug/termgfx stats 'entries:500,size:2.3 MB'")
            .expect("Failed to send");

        p.exp_string("entries").expect("Label not found");
        p.exp_string("500").expect("Value not found");
        p.exp_string("│").expect("Separator not found");
        p.exp_string("$").expect("Command did not complete");
    }

    #[test]
    #[ignore = "Requires TTY - run with: cargo test tty_tests -- --ignored"]
    fn test_stats_colors_in_tty() {
        let mut p = spawn_bash(Some(10_000)).expect("Failed to spawn bash");

        p.send_line("./target/debug/termgfx stats 'CPU:45%,Memory:92%'")
            .expect("Failed to send");

        // Should have ANSI colors
        p.exp_regex(r"\x1b\[3[12]m").expect("Color codes not found");
        p.exp_string("$").expect("Command did not complete");
    }

    #[test]
    #[ignore = "Requires TTY - run with: cargo test tty_tests -- --ignored"]
    fn test_stats_with_emoji_in_tty() {
        let mut p = spawn_bash(Some(10_000)).expect("Failed to spawn bash");

        p.send_line("./target/debug/termgfx stats 'count:100' --emoji '📊'")
            .expect("Failed to send");

        p.exp_string("📊").expect("Emoji not found");
        p.exp_string("100").expect("Value not found");
        p.exp_string("$").expect("Command did not complete");
    }
}
