use owo_colors::OwoColorize;
use serde_json::Value;
use std::io::{self, Read, Write};
use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub enum BorderStyle {
    Single,
    Double,
    Rounded,
    None,
}

impl BorderStyle {
    fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "double" => BorderStyle::Double,
            "rounded" => BorderStyle::Rounded,
            "none" => BorderStyle::None,
            _ => BorderStyle::Single,
        }
    }

    fn chars(&self) -> BorderChars {
        match self {
            BorderStyle::Single => BorderChars {
                top_left: "┌",
                top_right: "┐",
                bottom_left: "└",
                bottom_right: "┘",
                horizontal: "─",
                vertical: "│",
                cross: "┼",
                t_down: "┬",
                t_up: "┴",
                t_right: "├",
                t_left: "┤",
            },
            BorderStyle::Double => BorderChars {
                top_left: "╔",
                top_right: "╗",
                bottom_left: "╚",
                bottom_right: "╝",
                horizontal: "═",
                vertical: "║",
                cross: "╬",
                t_down: "╦",
                t_up: "╩",
                t_right: "╠",
                t_left: "╣",
            },
            BorderStyle::Rounded => BorderChars {
                top_left: "╭",
                top_right: "╮",
                bottom_left: "╰",
                bottom_right: "╯",
                horizontal: "─",
                vertical: "│",
                cross: "┼",
                t_down: "┬",
                t_up: "┴",
                t_right: "├",
                t_left: "┤",
            },
            BorderStyle::None => BorderChars {
                top_left: " ",
                top_right: " ",
                bottom_left: " ",
                bottom_right: " ",
                horizontal: " ",
                vertical: " ",
                cross: " ",
                t_down: " ",
                t_up: " ",
                t_right: " ",
                t_left: " ",
            },
        }
    }
}

struct BorderChars {
    top_left: &'static str,
    top_right: &'static str,
    bottom_left: &'static str,
    bottom_right: &'static str,
    horizontal: &'static str,
    vertical: &'static str,
    cross: &'static str,
    t_down: &'static str,
    t_up: &'static str,
    t_right: &'static str,
    t_left: &'static str,
}

#[derive(Clone)]
pub enum Alignment {
    Left,
    Right,
    Center,
}

impl Alignment {
    fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "right" => Alignment::Right,
            "center" => Alignment::Center,
            _ => Alignment::Left,
        }
    }

    fn align(&self, text: &str, width: usize) -> String {
        let text_width = unicode_width::UnicodeWidthStr::width(text);
        if text_width >= width {
            return text.to_string();
        }

        let padding = width - text_width;
        match self {
            Alignment::Left => format!("{}{}", text, " ".repeat(padding)),
            Alignment::Right => format!("{}{}", " ".repeat(padding), text),
            Alignment::Center => {
                let left_pad = padding / 2;
                let right_pad = padding - left_pad;
                format!("{}{}{}", " ".repeat(left_pad), text, " ".repeat(right_pad))
            }
        }
    }
}

pub struct TableOptions {
    pub border: BorderStyle,
    pub alignment: Alignment,
    pub header_color: bool,
    pub row_striping: bool,
    pub max_width: Option<usize>,
    pub animate: bool,
}

impl Default for TableOptions {
    fn default() -> Self {
        Self {
            border: BorderStyle::Single,
            alignment: Alignment::Left,
            header_color: true,
            row_striping: true,
            max_width: None,
            animate: false,
        }
    }
}

pub fn render(
    headers_str: Option<&str>,
    rows_str: Option<&str>,
    file: Option<&str>,
    border: &str,
    alignment: &str,
) {
    render_animated(headers_str, rows_str, file, border, alignment, false);
}

/// Render table with optional animation
pub fn render_animated(
    headers_str: Option<&str>,
    rows_str: Option<&str>,
    file: Option<&str>,
    border: &str,
    alignment: &str,
    animate: bool,
) {
    let border_style = BorderStyle::from_str(border);
    let align = Alignment::from_str(alignment);
    let options = TableOptions {
        border: border_style,
        alignment: align,
        animate,
        ..Default::default()
    };

    // Try to get data from different sources
    let (headers, rows) = if let (Some(h), Some(r)) = (headers_str, rows_str) {
        // Inline data via --headers and --rows
        parse_inline_data(h, r)
    } else if let Some(filepath) = file {
        // From file (CSV)
        parse_csv_file(filepath)
    } else {
        // From stdin (JSON)
        parse_json_stdin()
    };

    if headers.is_empty() {
        eprintln!("Error: No data to display");
        return;
    }

    render_table(&headers, &rows, &options);
}

fn parse_inline_data(headers_str: &str, rows_str: &str) -> (Vec<String>, Vec<Vec<String>>) {
    let headers: Vec<String> = headers_str.split(',').map(|h| h.trim().to_string()).collect();

    let rows: Vec<Vec<String>> = rows_str
        .split('|')
        .map(|row| row.split(',').map(|cell| cell.trim().to_string()).collect())
        .collect();

    (headers, rows)
}

fn parse_csv_file(filepath: &str) -> (Vec<String>, Vec<Vec<String>>) {
    let content = match std::fs::read_to_string(filepath) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return (vec![], vec![]);
        }
    };

    let mut lines = content.lines();
    let headers: Vec<String> = lines
        .next()
        .map(|line| line.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();

    let rows: Vec<Vec<String>> = lines
        .map(|line| line.split(',').map(|s| s.trim().to_string()).collect())
        .collect();

    (headers, rows)
}

