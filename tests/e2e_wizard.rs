#![allow(deprecated)]
use std::io::Write;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

#[allow(dead_code)]
fn create_test_config() -> String {
    serde_json::json!({
        "title": "User Registration",
        "steps": [
            {
                "id": "name",
                "type": "input",
                "prompt": "Enter your name"
            },
            {
                "id": "role",
                "type": "select",
                "prompt": "Select your role",
                "options": ["Admin", "User", "Guest"]
            },
            {
                "id": "confirm",
                "type": "confirm",
                "prompt": "Proceed?"
            },
            {
                "id": "summary",
                "type": "summary",
                "prompt": "Review your choices"
            }
        ]
    })
    .to_string()
}

#[test]
#[ignore] // Requires TTY for interactive input
fn test_wizard_inline_steps_input_and_select() {
    let mut child = Command::new("cargo")
        .args([
            "run",
            "--",
            "wizard",
            "--step",
            "input:name:Enter your name",
            "--step",
            "select:role:Choose role:Admin,User",
            "--output",
            "json",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn wizard");

    // Simulate user input: type "Alice", Enter, Down arrow, Enter
    if let Some(mut stdin) = child.stdin.take() {
        thread::sleep(Duration::from_millis(500));
        stdin.write_all(b"Alice\n").ok();
        thread::sleep(Duration::from_millis(300));
        stdin.write_all(b"\x1b[B").ok(); // Down arrow
        thread::sleep(Duration::from_millis(100));
        stdin.write_all(b"\n").ok(); // Enter
    }

    let result = child.wait_with_output().expect("Failed to wait for wizard");
    let stdout = String::from_utf8_lossy(&result.stdout);

    // Check JSON output contains the values
    assert!(stdout.contains("\"name\""));
    assert!(stdout.contains("\"role\""));
}

#[test]
#[ignore] // Requires TTY for interactive input
fn test_wizard_multiselect() {
    let mut output = Command::new("cargo")
        .args([
            "run",
            "--",
            "wizard",
            "--step",
            "multiselect:features:Select features:Email,SMS,Push",
            "--output",
            "json",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn wizard");

    // Simulate: Space (select Email), Down, Space (select SMS), Enter
    if let Some(mut stdin) = output.stdin.take() {
        thread::sleep(Duration::from_millis(500));
        stdin.write_all(b" ").ok(); // Space
        thread::sleep(Duration::from_millis(100));
        stdin.write_all(b"\x1b[B").ok(); // Down
        thread::sleep(Duration::from_millis(100));
        stdin.write_all(b" ").ok(); // Space
        thread::sleep(Duration::from_millis(100));
        stdin.write_all(b"\n").ok(); // Enter
    }

    let result = output
        .wait_with_output()
        .expect("Failed to wait for wizard");
    let stdout = String::from_utf8_lossy(&result.stdout);

    assert!(stdout.contains("\"features\""));
}

#[test]
#[ignore] // Requires TTY for interactive input
fn test_wizard_confirm_step() {
    let mut output = Command::new("cargo")
        .args([
            "run",
            "--",
            "wizard",
            "--step",
            "confirm:agree:Do you agree?",
            "--output",
            "json",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn wizard");

    // Simulate: y (yes)
    if let Some(mut stdin) = output.stdin.take() {
        thread::sleep(Duration::from_millis(500));
        stdin.write_all(b"y").ok();
    }

    let result = output
        .wait_with_output()
        .expect("Failed to wait for wizard");
    let stdout = String::from_utf8_lossy(&result.stdout);

    assert!(stdout.contains("\"agree\""));
    assert!(stdout.contains("true"));
}

#[test]
#[ignore] // Requires TTY for interactive input
fn test_wizard_env_output() {
    let mut output = Command::new("cargo")
        .args([
            "run",
            "--",
            "wizard",
            "--step",
            "input:username:Enter username",
            "--output",
            "env",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn wizard");

    // Type "testuser" and press Enter
    if let Some(mut stdin) = output.stdin.take() {
        thread::sleep(Duration::from_millis(500));
        stdin.write_all(b"testuser\n").ok();
    }

    let result = output
        .wait_with_output()
        .expect("Failed to wait for wizard");
    let stdout = String::from_utf8_lossy(&result.stdout);

    // ENV format should be USERNAME=testuser
    assert!(stdout.contains("USERNAME="));
    assert!(stdout.contains("testuser"));
}

#[test]
#[ignore] // Requires TTY for interactive input
fn test_wizard_with_title() {
    let mut output = Command::new("cargo")
        .args([
            "run",
            "--",
            "wizard",
            "--title",
            "Setup Wizard",
            "--step",
            "input:name:Your name",
            "--output",
            "json",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn wizard");

    // Type "John" and press Enter
    if let Some(mut stdin) = output.stdin.take() {
        thread::sleep(Duration::from_millis(500));
        stdin.write_all(b"John\n").ok();
    }

    let result = output
        .wait_with_output()
        .expect("Failed to wait for wizard");
    assert!(result.status.success());
}

#[test]
#[ignore] // Requires TTY for interactive input
fn test_wizard_progress_indicator() {
    // This test verifies that the wizard shows "Step X/Y" progress
    // We can't easily test the interactive display, but we can ensure
    // the wizard processes multiple steps correctly
    let mut output = Command::new("cargo")
        .args([
            "run",
            "--",
            "wizard",
            "--step",
            "input:step1:First step",
            "--step",
            "input:step2:Second step",
            "--step",
            "input:step3:Third step",
            "--output",
            "json",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn wizard");

    // Complete all three steps
    if let Some(mut stdin) = output.stdin.take() {
        thread::sleep(Duration::from_millis(500));
        stdin.write_all(b"value1\n").ok();
        thread::sleep(Duration::from_millis(300));
        stdin.write_all(b"value2\n").ok();
        thread::sleep(Duration::from_millis(300));
        stdin.write_all(b"value3\n").ok();
    }

    let result = output
        .wait_with_output()
        .expect("Failed to wait for wizard");
    let stdout = String::from_utf8_lossy(&result.stdout);

    // All three steps should be in output
    assert!(stdout.contains("\"step1\""));
    assert!(stdout.contains("\"step2\""));
    assert!(stdout.contains("\"step3\""));
}

#[test]
fn test_wizard_no_steps_error() {
    let output = Command::new("cargo")
        .args(["run", "--", "wizard", "--output", "json"])
        .output()
        .expect("Failed to run wizard");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Provide at least one --step or a --config file"));
    assert!(!output.status.success());
}

#[test]
fn test_wizard_invalid_step_format() {
    let output = Command::new("cargo")
        .args([
            "run", "--", "wizard", "--step", "invalid", // Missing format
            "--output", "json",
        ])
        .output()
        .expect("Failed to run wizard");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Invalid step format") || stderr.contains("Error"));
    assert!(!output.status.success());
}

#[test]
fn test_wizard_unknown_step_type() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "wizard",
            "--step",
            "unknown:id:prompt",
            "--output",
            "json",
        ])
        .output()
        .expect("Failed to run wizard");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown step type") || stderr.contains("Error"));
    assert!(!output.status.success());
}
