//! termgfx - Beautiful terminal graphics
//!
//! This library provides styled terminal output for CLI applications.
//! It can be compiled to WebAssembly for browser-based demos.

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

// Re-export modules for CLI usage
#[cfg(feature = "cli")]
pub mod output;
#[cfg(feature = "cli")]
pub mod charts;
#[cfg(feature = "cli")]
pub mod image;
#[cfg(feature = "cli")]
pub mod interactive;
#[cfg(feature = "cli")]
pub mod script;

// ============================================================================
// WASM Bindings - HTML Output for Browser
// ============================================================================

#[cfg(feature = "wasm")]
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Render a styled box with message (HTML output)
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn render_box(message: &str, style: &str, border: &str) -> String {
    let (color_class, emoji) = match style {
        "success" => ("green", "✓"),
        "warning" => ("yellow", "⚠"),
        "danger" => ("red", "✗"),
        "gradient" => ("magenta", "✨"),
        _ => ("blue", "ℹ"),
    };

    let (tl, tr, bl, br, h, v) = match border {
        "single" => ("┌", "┐", "└", "┘", "─", "│"),
        "double" => ("╔", "╗", "╚", "╝", "═", "║"),
        "thick" => ("┏", "┓", "┗", "┛", "━", "┃"),
        _ => ("╭", "╮", "╰", "╯", "─", "│"),
    };

    let content = format!("{} {}", emoji, message);
    let width = content.chars().count() + 4;
    let h_line = h.repeat(width);

    format!(
        "<span class=\"{}\">{}{}{}</span>\n<span class=\"{}\">{}</span>  {}  <span class=\"{}\">{}</span>\n<span class=\"{}\">{}{}{}</span>",
        color_class, tl, h_line, tr,
        color_class, v, content, color_class, v,
        color_class, bl, h_line, br
    )
}

/// Render an ASCII banner (HTML output)
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn render_banner(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut lines = vec![String::new(); 4];

    for ch in chars {
        let pattern = get_ascii_char(ch);
        for (i, line) in pattern.iter().enumerate() {
            lines[i].push_str(line);
            lines[i].push(' ');
        }
    }

    let width = lines[0].len() + 2;
    let border = "═".repeat(width);

    format!(
        "<span class=\"cyan\">╔{}╗</span>\n<span class=\"cyan\">║</span> <span class=\"bright-white\">{}</span> <span class=\"magenta\">║</span>\n<span class=\"cyan\">║</span> <span class=\"bright-white\">{}</span> <span class=\"magenta\">║</span>\n<span class=\"cyan\">║</span> <span class=\"bright-white\">{}</span> <span class=\"magenta\">║</span>\n<span class=\"cyan\">║</span> <span class=\"bright-white\">{}</span> <span class=\"magenta\">║</span>\n<span class=\"magenta\">╚{}╝</span>",
        border, lines[0], lines[1], lines[2], lines[3], border
    )
}

fn get_ascii_char(ch: char) -> [&'static str; 4] {
    match ch.to_ascii_uppercase() {
        'A' => [" ██ ", "█  █", "████", "█  █"],
        'B' => ["███ ", "█  █", "███ ", "████"],
        'C' => [" ███", "█   ", "█   ", " ███"],
        'D' => ["███ ", "█  █", "█  █", "███ "],
        'E' => ["████", "███ ", "█   ", "████"],
        'F' => ["████", "███ ", "█   ", "█   "],
        'G' => [" ███", "█   ", "█ ██", " ███"],
        'H' => ["█  █", "████", "█  █", "█  █"],
        'I' => ["████", " ██ ", " ██ ", "████"],
        'J' => ["████", "  █ ", "█ █ ", " █  "],
        'K' => ["█ █ ", "██  ", "█ █ ", "█  █"],
        'L' => ["█   ", "█   ", "█   ", "████"],
        'M' => ["█  █", "████", "█  █", "█  █"],
        'N' => ["█  █", "██ █", "█ ██", "█  █"],
        'O' => [" ██ ", "█  █", "█  █", " ██ "],
        'P' => ["███ ", "█  █", "███ ", "█   "],
        'Q' => [" ██ ", "█  █", "█ █ ", " █ █"],
        'R' => ["███ ", "█  █", "███ ", "█  █"],
        'S' => [" ███", "██  ", "  ██", "███ "],
        'T' => ["████", " ██ ", " ██ ", " ██ "],
        'U' => ["█  █", "█  █", "█  █", " ██ "],
        'V' => ["█  █", "█  █", " ██ ", " ██ "],
        'W' => ["█  █", "█  █", "████", "█  █"],
        'X' => ["█  █", " ██ ", " ██ ", "█  █"],
        'Y' => ["█  █", " ██ ", " ██ ", " ██ "],
        'Z' => ["████", "  █ ", " █  ", "████"],
        '0' => [" ██ ", "█ ██", "██ █", " ██ "],
        '1' => [" █  ", "██  ", " █  ", "███ "],
        '2' => ["██  ", "  █ ", " █  ", "████"],
        '3' => ["███ ", " ██ ", "  ██", "███ "],
        '4' => ["█ █ ", "█ █ ", "████", "  █ "],
        '5' => ["████", "██  ", "  ██", "██  "],
        '6' => [" ██ ", "█   ", "███ ", " ██ "],
        '7' => ["████", "  █ ", " █  ", "█   "],
        '8' => [" ██ ", "████", "█  █", " ██ "],
        '9' => [" ██ ", "███ ", "  █ ", " ██ "],
        ' ' => ["    ", "    ", "    ", "    "],
        _ => ["    ", "    ", "    ", "    "],
    }
}

