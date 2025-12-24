use assert_cmd::Command;
use predicates::prelude::*;

fn termgfx() -> Command {
    Command::cargo_bin("termgfx").unwrap()
}

// ============================================================================
// BOX COMMAND TESTS
// ============================================================================

#[test]
fn test_box_basic() {
    termgfx()
        .args(["box", "Hello World"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello World"));
}

#[test]
fn test_box_info_style() {
    termgfx()
        .args(["box", "Info message", "--style", "info"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Info message"));
}

#[test]
fn test_box_success_style() {
    termgfx()
        .args(["box", "Success!", "--style", "success"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Success!"));
}

#[test]
fn test_box_warning_style() {
    termgfx()
        .args(["box", "Warning!", "--style", "warning"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Warning!"));
}

#[test]
fn test_box_danger_style() {
    termgfx()
        .args(["box", "Danger!", "--style", "danger"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Danger!"));
}

#[test]
fn test_box_with_emoji() {
    termgfx()
        .args(["box", "With emoji", "--emoji", "🎉"])
        .assert()
        .success()
        .stdout(predicate::str::contains("With emoji"));
}

#[test]
fn test_box_border_single() {
    termgfx()
        .args(["box", "Single border", "--border", "single"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Single border"));
}

#[test]
fn test_box_border_double() {
    termgfx()
        .args(["box", "Double border", "--border", "double"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Double border"));
}

#[test]
fn test_box_border_rounded() {
    termgfx()
        .args(["box", "Rounded border", "--border", "rounded"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Rounded border"));
}

// ============================================================================
// BANNER COMMAND TESTS
// ============================================================================

#[test]
fn test_banner_basic() {
    termgfx()
        .args(["banner", "HI"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_banner_with_gradient() {
    termgfx()
        .args(["banner", "TEST", "--gradient", "cyan-purple"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_banner_single_char() {
    termgfx()
        .args(["banner", "A"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

// ============================================================================
// PROGRESS COMMAND TESTS
// ============================================================================

#[test]
fn test_progress_basic() {
    termgfx()
        .args(["progress", "50"])
        .assert()
        .success()
        .stdout(predicate::str::contains("50%"));
}

#[test]
fn test_progress_zero() {
    termgfx()
        .args(["progress", "0"])
        .assert()
        .success()
        .stdout(predicate::str::contains("0%"));
}

#[test]
fn test_progress_hundred() {
    termgfx()
        .args(["progress", "100"])
        .assert()
        .success()
        .stdout(predicate::str::contains("100%"));
}

#[test]
fn test_progress_style_blocks() {
    termgfx()
        .args(["progress", "75", "--style", "blocks"])
        .assert()
        .success()
        .stdout(predicate::str::contains("75%"));
}

#[test]
fn test_progress_style_gradient() {
    termgfx()
        .args(["progress", "75", "--style", "gradient"])
        .assert()
        .success()
        .stdout(predicate::str::contains("75%"));
}

#[test]
fn test_progress_style_modern() {
    termgfx()
        .args(["progress", "75", "--style", "modern"])
        .assert()
        .success()
        .stdout(predicate::str::contains("75%"));
}

#[test]
fn test_progress_style_classic() {
    termgfx()
        .args(["progress", "75", "--style", "classic"])
        .assert()
        .success()
        .stdout(predicate::str::contains("75%"));
}

#[test]
fn test_progress_style_thin() {
    termgfx()
        .args(["progress", "75", "--style", "thin"])
        .assert()
        .success()
        .stdout(predicate::str::contains("75%"));
}

#[test]
fn test_progress_custom_colors_named() {
    termgfx()
        .args(["progress", "50", "--from", "red", "--to", "green"])
        .assert()
        .success()
        .stdout(predicate::str::contains("50%"));
}

#[test]
fn test_progress_custom_colors_hex() {
    termgfx()
        .args(["progress", "50", "--from", "#ff0000", "--to", "#00ff00"])
        .assert()
        .success()
        .stdout(predicate::str::contains("50%"));
}

#[test]
fn test_progress_animate_non_tty() {
    // In non-TTY mode, --animate should just show final result
    termgfx()
        .args(["progress", "75", "--animate", "--duration", "100"])
        .assert()
        .success()
        .stdout(predicate::str::contains("75%"));
}

#[test]
fn test_progress_clamps_to_100() {
    termgfx()
        .args(["progress", "150"])
        .assert()
        .success()
        .stdout(predicate::str::contains("100%"));
}

// ============================================================================
// SPINNER COMMAND TESTS
// ============================================================================

#[test]
fn test_spinner_non_tty() {
    // In non-TTY mode, spinner prints static message and exits
    termgfx()
        .args(["spinner", "Loading..."])
        .assert()
        .success()
        .stdout(predicate::str::contains("Loading..."));
}

#[test]
fn test_spinner_dots_style() {
    termgfx()
        .args(["spinner", "Working", "--style", "dots"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Working"));
}

#[test]
fn test_spinner_line_style() {
    termgfx()
        .args(["spinner", "Processing", "--style", "line"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Processing"));
}

#[test]
fn test_spinner_moon_style() {
    termgfx()
        .args(["spinner", "Please wait", "--style", "moon"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Please wait"));
}

// ============================================================================
// TYPEWRITER COMMAND TESTS
// ============================================================================

#[test]
fn test_typewriter_basic() {
    // In non-TTY mode, prints full message immediately
    termgfx()
        .args(["typewriter", "Hello World"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello World"));
}

#[test]
fn test_typewriter_with_speed() {
    termgfx()
        .args(["typewriter", "Fast text", "--speed", "10"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Fast text"));
}

#[test]
fn test_typewriter_empty_message() {
    termgfx()
        .args(["typewriter", ""])
        .assert()
        .success();
}

// ============================================================================
// SPARKLINE COMMAND TESTS
// ============================================================================

#[test]
fn test_sparkline_basic() {
    termgfx()
        .args(["sparkline", "1,2,3,4,5"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_sparkline_varying_data() {
    termgfx()
        .args(["sparkline", "10,5,20,15,30,25"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_sparkline_single_value() {
    termgfx()
        .args(["sparkline", "50"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_sparkline_many_values() {
    termgfx()
        .args(["sparkline", "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}