fn parse_json_stdin() -> (Vec<String>, Vec<Vec<String>>) {
    let mut buffer = String::new();
    if io::stdin().read_to_string(&mut buffer).is_err() {
        eprintln!("Error reading from stdin");
        return (vec![], vec![]);
    }

    let json: Value = match serde_json::from_str(&buffer) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
            return (vec![], vec![]);
        }
    };

    let array = match json.as_array() {
        Some(arr) => arr,
        None => {
            eprintln!("Error: JSON must be an array of objects");
            return (vec![], vec![]);
        }
    };

    if array.is_empty() {
        return (vec![], vec![]);
    }

    // Extract headers from first object
    let first_obj = match array[0].as_object() {
        Some(obj) => obj,
        None => {
            eprintln!("Error: Array elements must be objects");
            return (vec![], vec![]);
        }
    };

    let headers: Vec<String> = first_obj.keys().map(|k| k.to_string()).collect();

    // Extract rows
    let rows: Vec<Vec<String>> = array
        .iter()
        .filter_map(|item| {
            item.as_object().map(|obj| {
                headers
                    .iter()
                    .map(|key| {
                        obj.get(key)
                            .and_then(|v| match v {
                                Value::String(s) => Some(s.clone()),
                                Value::Number(n) => Some(n.to_string()),
                                Value::Bool(b) => Some(b.to_string()),
                                Value::Null => Some("null".to_string()),
                                _ => None,
                            })
                            .unwrap_or_default()
                    })
                    .collect()
            })
        })
        .collect();

    (headers, rows)
}

fn render_table(headers: &[String], rows: &[Vec<String>], options: &TableOptions) {
    let border_chars = options.border.chars();

    // Calculate column widths
    let mut col_widths: Vec<usize> = headers
        .iter()
        .map(|h| unicode_width::UnicodeWidthStr::width(h.as_str()))
        .collect();

    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            if i < col_widths.len() {
                let width = unicode_width::UnicodeWidthStr::width(cell.as_str());
                col_widths[i] = col_widths[i].max(width);
            }
        }
    }

    // Apply max width if specified
    if let Some(max) = options.max_width {
        col_widths.iter_mut().for_each(|w| *w = (*w).min(max));
    }

    // Top border
    print_border_line(&col_widths, &border_chars, BorderLineType::Top);

    // Headers
    print!("{}", border_chars.vertical);
    for (i, header) in headers.iter().enumerate() {
        let width = col_widths.get(i).copied().unwrap_or(0);
        let truncated = truncate(header, width);
        let aligned = options.alignment.align(&truncated, width);

        if options.header_color {
            print!(" {} ", aligned.bright_cyan().bold());
        } else {
            print!(" {} ", aligned);
        }
        print!("{}", border_chars.vertical);
    }
    println!();

    // Header separator
    print_border_line(&col_widths, &border_chars, BorderLineType::Middle);

    let delay = if options.animate { Duration::from_millis(50) } else { Duration::ZERO };
    let mut stdout = io::stdout();

    // Rows
    for (row_idx, row) in rows.iter().enumerate() {
        print!("{}", border_chars.vertical);
        for (i, cell) in row.iter().enumerate() {
            let width = col_widths.get(i).copied().unwrap_or(0);
            let truncated = truncate(cell, width);
            let aligned = options.alignment.align(&truncated, width);

            if options.row_striping && row_idx % 2 == 1 {
                print!(" {} ", aligned.truecolor(180, 180, 180));
            } else {
                print!(" {} ", aligned);
            }
            print!("{}", border_chars.vertical);
        }
        println!();

        if options.animate {
            stdout.flush().unwrap();
            thread::sleep(delay);
        }
    }

    // Bottom border
    print_border_line(&col_widths, &border_chars, BorderLineType::Bottom);
}

enum BorderLineType {
    Top,
    Middle,
    Bottom,
}

fn print_border_line(col_widths: &[usize], chars: &BorderChars, line_type: BorderLineType) {
    match line_type {
        BorderLineType::Top => {
            print!("{}", chars.top_left);
            for (i, width) in col_widths.iter().enumerate() {
                print!("{}", chars.horizontal.repeat(width + 2));
                if i < col_widths.len() - 1 {
                    print!("{}", chars.t_down);
                }
            }
            println!("{}", chars.top_right);
        }
        BorderLineType::Middle => {
            print!("{}", chars.t_right);
            for (i, width) in col_widths.iter().enumerate() {
                print!("{}", chars.horizontal.repeat(width + 2));
                if i < col_widths.len() - 1 {
                    print!("{}", chars.cross);
                }
            }
            println!("{}", chars.t_left);
        }
        BorderLineType::Bottom => {
            print!("{}", chars.bottom_left);
            for (i, width) in col_widths.iter().enumerate() {
                print!("{}", chars.horizontal.repeat(width + 2));
                if i < col_widths.len() - 1 {
                    print!("{}", chars.t_up);
                }
            }
            println!("{}", chars.bottom_right);
        }
    }
}

fn truncate(text: &str, max_width: usize) -> String {
    let width = unicode_width::UnicodeWidthStr::width(text);
    if width <= max_width {
        return text.to_string();
    }

    if max_width <= 3 {
        return "...".chars().take(max_width).collect();
    }

    let mut result = String::new();
    let mut current_width = 0;

    for ch in text.chars() {
        let ch_width = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0);
        if current_width + ch_width + 3 > max_width {
            break;
        }
        result.push(ch);
        current_width += ch_width;
    }

    result.push_str("...");
    result
}
