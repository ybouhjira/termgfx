use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize)]
struct DashboardConfig {
    layout: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    panels: Vec<Panel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Panel {
    #[serde(rename = "type")]
    panel_type: String,
    content: String,
}

struct Layout {
    rows: usize,
    cols: usize,
}

/// Border characters for different styles
struct BorderStyle {
    horizontal: char,
    vertical: char,
    top_left: char,
    top_right: char,
    bottom_left: char,
    bottom_right: char,
    cross: char,
    t_down: char,
    t_up: char,
    t_right: char,
    t_left: char,
}

impl BorderStyle {
    fn from_name(name: &str) -> Self {
        match name {
            "double" => BorderStyle {
                horizontal: '═',
                vertical: '║',
                top_left: '╔',
                top_right: '╗',
                bottom_left: '╚',
                bottom_right: '╝',
                cross: '╬',
                t_down: '╦',
                t_up: '╩',
                t_right: '╠',
                t_left: '╣',
            },
            "rounded" => BorderStyle {
                horizontal: '─',
                vertical: '│',
                top_left: '╭',
                top_right: '╮',
                bottom_left: '╰',
                bottom_right: '╯',
                cross: '┼',
                t_down: '┬',
                t_up: '┴',
                t_right: '├',
                t_left: '┤',
            },
            _ => BorderStyle { // "single" or default
                horizontal: '─',
                vertical: '│',
                top_left: '┌',
                top_right: '┐',
                bottom_left: '└',
                bottom_right: '┘',
                cross: '┼',
                t_down: '┬',
                t_up: '┴',
                t_right: '├',
                t_left: '┤',
            },
        }
    }
}

/// Parse layout string like "2x2" into rows and cols
fn parse_layout(layout_str: &str) -> Result<Layout, String> {
    let parts: Vec<&str> = layout_str.split('x').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid layout format: '{}'. Expected format: NxM (e.g., 2x2, 3x1)", layout_str));
    }

    let rows = parts[0].parse::<usize>()
        .map_err(|_| format!("Invalid row count in layout: '{}'", parts[0]))?;
    let cols = parts[1].parse::<usize>()
        .map_err(|_| format!("Invalid column count in layout: '{}'", parts[1]))?;

    if rows == 0 || cols == 0 {
        return Err("Layout dimensions must be greater than 0".to_string());
    }

    Ok(Layout { rows, cols })
}

/// Parse panels string like "box:Hello,progress:75,sparkline:1,2,3"
/// Special handling: sparkline content can contain commas
fn parse_panels(panels_str: &str) -> Result<Vec<Panel>, String> {
    let mut panels = Vec::new();
    let mut current_pos = 0;
    let chars: Vec<char> = panels_str.chars().collect();

    while current_pos < chars.len() {
        // Find the panel type by looking for ':'
        let mut colon_pos = current_pos;
        while colon_pos < chars.len() && chars[colon_pos] != ':' {
            colon_pos += 1;
        }

        if colon_pos >= chars.len() {
            return Err(format!("Invalid panel definition at position {}: missing ':'", current_pos));
        }

        let panel_type: String = chars[current_pos..colon_pos].iter().collect::<String>().trim().to_string();

        // Validate panel type
        if !matches!(panel_type.as_str(), "box" | "progress" | "sparkline" | "gauge" | "text") {
            return Err(format!("Unknown panel type: '{}'. Valid types: box, progress, sparkline, gauge, text", panel_type));
        }

        // Find content end: next panel type or end of string
        let content_start = colon_pos + 1;
        let mut content_end = content_start;

        // For sparkline, we need to find the next "type:" pattern, not just comma
        if panel_type == "sparkline" {
            // Look for next panel type pattern (word followed by colon)
            let mut i = content_start;
            while i < chars.len() {
                // Check if this might be the start of a new panel
                if i > content_start && chars[i] == ',' {
                    // Check if the next part looks like "type:"
                    let mut j = i + 1;
                    while j < chars.len() && chars[j].is_whitespace() {
                        j += 1;
                    }
                    let mut word_end = j;
                    while word_end < chars.len() && chars[word_end].is_alphanumeric() {
                        word_end += 1;
                    }
                    if word_end < chars.len() && chars[word_end] == ':' {
                        let next_type: String = chars[j..word_end].iter().collect();
                        if matches!(next_type.as_str(), "box" | "progress" | "sparkline" | "gauge" | "text") {
                            content_end = i;
                            break;
                        }
                    }
                }
                i += 1;
            }
            if content_end == content_start {
                content_end = chars.len();
            }
        } else {
            // For other types, find next comma or end
            while content_end < chars.len() && chars[content_end] != ',' {
                content_end += 1;
            }
        }

        let content: String = chars[content_start..content_end].iter().collect::<String>().trim().to_string();

        panels.push(Panel {
            panel_type,
            content,
        });

        // Move to next panel (skip comma if present)
        current_pos = content_end;
        if current_pos < chars.len() && chars[current_pos] == ',' {
            current_pos += 1;
        }
    }

    Ok(panels)
}

