use owo_colors::OwoColorize;

const BRAILLE_OFFSET: u32 = 0x2800;
const HEIGHT: usize = 10;

pub fn render(data: &str, title: Option<&str>) {
    let values: Vec<f64> = data
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    if values.is_empty() {
        eprintln!("Error: No valid data points provided");
        return;
    }

    if let Some(title_text) = title {
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
