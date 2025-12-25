use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;
use owo_colors::OwoColorize;

const BLOCKS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

pub fn render(data: &str) {
    render_animated(data, false);
}

/// Render sparkline with optional animation
pub fn render_animated(data: &str, animate: bool) {
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
    let delay = if animate { Duration::from_millis(100) } else { Duration::ZERO };

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
