use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;
use owo_colors::OwoColorize;

const BLOCKS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

pub fn render(data: &str) {
    render_animated(data, false, 500);
}

/// Render sparkline with optional animation
/// animation_time_ms: total animation duration in milliseconds (delay is calculated per value)
pub fn render_animated(data: &str, animate: bool, animation_time_ms: u64) {
    let values: Vec<f64> = data
        .split(',')
        .filter_map(|s| s.trim().parse::<f64>().ok())
        .collect();

    if values.is_empty() {
        eprintln!("Error: No valid numeric values found");
        return;
    }

    if values.len() == 1 {
        print!("{}", BLOCKS[BLOCKS.len() - 1].cyan());
        println!();
        return;
    }

    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = max - min;

    let mut stdout = stdout();
    // Calculate delay per value: total_time / number_of_values
    let delay = if animate && !values.is_empty() {
        Duration::from_millis(animation_time_ms / values.len() as u64)
    } else {
        Duration::ZERO
    };

    for value in values {
        let normalized = if range == 0.0 {
            0.5
        } else {
            (value - min) / range
        };

        let index = (normalized * (BLOCKS.len() - 1) as f64).round() as usize;
        let index = index.min(BLOCKS.len() - 1);
        print!("{}", BLOCKS[index].cyan());

        if animate {
            stdout.flush().unwrap();
            thread::sleep(delay);
        }
    }

    println!();
}
