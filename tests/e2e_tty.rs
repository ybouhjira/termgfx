use rexpect::spawn;
use std::time::Duration;

const TIMEOUT: u64 = 10000; // 10 seconds

fn termgfx_bin() -> String {
    env!("CARGO_BIN_EXE_termgfx").to_string()
}

fn wait() {
    std::thread::sleep(Duration::from_millis(100));
}

// ============================================================================
// SPINNER TTY TESTS (with duration for auto-stop)
// These work because spinner auto-terminates with --duration
// ============================================================================

#[test]
fn test_spinner_duration_tty() {
    let mut p = spawn(&format!("{} spinner 'Loading...' --duration 1", termgfx_bin()), Some(5000)).unwrap();
    p.exp_string("Loading").unwrap();
    p.exp_eof().unwrap();
}

#[test]
fn test_spinner_dots_style_tty() {
    let mut p = spawn(&format!("{} spinner 'Processing' --style dots --duration 1", termgfx_bin()), Some(5000)).unwrap();
    p.exp_string("Processing").unwrap();
    p.exp_eof().unwrap();
}

#[test]
fn test_spinner_moon_style_tty() {
    let mut p = spawn(&format!("{} spinner 'Working' --style moon --duration 1", termgfx_bin()), Some(5000)).unwrap();
    p.exp_string("Working").unwrap();
    p.exp_eof().unwrap();
}

#[test]
fn test_spinner_line_style_tty() {
    let mut p = spawn(&format!("{} spinner 'Loading' --style line --duration 1", termgfx_bin()), Some(5000)).unwrap();
    p.exp_string("Loading").unwrap();
    p.exp_eof().unwrap();
}

// ============================================================================
// TYPEWRITER TTY TESTS
// These work because typewriter auto-completes
// ============================================================================

#[test]
fn test_typewriter_message_tty() {
    let mut p = spawn(&format!("{} typewriter 'Hello World!' --speed 5", termgfx_bin()), Some(TIMEOUT)).unwrap();
    p.exp_string("Hello World!").unwrap();
    p.exp_eof().unwrap();
}

#[test]
fn test_typewriter_fast_tty() {
    let mut p = spawn(&format!("{} typewriter 'Quick test' --speed 1", termgfx_bin()), Some(TIMEOUT)).unwrap();
    p.exp_string("Quick test").unwrap();
    p.exp_eof().unwrap();
}

// ============================================================================
// PROGRESS ANIMATION TTY TESTS
// These work because progress --animate auto-completes
// ============================================================================

#[test]
fn test_progress_animate_tty() {
    let mut p = spawn(&format!("{} progress 100 --animate --duration 200", termgfx_bin()), Some(TIMEOUT)).unwrap();
    p.exp_string("100%").unwrap();
    p.exp_eof().unwrap();
}

#[test]
fn test_progress_animate_50_tty() {
    let mut p = spawn(&format!("{} progress 50 --animate --duration 200", termgfx_bin()), Some(TIMEOUT)).unwrap();
    p.exp_string("50%").unwrap();
    p.exp_eof().unwrap();
}

#[test]
fn test_progress_animate_blocks_style_tty() {
    let mut p = spawn(&format!("{} progress 75 --animate --duration 200 --style blocks", termgfx_bin()), Some(TIMEOUT)).unwrap();
    p.exp_string("75%").unwrap();
    p.exp_eof().unwrap();
}

// ============================================================================
// ANIMATE COMMAND TTY TESTS
// These work because animate auto-completes after duration
// ============================================================================

#[test]
fn test_animate_progress_tty() {
    let mut p = spawn(&format!("{} animate -t progress -D 0.5", termgfx_bin()), Some(TIMEOUT)).unwrap();
    // Should complete and show 100% (with ANSI codes)
    p.exp_regex("100.*%").unwrap();
    p.exp_eof().unwrap();
}

#[test]
fn test_animate_counter_tty() {
    let mut p = spawn(&format!("{} animate -t counter --from 0 --to 10 -D 0.5", termgfx_bin()), Some(TIMEOUT)).unwrap();
    // Should complete and show final value
    p.exp_string("10").unwrap();
    p.exp_eof().unwrap();
}

#[test]
fn test_animate_typewriter_effect_tty() {
    let mut p = spawn(&format!("{} animate -t typewriter --text 'Hello' -D 0.5", termgfx_bin()), Some(TIMEOUT)).unwrap();
    p.exp_string("Hello").unwrap();
    p.exp_eof().unwrap();
}

// ============================================================================
// DEMO COMMAND TTY TESTS
// These work because demo sections auto-complete
// ============================================================================

#[test]
fn test_demo_boxes_tty() {
    let mut p = spawn(&format!("{} demo --section boxes", termgfx_bin()), Some(TIMEOUT)).unwrap();
    // Demo should show styled boxes section
    p.exp_string("STYLED BOXES").unwrap();
    p.exp_eof().unwrap();
}

#[test]
fn test_demo_progress_tty() {
    let mut p = spawn(&format!("{} demo --section progress", termgfx_bin()), Some(TIMEOUT)).unwrap();
    p.exp_string("PROGRESS").unwrap();
    p.exp_eof().unwrap();
}

#[test]
fn test_demo_charts_tty() {
    let mut p = spawn(&format!("{} demo --section charts", termgfx_bin()), Some(TIMEOUT)).unwrap();
    p.exp_string("CHARTS").unwrap();
    p.exp_eof().unwrap();
}

