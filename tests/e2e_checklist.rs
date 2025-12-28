#![allow(deprecated)]
use assert_cmd::Command;
use predicates::prelude::*;

fn termgfx() -> Command {
    Command::cargo_bin("termgfx").unwrap()
}

// ============================================================================
// CHECKLIST COMMAND TESTS
// ============================================================================

#[test]
fn test_checklist_basic() {
    termgfx()
        .args(["checklist", "--items", "Task A:done,Task B:pending"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Task A"))
        .stdout(predicate::str::contains("Task B"));
}

#[test]
fn test_checklist_with_checkbox_symbols() {
    termgfx()
        .args(["checklist", "--items", "Done:done,Not done:pending"])
        .assert()
        .success()
        .stdout(predicate::str::contains("☑"))
        .stdout(predicate::str::contains("☐"));
}

#[test]
fn test_checklist_with_columns() {
    termgfx()
        .args([
            "checklist",
            "--items",
            "Task A:done:2h,Task B:pending:1h",
            "--columns",
            "Duration",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Duration"))
        .stdout(predicate::str::contains("2h"))
        .stdout(predicate::str::contains("1h"));
}

#[test]
fn test_checklist_multiple_columns() {
    termgfx()
        .args([
            "checklist",
            "--items",
            "Task A:done:2h:High,Task B:pending:1h:Low",
            "--columns",
            "Duration,Priority",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Duration"))
        .stdout(predicate::str::contains("Priority"))
        .stdout(predicate::str::contains("High"))
        .stdout(predicate::str::contains("Low"));
}

#[test]
fn test_checklist_stats_display() {
    termgfx()
        .args([
            "checklist",
            "--items",
            "Task A:done,Task B:pending,Task C:done",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Stats:"))
        .stdout(predicate::str::contains("2/3"))
        .stdout(predicate::str::contains("67%"));
}

#[test]
fn test_checklist_no_stats() {
    termgfx()
        .args([
            "checklist",
            "--items",
            "Task A:done,Task B:pending",
            "--no-stats",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Task A"))
        .stdout(predicate::str::contains("Stats:").not());
}

#[test]
fn test_checklist_json_output() {
    termgfx()
        .args([
            "checklist",
            "--items",
            "Task A:done,Task B:pending",
            "--json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"label\": \"Task A\""))
        .stdout(predicate::str::contains("\"checked\": true"))
        .stdout(predicate::str::contains("\"checked\": false"))
        .stdout(predicate::str::contains("\"completed\": 1"))
        .stdout(predicate::str::contains("\"total\": 2"));
}

#[test]
fn test_checklist_various_done_statuses() {
    // All these should be marked as checked
    for status in &["done", "complete", "completed", "yes", "true", "1"] {
        let items = format!("Task:{}", status);
        termgfx()
            .args(["checklist", "--items", &items, "--json"])
            .assert()
            .success()
            .stdout(predicate::str::contains("\"checked\": true"));
    }
}

#[test]
fn test_checklist_various_pending_statuses() {
    // All these should be marked as unchecked
    for status in &["pending", "no", "false", "0", "todo", "x"] {
        let items = format!("Task:{}", status);
        termgfx()
            .args(["checklist", "--items", &items, "--json"])
            .assert()
            .success()
            .stdout(predicate::str::contains("\"checked\": false"));
    }
}

#[test]
fn test_checklist_all_complete_percentage() {
    termgfx()
        .args(["checklist", "--items", "A:done,B:done,C:done"])
        .assert()
        .success()
        .stdout(predicate::str::contains("100%"));
}

#[test]
fn test_checklist_half_complete_percentage() {
    termgfx()
        .args(["checklist", "--items", "A:done,B:pending"])
        .assert()
        .success()
        .stdout(predicate::str::contains("50%"));
}

#[test]
fn test_checklist_help() {
    termgfx()
        .args(["checklist", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("checklist"))
        .stdout(predicate::str::contains("--items"))
        .stdout(predicate::str::contains("--columns"));
}

#[test]
fn test_checklist_missing_items() {
    termgfx()
        .args(["checklist"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

// ============================================================================
// TTY BEHAVIOR TESTS (using rexpect for real PTY)
// ============================================================================

#[cfg(feature = "cli")]
mod tty_tests {
    use rexpect::spawn_bash;

    #[test]
    #[ignore = "Requires TTY - run with: cargo test tty_tests -- --ignored"]
    fn test_checklist_in_real_tty() {
        let mut p = spawn_bash(Some(10_000)).expect("Failed to spawn bash");

        // Build fresh
        p.send_line("cargo build --quiet").expect("Failed to send");
        p.exp_string("$").expect("Build failed");

        // Run checklist
        p.send_line("./target/debug/termgfx checklist --items 'Task A:done:2h,Task B:pending:1h' --columns Duration")
            .expect("Failed to send");

        // Expect output
        p.exp_regex("☑.*Task A").expect("Checked task not found");
        p.exp_regex("☐.*Task B").expect("Unchecked task not found");
        p.exp_string("Stats:").expect("Stats not found");
        p.exp_string("$").expect("Command did not complete");
    }

    #[test]
    #[ignore = "Requires TTY - run with: cargo test tty_tests -- --ignored"]
    fn test_checklist_json_in_real_tty() {
        let mut p = spawn_bash(Some(10_000)).expect("Failed to spawn bash");

        p.send_line("./target/debug/termgfx checklist --items 'A:done,B:pending' --json")
            .expect("Failed to send");

        p.exp_string("\"completed\": 1")
            .expect("JSON output not correct");
        p.exp_string("\"total\": 2")
            .expect("JSON output not correct");
        p.exp_string("$").expect("Command did not complete");
    }

    #[test]
    #[ignore = "Requires TTY - run with: cargo test tty_tests -- --ignored"]
    fn test_checklist_colors_in_tty() {
        let mut p = spawn_bash(Some(10_000)).expect("Failed to spawn bash");

        // Force color output
        p.send_line("FORCE_COLOR=1 ./target/debug/termgfx checklist --items 'Done:done,Pending:pending'")
            .expect("Failed to send");

        // ANSI escape codes for green (32m) should appear for checked items
        p.exp_regex(r"\x1b\[32m").expect("Green color not found");
        p.exp_string("$").expect("Command did not complete");
    }
}
