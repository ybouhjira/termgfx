use crossterm::{
    cursor::{Hide, MoveTo, Show},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use owo_colors::OwoColorize;
use std::f64::consts::PI;
use std::io::{stdout, IsTerminal, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const COLORS: [u8; 8] = [
    196, // Red
    208, // Orange
    226, // Yellow
    46,  // Green
    51,  // Cyan
    21,  // Blue
    129, // Purple
    201, // Magenta
];

const BLOCKS: [&str; 8] = ["█", "▓", "▒", "░", "▄", "▀", "▐", "▌"];

pub struct PieChart<'a> {
    data: &'a str,
    animate: bool,
    animation_time_ms: u64,
}

impl<'a> PieChart<'a> {
    pub fn new(data: &'a str, animate: bool, animation_time_ms: u64) -> Self {
        Self {
            data,
            animate,
            animation_time_ms,
        }
    }

    pub fn render(&self) {
        if self.animate {
            self._render_animated();
        } else {
            self._render_static();
        }
    }

    fn _render_static(&self) {
        let entries = parse_data(self.data);

        if entries.is_empty() {
            eprintln!("Error: No valid data provided");
            return;
        }

        // Calculate total and percentages
        let total: f64 = entries.iter().map(|(_, v)| v).sum();
        if total <= 0.0 {
            eprintln!("Error: Total value must be positive");
            return;
        }

        let segments: Vec<(String, f64, f64)> = entries
            .iter()
            .map(|(label, value)| {
                let percentage = (value / total) * 100.0;
                (label.clone(), *value, percentage)
            })
            .collect();

        // Render the pie chart
        render_circle(&segments);

        // Render legend
        println!();
        for (idx, (label, _, percentage)) in segments.iter().enumerate() {
            let block = BLOCKS[idx % BLOCKS.len()];
            let color = COLORS[idx % COLORS.len()];
            println!(
                "  {} {}: {:.1}%",
                block.repeat(2).color(owo_colors::XtermColors::from(color)),
                label,
                percentage
            );
        }
    }

    fn _render_animated(&self) {
        let entries = parse_data(self.data);

        if entries.is_empty() {
            eprintln!("Error: No valid data provided");
            return;
        }

        if !stdout().is_terminal() {
            self._render_static();
            return;
        }

        // Calculate total and percentages
        let total: f64 = entries.iter().map(|(_, v)| v).sum();
        if total <= 0.0 {
            eprintln!("Error: Total value must be positive");
            return;
        }

        let full_segments: Vec<(String, f64, f64)> = entries
            .iter()
            .map(|(label, value)| {
                let percentage = (value / total) * 100.0;
                (label.clone(), *value, percentage)
            })
            .collect();

        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        let _ = ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        });

        let mut stdout = stdout();
        let _ = stdout.execute(Hide); // Hide cursor

        let total_segments = full_segments.len();
        let delay_per_segment = if total_segments > 0 {
            Duration::from_millis(self.animation_time_ms / total_segments as u64)
        } else {
            Duration::from_millis(0)
        };

        let mut rendered_segments: Vec<(String, f64, f64)> = Vec::new();

        // Determine height of the pie chart output + legend
        // Circle height is fixed at 18 lines (0..18)
        // Legend has 1 empty line + 1 line per segment
        let total_output_lines = 18 + 1 + total_segments;

        for segment in full_segments.into_iter() {
            if !running.load(Ordering::SeqCst) {
                break;
            }

            rendered_segments.push(segment);

            // Clear previous content
            let _ = stdout.execute(MoveTo(0, 0)); // Move to top-left
            for _ in 0..total_output_lines {
                let _ = stdout.execute(Clear(ClearType::CurrentLine));
                let _ = writeln!(stdout);
            }
            let _ = stdout.execute(MoveTo(0, 0)); // Move back to top-left

            // Render current state
            render_circle(&rendered_segments);

            // Render legend for currently displayed segments
            let _ = writeln!(stdout);
            for (idx, (label, _, percentage)) in rendered_segments.iter().enumerate() {
                let block = BLOCKS[idx % BLOCKS.len()];
                let color = COLORS[idx % COLORS.len()];
                let _ = writeln!(
                    stdout,
                    "  {} {}: {:.1}%",
                    block.repeat(2).color(owo_colors::XtermColors::from(color)),
                    label,
                    percentage
                );
            }
            let _ = stdout.flush();
            thread::sleep(delay_per_segment);
        }

        // Ensure final state is visible if animation finishes or is interrupted
        let _ = stdout.execute(Show);
        let _ = stdout.execute(MoveTo(0, 0));
        for _ in 0..total_output_lines {
            let _ = stdout.execute(Clear(ClearType::CurrentLine));
            let _ = writeln!(stdout);
        }
        let _ = stdout.execute(MoveTo(0, 0));
        self._render_static();
        let _ = stdout.flush();
        let _ = stdout.execute(Show);
    }
} // Close impl<'a> PieChart<'a>