/// Render a progress bar (HTML output)
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn render_progress(percent: u8, style: &str) -> String {
    let width = 30;
    let filled = (percent as usize * width) / 100;
    let empty = width - filled;

    match style {
        "blocks" => {
            let bar = "<span class=\"cyan\">█</span>".repeat(filled);
            let empty_bar = "<span class=\"dim\">░</span>".repeat(empty);
            format!("{}{} <span class=\"cyan bold\">{}%</span>", bar, empty_bar, percent)
        }
        "dots" => {
            let bar = "<span class=\"cyan\">●</span>".repeat(filled);
            let empty_bar = "<span class=\"dim\">○</span>".repeat(empty);
            format!("{}{} <span class=\"cyan bold\">{}%</span>", bar, empty_bar, percent)
        }
        "animated" => {
            let pattern = ['█', '▓', '▒', '░'];
            let bar: String = (0..filled)
                .map(|i| format!("<span class=\"cyan\">{}</span>", pattern[i % pattern.len()]))
                .collect();
            let empty_bar = "<span class=\"dim\"> </span>".repeat(empty);
            format!("{}{} <span class=\"cyan bold\">{}%</span>", bar, empty_bar, percent)
        }
        _ => {
            // Gradient: red -> yellow -> green
            let bar: String = (0..filled)
                .map(|i| {
                    let pos = (i * 100) / width;
                    if pos < 35 {
                        "<span class=\"red\">█</span>"
                    } else if pos < 70 {
                        "<span class=\"yellow\">█</span>"
                    } else {
                        "<span class=\"green\">█</span>"
                    }
                })
                .collect();
            let empty_bar = "<span class=\"dim\">░</span>".repeat(empty);
            let pct_class = if percent < 35 {
                "red"
            } else if percent < 70 {
                "yellow"
            } else {
                "green"
            };
            format!("{}{} <span class=\"{} bold\">{}%</span>", bar, empty_bar, pct_class, percent)
        }
    }
}

/// Render a sparkline (plain text - no colors needed)
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn render_sparkline(data: &str) -> String {
    let values: Vec<f64> = data
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    if values.is_empty() {
        return String::new();
    }

    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = max - min;

    let blocks = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

    let sparkline: String = values
        .iter()
        .map(|&v| {
            let normalized = if range > 0.0 {
                ((v - min) / range * 7.0) as usize
            } else {
                4
            };
            blocks[normalized.min(7)]
        })
        .collect();

    format!("<span class=\"cyan\">{}</span>", sparkline)
}

