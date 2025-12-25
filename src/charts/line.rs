use owo_colors::OwoColorize;
use crossterm::{
    cursor::{Hide, Show, MoveToColumn, MoveTo},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::{stdout, IsTerminal, Write};
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const BRAILLE_OFFSET: u32 = 0x2800;
const HEIGHT: usize = 10;

pub struct LineChart<'a> {
    data: &'a str,
    title: Option<&'a str>,
    animate: bool,
    animation_time_ms: u64,
}

impl<'a> LineChart<'a> {
    pub fn new(data: &'a str, title: Option<&'a str>, animate: bool, animation_time_ms: u64) -> Self {
        Self {
            data,
            title,
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
        let values: Vec<f64> = self.data
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        if values.is_empty() {
            eprintln!("Error: No valid data points provided");
            return;
        }

        if let Some(title_text) = self.title {
            println!("{}", title_text.bright_cyan().bold());
            println!();
        }

        let max_val = values.iter().cloned().fold(f64::MIN, f64::max);
        let min_val = values.iter().cloned().fold(f64::MAX, f64::min);
        let range = if (max_val - min_val).abs() < f64::EPSILON { 1.0 } else { max_val - min_val };

        let width = values.len() * 2;
        let mut canvas = vec![vec![0u8; width]; HEIGHT * 4];

        for (i, &val) in values.iter().enumerate() {
            let normalized = ((val - min_val) / range).clamp(0.0, 1.0);
            let y = ((HEIGHT * 4 - 1) as f64 * normalized) as usize;
            let x = i * 2;

            if x < width && y < HEIGHT * 4 {
                canvas[HEIGHT * 4 - 1 - y][x] = 1;
            }

            if i > 0 {
                let prev_val = values[i - 1];
                let prev_normalized = ((prev_val - min_val) / range).clamp(0.0, 1.0);
                let prev_y = ((HEIGHT * 4 - 1) as f64 * prev_normalized) as usize;

                let y_start = prev_y.min(y);
                let y_end = prev_y.max(y);

                for y_pos in y_start..=y_end {
                    if y_pos < HEIGHT * 4 {
                        let x_interp = i * 2 - 1;
                        if x_interp < width {
                            canvas[HEIGHT * 4 - 1 - y_pos][x_interp] = 1;
                        }
                    }
                }
            }
        }

        let max_label_width = format!("{:.1}", max_val).len();

        for row in 0..HEIGHT {
            let y_value = max_val - (row as f64 / (HEIGHT - 1) as f64) * range;
            let label = format!("{:>width$.1}", y_value, width = max_label_width);
            print!("{} ", label.bright_black());

            let mut line = String::new();
            for col in (0..width).step_by(2) {
                let mut dots: u32 = 0;

                for dy in 0..4 {
                    let y = row * 4 + dy;
                    if y < HEIGHT * 4 {
                        for dx in 0..2 {
                            let x = col + dx;
                            if x < width && canvas[y][x] == 1 {
                                let dot_index = dy + dx * 4;
                                dots |= 1 << dot_index;
                            }
                        }
                    }
                }

                let braille_char = char::from_u32(BRAILLE_OFFSET + dots).unwrap_or(' ');
                line.push(braille_char);
            }

            println!("{}", line.bright_green());
        }

        let axis_line = " ".repeat(max_label_width + 1) + &"─".repeat(width / 2);
        println!("{}", axis_line.bright_black());
    }

    fn _render_animated(&self) {
        let values: Vec<f64> = self.data
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        if values.is_empty() {
            eprintln!("Error: No valid data points provided");
            return;
        }

        if !stdout().is_terminal() {
            self._render_static();
            return;
        }

        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        let _ = ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        });

        let mut stdout = stdout();

        // Hide cursor
        let _ = stdout.execute(Hide);

        let max_val = values.iter().cloned().fold(f64::MIN, f64::max);
        let min_val = values.iter().cloned().fold(f64::MAX, f64::min);
        let range = if (max_val - min_val).abs() < f64::EPSILON { 1.0 } else { max_val - min_val };

