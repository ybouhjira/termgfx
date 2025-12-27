use crate::animation::engine::Animator;
use owo_colors::OwoColorize;

/// Animate a progress bar from 0 to 100%
pub fn progress(duration_secs: f64, style: &str) {
    let style = style.to_string();
    let animator = Animator::default();

    animator.run_timed(duration_secs, move |_frame, progress| {
        let percent = (progress * 100.0) as u8;
        render_progress_inline(percent, &style)
    });
}

fn render_progress_inline(percent: u8, style: &str) -> String {
    let width = 30;
    let filled = (width * percent as usize) / 100;
    let empty = width - filled;

    match style {
        "blocks" => {
            let bar: String = "█".repeat(filled) + &"░".repeat(empty);
            format!(
                "{} {}%",
                bar.cyan(),
                percent.to_string().bright_cyan().bold()
            )
        }
        "gradient" => {
            let mut bar = String::new();
            for i in 0..filled {
                let p = (i as f32 / width as f32) * 100.0;
                let c = if p < 33.0 {
                    '█'.red().to_string()
                } else if p < 66.0 {
                    '█'.yellow().to_string()
                } else {
                    '█'.green().to_string()
                };
                bar.push_str(&c);
            }
            bar.push_str(&"░".bright_black().to_string().repeat(empty));
            let pct = if percent < 33 {
                percent.to_string().red().bold().to_string()
            } else if percent < 66 {
                percent.to_string().yellow().bold().to_string()
            } else {
                percent.to_string().green().bold().to_string()
            };
            format!("{} {}%", bar, pct)
        }
        "thin" => {
            let bar = "━".cyan().to_string().repeat(filled)
                + &"━".bright_black().to_string().repeat(empty);
            format!("{} {}%", bar, percent.to_string().bright_cyan().bold())
        }
        _ => {
            let bar: String = "█".repeat(filled) + &"░".repeat(empty);
            format!(
                "{} {}%",
                bar.cyan(),
                percent.to_string().bright_cyan().bold()
            )
        }
    }
}

/// Typewriter effect - reveal text character by character
pub fn typewriter(text: &str, chars_per_sec: f64) {
    use crossterm::{cursor::Hide, cursor::Show, ExecutableCommand};
    use std::io::{stdout, Write};
    use std::thread;
    use std::time::Duration;

    let mut stdout = stdout();
    stdout.execute(Hide).unwrap();

    let delay = Duration::from_secs_f64(1.0 / chars_per_sec);

    for ch in text.chars() {
        print!("{}", ch);
        stdout.flush().unwrap();
        thread::sleep(delay);
    }
    println!();

    stdout.execute(Show).unwrap();
}

/// Animate a counter from start to end value
pub fn counter(from: i64, to: i64, duration_secs: f64, prefix: &str, suffix: &str) {
    let prefix = prefix.to_string();
    let suffix = suffix.to_string();
    let animator = Animator::default();

    animator.run_timed(duration_secs, move |_frame, progress| {
        let range = (to - from) as f64;
        let value = from + (range * progress) as i64;
        format!(
            "{}{}{}",
            prefix.bright_black(),
            value.to_string().cyan().bold(),
            suffix.bright_black()
        )
    });
}

/// Animate chart data appearing progressively
pub fn chart_build(data: &str, duration_secs: f64) {
    use crossterm::{
        cursor::{Hide, MoveToColumn, Show},
        terminal::{Clear, ClearType},
        ExecutableCommand,
    };
    use std::io::{stdout, Write};
    use std::thread;
    use std::time::{Duration, Instant};

    let points: Vec<&str> = data.split(',').collect();
    if points.is_empty() {
        return;
    }

    let mut stdout = stdout();
    stdout.execute(Hide).unwrap();

    let start = Instant::now();
    let duration = Duration::from_secs_f64(duration_secs);
    let delay = Duration::from_millis(50);

    loop {
        let elapsed = start.elapsed();
        if elapsed >= duration {
            break;
        }

        let progress = elapsed.as_secs_f64() / duration_secs;
        let show_count = ((points.len() as f64 * progress) as usize)
            .max(1)
            .min(points.len());

        let partial_data: String = points[..show_count].join(",");

        stdout.execute(MoveToColumn(0)).unwrap();
        stdout.execute(Clear(ClearType::CurrentLine)).unwrap();

        // Render sparkline inline
        let sparkline = render_sparkline_inline(&partial_data);
        print!("{}", sparkline);
        stdout.flush().unwrap();

        thread::sleep(delay);
    }

    // Final render
    stdout.execute(MoveToColumn(0)).unwrap();
    stdout.execute(Clear(ClearType::CurrentLine)).unwrap();
    println!("{}", render_sparkline_inline(data));

    stdout.execute(Show).unwrap();
}

