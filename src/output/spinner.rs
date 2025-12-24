use crossterm::{
    cursor::{Hide, Show, MoveToColumn},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::{stdout, IsTerminal, Write};
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Get spinner frames for different styles
fn get_spinner_frames(style: &str) -> Vec<&'static str> {
    match style {
        "dots" => vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
        "line" => vec!["|", "/", "-", "\\"],
        "arc" => vec!["◜", "◠", "◝", "◞", "◡", "◟"],
        "bouncing" => vec!["⠁", "⠂", "⠄", "⠂"],
        "clock" => vec!["🕐", "🕑", "🕒", "🕓", "🕔", "🕕", "🕖", "🕗", "🕘", "🕙", "🕚", "🕛"],
        "circle" => vec!["◐", "◓", "◑", "◒"],
        "bounce" => vec!["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"],
        "moon" => vec!["🌑", "🌒", "🌓", "🌔", "🌕", "🌖", "🌗", "🌘"],
        _ => vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"], // Default to dots
    }
}

/// Render an animated loading spinner
pub fn render(message: &str, style: &str) {
    let frames = get_spinner_frames(style);

    // If not a TTY (piped/captured), just print static message and return
    if !stdout().is_terminal() {
        println!("{} {}", frames[0], message);
        return;
    }

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Set up Ctrl+C handler
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let mut stdout = stdout();

    // Hide cursor
    stdout.execute(Hide).unwrap();

    let mut frame_idx = 0;

    while running.load(Ordering::SeqCst) {
        // Move to beginning of line and clear it
        stdout.execute(MoveToColumn(0)).unwrap();
        stdout.execute(Clear(ClearType::CurrentLine)).unwrap();

        // Print spinner frame and message
        print!("{} {}", frames[frame_idx], message);
        stdout.flush().unwrap();

        // Next frame
        frame_idx = (frame_idx + 1) % frames.len();

        // Sleep between frames (80ms for smooth animation)
        thread::sleep(Duration::from_millis(80));
    }

    // Clean up: clear line, show cursor
    stdout.execute(MoveToColumn(0)).unwrap();
    stdout.execute(Clear(ClearType::CurrentLine)).unwrap();
    stdout.execute(Show).unwrap();
    stdout.flush().unwrap();
}
