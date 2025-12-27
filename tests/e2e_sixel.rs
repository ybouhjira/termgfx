use assert_cmd::Command;

const TEST_IMAGE: &str = "docs/gifs/box.gif";

/// Test that sixel protocol does NOT show fallback message
#[test]
fn test_sixel_no_fallback_message() {
    if !std::path::Path::new(TEST_IMAGE).exists() {
        eprintln!("Skipping: {} not found", TEST_IMAGE);
        return;
    }

    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    let output = cmd
        .arg("image")
        .arg(TEST_IMAGE)
        .arg("--protocol")
        .arg("sixel")
        .output()
        .expect("Failed to execute command");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should NOT contain fallback message
    assert!(
        !stderr.contains("fallback"),
        "Sixel should not fall back. Stderr: {}",
        stderr
    );
    assert!(
        !stderr.contains("not fully implemented"),
        "Sixel should be implemented. Stderr: {}",
        stderr
    );

    // Should produce output
    assert!(
        !stdout.is_empty(),
        "Sixel should produce output. Stderr: {}",
        stderr
    );

    // Should contain Sixel DCS sequence
    assert!(
        stdout.contains("\x1bP"),
        "Output should contain Sixel DCS (\\x1bP). Got {} bytes",
        stdout.len()
    );
}

/// Test that sixel output has proper DCS sequence
#[test]
fn test_sixel_dcs_sequence() {
    if !std::path::Path::new(TEST_IMAGE).exists() {
        return;
    }

    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    let output = cmd
        .arg("image")
        .arg(TEST_IMAGE)
        .arg("--protocol")
        .arg("sixel")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check for Sixel DCS start: ESC P 0;0;0 q
    assert!(
        stdout.contains("\x1bP0;0;0q"),
        "Should start with \\x1bP0;0;0q"
    );

    // Check for Sixel end: ESC \
    assert!(stdout.contains("\x1b\\"), "Should end with \\x1b\\\\");
}

/// Test that sixel contains color palette
#[test]
fn test_sixel_has_palette() {
    if !std::path::Path::new(TEST_IMAGE).exists() {
        return;
    }

    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    let output = cmd
        .arg("image")
        .arg(TEST_IMAGE)
        .arg("--protocol")
        .arg("sixel")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Sixel palette: #N;2;R;G;B
    assert!(
        stdout.contains("#") && stdout.contains(";2;"),
        "Should have palette (#N;2;R;G;B)"
    );
}
