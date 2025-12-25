use std::fs;
use std::io::{self, IsTerminal};
use std::thread;
use std::time::Duration;

/// Render a 2D heatmap visualization
pub fn render(
    data: Option<&str>,
    file: Option<&str>,
    x_labels: Option<&str>,
    y_labels: Option<&str>,
    title: Option<&str>,
    colors: &str,
    animate: bool,
) {
    // Parse data from either inline or file
    let grid = match (data, file) {
        (Some(d), _) => parse_data(d),
        (None, Some(f)) => parse_file(f),
        (None, None) => {
            eprintln!("Error: Either --data or --file is required");
            std::process::exit(1);
        }
    };

    if grid.is_empty() {
        eprintln!("Error: Empty data provided");
        std::process::exit(1);
    }

    let x_labels_vec = x_labels.map(|s| s.split(',').map(String::from).collect::<Vec<_>>());
    let y_labels_vec = y_labels.map(|s| s.split(',').map(String::from).collect::<Vec<_>>());

    if animate && io::stdout().is_terminal() {
        render_animated(&grid, &x_labels_vec, &y_labels_vec, title, colors);
    } else {
        render_static(&grid, &x_labels_vec, &y_labels_vec, title, colors);
    }
}

fn parse_data(data: &str) -> Vec<Vec<f64>> {
    if data.trim().is_empty() {
        return Vec::new();
    }

    data.split(';')
        .map(|row| {
            row.split(',')
                .filter_map(|cell| cell.trim().parse::<f64>().ok())
                .collect()
        })
        .filter(|row: &Vec<f64>| !row.is_empty())
        .collect()
}

fn parse_file(path: &str) -> Vec<Vec<f64>> {
    match fs::read_to_string(path) {
        Ok(content) => {
            content
                .lines()
                .map(|line| {
                    line.split(',')
                        .filter_map(|cell| cell.trim().parse::<f64>().ok())
                        .collect()
                })
                .filter(|row: &Vec<f64>| !row.is_empty())
                .collect()
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    }
}

fn render_static(
    grid: &[Vec<f64>],
    x_labels: &Option<Vec<String>>,
    y_labels: &Option<Vec<String>>,
    title: Option<&str>,
    colors: &str,
) {
    // Print title if provided
    if let Some(t) = title {
        println!("\n  {}\n", t);
    }

    // Find min and max values for normalization
    let (min_val, max_val) = find_min_max(grid);
    let range = max_val - min_val;

    // Determine max width for all rows
    let max_cols = grid.iter().map(|row| row.len()).max().unwrap_or(0);

    // Print x-axis labels
    if let Some(labels) = x_labels {
        print!("      "); // Offset for y-labels
        for (i, label) in labels.iter().take(max_cols).enumerate() {
            if i > 0 {
                print!(" ");
            }
            print!("{:^4}", label);
        }
        println!();
    }

    // Print grid with y-labels
    for (row_idx, row) in grid.iter().enumerate() {
        // Print y-label if available
        if let Some(labels) = y_labels {
            if row_idx < labels.len() {
                print!("{:>5} ", labels[row_idx]);
            } else {
                print!("      ");
            }
        } else {
            print!("      ");
        }

        // Print cells
        for (col_idx, &value) in row.iter().enumerate() {
            if col_idx > 0 {
                print!(" ");
            }
            let normalized = if range > 0.0 {
                (value - min_val) / range
            } else {
                0.5
            };
            print!("{}", colorize_cell(normalized, colors));
        }
        println!();
    }
    println!();
}

fn render_animated(
    grid: &[Vec<f64>],
    x_labels: &Option<Vec<String>>,
    y_labels: &Option<Vec<String>>,
    title: Option<&str>,
    colors: &str,
) {
    let (min_val, max_val) = find_min_max(grid);
    let range = max_val - min_val;
    let max_cols = grid.iter().map(|row| row.len()).max().unwrap_or(0);

    // Animate row by row
    if let Some(t) = title {
        println!("\n  {}\n", t);
    }

    // Print x-axis labels
    if let Some(labels) = x_labels {
        print!("      ");
        for (i, label) in labels.iter().take(max_cols).enumerate() {
            if i > 0 {
                print!(" ");
            }
            print!("{:^4}", label);
        }
        println!();
    }

    for (row_idx, row) in grid.iter().enumerate() {
        // Print y-label
        if let Some(labels) = y_labels {
            if row_idx < labels.len() {
                print!("{:>5} ", labels[row_idx]);
            } else {
                print!("      ");
            }
        } else {
            print!("      ");
        }

        // Animate cells in this row
        for (col_idx, &value) in row.iter().enumerate() {
            if col_idx > 0 {
                print!(" ");
            }
            let normalized = if range > 0.0 {
                (value - min_val) / range
            } else {
                0.5
            };
            print!("{}", colorize_cell(normalized, colors));
            let _ = io::Write::flush(&mut io::stdout());
            thread::sleep(Duration::from_millis(50));
        }
        println!();
        thread::sleep(Duration::from_millis(100));
    }
    println!();
}

fn find_min_max(grid: &[Vec<f64>]) -> (f64, f64) {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;

    for row in grid {
        for &val in row {
            if val < min {
                min = val;
            }
            if val > max {
                max = val;
            }
        }
    }

    (min, max)
}

fn colorize_cell(normalized: f64, scheme: &str) -> String {
    // Use block characters with different densities: ░▒▓█
    let blocks = ['░', '▒', '▓', '█'];
    let idx = ((normalized * (blocks.len() - 1) as f64).round() as usize).min(blocks.len() - 1);
    let block = blocks[idx];

    // Apply color scheme
    let colored = match scheme {
        "blue-red" => {
            if normalized < 0.33 {
                format!("\x1b[34m{}\x1b[0m", block) // Blue
            } else if normalized < 0.67 {
                format!("\x1b[33m{}\x1b[0m", block) // Yellow
            } else {
                format!("\x1b[31m{}\x1b[0m", block) // Red
            }
        }
        "green-red" => {
            if normalized < 0.5 {
                format!("\x1b[32m{}\x1b[0m", block) // Green
            } else {
                format!("\x1b[31m{}\x1b[0m", block) // Red
            }
        }
        "viridis" => {
            // Approximation of viridis: purple -> blue -> green -> yellow
            if normalized < 0.25 {
                format!("\x1b[35m{}\x1b[0m", block) // Magenta
            } else if normalized < 0.5 {
                format!("\x1b[34m{}\x1b[0m", block) // Blue
            } else if normalized < 0.75 {
                format!("\x1b[32m{}\x1b[0m", block) // Green
            } else {
                format!("\x1b[33m{}\x1b[0m", block) // Yellow
            }
        }
        "magma" => {
            // Approximation of magma: black -> purple -> red -> yellow
            if normalized < 0.33 {
                format!("\x1b[35m{}\x1b[0m", block) // Magenta
            } else if normalized < 0.67 {
                format!("\x1b[31m{}\x1b[0m", block) // Red
            } else {
                format!("\x1b[33m{}\x1b[0m", block) // Yellow
            }
        }
        _ => format!("{}", block), // No color
    };

    // Make it 4 characters wide for alignment
    format!("{:^4}", colored)
}
