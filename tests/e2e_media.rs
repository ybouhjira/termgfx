use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

fn termgfx() -> Command {
    Command::cargo_bin("termgfx").unwrap()
}

// ============================================================================
// IMAGE COMMAND TESTS
// ============================================================================

#[test]
fn test_image_help() {
    termgfx()
        .args(["image", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::is_match("(?i)path").unwrap())
        .stdout(predicate::str::is_match("(?i)protocol").unwrap());
}

#[test]
fn test_image_nonexistent_file() {
    termgfx()
        .args(["image", "/nonexistent/image.png"])
        .assert()
        .failure();
}

#[test]
fn test_image_protocol_options() {
    termgfx()
        .args(["image", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("auto"))
        .stdout(predicate::str::contains("kitty"))
        .stdout(predicate::str::contains("sixel"))
        .stdout(predicate::str::contains("halfblock"));
}

// ============================================================================
// RECORD COMMAND TESTS
// ============================================================================

#[test]
fn test_record_help() {
    termgfx()
        .args(["record", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("start"))
        .stdout(predicate::str::contains("play"))
        .stdout(predicate::str::contains("export"));
}

#[test]
fn test_record_start_help() {
    termgfx()
        .args(["record", "start", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::is_match("(?i)output").unwrap());
}

#[test]
fn test_record_play_help() {
    termgfx()
        .args(["record", "play", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::is_match("(?i)input").unwrap())
        .stdout(predicate::str::is_match("(?i)speed").unwrap());
}

#[test]
fn test_record_export_help() {
    termgfx()
        .args(["record", "export", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::is_match("(?i)input").unwrap())
        .stdout(predicate::str::is_match("(?i)format").unwrap())
        .stdout(predicate::str::is_match("(?i)output").unwrap());
}

#[test]
fn test_record_play_nonexistent_file() {
    termgfx()
        .args(["record", "play", "/nonexistent/recording.json"])
        .assert()
        .failure();
}

// ============================================================================
// SCRIPT COMMAND TESTS
// ============================================================================

#[test]
fn test_script_help() {
    termgfx()
        .args(["script", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("file"))
        .stdout(predicate::str::contains("inline"));
}

#[test]
fn test_script_inline_banner() {
    termgfx()
        .args(["script", "--inline", "banner HI"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_script_inline_box() {
    termgfx()
        .args(["script", "--inline", "box \"Hello World\""])
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello World"));
}

#[test]
fn test_script_inline_progress() {
    termgfx()
        .args(["script", "--inline", "progress 50"])
        .assert()
        .success()
        .stdout(predicate::str::contains("50%"));
}

#[test]
fn test_script_inline_sparkline() {
    termgfx()
        .args(["script", "--inline", "sparkline 1,2,3,4,5"])
        .assert()
        .success();
}

#[test]
fn test_script_inline_typewriter() {
    termgfx()
        .args(["script", "--inline", "typewriter \"Test message\""])
        .assert()
        .success()
        .stdout(predicate::str::contains("Test message"));
}

#[test]
fn test_script_inline_multiple_commands() {
    termgfx()
        .args(["script", "--inline", "box \"First\"\nbox \"Second\""])
        .assert()
        .success()
        .stdout(predicate::str::contains("First"))
        .stdout(predicate::str::contains("Second"));
}

#[test]
fn test_script_from_file() {
    let mut script_file = NamedTempFile::with_suffix(".termgfx").unwrap();
    writeln!(script_file, "# Comment line").unwrap();
    writeln!(script_file, "box \"From file\" style:success").unwrap();

    termgfx()
        .args(["script", "--file", script_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("From file"));
}

#[test]
fn test_script_with_options() {
    termgfx()
        .args([
            "script",
            "--inline",
            "box \"Styled\" style:warning border:double",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Styled"));
}

#[test]
fn test_script_unknown_command() {
    termgfx()
        .args(["script", "--inline", "unknowncommand"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Unknown command"));
}

#[test]
fn test_script_no_file_or_inline() {
    termgfx()
        .args(["script"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Either --file or --inline must be provided",
        ));
}

#[test]
fn test_script_nonexistent_file() {
    termgfx()
        .args(["script", "--file", "/nonexistent/script.termgfx"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read"));
}