        let width = values.len() * 2;
        let mut current_canvas = vec![vec![0u8; width]; HEIGHT * 4];

        let total_elements = values.len();
        let delay_per_element = if total_elements > 0 {
            Duration::from_millis(self.animation_time_ms / total_elements as u64)
        } else {
            Duration::from_millis(0)
        };

        let max_label_width = format!("{:.1}", max_val).len();
        let num_lines_before_chart = if self.title.is_some() { 2 } else { 0 };

        for i in 0..total_elements {
            if !running.load(Ordering::SeqCst) {
                break;
            }

            // Update canvas with new point
            let val = values[i];
            let normalized = ((val - min_val) / range).clamp(0.0, 1.0);
            let y = ((HEIGHT * 4 - 1) as f64 * normalized) as usize;
            let x = i * 2;

            if x < width && y < HEIGHT * 4 {
                current_canvas[HEIGHT * 4 - 1 - y][x] = 1;
            }

            if i > 0 {
                let prev_val = values[i - 1];
                let prev_normalized = ((prev_val - min_val) / range).clamp(0.0, 1.0);
                let prev_y = ((HEIGHT * 4 - 1) as f64 * prev_normalized) as usize;

                let y_start = prev_y.min(y);
                let y_end = prev_y.max(y);

                for y_pos in y_start..=y_end {
                    if y_pos < HEIGHT * 4 {
                        let x_interp = i * 2 - 1;
                        if x_interp < width {
                            current_canvas[HEIGHT * 4 - 1 - y_pos][x_interp] = 1;
                        }
                    }
                }
            }

            // Clear previous chart drawing and redraw
            let _ = stdout.execute(MoveTo(0, num_lines_before_chart as u16));
            for _ in 0..(HEIGHT + 1) { // Clear chart area + axis line
                let _ = stdout.execute(Clear(ClearType::CurrentLine));
                let _ = writeln!(stdout);
            }
            let _ = stdout.execute(MoveTo(0, num_lines_before_chart as u16)); // Move back up

            if let Some(title_text) = self.title {
                let _ = writeln!(stdout, "{}
", title_text.bright_cyan().bold());
            }

            for row in 0..HEIGHT {
                let y_value = max_val - (row as f64 / (HEIGHT - 1) as f64) * range;
                let label = format!("{:>width$.1}", y_value, width = max_label_width);
                let _ = write!(stdout, "{} ", label.bright_black());

                let mut line = String::new();
                for col in (0..width).step_by(2) {
                    let mut dots: u32 = 0;

                    for dy in 0..4 {
                        let y_canvas = row * 4 + dy;
                        if y_canvas < HEIGHT * 4 {
                            for dx in 0..2 {
                                let x_canvas = col + dx;
                                if x_canvas < width && current_canvas[y_canvas][x_canvas] == 1 {
                                    let dot_index = dy + dx * 4;
                                    dots |= 1 << dot_index;
                                }
                            }
                        }
                    }

                    let braille_char = char::from_u32(BRAILLE_OFFSET + dots).unwrap_or(' ');
                    line.push(braille_char);
                }

                let _ = writeln!(stdout, "{}", line.bright_green());
            }

            let axis_line = " ".repeat(max_label_width + 1) + &"─".repeat(width / 2);
            let _ = writeln!(stdout, "{}", axis_line.bright_black());
            let _ = stdout.flush();
            thread::sleep(delay_per_element);
        }

        // Final render to ensure the complete chart is displayed if animation finishes or is interrupted
        let _ = stdout.execute(Show);
        let _ = stdout.execute(MoveToColumn(0));
        for _ in 0..(HEIGHT + 1 + num_lines_before_chart) {
            let _ = stdout.execute(Clear(ClearType::CurrentLine));
            let _ = writeln!(stdout);
        }
        let _ = stdout.execute(MoveTo(0, num_lines_before_chart as u16));
        self._render_static();
        let _ = stdout.flush();
        let _ = stdout.execute(Show);
    }
}