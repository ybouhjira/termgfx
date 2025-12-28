#![allow(deprecated)]
use assert_cmd::Command;
use predicates::prelude::*;

fn termgfx() -> Command {
    Command::cargo_bin("termgfx").unwrap()
}

// ============================================================================
// STYLE PREVIEW COMMAND TESTS
// ============================================================================

#[test]
fn test_style_preview_all() {
    termgfx()
        .args(["style", "preview"])
        .assert()
        .success()
        .stdout(predicate::str::contains("STYLE PRESETS"))
        .stdout(predicate::str::contains("SEMANTIC PRESETS"))
        .stdout(predicate::str::contains("DESIGN PRESETS"));
}

#[test]
fn test_style_preview_info() {
    termgfx()
        .args(["style", "preview", "info"])
        .assert()
        .success()
        .stdout(predicate::str::contains("INFO"))
        .stdout(predicate::str::contains("Information"));
}

#[test]
fn test_style_preview_success() {
    termgfx()
        .args(["style", "preview", "success"])
        .assert()
        .success()
        .stdout(predicate::str::contains("SUCCESS"));
}

#[test]
fn test_style_preview_warning() {
    termgfx()
        .args(["style", "preview", "warning"])
        .assert()
        .success()
        .stdout(predicate::str::contains("WARNING"));
}

#[test]
fn test_style_preview_danger() {
    termgfx()
        .args(["style", "preview", "danger"])
        .assert()
        .success()
        .stdout(predicate::str::contains("DANGER"));
}

// ============================================================================
// DESIGN PRESET TESTS
// ============================================================================

#[test]
fn test_style_preview_corporate() {
    termgfx()
        .args(["style", "preview", "corporate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("CORPORATE"))
        .stdout(predicate::str::contains("Professional"));
}

#[test]
fn test_style_preview_playful() {
    termgfx()
        .args(["style", "preview", "playful"])
        .assert()
        .success()
        .stdout(predicate::str::contains("PLAYFUL"))
        .stdout(predicate::str::contains("Rainbow"));
}

#[test]
fn test_style_preview_minimal() {
    termgfx()
        .args(["style", "preview", "minimal"])
        .assert()
        .success()
        .stdout(predicate::str::contains("MINIMAL"))
        .stdout(predicate::str::contains("Monochrome"));
}

#[test]
fn test_style_preview_retro() {
    termgfx()
        .args(["style", "preview", "retro"])
        .assert()
        .success()
        .stdout(predicate::str::contains("RETRO"))
        .stdout(predicate::str::contains("Matrix"));
}

// ============================================================================
// OTHER PRESETS
// ============================================================================

#[test]
fn test_style_preview_gradient() {
    termgfx()
        .args(["style", "preview", "gradient"])
        .assert()
        .success()
        .stdout(predicate::str::contains("GRADIENT"));
}

#[test]
fn test_style_preview_neutral() {
    termgfx()
        .args(["style", "preview", "neutral"])
        .assert()
        .success()
        .stdout(predicate::str::contains("NEUTRAL"));
}

// ============================================================================
// STYLE LIST COMMAND TESTS
// ============================================================================

#[test]
fn test_style_list() {
    termgfx()
        .args(["style", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Available Style Presets"))
        .stdout(predicate::str::contains("info"))
        .stdout(predicate::str::contains("success"))
        .stdout(predicate::str::contains("warning"))
        .stdout(predicate::str::contains("danger"))
        .stdout(predicate::str::contains("corporate"))
        .stdout(predicate::str::contains("playful"))
        .stdout(predicate::str::contains("minimal"))
        .stdout(predicate::str::contains("retro"))
        .stdout(predicate::str::contains("gradient"))
        .stdout(predicate::str::contains("neutral"));
}

#[test]
fn test_style_preview_unknown() {
    termgfx()
        .args(["style", "preview", "unknown"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown style preset"));
}

#[test]
fn test_style_help() {
    termgfx()
        .args(["style", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("style"))
        .stdout(predicate::str::contains("preview"))
        .stdout(predicate::str::contains("list"));
}

// ============================================================================
// CASE INSENSITIVITY TESTS
// ============================================================================

#[test]
fn test_style_preview_case_insensitive() {
    termgfx()
        .args(["style", "preview", "CORPORATE"])
        .assert()
        .success()
        .stdout(predicate::str::contains("CORPORATE"));

    termgfx()
        .args(["style", "preview", "Playful"])
        .assert()
        .success()
        .stdout(predicate::str::contains("PLAYFUL"));
}

// ============================================================================
// TTY BEHAVIOR TESTS (using rexpect for real PTY)
// ============================================================================

#[cfg(feature = "cli")]
mod tty_tests {
    use rexpect::spawn_bash;

    #[test]
    #[ignore = "Requires TTY - run with: cargo test tty_tests -- --ignored"]
    fn test_style_preview_in_real_tty() {
        let mut p = spawn_bash(Some(10_000)).expect("Failed to spawn bash");

        p.send_line("./target/debug/termgfx style preview")
            .expect("Failed to send");

        p.exp_string("SEMANTIC PRESETS")
            .expect("Semantic header not found");
        p.exp_string("DESIGN PRESETS")
            .expect("Design header not found");
        p.exp_string("$").expect("Command did not complete");
    }

    #[test]
    #[ignore = "Requires TTY - run with: cargo test tty_tests -- --ignored"]
    fn test_style_preview_corporate_in_tty() {
        let mut p = spawn_bash(Some(10_000)).expect("Failed to spawn bash");

        p.send_line("./target/debug/termgfx style preview corporate")
            .expect("Failed to send");

        p.exp_string("CORPORATE").expect("Corporate not found");
        // Double border characters
        p.exp_regex(r"╔|═").expect("Double border not found");
        p.exp_string("$").expect("Command did not complete");
    }

    #[test]
    #[ignore = "Requires TTY - run with: cargo test tty_tests -- --ignored"]
    fn test_style_preview_retro_in_tty() {
        let mut p = spawn_bash(Some(10_000)).expect("Failed to spawn bash");

        p.send_line("./target/debug/termgfx style preview retro")
            .expect("Failed to send");

        p.exp_string("RETRO").expect("Retro not found");
        // ASCII border characters
        p.exp_regex(r"\+|-").expect("ASCII border not found");
        p.exp_string("$").expect("Command did not complete");
    }
}