// ============================================================================
// INTERACTIVE COMMAND PROMPT VERIFICATION TESTS
// These verify that prompts are displayed correctly
// Note: Full interaction testing requires crossterm-compatible PTY which
// is not fully supported by rexpect. These tests verify the UI renders.
// ============================================================================

#[test]
fn test_input_shows_prompt_tty() {
    let mut p = spawn(&format!("{} input 'Enter your name:'", termgfx_bin()), Some(3000)).unwrap();
    // Verify prompt is displayed
    p.exp_string("Enter your name:").unwrap();
    // Send Ctrl+C to cancel (crossterm should handle this)
    p.send("\x03").unwrap();
    wait();
}

#[test]
fn test_confirm_shows_prompt_tty() {
    let mut p = spawn(&format!("{} confirm 'Continue?'", termgfx_bin()), Some(3000)).unwrap();
    p.exp_string("Continue?").unwrap();
    // Should show [Y/n] or similar
    p.exp_regex("\\[.*\\]").unwrap();
    p.send("\x03").unwrap();
    wait();
}

#[test]
fn test_select_shows_options_tty() {
    let mut p = spawn(&format!("{} select 'Pick one:' apple banana", termgfx_bin()), Some(3000)).unwrap();
    p.exp_string("Pick one").unwrap();
    // Should show at least one option
    p.exp_string("apple").unwrap();
    // Cancel with Escape
    p.send("\x1b").unwrap();
    wait();
}

#[test]
fn test_choose_shows_options_tty() {
    let mut p = spawn(&format!("{} choose 'Select:' one two three", termgfx_bin()), Some(3000)).unwrap();
    p.exp_string("Select").unwrap();
    p.exp_string("one").unwrap();
    p.send("\x1b").unwrap();
    wait();
}

// ============================================================================
// NEW INTERACTIVE COMMANDS TTY TESTS (v0.4.0+)
// These test playground, tui, wizard, form, file picker, and filter
// ============================================================================

#[test]
fn test_playground_shows_ui_tty() {
    let mut p = spawn(&format!("{} playground", termgfx_bin()), Some(3000)).unwrap();
    // Playground shows title and interactive UI
    p.exp_string("Playground").unwrap();
    // Quit with 'q'
    p.send("q").unwrap();
    wait();
}

#[test]
fn test_tui_renders_widgets_tty() {
    let mut p = spawn(&format!("{} tui --layout 1x1 --widgets 'box:Hello'", termgfx_bin()), Some(3000)).unwrap();
    // TUI should show the box content
    p.exp_string("Hello").unwrap();
    // Quit with 'q'
    p.send("q").unwrap();
    wait();
}

#[test]
fn test_tui_sparkline_widget_tty() {
    let mut p = spawn(&format!("{} tui --layout 1x1 --widgets 'sparkline:1,2,3,4,5'", termgfx_bin()), Some(3000)).unwrap();
    // TUI should render and be quittable
    wait();
    p.send("q").unwrap();
    wait();
}

#[test]
fn test_wizard_input_step_tty() {
    let mut p = spawn(&format!("{} wizard --step 'input:name:Enter your name'", termgfx_bin()), Some(3000)).unwrap();
    // Wizard should show the input prompt
    p.exp_string("Enter your name").unwrap();
    // Cancel with Escape
    p.send("\x1b").unwrap();
    wait();
}

#[test]
fn test_wizard_select_step_tty() {
    let mut p = spawn(&format!("{} wizard --step 'select:color:Pick a color:red,green,blue'", termgfx_bin()), Some(3000)).unwrap();
    // Wizard should show options
    p.exp_string("Pick a color").unwrap();
    p.exp_string("red").unwrap();
    p.send("\x1b").unwrap();
    wait();
}

#[test]
fn test_wizard_confirm_step_tty() {
    let mut p = spawn(&format!("{} wizard --step 'confirm:proceed:Continue?'", termgfx_bin()), Some(3000)).unwrap();
    // Wizard should show confirmation prompt
    p.exp_string("Continue").unwrap();
    p.send("\x1b").unwrap();
    wait();
}

#[test]
fn test_form_shows_fields_tty() {
    let mut p = spawn(&format!("{} form --field 'name:text:Your Name'", termgfx_bin()), Some(3000)).unwrap();
    // Form should show the field label
    p.exp_string("Your Name").unwrap();
    // Cancel with Escape
    p.send("\x1b").unwrap();
    wait();
}

#[test]
fn test_form_multiple_fields_tty() {
    let mut p = spawn(&format!("{} form --field 'name:text:Name' --field 'email:text:Email'", termgfx_bin()), Some(3000)).unwrap();
    p.exp_string("Name").unwrap();
    p.exp_string("Email").unwrap();
    p.send("\x1b").unwrap();
    wait();
}

#[test]
fn test_form_select_field_tty() {
    let mut p = spawn(&format!("{} form --field 'role:select:Role:admin,user,guest'", termgfx_bin()), Some(3000)).unwrap();
    p.exp_string("Role").unwrap();
    p.send("\x1b").unwrap();
    wait();
}

#[test]
fn test_file_picker_shows_ui_tty() {
    let mut p = spawn(&format!("{} file", termgfx_bin()), Some(3000)).unwrap();
    // File picker shows path and filter
    p.exp_string("Path").unwrap();
    p.exp_string("Filter").unwrap();
    // Cancel with Escape
    p.send("\x1b").unwrap();
    wait();
}