/// Load dashboard configuration from JSON file
fn load_config(config_path: &str) -> Result<DashboardConfig, String> {
    let content = fs::read_to_string(config_path)
        .map_err(|e| format!("Failed to read config file '{}': {}", config_path, e))?;

    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse config file: {}", e))
}

/// Render a single panel content
fn render_panel_content(panel: &Panel, width: usize, height: usize) -> Vec<String> {
    let mut lines = Vec::new();

    match panel.panel_type.as_str() {
        "box" => {
            // Centered text in box
            let text = &panel.content;
            let padding = (height.saturating_sub(1)) / 2;

            for _ in 0..padding {
                lines.push(" ".repeat(width));
            }

            let text_padding = if width > text.len() {
                (width - text.len()) / 2
            } else {
                0
            };

            let mut line = " ".repeat(text_padding);
            line.push_str(text);
            line.push_str(&" ".repeat(width.saturating_sub(line.len())));
            lines.push(line);

            while lines.len() < height {
                lines.push(" ".repeat(width));
            }
        }
        "progress" => {
            let percent: u8 = panel.content.parse().unwrap_or(0).min(100);
            let bar_width = width.saturating_sub(6); // Reserve space for "XX% "
            let filled = (bar_width * percent as usize) / 100;

            let padding = (height.saturating_sub(1)) / 2;
            for _ in 0..padding {
                lines.push(" ".repeat(width));
            }

            let mut bar = String::new();
            bar.push_str(&format!("{:>3}% ", percent));
            bar.push_str(&"█".repeat(filled));
            bar.push_str(&"░".repeat(bar_width.saturating_sub(filled)));
            bar.push_str(&" ".repeat(width.saturating_sub(bar.len())));
            lines.push(bar);

            while lines.len() < height {
                lines.push(" ".repeat(width));
            }
        }
        "sparkline" => {
            let values: Vec<f64> = panel.content
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();

            let padding = (height.saturating_sub(1)) / 2;
            for _ in 0..padding {
                lines.push(" ".repeat(width));
            }

            let spark = generate_sparkline(&values, width);
            lines.push(format!("{:width$}", spark, width = width));

            while lines.len() < height {
                lines.push(" ".repeat(width));
            }
        }
        "gauge" => {
            let percent: u8 = panel.content.parse().unwrap_or(0).min(100);

            let padding = (height.saturating_sub(1)) / 2;
            for _ in 0..padding {
                lines.push(" ".repeat(width));
            }

            let gauge_text = format!("{}%", percent);
            let text_padding = if width > gauge_text.len() {
                (width - gauge_text.len()) / 2
            } else {
                0
            };

            let mut line = " ".repeat(text_padding);
            line.push_str(&gauge_text);
            line.push_str(&" ".repeat(width.saturating_sub(line.len())));
            lines.push(line);

            while lines.len() < height {
                lines.push(" ".repeat(width));
            }
        }
        "text" => {
            // Word-wrapped text
            let words: Vec<&str> = panel.content.split_whitespace().collect();
            let mut current_line = String::new();

            for word in words {
                if current_line.len() + word.len() + 1 <= width {
                    if !current_line.is_empty() {
                        current_line.push(' ');
                    }
                    current_line.push_str(word);
                } else {
                    if !current_line.is_empty() {
                        lines.push(format!("{:width$}", current_line, width = width));
                        current_line = String::new();
                    }
                    current_line.push_str(word);
                }
            }

            if !current_line.is_empty() {
                lines.push(format!("{:width$}", current_line, width = width));
            }

            while lines.len() < height {
                lines.push(" ".repeat(width));
            }
        }
        _ => {
            // Fallback for unknown types
            for _ in 0..height {
                lines.push(" ".repeat(width));
            }
        }
    }

    lines.truncate(height);
    lines
}

