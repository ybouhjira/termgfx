use crossterm::{
    cursor,
    style::{Color, ResetColor, SetForegroundColor},
    terminal, ExecutableCommand,
};
use std::io::{self, Write};

pub fn render(
    value: f64,
    min: f64,
    max: f64,
    label: Option<&str>,
    style: &str,
    color: Option<&str>,
    animate: bool,
) {
    let mut stdout = io::stdout();

    // Check if TTY for animations
    let is_tty = terminal::is_raw_mode_enabled().unwrap_or(false);
    let should_animate = animate && is_tty;

    if should_animate {
        // Animate from 0 to value
        let steps = 20;
        let step_delay = std::time::Duration::from_millis(50);

        for i in 0..=steps {
            let current_value = (value * i as f64) / steps as f64;
            stdout.execute(cursor::MoveToColumn(0)).ok();
            render_gauge(&mut stdout, current_value, min, max, label, style, color);
            stdout.flush().ok();
            std::thread::sleep(step_delay);
        }
        println!(); // Final newline
    } else {
        render_gauge(&mut stdout, value, min, max, label, style, color);
        println!();
    }
}

fn render_gauge(
    stdout: &mut io::Stdout,
    value: f64,
    min: f64,
    max: f64,
    label: Option<&str>,
    style: &str,
    color: Option<&str>,
) {
    // Calculate percentage (0-100)
    let percentage = if max > min {
        ((value - min) / (max - min) * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    };

    // Select color based on value or override
    let gauge_color = if let Some(c) = color {
        parse_color(c)
    } else {
        // Auto color based on percentage
        if percentage < 33.0 {
            Color::Red
        } else if percentage < 66.0 {
            Color::Yellow
        } else {
            Color::Green
        }
    };

    match style {
        "full" => render_full_gauge(stdout, percentage, value, gauge_color, label),
        "minimal" => render_minimal_gauge(stdout, percentage, value, gauge_color, label),
        _ => render_semicircle_gauge(stdout, percentage, value, gauge_color, label),
    }
}

fn render_semicircle_gauge(
    stdout: &mut io::Stdout,
    percentage: f64,
    value: f64,
    color: Color,
    label: Option<&str>,
) {
    // Semicircle gauge using Unicode block characters
    let width = 40;
    let filled = (width as f64 * percentage / 100.0) as usize;

    // Draw the arc
    let chars = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

    stdout.execute(SetForegroundColor(Color::DarkGrey)).ok();
    write!(stdout, "╭").ok();
    for _ in 0..width {
        write!(stdout, "─").ok();
    }
    writeln!(stdout, "╮").ok();

    write!(stdout, "│").ok();
    stdout.execute(SetForegroundColor(color)).ok();

    for i in 0..width {
        if i < filled {
            write!(stdout, "{}", chars[7]).ok();
        } else {
            stdout.execute(SetForegroundColor(Color::DarkGrey)).ok();
            write!(stdout, "{}", chars[0]).ok();
            stdout.execute(SetForegroundColor(color)).ok();
        }
    }

    stdout.execute(SetForegroundColor(Color::DarkGrey)).ok();
    writeln!(stdout, "│").ok();
    write!(stdout, "╰").ok();
    for _ in 0..width {
        write!(stdout, "─").ok();
    }
    write!(stdout, "╯").ok();

    stdout.execute(ResetColor).ok();

    // Display label and value
    write!(stdout, " ").ok();
    if let Some(l) = label {
        stdout.execute(SetForegroundColor(Color::Cyan)).ok();
        write!(stdout, "{}: ", l).ok();
    }
    stdout.execute(SetForegroundColor(color)).ok();
    write!(stdout, "{:.1}%", percentage).ok();
    if value != percentage {
        write!(stdout, " ({:.1})", value).ok();
    }
    stdout.execute(ResetColor).ok();
}

fn render_full_gauge(
    stdout: &mut io::Stdout,
    percentage: f64,
    value: f64,
    color: Color,
    label: Option<&str>,
) {
    // Full circle gauge
    let radius = 10;
    let center_x = radius;
    let center_y = radius;

    // Simple circular representation
    for y in 0..=(radius * 2) {
        for x in 0..=(radius * 2) {
            let dx = (x as f64 - center_x as f64).abs();
            let dy = (y as f64 - center_y as f64).abs();
            let distance = (dx * dx + dy * dy).sqrt();

            if (distance - radius as f64).abs() < 1.5 {
                // On the circle edge
                let angle = (dy.atan2(dx) * 180.0 / std::f64::consts::PI) as i32;
                let fill_angle = (percentage * 3.6) as i32; // 360 degrees for 100%

                if angle <= fill_angle {
                    stdout.execute(SetForegroundColor(color)).ok();
                    write!(stdout, "●").ok();
                } else {
                    stdout.execute(SetForegroundColor(Color::DarkGrey)).ok();
                    write!(stdout, "○").ok();
                }
                stdout.execute(ResetColor).ok();
            } else if distance < radius as f64 - 2.0 && x == center_x && y == center_y {
                // Center value
                write!(stdout, "{:.0}%", percentage).ok();
            } else {
                write!(stdout, " ").ok();
            }
        }
        writeln!(stdout).ok();
    }

    // Label below
    if let Some(l) = label {
        stdout.execute(SetForegroundColor(Color::Cyan)).ok();
        write!(stdout, "{}: ", l).ok();
    }
    stdout.execute(SetForegroundColor(color)).ok();
    write!(stdout, "{:.1}", value).ok();
    stdout.execute(ResetColor).ok();
}

fn render_minimal_gauge(
    stdout: &mut io::Stdout,
    percentage: f64,
    value: f64,
    color: Color,
    label: Option<&str>,
) {
    // Compact single-line gauge
    let width = 20;
    let filled = (width as f64 * percentage / 100.0) as usize;

    if let Some(l) = label {
        stdout.execute(SetForegroundColor(Color::Cyan)).ok();
        write!(stdout, "{}: ", l).ok();
        stdout.execute(ResetColor).ok();
    }

    write!(stdout, "[").ok();
    stdout.execute(SetForegroundColor(color)).ok();

    for i in 0..width {
        if i < filled {
            write!(stdout, "█").ok();
        } else {
            stdout.execute(SetForegroundColor(Color::DarkGrey)).ok();
            write!(stdout, "░").ok();
            stdout.execute(SetForegroundColor(color)).ok();
        }
    }

    stdout.execute(ResetColor).ok();
    write!(stdout, "] ").ok();

    stdout.execute(SetForegroundColor(color)).ok();
    write!(stdout, "{:.1}%", percentage).ok();
    if value != percentage {
        write!(stdout, " ({:.1})", value).ok();
    }
    stdout.execute(ResetColor).ok();
}

fn parse_color(color_name: &str) -> Color {
    match color_name.to_lowercase().as_str() {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        "yellow" => Color::Yellow,
        "cyan" => Color::Cyan,
        "magenta" => Color::Magenta,
        "white" => Color::White,
        "grey" | "gray" => Color::Grey,
        _ => Color::Green,
    }
}