fn render_circle(segments: &[(String, f64, f64)]) {
    let radius = 9.0;
    let center_x = 10.0;
    let center_y = 9.0;

    // Calculate cumulative angles for each segment
    let mut cumulative_angle = 0.0;
    let mut segment_angles: Vec<(f64, f64, usize)> = Vec::new();

    for (idx, (_, _, percentage)) in segments.iter().enumerate() {
        let angle_span = (percentage / 100.0) * 2.0 * PI;
        segment_angles.push((cumulative_angle, cumulative_angle + angle_span, idx));
        cumulative_angle += angle_span;
    }

    // Render the circle grid
    for y in 0..18 {
        for x in 0..20 {
            let dx = x as f64 - center_x;
            let dy = (y as f64 - center_y) * 2.0; // Adjust for character aspect ratio
            let distance = (dx * dx + dy * dy).sqrt();

            if distance <= radius {
                // Calculate angle from center
                let angle = dy.atan2(dx) + PI; // Normalize to 0..2π

                // Find which segment this point belongs to
                let segment_idx = segment_angles
                    .iter()
                    .find(|(start, end, _)| angle >= *start && angle < *end)
                    .map(|(_, _, idx)| *idx)
                    .unwrap_or(segment_angles.last().unwrap().2);

                let block = BLOCKS[segment_idx % BLOCKS.len()];
                let color = COLORS[segment_idx % COLORS.len()];
                print!("{}", block.color(owo_colors::XtermColors::from(color)));
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn parse_data(data: &str) -> Vec<(String, f64)> {
    data.split(',')
        .filter_map(|entry| {
            let parts: Vec<&str> = entry.trim().split(':').collect();
            if parts.len() != 2 {
                eprintln!("Warning: Invalid entry '{}' (expected Label:Value)", entry);
                return None;
            }

            let label = parts[0].trim().to_string();
            match parts[1].trim().parse::<f64>() {
                Ok(value) if value >= 0.0 => Some((label, value)),
                Ok(_) => {
                    eprintln!("Warning: Negative value for '{}' ignored", label);
                    None
                }
                Err(_) => {
                    eprintln!("Warning: Invalid number '{}' for '{}'", parts[1], label);
                    None
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_data_valid() {
        let data = "A:30,B:50,C:20";
        let result = parse_data(data);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], ("A".to_string(), 30.0));
        assert_eq!(result[1], ("B".to_string(), 50.0));
        assert_eq!(result[2], ("C".to_string(), 20.0));
    }

    #[test]
    fn test_parse_data_decimals() {
        let data = "A:10.5,B:20.75";
        let result = parse_data(data);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], ("A".to_string(), 10.5));
        assert_eq!(result[1], ("B".to_string(), 20.75));
    }

    #[test]
    fn test_parse_data_whitespace() {
        let data = " A : 30 , B : 50 ";
        let result = parse_data(data);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], ("A".to_string(), 30.0));
    }

    #[test]
    fn test_parse_data_invalid_entries() {
        let data = "A:30,Invalid,B:abc,C:20";
        let result = parse_data(data);
        assert_eq!(result.len(), 2); // Only A and C
        assert_eq!(result[0], ("A".to_string(), 30.0));
        assert_eq!(result[1], ("C".to_string(), 20.0));
    }

    #[test]
    fn test_parse_data_negative_values() {
        let data = "A:10,B:-5,C:20";
        let result = parse_data(data);
        assert_eq!(result.len(), 2); // B ignored
        assert_eq!(result[0], ("A".to_string(), 10.0));
        assert_eq!(result[1], ("C".to_string(), 20.0));
    }

    #[test]
    fn test_parse_data_empty() {
        let data = "";
        let result = parse_data(data);
        assert_eq!(result.len(), 0);
    }
}