fn render_sparkline_inline(data: &str) -> String {
    let chars = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    let values: Vec<f64> = data
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    if values.is_empty() {
        return String::new();
    }

    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = if (max - min).abs() < f64::EPSILON {
        1.0
    } else {
        max - min
    };

    values
        .iter()
        .map(|&v| {
            let normalized = (v - min) / range;
            let idx = (normalized * 7.0).round() as usize;
            chars[idx.min(7)].to_string().cyan().to_string()
        })
        .collect()
}

/// Animate bar chart bars growing
pub fn bars_build(data: &str, duration_secs: f64) {
    use crossterm::{
        cursor::{Hide, MoveTo, Show},
        terminal::{Clear, ClearType},
        ExecutableCommand,
    };
    use std::io::{stdout, Write};
    use std::thread;
    use std::time::{Duration, Instant};

    // Parse data: "Label:Value,Label:Value"
    let items: Vec<(&str, f64)> = data
        .split(',')
        .filter_map(|s| {
            let parts: Vec<&str> = s.trim().split(':').collect();
            if parts.len() == 2 {
                parts[1].parse().ok().map(|v| (parts[0], v))
            } else {
                None
            }
        })
        .collect();

    if items.is_empty() {
        return;
    }

    let max_val = items.iter().map(|(_, v)| *v).fold(0.0_f64, f64::max);
    let bar_width = 20;

    let mut stdout = stdout();
    stdout.execute(Hide).unwrap();

    // Print initial empty lines
    for _ in 0..items.len() {
        println!();
    }

    let start = Instant::now();
    let duration = Duration::from_secs_f64(duration_secs);
    let delay = Duration::from_millis(50);

    loop {
        let elapsed = start.elapsed();
        let progress = (elapsed.as_secs_f64() / duration_secs).min(1.0);

        // Move cursor up to redraw
        for (i, (label, value)) in items.iter().enumerate() {
            let row = items.len() - 1 - i;
            stdout.execute(MoveTo(0, row as u16)).unwrap();
            stdout.execute(Clear(ClearType::CurrentLine)).unwrap();

            let current_val = value * progress;
            let filled = ((current_val / max_val) * bar_width as f64) as usize;
            let bar = "█".repeat(filled);

            print!(
                "{:>8} {} {:.0}",
                label.bright_black(),
                bar.cyan(),
                current_val
            );
        }
        stdout.flush().unwrap();

        if elapsed >= duration {
            break;
        }
        thread::sleep(delay);
    }

    // Move below the chart
    stdout.execute(MoveTo(0, items.len() as u16)).unwrap();
    println!();
    stdout.execute(Show).unwrap();
}

/// Run an animation effect by name
#[allow(clippy::too_many_arguments)]
pub fn run(
    effect_type: &str,
    text: Option<&str>,
    data: Option<&str>,
    duration: f64,
    speed: f64,
    from: i64,
    to: i64,
    style: &str,
    prefix: Option<&str>,
    suffix: Option<&str>,
) {
    match effect_type {
        "progress" => progress(duration, style),
        "typewriter" => {
            if let Some(t) = text {
                typewriter(t, speed);
            } else {
                eprintln!("Error: --text required for typewriter effect");
            }
        }
        "counter" => counter(from, to, duration, prefix.unwrap_or(""), suffix.unwrap_or("")),
        "chart-build" | "sparkline" => {
            if let Some(d) = data {
                chart_build(d, duration);
            } else {
                eprintln!("Error: --data required for chart-build effect");
            }
        }
        "bars" | "bar-build" => {
            if let Some(d) = data {
                bars_build(d, duration);
            } else {
                eprintln!("Error: --data required for bars effect");
            }
        }
        _ => eprintln!("Unknown animation type: {}. Available: progress, typewriter, counter, chart-build, bars", effect_type),
    }
}