/// Generate sparkline from values
fn generate_sparkline(values: &[f64], max_width: usize) -> String {
    if values.is_empty() {
        return String::new();
    }

    let chars = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    let max_val = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min_val = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let range = max_val - min_val;

    let mut result = String::new();
    let display_values: Vec<f64> = if values.len() > max_width {
        // Sample values to fit width
        values.iter().step_by(values.len() / max_width).cloned().collect()
    } else {
        values.to_vec()
    };

    for &val in &display_values {
        if result.len() >= max_width {
            break;
        }

        let normalized = if range == 0.0 {
            0.5
        } else {
            (val - min_val) / range
        };

        let idx = (normalized * (chars.len() - 1) as f64).round() as usize;
        result.push(chars[idx.min(chars.len() - 1)]);
    }

    result
}

/// Render the complete dashboard
pub fn render(
    layout_str: &str,
    title: Option<&str>,
    panels_str: Option<&str>,
    config_path: Option<&str>,
    border_style: &str,
) {
    // Load configuration
    let mut config = if let Some(path) = config_path {
        match load_config(path) {
            Ok(cfg) => cfg,
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        DashboardConfig {
            layout: layout_str.to_string(),
            title: title.map(|s| s.to_string()),
            panels: Vec::new(),
        }
    };

    // Override config with CLI arguments
    if let Some(panels) = panels_str {
        match parse_panels(panels) {
            Ok(parsed_panels) => config.panels = parsed_panels,
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
    if title.is_some() {
        config.title = title.map(|s| s.to_string());
    }

    // Parse layout
    let layout = match parse_layout(&config.layout) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // Validate panel count
    let expected_panels = layout.rows * layout.cols;
    if config.panels.len() != expected_panels {
        eprintln!(
            "Error: Expected {} panels for layout {}, but got {}",
            expected_panels,
            config.layout,
            config.panels.len()
        );
        std::process::exit(1);
    }

    let border = BorderStyle::from_name(border_style);

    // Calculate dimensions
    let term_width = 80; // Default terminal width
    let panel_width = (term_width / layout.cols).saturating_sub(2); // -2 for borders
    let panel_height = 5; // Fixed height per panel

    // Render title if present
    if let Some(ref title_text) = config.title {
        let title_line = format!(" {} ", title_text);
        let padding = (term_width.saturating_sub(title_line.len())) / 2;
        println!("{}{}", " ".repeat(padding), title_line);
        println!();
    }

    // Render dashboard
    let mut output = String::new();

    // Top border
    output.push(border.top_left);
    for col in 0..layout.cols {
        output.push_str(&border.horizontal.to_string().repeat(panel_width));
        if col < layout.cols - 1 {
            output.push(border.t_down);
        }
    }
    output.push(border.top_right);
    output.push('\n');

    // Render rows
    for row in 0..layout.rows {
        // Render panel content lines
        for line_idx in 0..panel_height {
            output.push(border.vertical);
            for col in 0..layout.cols {
                let panel_idx = row * layout.cols + col;
                let panel = &config.panels[panel_idx];
                let content_lines = render_panel_content(panel, panel_width, panel_height);
                let line = content_lines.get(line_idx).cloned().unwrap_or_else(|| " ".repeat(panel_width));
                output.push_str(&line);
                output.push(border.vertical);
            }
            output.push('\n');
        }

        // Middle border (between rows)
        if row < layout.rows - 1 {
            output.push(border.t_right);
            for col in 0..layout.cols {
                output.push_str(&border.horizontal.to_string().repeat(panel_width));
                if col < layout.cols - 1 {
                    output.push(border.cross);
                }
            }
            output.push(border.t_left);
            output.push('\n');
        }
    }

    // Bottom border
    output.push(border.bottom_left);
    for col in 0..layout.cols {
        output.push_str(&border.horizontal.to_string().repeat(panel_width));
        if col < layout.cols - 1 {
            output.push(border.t_up);
        }
    }
    output.push(border.bottom_right);
    output.push('\n');

    // Write to stdout
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(output.as_bytes()).unwrap();
    handle.flush().unwrap();
}
