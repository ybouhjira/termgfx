use crossterm::{cursor, terminal, ExecutableCommand};
use std::io::{self, Write};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Parse interval string like "1s", "500ms", "2.5s" into Duration
pub fn parse_interval(interval: &str) -> Result<Duration, String> {
    let interval = interval.trim();

    if interval.ends_with("ms") {
        let ms = interval
            .trim_end_matches("ms")
            .parse::<u64>()
            .map_err(|_| format!("Invalid milliseconds: {}", interval))?;
        Ok(Duration::from_millis(ms))
    } else if interval.ends_with('s') {
        let secs = interval
            .trim_end_matches('s')
            .parse::<f64>()
            .map_err(|_| format!("Invalid seconds: {}", interval))?;
        Ok(Duration::from_secs_f64(secs))
    } else {
        // Default to seconds
        let secs = interval
            .parse::<f64>()
            .map_err(|_| format!("Invalid interval: {}", interval))?;
        Ok(Duration::from_secs_f64(secs))
    }
}

/// Execute a command and return its stdout output
pub fn exec_command(command: &str) -> Result<String, String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    // Combine stdout and stderr for full output
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !stderr.is_empty() && !output.status.success() {
        return Err(stderr.trim().to_string());
    }

    Ok(stdout.trim().to_string())
}

/// Setup Ctrl+C handler
pub fn setup_ctrl_c() -> Arc<AtomicBool> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");

    running
}

/// Clear screen and move cursor to top
pub fn clear_screen() {
    let mut stdout = io::stdout();
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .ok();
    stdout.execute(cursor::MoveTo(0, 0)).ok();
    stdout.flush().ok();
}

/// Clear the current line and move cursor to beginning
#[allow(dead_code)]
pub fn clear_line() {
    let mut stdout = io::stdout();
    stdout
        .execute(terminal::Clear(terminal::ClearType::CurrentLine))
        .ok();
    stdout.execute(cursor::MoveToColumn(0)).ok();
    stdout.flush().ok();
}

/// Move cursor to beginning of line
#[allow(dead_code)]
pub fn move_to_line_start() {
    let mut stdout = io::stdout();
    stdout.execute(cursor::MoveToColumn(0)).ok();
    stdout.flush().ok();
}

/// Format a duration as human readable
fn format_duration(d: Duration) -> String {
    let secs = d.as_secs();
    if secs < 60 {
        format!("{}s", secs)
    } else if secs < 3600 {
        format!("{}m {}s", secs / 60, secs % 60)
    } else {
        format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
    }
}

/// Run watch mode - repeatedly execute command and display output
pub fn render(
    command: &str,
    interval: Duration,
    no_title: bool,
    differences: bool,
    exit_on_error: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let running = setup_ctrl_c();
    let start_time = Instant::now();
    let mut last_output: Option<String> = None;
    let mut iteration = 0u64;

    while running.load(Ordering::SeqCst) {
        iteration += 1;
        clear_screen();

        // Print header unless no_title
        if !no_title {
            let elapsed = format_duration(start_time.elapsed());
            println!(
                "\x1b[7m Every {} | {} | Elapsed: {} | Iteration: {} \x1b[0m",
                format_interval(interval),
                command,
                elapsed,
                iteration
            );
            println!();
        }

        // Execute command
        match exec_command(command) {
            Ok(output) => {
                if differences {
                    // Show differences from last output
                    if let Some(ref prev) = last_output {
                        print_with_differences(prev, &output);
                    } else {
                        println!("{}", output);
                    }
                    last_output = Some(output);
                } else {
                    println!("{}", output);
                }
            }
            Err(e) => {
                println!("\x1b[31mError: {}\x1b[0m", e);
                if exit_on_error {
                    return Err(e.into());
                }
            }
        }

        // Wait for interval, checking for Ctrl+C
        let sleep_step = Duration::from_millis(100);
        let mut slept = Duration::ZERO;
        while slept < interval && running.load(Ordering::SeqCst) {
            std::thread::sleep(sleep_step);
            slept += sleep_step;
        }
    }

    // Clear status line on exit
    println!("\n\x1b[2mWatch stopped.\x1b[0m");
    Ok(())
}

/// Format interval for display
fn format_interval(d: Duration) -> String {
    let ms = d.as_millis();
    if ms < 1000 {
        format!("{}ms", ms)
    } else if ms.is_multiple_of(1000) {
        format!("{}s", ms / 1000)
    } else {
        format!("{:.1}s", d.as_secs_f64())
    }
}

/// Print output with differences highlighted
fn print_with_differences(old: &str, new: &str) {
    let old_lines: Vec<&str> = old.lines().collect();
    let new_lines: Vec<&str> = new.lines().collect();

    for (i, new_line) in new_lines.iter().enumerate() {
        if i >= old_lines.len() {
            // New line added
            println!("\x1b[32m{}\x1b[0m", new_line);
        } else if old_lines[i] != *new_line {
            // Line changed
            println!("\x1b[33m{}\x1b[0m", new_line);
        } else {
            println!("{}", new_line);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_interval_ms() {
        assert_eq!(parse_interval("500ms").unwrap(), Duration::from_millis(500));
        assert_eq!(
            parse_interval("1000ms").unwrap(),
            Duration::from_millis(1000)
        );
    }

    #[test]
    fn test_parse_interval_seconds() {
        assert_eq!(parse_interval("1s").unwrap(), Duration::from_secs(1));
        assert_eq!(
            parse_interval("2.5s").unwrap(),
            Duration::from_secs_f64(2.5)
        );
        assert_eq!(parse_interval("1").unwrap(), Duration::from_secs(1));
    }

    #[test]
    fn test_exec_command() {
        let result = exec_command("echo 42");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "42");
    }

    #[test]
    fn test_exec_command_failure() {
        // Use a command that outputs to stderr and fails
        let result = exec_command("echo 'error message' >&2 && exit 1");
        assert!(result.is_err());
    }

    #[test]
    fn test_format_interval() {
        assert_eq!(format_interval(Duration::from_millis(500)), "500ms");
        assert_eq!(format_interval(Duration::from_secs(1)), "1s");
        assert_eq!(format_interval(Duration::from_secs(2)), "2s");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_secs(30)), "30s");
        assert_eq!(format_duration(Duration::from_secs(90)), "1m 30s");
        assert_eq!(format_duration(Duration::from_secs(3700)), "1h 1m");
    }
}
