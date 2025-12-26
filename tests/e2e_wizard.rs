use std::process::Command;

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
            "run",
            "--",
            "wizard",
            "--step",
            "invalid", // Missing format
            "--output",
            "json",
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

#[test]
fn test_wizard_help() {
    let output = Command::new("cargo")
        .args(["run", "--", "wizard", "--help"])
        .output()
        .expect("Failed to run wizard");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Multi-step wizard"));
    assert!(stdout.contains("--step"));
    assert!(stdout.contains("--config"));
    assert!(output.status.success());
}

#[test]
fn test_wizard_select_requires_options() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "wizard",
            "--step",
            "select:id:prompt", // Missing options
            "--output",
            "json",
        ])
        .output()
        .expect("Failed to run wizard");

    // Should fail because select needs options
    assert!(!output.status.success());
}
