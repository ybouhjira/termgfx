#![allow(deprecated)]
use assert_cmd::Command;
use predicates::prelude::*;

fn termgfx() -> Command {
    Command::cargo_bin("termgfx").unwrap()
}

// ============================================================================
// DANGER ZONE COMMAND TESTS
// ============================================================================

#[test]
fn test_danger_zone_basic() {
    termgfx()
        .args(["danger-zone", "This is dangerous!"])
        .assert()
        .success()
        .stdout(predicate::str::contains("DANGER ZONE"))
        .stdout(predicate::str::contains("This is dangerous!"));
}

#[test]
fn test_danger_zone_default_header() {
    termgfx()
        .args(["danger-zone", "Warning message"])
        .assert()
        .success()
        .stdout(predicate::str::contains("⚠️"))
        .stdout(predicate::str::contains("DANGER ZONE"));
}

#[test]
fn test_danger_zone_custom_title() {
    termgfx()
        .args([
            "danger-zone",
            "Delete operation",
            "--title",
            "🗑️  DELETE CONFIRMATION",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("DELETE CONFIRMATION"))
        .stdout(predicate::str::contains("Delete operation"));
}

#[test]
fn test_danger_zone_double_border() {
    termgfx()
        .args(["danger-zone", "Test", "--border", "double"])
        .assert()
        .success()
        .stdout(predicate::str::contains("╔"))
        .stdout(predicate::str::contains("╗"))
        .stdout(predicate::str::contains("╚"))
        .stdout(predicate::str::contains("╝"));
}

#[test]
fn test_danger_zone_single_border() {
    termgfx()
        .args(["danger-zone", "Test", "--border", "single"])
        .assert()
        .success()
        .stdout(predicate::str::contains("┌"))
        .stdout(predicate::str::contains("┐"));
}

#[test]
fn test_danger_zone_thick_border() {
    termgfx()
        .args(["danger-zone", "Test", "--border", "thick"])
        .assert()
        .success()
        .stdout(predicate::str::contains("┏"))
        .stdout(predicate::str::contains("┓"));
}

#[test]
fn test_danger_zone_multiline_message() {
    termgfx()
        .args(["danger-zone", "Line 1\nLine 2\nLine 3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Line 1"))
        .stdout(predicate::str::contains("Line 2"))
        .stdout(predicate::str::contains("Line 3"));
}

#[test]
fn test_danger_zone_has_red_coloring() {
    // Check for ANSI red escape codes
    termgfx()
        .args(["danger-zone", "Red warning"])
        .assert()
        .success()
        // ANSI bright red is typically 91m
        .stdout(predicate::str::contains("\u{1b}[91"));
}

#[test]
fn test_danger_zone_has_header_separator() {
    // Check for header separator line (╠ for double border)
    termgfx()
        .args(["danger-zone", "Test", "--border", "double"])
        .assert()
        .success()
        .stdout(predicate::str::contains("╠"))
        .stdout(predicate::str::contains("╣"));
}

#[test]
fn test_danger_zone_help() {
    termgfx()
        .args(["danger-zone", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("danger-zone"))
        .stdout(predicate::str::contains("--title"))
        .stdout(predicate::str::contains("--border"))
        .stdout(predicate::str::contains("destructive"));
}

#[test]
fn test_danger_zone_message_required() {
    termgfx()
        .args(["danger-zone"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_danger_zone_animate_flag() {
    // Just verify the flag is accepted
    termgfx()
        .args(["danger-zone", "Animated warning", "--animate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("DANGER ZONE"));
}

// ============================================================================
// TTY BEHAVIOR TESTS (using rexpect for real PTY)
// ============================================================================

#[cfg(feature = "cli")]
mod tty_tests {
    use rexpect::spawn_bash;

    #[test]
    #[ignore = "Requires TTY - run with: cargo test tty_tests -- --ignored"]
    fn test_danger_zone_in_real_tty() {
        let mut p = spawn_bash(Some(10_000)).expect("Failed to spawn bash");

        p.send_line("./target/debug/termgfx danger-zone 'This is a test warning'")
            .expect("Failed to send");

        p.exp_string("DANGER ZONE").expect("Header not found");
        p.exp_string("This is a test warning")
            .expect("Message not found");
        p.exp_string("$").expect("Command did not complete");
    }

    #[test]
    #[ignore = "Requires TTY - run with: cargo test tty_tests -- --ignored"]
    fn test_danger_zone_colors_in_tty() {
        let mut p = spawn_bash(Some(10_000)).expect("Failed to spawn bash");

        p.send_line("./target/debug/termgfx danger-zone 'Red warning'")
            .expect("Failed to send");

        // ANSI escape codes for bright red (91m) and red background (101m)
        p.exp_regex(r"\x1b\[91").expect("Red color not found");
        p.exp_string("$").expect("Command did not complete");
    }

    #[test]
    #[ignore = "Requires TTY - run with: cargo test tty_tests -- --ignored"]
    fn test_danger_zone_custom_title_in_tty() {
        let mut p = spawn_bash(Some(10_000)).expect("Failed to spawn bash");

        p.send_line(
            "./target/debug/termgfx danger-zone 'Delete all data' --title 'CONFIRM DELETE'",
        )
        .expect("Failed to send");

        p.exp_string("CONFIRM DELETE")
            .expect("Custom title not found");
        p.exp_string("Delete all data")
            .expect("Message not found");
        p.exp_string("$").expect("Command did not complete");
    }
}