/// Render a table (HTML output)
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn render_table(headers: &str, rows: &str, border: &str) -> String {
    let headers: Vec<&str> = headers.split(',').collect();
    let rows: Vec<Vec<&str>> = rows
        .split('|')
        .map(|row| row.split(',').collect())
        .collect();

    // Calculate column widths
    let mut widths: Vec<usize> = headers.iter().map(|h| h.chars().count()).collect();
    for row in &rows {
        for (i, cell) in row.iter().enumerate() {
            if i < widths.len() {
                let cell_width = cell.chars().count();
                if cell_width > widths[i] {
                    widths[i] = cell_width;
                }
            }
        }
    }

    let (tl, tr, bl, br, h, v, cross, t_down, t_up, t_right, t_left) = match border {
        "double" => ("╔", "╗", "╚", "╝", "═", "║", "╬", "╦", "╩", "╠", "╣"),
        "heavy" => ("┏", "┓", "┗", "┛", "━", "┃", "╋", "┳", "┻", "┣", "┫"),
        "single" => ("┌", "┐", "└", "┘", "─", "│", "┼", "┬", "┴", "├", "┤"),
        _ => ("╭", "╮", "╰", "╯", "─", "│", "┼", "┬", "┴", "├", "┤"),
    };

    let mut output = String::new();

    // Top border
    output.push_str(&format!("<span class=\"dim\">{}</span>", tl));
    for (i, w) in widths.iter().enumerate() {
        output.push_str(&format!("<span class=\"dim\">{}</span>", h.repeat(w + 2)));
        if i < widths.len() - 1 {
            output.push_str(&format!("<span class=\"dim\">{}</span>", t_down));
        }
    }
    output.push_str(&format!("<span class=\"dim\">{}</span>\n", tr));

    // Headers
    output.push_str(&format!("<span class=\"dim\">{}</span>", v));
    for (i, header) in headers.iter().enumerate() {
        let padding = widths[i] - header.chars().count();
        output.push_str(&format!(" <span class=\"cyan bold\">{}</span>{} ", header, " ".repeat(padding)));
        output.push_str(&format!("<span class=\"dim\">{}</span>", v));
    }
    output.push('\n');

    // Header separator
    output.push_str(&format!("<span class=\"dim\">{}</span>", t_right));
    for (i, w) in widths.iter().enumerate() {
        output.push_str(&format!("<span class=\"dim\">{}</span>", h.repeat(w + 2)));
        if i < widths.len() - 1 {
            output.push_str(&format!("<span class=\"dim\">{}</span>", cross));
        }
    }
    output.push_str(&format!("<span class=\"dim\">{}</span>\n", t_left));

    // Rows
    for (row_idx, row) in rows.iter().enumerate() {
        output.push_str(&format!("<span class=\"dim\">{}</span>", v));
        let row_class = if row_idx % 2 == 1 { "dim" } else { "" };
        for (i, cell) in row.iter().enumerate() {
            if i < widths.len() {
                let padding = widths[i] - cell.chars().count();
                if row_class.is_empty() {
                    output.push_str(&format!(" {}{} ", cell, " ".repeat(padding)));
                } else {
                    output.push_str(&format!(" <span class=\"{}\">{}</span>{} ", row_class, cell, " ".repeat(padding)));
                }
                output.push_str(&format!("<span class=\"dim\">{}</span>", v));
            }
        }
        output.push('\n');
    }

    // Bottom border
    output.push_str(&format!("<span class=\"dim\">{}</span>", bl));
    for (i, w) in widths.iter().enumerate() {
        output.push_str(&format!("<span class=\"dim\">{}</span>", h.repeat(w + 2)));
        if i < widths.len() - 1 {
            output.push_str(&format!("<span class=\"dim\">{}</span>", t_up));
        }
    }
    output.push_str(&format!("<span class=\"dim\">{}</span>", br));

    output
}

/// Typewriter effect - returns array of partial strings for animation
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn typewriter_frames(message: &str) -> Vec<JsValue> {
    message
        .chars()
        .scan(String::new(), |state, ch| {
            state.push(ch);
            Some(JsValue::from_str(state))
        })
        .collect()
}

/// Render a tree structure from JSON (HTML output)
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn render_tree(json: &str) -> String {
    match serde_json::from_str::<serde_json::Value>(json) {
        Ok(value) => render_tree_value(&value, "", 0),
        Err(_) => "<span class=\"red\">Error: Invalid JSON</span>".to_string(),
    }
}

#[cfg(feature = "wasm")]
fn render_tree_value(value: &serde_json::Value, prefix: &str, depth: usize) -> String {
    let colors = ["cyan", "blue", "green", "yellow", "magenta", "bright-blue"];
    let color = colors[depth % colors.len()];

    let mut output = String::new();

    match value {
        serde_json::Value::Object(map) => {
            let entries: Vec<_> = map.iter().collect();
            for (i, (key, val)) in entries.iter().enumerate() {
                let is_last_item = i == entries.len() - 1;
                let connector = if is_last_item { "└── " } else { "├── " };
                let icon = if val.is_object() { "📁" } else if val.is_array() { "📦" } else { "📄" };

                output.push_str(&format!(
                    "{}<span class=\"{}\">{}</span>{} <span class=\"bold\">{}</span>\n",
                    prefix, color, connector, icon, key
                ));

                let new_prefix = format!("{}{}",
                    prefix,
                    if is_last_item { "    " } else { "<span class=\"dim\">│</span>   " }
                );
                output.push_str(&render_tree_value(val, &new_prefix, depth + 1));
            }
        }
        serde_json::Value::Array(arr) => {
            for (i, val) in arr.iter().enumerate() {
                let is_last_item = i == arr.len() - 1;
                let connector = if is_last_item { "└── " } else { "├── " };

                if val.is_string() {
                    output.push_str(&format!(
                        "{}<span class=\"{}\">{}</span><span class=\"green\">{}</span>\n",
                        prefix, color, connector, val.as_str().unwrap()
                    ));
                } else {
                    output.push_str(&format!(
                        "{}<span class=\"{}\">{}</span>\n",
                        prefix, color, connector
                    ));
                    let new_prefix = format!("{}{}",
                        prefix,
                        if is_last_item { "    " } else { "<span class=\"dim\">│</span>   " }
                    );
                    output.push_str(&render_tree_value(val, &new_prefix, depth + 1));
                }
            }
        }
        _ => {}
    }

    output
}
